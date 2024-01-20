#![allow(dead_code)]

use crate::app::*;
use crate::color::*;
use crate::dimensions::Size2i;
use crate::gpu::*;
use crate::pixels::*;

use ctrlc;
use std::sync;
use std::time;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
};

pub struct Window {
    /// The GPU device that this window is using.
    pub gpu: Gpu,
    /// Pixels that will be sent to the GPU every frame.  Note that
    /// these will *not* automatically stay in sync with the window size;
    /// `window.pixels.size()` corresponds to the app's display resolution.
    /// In case of any mismatch, these pixels are scaled and centered in
    /// the available inner area of the window, maintaining aspect ratio.
    pub pixels: Pixels,
    /// Background color, in case of letterboxing with pixels.
    pub background: Color,
    /// Desired frames per second.
    // TODO: Switch to Duration here and add `set_fps` convenience method
    pub fps: f64,
    last_instant: time::Instant,
    // Keep `window` above `surface` to ensure the window is always
    // in scope for the surface.
    window: winit::window::Window,
    surface: Surface,
    ctrlc_receiver: sync::mpsc::Receiver<()>,
}

impl Window {
    async fn new_with_loop() -> (Self, EventLoop<()>) {
        let (ctrlc_sender, ctrlc_receiver) = sync::mpsc::channel();
        ctrlc::set_handler(move || ctrlc_sender.send(()).expect("should send signal"))
            .expect("should set Ctrl-C handler");

        let event_loop = EventLoop::new().expect("should build a loop");
        let window = winit::window::WindowBuilder::new()
            .build(&event_loop)
            .unwrap();

        let wgpu = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        // The winit window needs to be in scope longer than this surface,
        // but that should be the case since Window holds both.
        let surface = unsafe { wgpu.create_surface(&window) }.unwrap();

        let gpu_adapter = wgpu
            .request_adapter(&wgpu::RequestAdapterOptions {
                compatible_surface: Some(&surface),
                // Don't force software rendering:
                force_fallback_adapter: false,
                power_preference: wgpu::PowerPreference::default(),
            })
            .await
            .unwrap();

        let (device, queue) = gpu_adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("window.gpu"),
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                },
                None,
            )
            .await
            .unwrap();
        let mut gpu = Gpu { device, queue };

        let surface_capabilities = surface.get_capabilities(&gpu_adapter);
        let surface_format = surface_capabilities
            .formats
            .iter()
            .copied()
            .find(|f| *f == wgpu::TextureFormat::Rgba8Unorm)
            .unwrap_or(wgpu::TextureFormat::Bgra8Unorm); // guaranteed
                                                         // TODO: update gpu.preferred_format here before creating gpu.pixels

        let size = window.inner_size();
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_capabilities.present_modes[0],
            alpha_mode: surface_capabilities.alpha_modes[0],
            view_formats: vec![],
        };
        surface.configure(&gpu.device, &config);
        let pixels = gpu.pixels(Window::default_resolution());

        (
            Self {
                gpu,
                pixels,
                background: Color::red(234),
                fps: 1.0,
                last_instant: time::Instant::now(),
                window,
                surface: Surface { surface, config },
                ctrlc_receiver,
            },
            event_loop,
        )
    }

    pub(crate) fn id(&self) -> winit::window::WindowId {
        self.window.id()
    }

    pub fn default_resolution() -> Size2i {
        Size2i::new(960, 512)
    }

    // TODO: call this on Ctrl+Z -> Resume so time doesn't go crazy
    fn update_instant(&mut self) -> time::Duration {
        let new_instant = time::Instant::now();
        let duration = new_instant.duration_since(self.last_instant);
        self.last_instant = new_instant;
        duration
    }
}

struct Surface {
    surface: wgpu::Surface,
    config: wgpu::SurfaceConfiguration,
}

impl Surface {
    fn resize(&mut self, gpu: &mut Gpu, new_size: winit::dpi::PhysicalSize<u32>) -> bool {
        if new_size.width == 0 || new_size.height == 0 {
            // Don't allow an invalid size.
            return false;
        }
        let resized = new_size.width != self.config.width || new_size.height != self.config.height;
        self.config.width = new_size.width;
        self.config.height = new_size.height;
        self.surface.configure(&gpu.device, &self.config);
        resized
    }
}

pub async fn run(mut app: Box<dyn App>) {
    env_logger::init(); // Enable logging from WGPU

    let (mut window, event_loop) = Window::new_with_loop().await;

    if handle_app_event(&mut app, AppEvent::Start, &mut window) {
        eprint!("App shut down immediately...\n");
        return;
    }

    event_loop
        .run(move |event: Event<()>, target| {
            // TODO: try to converge on FPS based on work-time + wait-time.
            target.set_control_flow(ControlFlow::wait_duration(fps_to_duration(window.fps)));
            if let Some(app_event) = handle_or_convert(event, &mut window) {
                if handle_app_event(&mut app, app_event, &mut window) {
                    target.exit();
                }
            }
        })
        .expect("should run loop ok");
}

/// Ensures that the App handles an AppEvent, including moving to a new App
/// if the App should be replaced.  Returns true iff we should stop running
/// the program entirely.
fn handle_app_event(app: &mut Box<dyn App>, mut app_event: AppEvent, window: &mut Window) -> bool {
    if app_event == AppEvent::End {
        // Don't let the App get away with changing how it behaves here.
        let _ignored = app.handle(AppEvent::End, window);
        return true;
    }
    loop {
        match app.handle(app_event, window) {
            AppPlease::Continue => {
                return false;
            }
            AppPlease::Terminate => {
                let _ignored = app.handle(AppEvent::End, window);
                return true;
            }
            AppPlease::Replace(new_app) => {
                let _ignored = app.handle(AppEvent::End, window);
                *app = new_app;
                app_event = AppEvent::Start;
            }
        }
    }
}

/// Converts a winit event into an app event, or handles it for you.
fn handle_or_convert(event: Event<()>, window: &mut Window) -> Option<AppEvent> {
    match event {
        Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            window_id,
        } if window_id == window.id() => Some(AppEvent::WindowCloseRequested),
        Event::WindowEvent {
            event: WindowEvent::KeyboardInput { event, .. },
            window_id,
        } if window_id == window.id() => Some(AppEvent::KeyInput(KeyInput {
            key: event.logical_key,
            state: event.state,
            repeating: event.repeat,
        })),
        Event::WindowEvent {
            event: WindowEvent::Resized(physical_size),
            window_id,
        } if window_id == window.id() => {
            window.surface.resize(&mut window.gpu, physical_size);
            None
        }
        Event::LoopExiting => Some(AppEvent::End),
        Event::NewEvents(start_cause) => match start_cause {
            StartCause::ResumeTimeReached { .. } => {
                let duration = window.update_instant();
                Some(AppEvent::TimeElapsed(duration))
            }
            other => {
                eprint!("unhandled new event: {:?}\n", other); // TODO: remove this eventually.
                None
            }
        },
        Event::AboutToWait => {
            // We'll handle random stuff here since this event is fired before
            // waiting for the next frame.
            if window.ctrlc_receiver.try_recv().is_ok() {
                Some(AppEvent::End)
            } else {
                None
            }
        }
        _ => {
            eprint!("unhandled event: {:?}\n", event); // TODO: remove this eventually.
            None
        }
    }
}

fn fps_to_duration(fps: f64) -> time::Duration {
    time::Duration::from_nanos((1_000_000_000.0 / fps).floor() as u64)
}
