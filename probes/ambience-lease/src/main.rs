//! P3 of mere's spatial compute plan: the wing projection.
//!
//! The second consumer of a resident spatial buffer, and the one that
//! decides whether the lease is domain-neutral. Ambience motes live in
//! a padded-3D position buffer that explicit-regime kernels advance;
//! renderling draws them as instanced geometry under a real 3D camera,
//! reading this frame's positions with no readback between the compute
//! that wrote them and the draw that consumed them.
//!
//! What P2 could not test, this does: z is meaningful (the camera looks
//! down at an angle, so depth is visible truth rather than a reserved
//! lane), and the consumer has a *different memory model*. renderling
//! addresses geometry through a craballoc slab rather than through
//! buffers a caller binds, so the lease is read by an adapter kernel
//! that writes into renderling's own storage. That mismatch, and its
//! cost, is the finding this gate exists to produce.

mod lease;

use std::time::Instant;

use netrender::{
    Compositor, ExternalTextureComposite, ExternalTexturePlacement, NetrenderOptions,
    PresentedFrame, Renderer, Scene, SurfaceKey, TenantNeeds, WgpuHandles, boot_shared,
    create_netrender_instance,
};
use renderling::camera::Camera;
use renderling::context::{Context, RenderTarget};
use renderling::geometry::{Geometry, Vertex};
use renderling::glam::{Mat4, Vec3, Vec4};
use renderling::light::{AnalyticalLight, DirectionalLight, Lux};
use renderling::primitive::Primitive;
use renderling::stage::Stage;
use renderling::transform::Transform;

const SIZE: [u32; 2] = [1280, 720];
const MASTER_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Rgba8Unorm;
/// A `TransformDescriptor` is translation (3) + rotation (4) + scale
/// (3): ten u32 words in the slab, translation first. The publish
/// kernel writes those first three and never touches the rest.
const TRANSFORM_WORDS: u32 = 10;
const MOTES: u32 = 20_000;
const EXTENT: f32 = 120.0;
const FRAMES: u32 = 300;

fn main() {
    // One device for the ambience kernels, renderling, and netrender.
    // The mesh tenant's usual asks, declared rather than greedy: this
    // is not a JIT runtime, so it states what it needs.
    let handles = boot_shared(
        wgpu::Backends::all(),
        None,
        &TenantNeeds {
            optional_features: wgpu::Features::INDIRECT_FIRST_INSTANCE
                | wgpu::Features::MULTI_DRAW_INDIRECT_COUNT
                | wgpu::Features::VERTEX_WRITABLE_STORAGE
                | wgpu::Features::CLEAR_TEXTURE,
            label: Some("ambience-lease probe"),
            ..Default::default()
        },
    )
    .expect("one device for kernels, renderling, and netrender");
    println!("adapter: {}", handles.adapter.get_info().name);
    println!("motes: {MOTES}, extent {EXTENT}");

    let mut ambience = lease::Ambience::new(&handles.device, &handles.queue, MOTES, EXTENT);
    let lease = ambience.lease();
    // The consumer frames itself from the lease rather than from the
    // probe's constants: the camera's distance is the lease's extent,
    // and the adapter's stride assumption is checked against the
    // lease's declared stride instead of assumed.
    assert_eq!(
        lease.stride_bytes, 16,
        "the adapter kernel indexes vec4f; a different stride needs a different kernel"
    );
    let tenant = Tenant::new(&handles, SIZE, lease.count, lease.extent);

    // The lease's consumer-side truth: where renderling put the
    // transforms, and which buffer they live in. Both are read *from
    // renderling*, never assumed.
    let (slab, base) = tenant.slab_and_transform_base();
    println!("renderling slab: transforms begin at word {base}, stride {TRANSFORM_WORDS}");
    ambience.attach_slab(&slab);

    // Warm up: compile pipelines, let the cloud start moving.
    for _ in 0..30 {
        ambience.step(1.0 / 60.0, base, TRANSFORM_WORDS);
        tenant.draw();
    }

    let start = Instant::now();
    let mut worst = 0.0f64;
    for _ in 0..FRAMES {
        let frame = Instant::now();
        ambience.step(1.0 / 60.0, base, TRANSFORM_WORDS);
        tenant.draw();
        handles
            .device
            .poll(wgpu::PollType::wait_indefinitely())
            .expect("frame poll");
        let ms = frame.elapsed().as_secs_f64() * 1e3;
        worst = worst.max(ms);
    }
    let avg = start.elapsed().as_secs_f64() * 1e3 / FRAMES as f64;
    println!("wing projection: {FRAMES} frames, avg {avg:.2} ms (worst {worst:.2})");

    // The receipt that renderling drew *these* motes: read the resident
    // positions once (a diagnostic, outside the frame discipline), and
    // check the picture's lit pixels sit where the cloud projects.
    let positions = ambience.read_positions_once();
    let mut lo = [f32::MAX; 3];
    let mut hi = [f32::MIN; 3];
    for p in &positions {
        for axis in 0..3 {
            lo[axis] = lo[axis].min(p[axis]);
            hi[axis] = hi[axis].max(p[axis]);
        }
    }
    println!(
        "cloud bounds: x [{:.0}, {:.0}]  y [{:.0}, {:.0}]  z [{:.0}, {:.0}]",
        lo[0], hi[0], lo[1], hi[1], lo[2], hi[2]
    );
    // z must carry real extent, not sit in a plane: this is the axis a
    // 2D canvas never exercised, and the reason padded 3D was chosen
    // before any consumer needed it.
    let depth_span = hi[2] - lo[2];
    assert!(
        depth_span > EXTENT / 2.0,
        "the cloud is flat: depth span {depth_span:.1} is not carrying anything"
    );

    let composer = Composer::new(handles.clone(), SIZE);
    let chrome = Scene::new(SIZE[0], SIZE[1]);
    let master = composer.compose(&chrome, &tenant.view);
    let capture = composer.capture(&master);
    let path = std::path::Path::new("../../../testing/paredros/p3_ambience_lease.png");
    capture.write_png(path).expect("write receipt png");
    println!(
        "receipt: {} distinct colours, {:.1}% lit -> {}",
        capture.distinct,
        capture.lit * 100.0,
        path.display()
    );
    // Shaded 3D motes, so the distinct-colour guard is the right one
    // here (unlike mere's flat 2D graph, which needed coverage).
    assert!(
        capture.distinct > 64,
        "only {} colours: renderling drew nothing lit",
        capture.distinct
    );
    assert!(capture.lit > 0.01, "the cloud is not in frame");
}

