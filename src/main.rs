pub mod color;
pub mod dimensions;
pub mod gpu;
pub mod pixels;
mod synced;

use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    keyboard::{Key, NamedKey},
    window::WindowBuilder,
};

fn main() {
    env_logger::init(); // Enable logging from WGPU
    let event_loop = EventLoop::new().expect("should build a loop");
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    event_loop
        .run(move |event, target| {
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
