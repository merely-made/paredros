// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
// SPDX-License-Identifier: MPL-2.0

//! The headed room probe.
//!
//! A winit window presenting netrender's composed master: the renderling
//! room underneath, a vello chrome bar over it. With `ROOM_TRACE=1` the run
//! drives itself from the fixed trace, writes the screenshot receipt, prints
//! the hashes and the frame spans, and exits. Without it the window stays up
//! after the trace ends so a human can look at the room.

use std::path::PathBuf;
use std::sync::Arc;
use std::time::Instant;

use paredros_room::gpu::{self, Composer, SIZE, Tenant};
use paredros_room::room::SEED;
use paredros_room::{Probe, TICKS, scene};
use winit::application::ApplicationHandler;
use winit::dpi::PhysicalSize;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

/// Where the receipt lands. The screenshots layout is per-repo under
/// `Code/testing/<repo>/`.
const CAPTURE: &str = r"C:\Users\mark_\Code\testing\paredros\s0_room.png";

fn main() {
    let trace_mode = std::env::var("ROOM_TRACE").is_ok_and(|v| v != "0");
    let probe = Probe::new(SEED).expect("the room probe grows its world");
    println!(
        "room at {:?}, stance {:?}, ground hash {:#018x}",
        probe.room().centre,
        probe.at(),
        probe.ground_hash()
    );

    let event_loop = EventLoop::new().expect("winit event loop");
    event_loop.set_control_flow(ControlFlow::Poll);
    let mut app = RoomApp {
        instance: wgpu::Instance::new(
            wgpu::InstanceDescriptor::new_without_display_handle_from_env(),
        ),
        probe,
        trace_mode,
        live: None,
        frames: 0,
        captured: false,
    };
    event_loop.run_app(&mut app).expect("winit run");
}

struct Live {
    window: Arc<Window>,
    surface: wgpu::Surface<'static>,
    format: wgpu::TextureFormat,
    device: wgpu::Device,
    tenant: Tenant,
    composer: Composer,
    /// The room is meshed once. Only the torch changes, and the torch rides
    /// the eye, so the vertices are re-shaded per frame rather than remeshed.
    room: Vec<mesocosm_render::geometry::Vertex>,
}

struct RoomApp {
    instance: wgpu::Instance,
    probe: Probe,
    trace_mode: bool,
    live: Option<Live>,
    frames: u64,
    captured: bool,
}

impl ApplicationHandler for RoomApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.live.is_some() {
            return;
        }

        let attributes = Window::default_attributes()
            .with_title("Paredros S0: the room probe")
            .with_inner_size(PhysicalSize::new(SIZE[0], SIZE[1]));
        let window = Arc::new(event_loop.create_window(attributes).expect("window"));
        let surface = self
            .instance
            .create_surface(window.clone())
            .expect("surface");

        // One device for both tenants, chosen against the surface the window
        // actually got, so the picture and the presentation cannot end up on
        // different adapters.
        let handles = gpu::boot(&self.instance, Some(&surface));
        let device = handles.device.clone();
        let capabilities = surface.get_capabilities(&handles.adapter);
        // Prefer a linear format: the master is already encoded, and letting
        // the swapchain apply a second srgb transfer washes the room out.
        let format = capabilities
            .formats
            .iter()
            .copied()
            .find(|f| !f.is_srgb())
            .unwrap_or(capabilities.formats[0]);

        let tenant = Tenant::new(&handles, SIZE);
        let room = scene::room_vertices(self.probe.room());
        println!("room mesh: {} triangles", room.len() / 3);
        let composer = Composer::new(handles, SIZE);

        let mut live = Live {
            window,
            surface,
            format,
            device,
            tenant,
            composer,
            room,
        };
        configure(&mut live);
        live.window.request_redraw();
        self.live = Some(live);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(_) => {
                if let Some(live) = self.live.as_mut() {
                    configure(live);
                }
            }
            WindowEvent::RedrawRequested => self.frame(event_loop),
            _ => {}
        }
    }
}

impl RoomApp {
    fn frame(&mut self, event_loop: &ActiveEventLoop) {
        let Some(live) = self.live.as_mut() else {
            return;
        };
        let began = Instant::now();

        // One tick of the fixed trace per frame. The trace is the input;
        // there is no other.
        self.probe.advance();

        let size = live.window.inner_size();
        let aspect = SIZE[0] as f32 / SIZE[1] as f32;
        let camera = scene::camera(
            self.probe.room(),
            self.probe.at(),
            self.probe.heading(),
            aspect,
        );
        live.tenant.look(camera.projection, camera.view);
        live.tenant.set_room(&live.room, camera.eye);
        live.tenant
            .set_body(&scene::body_vertices(self.probe.at()), camera.eye);
        live.tenant.draw();

        let chrome = scene::chrome(SIZE, self.probe.tick_count(), TICKS);
        let master = live.composer.compose(&chrome, &live.tenant.view);

        use wgpu::CurrentSurfaceTexture as Acquired;
        match live.surface.get_current_texture() {
            Acquired::Success(frame) | Acquired::Suboptimal(frame) => {
                let target = frame.texture.create_view(&Default::default());
                live.composer.present(
                    &master,
                    &target,
                    live.format,
                    [size.width.max(1), size.height.max(1)],
                );
                live.window.pre_present_notify();
                frame.present();
            }
            Acquired::Outdated | Acquired::Lost => configure(live),
            Acquired::Timeout | Acquired::Occluded => {}
            Acquired::Validation => panic!("surface acquisition failed validation"),
        }

        let span = began.elapsed();
        self.frames += 1;

        if self.trace_mode && !self.probe.running() && !self.captured {
            self.captured = true;
            report(&live.composer, &master, &self.probe, span, self.frames);
            event_loop.exit();
            return;
        }
        if self.frames.is_multiple_of(60) {
            report_spans(&live.composer, span);
        }
        live.window.request_redraw();
    }
}

fn configure(live: &mut Live) {
    let size = live.window.inner_size();
    live.surface.configure(
        &live.device,
        &wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: live.format,
            width: size.width.max(1),
            height: size.height.max(1),
            present_mode: wgpu::PresentMode::AutoVsync,
            desired_maximum_frame_latency: 2,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            view_formats: vec![],
        },
    );
}

/// The receipt: the capture, its variety, the replay hash, and the spans.
fn report(
    composer: &Composer,
    master: &wgpu::Texture,
    probe: &Probe,
    span: std::time::Duration,
    frames: u64,
) {
    let capture = composer.capture(master);
    let path = PathBuf::from(CAPTURE);
    capture.write_png(&path).expect("write the receipt");
    assert!(
        !capture.is_trivial(),
        "the capture is one flat colour ({} distinct); nothing rendered",
        capture.distinct
    );

    println!("--- S0 room probe ---");
    println!("ticks: {} over {frames} frames", probe.tick_count());
    println!("final position: {:?}", probe.at());
    println!("position-log hash: {:#018x}", probe.hash());
    println!("ground hash: {:#018x}", probe.ground_hash());
    println!(
        "capture: {} ({}x{}, {} distinct colours)",
        path.display(),
        capture.size[0],
        capture.size[1],
        capture.distinct
    );
    report_spans(composer, span);
}

fn report_spans(composer: &Composer, span: std::time::Duration) {
    print!("frame span: probe_frame {:?}", span);
    match composer.net.last_frame_timings() {
        Some(timings) => {
            print!(" | netrender total {:?}", timings.total);
            for named in &timings.spans {
                print!(" | {} {:?}", named.name, named.duration);
            }
        }
        None => print!(" | netrender reported no timings"),
    }
    println!();
}
