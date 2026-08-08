// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
// SPDX-License-Identifier: MPL-2.0

//! The picture: renderling as a tenant on netrender's device.
//!
//! One `wgpu::Device` serves both. The tenant draws the room into a texture
//! it owns; netrender renders the chrome into its master and composites that
//! texture in at scene-op boundary zero, so the chrome lands over the room
//! rather than under it. Nothing here creates a second device, which is the
//! whole point of the cohesion contract this gate is proving.

use mesocosm_render::geometry::Vertex as MeshVertex;
use netrender::{
    Compositor, ExternalTextureComposite, ExternalTexturePlacement, NetrenderOptions,
    PresentedFrame, Renderer, Scene, SurfaceKey, WgpuHandles, create_netrender_instance,
};
use renderling::camera::Camera;
use renderling::context::{Context, RenderTarget};
use renderling::geometry::Vertex;
use renderling::glam::{Mat4, Vec3};
use renderling::primitive::Primitive;
use renderling::stage::Stage;

/// The composed frame's size. Fixed, so the receipt is the same picture on
/// every machine.
pub const SIZE: [u32; 2] = [1280, 720];
/// The master texture's format. Non-srgb so a readback is already the bytes
/// a PNG wants.
pub const MASTER_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Rgba8Unorm;

/// Boots one device for both tenants.
///
/// The feature set is netrender's requirement plus what renderling asks for,
/// intersected with what the adapter actually has, so a thin adapter reports
/// a missing feature rather than failing at pipeline creation.
pub fn boot(instance: &wgpu::Instance, compatible: Option<&wgpu::Surface<'_>>) -> WgpuHandles {
    let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::HighPerformance,
        force_fallback_adapter: false,
        compatible_surface: compatible,
    }))
    .expect("no wgpu adapter");

    let features = (netrender::REQUIRED_FEATURES
        | wgpu::Features::INDIRECT_FIRST_INSTANCE
        | wgpu::Features::MULTI_DRAW_INDIRECT_COUNT
        | wgpu::Features::VERTEX_WRITABLE_STORAGE
        | wgpu::Features::CLEAR_TEXTURE)
        .intersection(adapter.features());
    assert!(
        adapter.features().contains(netrender::REQUIRED_FEATURES),
        "this adapter cannot host netrender: missing {:?}",
        netrender::REQUIRED_FEATURES - adapter.features()
    );

    let (device, queue) = pollster::block_on(adapter.request_device(&wgpu::DeviceDescriptor {
        label: Some("paredros room probe"),
        required_features: features,
        required_limits: wgpu::Limits {
            max_inter_stage_shader_variables: 28,
            ..Default::default()
        },
        ..Default::default()
    }))
    .expect("no wgpu device");

    WgpuHandles {
        instance: instance.clone(),
        adapter,
        device,
        queue,
    }
}

/// The renderling half: the room and the body, drawn into a texture netrender
/// will composite.
pub struct Tenant {
    pub view: wgpu::TextureView,
    size: [u32; 2],
    ctx: Context,
    stage: Stage,
    camera: Camera,
    room: Primitive,
    body: Primitive,
}

impl Tenant {
    pub fn new(handles: &WgpuHandles, size: [u32; 2]) -> Self {
        let target = handles.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("room tenant target"),
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
        // Underground dark, not black: the room reads as a place with air in
        // it rather than a cutout.
        let stage = ctx
            .new_stage()
            .with_background_color([0.03, 0.03, 0.045, 1.0]);
        let camera = stage.new_camera();
        stage.use_camera(&camera);
        let room = stage.new_primitive();
        let body = stage.new_primitive();

        Self {
            view,
            size,
            ctx,
            stage,
            camera,
            room,
            body,
        }
    }

    pub fn size(&self) -> [u32; 2] {
        self.size
    }

    pub fn set_room(&self, vertices: &[MeshVertex], eye: Vec3) {
        self.room
            .set_vertices(self.stage.new_vertices(shaded(vertices, eye)));
    }

    pub fn set_body(&self, vertices: &[MeshVertex], eye: Vec3) {
        self.body
            .set_vertices(self.stage.new_vertices(shaded(vertices, eye)));
    }

