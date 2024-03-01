#![allow(dead_code)]

use crate::app::*;
use crate::color::*;
use crate::dimensions::*;
use crate::gpu::*;
use crate::options::*;

use ctrlc;
use std::sync;
use std::time;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget},
};

pub struct Window {
    /// The GPU device that this window is using.
    pub gpu: Gpu,
    /// Includes the pixels and other things for drawing to the window.
    // Note `pixels` is nested in here so that we can borrow `window.gpu`
    // and `window.bvmt.binds()` for drawing to the shader.  If rust ever
    // supports partial borrowing (https://github.com/rust-lang/rfcs/issues/1215)
    // then we could implement `Globals` for `Window`, or at least create
    // a helper function `window.pixels()` that grabs `window.bvmt.pixels`
    // without borrowing from the entire window.
    pub bvmt: WindowGlobals,
    /// Shader for drawing pixels to the window.
    shader: Shader<DefaultVertexVariables, DefaultFragmentVariables, WindowGlobals>,
    vertices: Vertices<DefaultVertexVariables>,
    fragments: Fragments<DefaultFragmentVariables>,
    /// Desired amount of time between frames.
    desired_frame_duration: time::Duration,
    last_frame_instant: time::Instant,
    /// Amount of time we used to wait in the last frame.
    last_frame_wait: time::Duration,
    // Keep `window` above `surface` to ensure the window is always
    // in scope for the surface.
    winit_window: winit::window::Window,
    surface: Surface,
    ctrlc_receiver: sync::mpsc::Receiver<()>,
}

impl Window {
    pub fn default_resolution() -> Size2i {
        Size2i::new(960, 512)
    }

    // TODO: add `shake` method that will erratically bounce the main window.bvmt
    // vertex coordinates when drawing to the window surface.
    // ACTUALLY - don't add shake here, it makes more sense copying texture to texture.
    // i.e., we want to support drawing to pixels outside the window so when shaking
    // we get some of those pixels.  e.g., a 2 pixel border

    pub fn set_fps(&mut self, fps: f64) {
        self.set_frame_duration(time::Duration::from_nanos(
            (1_000_000_000.0 / fps).floor() as u64
        ));
    }

    pub fn set_frame_duration(&mut self, new_frame_duration: time::Duration) {
        self.desired_frame_duration = new_frame_duration;
        // TODO: update self.last_frame_wait to make this calculation smoother.
    }

    pub(crate) fn id(&self) -> winit::window::WindowId {
        self.winit_window.id()
    }

