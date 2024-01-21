#![allow(dead_code)]

use crate::color::*;
use crate::dimensions::*;
use crate::pixels::*;
use crate::window::*;

use std::time;

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

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum AppEvent {
    /// The App was just started.
    Start,
    /// The App was just quit.  Any `AppPlease::Continue` or `Replace`
    /// requests after handling this will be ignored.  You can use this
    /// to save/etc.
    End,
    /// The window should close -- when handling, prefer returning
    /// AppPlease::Terminate unless you have some "Save first" logic.
    WindowCloseRequested,
    /// Some keyboard input.
    KeyInput(KeyInput),
    /// Some amount of time has elapsed, measured since the last time
    /// `TimeElapsed` has been passed.
    TimeElapsed(time::Duration),
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct KeyInput {
    /// The logical key pressed, dependent on layout/locale.
    pub key: Key,
    /// Whether pressed or released.
    pub state: InputState,
    /// Whether this event was due to an OS-level repeat.
    pub repeating: bool,
    // TODO: add modifiers
}

#[derive(Debug, Default)]
pub struct DefaultApp {
    total_elapsed_time: time::Duration,
}

impl App for DefaultApp {
    fn handle(&mut self, event: AppEvent, window: &mut Window) -> AppPlease {
        match event {
            AppEvent::Start => {
                eprint!("default app starting\n");
                self.total_elapsed_time = time::Duration::default();
                window.pixels = Pixels::new(Size2i::new(100, 80));
                window
                    .pixels
                    .write_pixel(&mut window.gpu, Vector2i::new(99, 0), Color::red(250));
                window.pixels.write_pixel(
                    &mut window.gpu,
                    Vector2i::new(99, 79),
                    Color {
                        r: 250,
                        g: 0,
                        b: 250,
                        a: 255,
                    },
                );
                window
                    .pixels
                    .write_pixel(&mut window.gpu, Vector2i::new(0, 79), Color::blue(250));
                window.background = Color::blue(70);
                AppPlease::Continue
            }
            AppEvent::End => {
                eprint!("default app ending\n");
                // This is ignored.
                AppPlease::Continue
            }
            AppEvent::WindowCloseRequested => AppPlease::Terminate,
            AppEvent::KeyInput(key_input) => {
                if key_input.key == Key::Named(NamedKey::Escape) {
                    AppPlease::Terminate
                } else {
                    AppPlease::Continue
                }
            }
            AppEvent::TimeElapsed(duration) => {
                eprint!("time elapsed: {:?}\n", duration);
                self.total_elapsed_time += duration;
                if self.total_elapsed_time > time::Duration::from_secs(8) {
                    return AppPlease::Terminate;
                }
                let z =
                    (self.total_elapsed_time.as_nanos() as f64 / 1_000_000_000.0).round() as i32;
                window.pixels.write_pixel(
                    &mut window.gpu,
                    Vector2i::new(z, z),
                    Color::red((z * 25) as u8),
                );
                AppPlease::Continue
            }
        }
    }
}
