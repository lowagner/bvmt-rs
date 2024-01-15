#![allow(dead_code)]

use crate::color::*;
use crate::dimensions::*;
use crate::gpu::Synced;

pub struct Pixels {
    pub(crate) size: Size2i,
    pub(crate) synced: Synced,
    /// Invariant: this should be `Some(texture)` iff `self.synced.on_gpu()` is true.
    // TODO: these invariants probably could be taken care of with `Synced` being a
    //       nicer enum, e.g., `GpuOnly(Texture), CpuOnly(Array), Both(Array, Texture)`,
    //       but i didn't want to deal with copying/moving the texture/array around
    //       every time the sync state changed.
    pub(crate) texture: Option<wgpu::Texture>,
    /// Rows of pixels from the top (y = 0) to the bottom (y = self.size.height() - 1),
    /// with each row going from left (x = 0) to right (x = self.size.width() - 1).
    /// Stored as `self.array[y][x]` for pixel at coordinate `(x, y)`.
    /// Invariant: each `Vec` has the correct size based on `self.size`,
    /// unless the pixels are stored only on the GPU.
    pub(crate) array: Vec<Vec<Color>>,
}

impl Pixels {
    pub fn new(&self, size: Size2i) -> Self {
        Self {
            size,
            synced: Synced::CpuOnly,
            array: Self::transparent_pixels_array(size),
            texture: None,
        }
    }

    pub fn size(&self) -> Size2i {
        self.size
    }

    pub fn width(&self) -> i32 {
        self.size.width()
    }

    pub fn height(&self) -> i32 {
        self.size.height()
    }

    pub(crate) fn transparent_pixels_array(size: Size2i) -> Vec<Vec<Color>> {
        let (width, height) = (size.width() as usize, size.height() as usize);
        let mut array = Vec::with_capacity(height);
        let mut row = Vec::with_capacity(width);
        row.resize(width, Color::default());
        array.resize(height, row);
        array
    }
}

pub enum PixelsError {
    OutOfMemory,
}