/// The renderling half: one instanced mote mesh, N transforms, a camera
/// that looks at the cloud from an angle so depth reads.
struct Tenant {
    view: wgpu::TextureView,
    ctx: Context,
    stage: Stage,
    _camera: Camera,
    _sun: AnalyticalLight<DirectionalLight>,
    _motes: Vec<Primitive>,
    transforms: Vec<Transform>,
}

impl Tenant {
    fn new(handles: &WgpuHandles, size: [u32; 2], count: u32, extent: f32) -> Self {
        let target = handles.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("ambience tenant target"),
            size: wgpu::Extent3d {
                width: size[0],
                height: size[1],
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                | wgpu::TextureUsages::TEXTURE_BINDING
                | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[],
        });
        let view = target.create_view(&Default::default());

        let ctx = Context::new(
            RenderTarget::from(target),
            handles.adapter.clone(),
            handles.device.clone(),
            handles.queue.clone(),
        );
        let stage = ctx
            .new_stage()
            .with_background_color([0.02, 0.02, 0.04, 1.0]);

        let camera = stage.new_camera();
        let eye = Vec3::new(extent * 2.1, extent * 1.4, extent * 2.1);
        let projection = Mat4::perspective_rh(
            std::f32::consts::FRAC_PI_4,
            size[0] as f32 / size[1] as f32,
            1.0,
            extent * 12.0,
        );
        camera.set_projection_and_view(projection, Mat4::look_at_rh(eye, Vec3::ZERO, Vec3::Y));
        stage.use_camera(&camera);

        // A sun, so the motes are shaded by orientation rather than
        // flat-filled. This is what makes the picture a 2.5D/3D receipt
        // instead of coloured dots: the same buffer, drawn by a real
        // renderer doing real lighting.
        let sun = stage
            .new_directional_light()
            .with_direction(Vec3::new(-0.6, -1.0, -0.35).normalize());
        sun.set_color(Vec4::new(1.0, 0.96, 0.90, 1.0));
        sun.set_intensity(Lux::OUTDOOR_DIRECT_SUNLIGHT_HIGH);

        // One mote mesh, shared by every instance: a small tetra-ish
        // billboard-free blob, so the picture is honestly 3D geometry
        // rather than points.
        let vertices = stage.new_vertices(mote_mesh());

        // A material, because renderling shades through PBR: without one
        // the sun above lights nothing and the motes fill flat. This is
        // the difference between "coloured dots" and a 3D consumer.
        let material = stage
            .new_material()
            .with_albedo_factor(Vec4::new(0.62, 0.78, 1.0, 1.0));
        material.set_metallic_factor(0.0);
        material.set_roughness_factor(0.55);

        // Transforms first, in one unbroken pass, *then* the primitives.
        // Interleaving the two allocations puts a primitive descriptor
        // between every pair of transforms and the slab stride stops
        // being the transform's own size, which the adapter kernel
        // indexes by. The consumer's allocation order is part of the
        // contract, and this is the line that makes it so.
        let transforms: Vec<Transform> = (0..count).map(|_| stage.new_transform()).collect();
        let motes: Vec<Primitive> = transforms
            .iter()
            .map(|transform| {
                stage
                    .new_primitive()
                    .with_vertices(vertices.clone())
                    .with_material(&material)
                    .with_transform(transform)
            })
            .collect();

