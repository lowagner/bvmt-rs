#![allow(dead_code)]

use crate::color::*;

use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    keyboard::{Key, NamedKey},
};

struct Window {
    window: winit::window::Window,
    // TODO: add GPU and Pixels
    /// Pixels that will be sent to the GPU every frame.
    // pub pixels: Pixels,
    /// Background color, in case of letterboxing with pixels.
    pub background: Color,
}

impl Window {
    pub(crate) fn new_with_loop() -> (Self, EventLoop<()>) {
        let event_loop = EventLoop::new().expect("should build a loop");
        let window = winit::window::WindowBuilder::new()
            .build(&event_loop)
            .unwrap();

        (
            Self {
                background: Color::red(234),
                window,
            },
            event_loop,
        )
    }

    pub(crate) fn id(&self) -> winit::window::WindowId {
        self.window.id()
    }
}

// TODO: add an `App`(?) class and pass it in here as mutable.
//       App should have methods like: `handle(event)`, `draw(window)`;
//       maybe put `TimeElapsedEvent` into Event enum.
pub fn run() {
    env_logger::init(); // Enable logging from WGPU
    let (mut window, event_loop) = Window::new_with_loop();

    event_loop
        .run(move |event: Event<()>, target| {
            target.set_control_flow(ControlFlow::Wait);

            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    window_id,
                } if window_id == window.id() => {
                    target.exit();
                }
                Event::WindowEvent {
                    event: WindowEvent::KeyboardInput { event, .. },
                    window_id,
                } if window_id == window.id() => {
                    if event.logical_key == Key::Named(NamedKey::Escape) {
                        target.exit();
                    }
                }
                _ => (),
            }
        })
        .expect("should run loop ok");
}