    async fn new_with_loop() -> (Self, EventLoop<()>) {
        let (ctrlc_sender, ctrlc_receiver) = sync::mpsc::channel();
        ctrlc::set_handler(move || ctrlc_sender.send(()).expect("should send signal"))
            .expect("should set Ctrl-C handler");

        let event_loop = EventLoop::new().expect("should build a loop");
        let winit_window = winit::window::WindowBuilder::new()
            .build(&event_loop)
            .unwrap();

        let wgpu = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        // The winit window needs to be in scope longer than this surface,
        // but that should be the case since Window holds both.
        let wgpu_surface = unsafe { wgpu.create_surface(&winit_window) }.unwrap();

        let gpu_adapter = wgpu
            .request_adapter(&wgpu::RequestAdapterOptions {
                compatible_surface: Some(&wgpu_surface),
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
        let mut gpu = Gpu::new(device, queue);

        let surface_capabilities = wgpu_surface.get_capabilities(&gpu_adapter);
        let surface_format = surface_capabilities
            .formats
            .iter()
            .copied()
            .find(|f| *f == wgpu::TextureFormat::Rgba8Unorm)
            .unwrap_or(wgpu::TextureFormat::Bgra8Unorm); // guaranteed

        let size = winit_window.inner_size();
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_capabilities.present_modes[0],
            alpha_mode: surface_capabilities.alpha_modes[0],
            view_formats: vec![],
        };
        let mut surface = Surface {
            wgpu_surface,
            config,
        };
        surface.reconfigure(&mut gpu);

        let initial_frame_duration = time::Duration::from_secs(1);
        let shader = Shader::default();
        (
            Self {
                gpu,
                desired_frame_duration: initial_frame_duration,
                last_frame_wait: initial_frame_duration,
                last_frame_instant: time::Instant::now(),
                bvmt: WindowGlobals::default(),
                shader,
                vertices: Vertices::new(vec![]),
                fragments: Fragments::new(DefaultFragmentVariables {}, vec![]),
                winit_window,
                surface,
                ctrlc_receiver,
            },
            event_loop,
        )
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let mut surface_texture = self.surface.wgpu_surface.get_current_texture()?;

        // Technically we need the pixels *for this frame* but the pixels will be
        // updated before other GPU commands are run with `gpu.queue.submit()` later.
        // TODO: verify
        self.bvmt
            .pixels
            .ensure_up_to_date_on_gpu(&mut self.gpu, NeedIt::Later);

        let scene = Scene {
            background: self.bvmt.background,
        };
        scene.draw_on_texture(
            &mut self.gpu,
            &mut surface_texture.texture,
            Some("window"),
            |drawer| {
                /* TODO
                drawer.shade(
                    &mut self.shader,
                    &mut self.vertices,
                    &mut self.fragments,
                    &self.bvmt,
                );
                */
            },
        );

        surface_texture.present();

        Ok(())
    }

    // TODO: call this on Ctrl+Z -> Resume so time doesn't go crazy
    //       -> may need to switch to signal-hook instead of ctrlc crate.
    // https://docs.rs/signal-hook/latest/signal_hook/#a-complex-signal-handling-with-a-background-thread
    fn reset_frame_wait(&mut self) -> time::Duration {
        let new_instant = time::Instant::now();
        self.last_frame_instant = new_instant;
        self.last_frame_wait = self.desired_frame_duration;
        self.desired_frame_duration
    }

    /// Returns the time to wait for this frame so that the desired_frame_duration
    /// is reached.
    fn update_frame_wait(&mut self) -> time::Duration {
        let new_instant = time::Instant::now();
        let actual_frame_duration = new_instant.duration_since(self.last_frame_instant);
        self.last_frame_instant = new_instant;
        // last frame:
        //      work_duration + last_frame_wait = actual_frame_duration
        // this frame, assume work_duration is the same:
        //      work_duration + this_frame_wait = desired_frame_duration
        // solve for this_frame_wait and update:
        // NOTE: we also have to be careful for overflow when subtracting durations,
        // so convert to nanos in i64 and be careful putting them back into a duration.
        let delta_nanos =
            self.desired_frame_duration.as_nanos() as i64 - actual_frame_duration.as_nanos() as i64;
        let this_frame_wait_nanos = self.last_frame_wait.as_nanos() as i64 + delta_nanos;
        if this_frame_wait_nanos < 0 {
            eprint!("probably dropping frames\n");
            self.last_frame_wait = time::Duration::from_nanos(0);
        } else {
            self.last_frame_wait = time::Duration::from_nanos(this_frame_wait_nanos as u64);
        }
        self.last_frame_wait
    }
}

// TODO: `globals` macro for generating the struct plus `Variables` and `Globals` traits.
pub struct WindowGlobals {
    /// Pixels that will be sent to the GPU every frame.  Note that
    /// these will *not* automatically stay in sync with the window size;
    /// `window.bvmt.pixels.size()` corresponds to the app's display resolution.
    /// In case of any mismatch, these pixels are scaled and centered in
    /// the available inner area of the window, maintaining aspect ratio.
    pub pixels: Pixels,
    /// Background color, in case of letterboxing with pixels.
    pub background: Color,
    /// Corner of the screen where the pixels will start displaying.
    top_left: Vector2f,
    /// Corner of the screen where the pixels stop.
    bottom_right: Vector2f,
}

impl std::default::Default for WindowGlobals {
    fn default() -> Self {
        Self {
            pixels: Pixels::new(Window::default_resolution()),
            background: Color::red(234),
            // TODO: determine if we need to swap up/down here.
            top_left: Vector2f::new(-1.0, 1.0),
            bottom_right: Vector2f::new(1.0, -1.0),
        }
    }
}

impl Globals for WindowGlobals {
    fn binds<'a>(&'a self) -> Vec<Bind<'a>> {
        vec![
            Bind::Struct(
                0,
                UniformStruct {
                    name: "Globals",
                    // NOTE! `background` should *not* be in this list of binds.
                    // The background color is passed in a different way (via `Scene`)
                    // and doesn't need to be bound to the GPU like these values.
                    values: vec![
                        Value::Vector2f("top_left", &self.top_left),
                        Value::Vector2f("bottom_right", &self.bottom_right),
                    ],
                },
            ),
            Bind::Pixels(1, &self.pixels),
        ]
    }
}

