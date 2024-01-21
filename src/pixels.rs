#![allow(dead_code)]

use crate::color::*;
use crate::dimensions::*;
use crate::gpu::*;
use crate::options::*;

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
    pub fn new(size: Size2i) -> Self {
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

    /// Writes a single pixel at the coordinates specified.  If these `Pixels`
    /// are on the GPU, the command will be done asynchronously, i.e.,
    /// the next time that the GPU flushes its commands.  This can happen
    /// automatically when the window redraws the next frame or manually
    /// if `gpu.flush()` is called.
    // TODO: consider adding `NeedIt` method here.  it only applies for GPU rendering, though.
    // TODO: double check that this does what we want with Color::TRANSPARENT.
    //       i think we want it to erase the pixel, but make sure that happens with GPU implementation.
    pub fn write_pixel(&mut self, gpu: &mut Gpu, coordinates: Vector2i, color: Color) {
        let (width, height) = (self.width(), self.height());
        if coordinates.x < 0
            || coordinates.x >= width
            || coordinates.y < 0
            || coordinates.y >= height
        {
            return;
        }
        if self.synced.prefers_writing_to_cpu() {
            // Write to the CPU:
            self.array[coordinates.y as usize][coordinates.x as usize] = color;
            self.synced.cpu_was_updated();
        } else {
            let texture = self
                .texture
                .as_mut()
                .expect("should have a texture since Pixels prefer GPU writes");
            // Write to the GPU:
            // AFAICT there's not a better way to write single pixels to the texture.
            // That's probably ok, this isn't meant to be an efficient API.
            let color = [color];
            gpu.queue.write_texture(
                wgpu::ImageCopyTexture {
                    texture,
                    mip_level: 0,
                    // TODO: check if we need to flip coordinates to `height - coordinates.y - 1`
                    origin: wgpu::Origin3d {
                        x: coordinates.x as u32,
                        y: coordinates.y as u32,
                        z: 0,
                    },
                    aspect: wgpu::TextureAspect::All,
                },
                bytemuck::cast_slice(&color),
                wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: None,  // no rows
                    rows_per_image: None, // no rows
                },
                wgpu::Extent3d {
                    width: 1,
                    height: 1,
                    depth_or_array_layers: 1,
                },
            );
            self.synced.gpu_was_updated();
        }
    }

    // TODO: copy(&mut self, from: DataHere { pixels: &mut Pixels, box2: Box2i }, to: Box2i)

    /// Puts the `Pixels` onto the GPU if they're not there already and up to date.
    /// Afterwards, call `self.flush()` if you need the pixel update immediately.
    /// If drawing to `window.pixels`, this will be called automatically for
    /// you each frame before drawing to the screen.
    pub fn ensure_up_to_date_on_gpu(&mut self, gpu: &mut Gpu, need_it: NeedIt) {
        if !self.synced.needs_gpu_update() {
            // Everything already up to date.
            return;
        }
        let (width, height) = (self.width() as u32, self.height() as usize);
        if self.texture.is_none() {
            self.texture = Some(gpu.create_texture(self.size));
        }
        let texture = self.texture.as_mut().expect("is present now for sure");
        // We have to write multiple times to the GPU because of our pixel
        // layout as `Vec<Vec<Color>>`.
        // TODO: reconsider and use a single array internally, padded to rows of multiples of 256 bytes (64 colors)
        for y in 0..height {
            gpu.queue.write_texture(
                wgpu::ImageCopyTexture {
                    texture,
                    mip_level: 0,
                    // TODO: check if we need to flip coordinates to `height - y - 1`
                    origin: wgpu::Origin3d {
                        x: 0,
                        y: y as u32,
                        z: 0,
                    },
                    aspect: wgpu::TextureAspect::All,
                },
                bytemuck::cast_slice(&self.array[y]),
                wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: None,
                    rows_per_image: None,
                },
                wgpu::Extent3d {
                    width,
                    height: 0,
                    depth_or_array_layers: 1,
                },
            );
        }
        if need_it == NeedIt::Now {
            gpu.flush();
        }
    }

    pub fn ensure_up_to_date_on_cpu(&mut self, gpu: &mut Gpu) {
        if !self.synced.needs_cpu_update() {
            // Everything already up to date.
            return;
        }
        // For GPU to CPU, see:
        // https://github.com/gfx-rs/wgpu/tree/trunk/examples/src/hello_compute
        todo!();
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