        Self {
            view,
            ctx,
            stage,
            _camera: camera,
            _sun: sun,
            _motes: motes,
            transforms,
        }
    }

    /// Where renderling actually put the transform descriptors, asked of
    /// renderling rather than assumed. Commit first: the slab buffer
    /// does not exist until the stage has staged its writes.
    fn slab_and_transform_base(&self) -> (wgpu::Buffer, u32) {
        let geometry: &Geometry = self.stage.as_ref();
        let slab = geometry.commit();
        let base = self
            .transforms
            .first()
            .expect("at least one mote")
            .id()
            .inner();
        // The allocator is expected to have laid them out contiguously;
        // the adapter kernel indexes by stride from the first, so a gap
        // would corrupt the publish. Check it rather than trust it.
        for (i, transform) in self.transforms.iter().enumerate() {
            assert_eq!(
                transform.id().inner(),
                base + i as u32 * TRANSFORM_WORDS,
                "transforms are not contiguous: the adapter's stride assumption fails"
            );
        }
        ((*slab).clone(), base)
    }

    fn draw(&self) {
        let frame = self.ctx.get_next_frame().expect("tenant frame");
        self.stage.render(&frame.view());
        frame.present();
    }
}

/// A small octahedron: cheap, closed, and shaded differently per face,
/// so lighting varies with orientation and the picture reads as solid.
fn mote_mesh() -> Vec<Vertex> {
    const R: f32 = 1.6;
    let top = Vec3::new(0.0, R, 0.0);
    let bottom = Vec3::new(0.0, -R, 0.0);
    let ring = [
        Vec3::new(R, 0.0, 0.0),
        Vec3::new(0.0, 0.0, R),
        Vec3::new(-R, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -R),
    ];
    let mut out = Vec::with_capacity(24);
    for i in 0..4 {
        let a = ring[i];
        let b = ring[(i + 1) % 4];
        for (apex, first, second) in [(top, a, b), (bottom, b, a)] {
            let normal = (first - apex).cross(second - apex).normalize_or_zero();
            let tint = 0.55 + 0.45 * normal.y.abs();
            for point in [apex, first, second] {
                out.push(
                    Vertex::default()
                        .with_position(point.to_array())
                        .with_normal(normal.to_array())
                        .with_color([0.60 * tint, 0.78 * tint, 1.0 * tint, 1.0]),
                );
            }
        }
    }
    out
}

/// The netrender half, the room-probe pattern trimmed to an offscreen
/// master plus a capture.
struct Composer {
    net: Renderer,
    size: [u32; 2],
}

impl Composer {
    fn new(handles: WgpuHandles, size: [u32; 2]) -> Self {
        let net = create_netrender_instance(
            handles,
            NetrenderOptions {
                tile_cache_size: Some(64),
                enable_vello: true,
                ..Default::default()
            },
        )
        .expect("netrender init");
        Self { net, size }
    }

    fn compose(&self, chrome: &Scene, tenant: &wgpu::TextureView) -> wgpu::Texture {
        let external = [ExternalTextureComposite::new(
            tenant,
            ExternalTexturePlacement::new([0.0, 0.0, self.size[0] as f32, self.size[1] as f32]),
        )
        .with_scene_op_boundary(0)];
        let mut grab = MasterGrab { master: None };
        self.net.render_with_compositor_and_external_textures(
            chrome,
            MASTER_FORMAT,
            &mut grab,
            netrender::peniko::Color::new([0.0, 0.0, 0.0, 0.0]),
            &external,
        );
        grab.master.expect("netrender presented no master")
    }

    fn capture(&self, master: &wgpu::Texture) -> Capture {
        let pixels = self
            .net
            .wgpu_device
            .read_rgba8_texture(master, self.size[0], self.size[1]);
        let mut seen = std::collections::HashSet::new();
        let background = [pixels[0], pixels[1], pixels[2]];
        let mut lit = 0usize;
        for pixel in pixels.chunks_exact(4) {
            seen.insert([pixel[0], pixel[1], pixel[2]]);
            if pixel[..3] != background {
                lit += 1;
            }
        }
        Capture {
            pixels,
            size: self.size,
            distinct: seen.len(),
            lit: lit as f32 / (self.size[0] * self.size[1]) as f32,
        }
    }
}

struct MasterGrab {
    master: Option<wgpu::Texture>,
}

impl Compositor for MasterGrab {
    fn declare_surface(&mut self, _key: SurfaceKey, _bounds: [f32; 4]) {}
    fn destroy_surface(&mut self, _key: SurfaceKey) {}
    fn present_frame(&mut self, frame: PresentedFrame<'_>) {
        self.master = Some(frame.master.clone());
    }
}

struct Capture {
    pixels: Vec<u8>,
    size: [u32; 2],
    distinct: usize,
    lit: f32,
}

impl Capture {
    fn write_png(&self, path: &std::path::Path) -> std::io::Result<()> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let file = std::fs::File::create(path)?;
        let mut encoder = png::Encoder::new(file, self.size[0], self.size[1]);
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);
        encoder
            .write_header()
            .and_then(|mut writer| writer.write_image_data(&self.pixels))
            .map_err(std::io::Error::other)
    }
}