    pub fn look(&self, projection: Mat4, view: Mat4) {
        self.camera.set_projection_and_view(projection, view);
    }

    /// Draws one frame into the tenant texture.
    pub fn draw(&self) {
        let frame = self.ctx.get_next_frame().expect("tenant frame");
        self.stage.render(&frame.view());
        frame.present();
    }
}

/// Mesocosm's triangles as renderling vertices, with a per-face normal taken
/// from the winding and the torch applied per vertex.
///
/// The colour already carries the mesher's face shading. The torch is the
/// other half: greedy meshing merges a whole wall into one quad, and one
/// quad of one colour is a flat plane until something varies across it.
fn shaded(vertices: &[MeshVertex], eye: Vec3) -> Vec<Vertex> {
    let mut out = Vec::with_capacity(vertices.len());
    for triangle in vertices.chunks_exact(3) {
        let [a, b, c] = [
            triangle[0].position,
            triangle[1].position,
            triangle[2].position,
        ];
        let normal = (Vec3::from(b) - Vec3::from(a))
            .cross(Vec3::from(c) - Vec3::from(a))
            .normalize_or_zero()
            .to_array();
        for vertex in triangle {
            let lit = crate::scene::torch(eye, vertex.position);
            out.push(
                Vertex::default()
                    .with_position(vertex.position)
                    .with_normal(normal)
                    .with_color([
                        vertex.color[0] * lit,
                        vertex.color[1] * lit,
                        vertex.color[2] * lit,
                        1.0,
                    ]),
            );
        }
    }
    out
}

/// The netrender half: chrome into the master, tenant texture composited in
/// under it, and the master handed back.
pub struct Composer {
    pub net: Renderer,
    size: [u32; 2],
}

impl Composer {
    pub fn new(handles: WgpuHandles, size: [u32; 2]) -> Self {
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

    /// One composed master: chrome over room.
    pub fn compose(&self, chrome: &Scene, tenant: &wgpu::TextureView) -> wgpu::Texture {
        let external = [ExternalTextureComposite::new(
            tenant,
            ExternalTexturePlacement::new([0.0, 0.0, self.size[0] as f32, self.size[1] as f32]),
        )
        // Boundary zero: the room goes under every chrome op, so the bar
        // paints over the picture instead of being hidden by it.
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

    /// Blits the composed master onto a surface texture, filling it. The
    /// window may be any size; the receipt is always [`SIZE`].
    pub fn present(
        &self,
        master: &wgpu::Texture,
        target: &wgpu::TextureView,
        format: wgpu::TextureFormat,
        target_size: [u32; 2],
    ) {
        let view = master.create_view(&Default::default());
        self.net.compose_external_texture(
            &view,
            target,
            format,
            target_size[0],
            target_size[1],
            ExternalTexturePlacement::new([0.0, 0.0, target_size[0] as f32, target_size[1] as f32]),
        );
    }

    pub fn capture(&self, master: &wgpu::Texture) -> Capture {
        Capture::of(
            self.net
                .wgpu_device
                .read_rgba8_texture(master, self.size[0], self.size[1]),
            self.size,
        )
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

/// A read-back frame, with the one property that separates a receipt from a
/// blank: how many distinct colours are actually in it.
pub struct Capture {
    pub pixels: Vec<u8>,
    pub size: [u32; 2],
    pub distinct: usize,
}

impl Capture {
    fn of(pixels: Vec<u8>, size: [u32; 2]) -> Self {
        let mut seen = std::collections::HashSet::new();
        for pixel in pixels.chunks_exact(4) {
            seen.insert([pixel[0], pixel[1], pixel[2]]);
        }
        Self {
            pixels,
            size,
            distinct: seen.len(),
        }
    }

    /// A frame of one flat colour is a failure that a written file hides.
    /// The threshold is deliberately low: this catches "nothing rendered",
    /// not "the picture is ugly".
    pub fn is_trivial(&self) -> bool {
        self.distinct < 16
    }

    pub fn write_png(&self, path: &std::path::Path) -> std::io::Result<()> {
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
