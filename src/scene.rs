#![allow(dead_code)]

use crate::gpu::*;

pub struct Scene<'a, G: Variables + 'a, Shadings: IntoIterator<Item = &'a Shading<'a, G>>> {
    /// Color to use before drawing anything.
    pub background: Color,
    pub shadings: Shadings,
}

impl<'a, G: Variables + 'a, S: IntoIterator<Item = &'a Shading<'a, G>>> Scene<'a, G, S> {
    /// Draws this `Scene` to the specified `Pixels`.
    pub fn draw(&mut self, _gpu: &mut Gpu, _pixels: &mut Pixels) {
        todo!();
    }
}