struct Surface {
    wgpu_surface: wgpu::Surface,
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
        self.reconfigure(gpu);
        resized
    }

    fn reconfigure(&mut self, gpu: &mut Gpu) {
        self.wgpu_surface.configure(&gpu.device, &self.config);
    }
}

pub async fn run(mut app: Box<dyn App>) {
    env_logger::init(); // Enable logging from WGPU

    let (mut window, event_loop) = Window::new_with_loop().await;

    if handle_app_event(&mut app, AppEvent::Start, &mut window) {
        eprint!("App shut down immediately...?\n");
        return;
    }
    // TODO: window.render() doesn't help fix a weird blank window at the start of `cargo run`.
    event_loop
        .run(move |event: Event<()>, target| {
            if let Some(app_event) = handle_or_convert(event, &mut window, &target) {
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
        // This should only be called if `target.exit()` has already been called,
        // but for safety we'll ensure it happens here.
        return true;
    }
    loop {
        match app.handle(app_event, window) {
            AppPlease::Continue => {
                return false;
            }
            AppPlease::Terminate => {
                // Don't handle AppEvent::End here, that will occur when
                // `Event::LoopExiting` fires.
                return true;
            }
            AppPlease::Replace(new_app) => {
                // We do need to handle AppEvent::End here because this is the
                // last time we have a handle on the app before we replace it.
                let _ignored = app.handle(AppEvent::End, window);
                *app = new_app;
                app_event = AppEvent::Start;
            }
        }
    }
}

/// Converts a winit event into an app event, or handles it for you.
fn handle_or_convert(
    event: Event<()>,
    window: &mut Window,
    target: &EventLoopWindowTarget<()>,
) -> Option<AppEvent> {
    match event {
        Event::WindowEvent {
            event: WindowEvent::RedrawRequested,
            window_id,
        } if window_id == window.id() => {
            match window.render() {
                Ok(_) => {}
                Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                    window.surface.reconfigure(&mut window.gpu)
                }
                Err(wgpu::SurfaceError::OutOfMemory) => {
                    eprint!("wgpu surface out of memory!\n");
                    target.exit();
                }
                Err(wgpu::SurfaceError::Timeout) => {
                    eprint!("surface timeout\n");
                }
            }
            None
        }
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
        // NOTE! Only this case should be allowed to return `AppEvent::End`;
        // Use `target.exit()` in other cases where you want the loop to end.
        Event::LoopExiting => Some(AppEvent::End),
        Event::NewEvents(start_cause) => match start_cause {
            StartCause::Init { .. } => {
                target.set_control_flow(ControlFlow::wait_duration(window.reset_frame_wait()));
                None
            }
            StartCause::ResumeTimeReached { .. } => {
                let duration = window.update_frame_wait();
                target.set_control_flow(ControlFlow::wait_duration(duration));
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
                // Rely on the `Event::LoopExiting` to generate the `AppEvent::End`:
                target.exit();
            }
            None
        }
        _ => {
            eprint!("unhandled event: {:?}\n", event); // TODO: remove this eventually.
            None
        }
    }
}
