#![allow(dead_code)]

use crate::window::*;

pub use winit::event::ElementState as InputState;
pub use winit::keyboard::{Key, NamedKey, NativeKey};

pub trait App {
    fn handle(&mut self, event: AppEvent, window: &mut Window) -> AppPlease;
    // TODO: is this even necessary? `fn draw(window: &mut Window);`
    //       we'll have logic for the GPU to automatically upscale the window.pixels
}

pub enum AppPlease {
    /// Let the current app keep running.
    Continue,
    /// The current app should terminate (usually ending the program).
    Terminate,
    /// The current app wants to be replaced with another app.
    Replace(Box<dyn App>),
}

// TODO: maybe put `TimeElapsedEvent` into Event enum.
pub enum AppEvent {
    WindowCloseRequested,
    KeyInput(KeyInput),
}

pub struct KeyInput {
    /// The logical key pressed, dependent on physical layout.
    pub key: Key,
    /// Whether pressed or released.
    pub state: InputState,
    /// Whether this event was due to an OS-level repeat.
    pub repeating: bool,
}

#[derive(Debug, Default)]
pub struct DefaultApp {}

impl App for DefaultApp {
    fn handle(&mut self, event: AppEvent, _window: &mut Window) -> AppPlease {
        match event {
            AppEvent::WindowCloseRequested => AppPlease::Terminate,
            AppEvent::KeyInput(key_input) => {
                if key_input.key == Key::Named(NamedKey::Escape) {
                    AppPlease::Terminate
                } else {
                    AppPlease::Continue
                }
            }
        }
    }
}
