#![allow(dead_code)]

use crate::color::*;
use crate::dimensions::*;
use crate::pixels::*;
use crate::synced::*;

pub struct Gpu {
    pub(crate) device: wgpu::Device,
    pub(crate) queue: wgpu::Queue,
}

impl Gpu {
    // TODO: draw_to_window(&mut self, &mut Window)

    pub fn pixels(&mut self, size: Size2i) -> Pixels {
        let description = wgpu::TextureDescriptor {
            size: wgpu::Extent3d {
                width: size.width() as u32,
                height: size.height() as u32,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            // TODO: GPU should probably hold onto the preferred window.surface format.
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::all(),
            label: None,
            view_formats: &[],
        };
        let texture = self.device.create_texture(&description);
        Pixels {
            size,
            synced: Synced::GpuOnly,
            array: vec![],
            texture: Some(texture),
        }
    }

    // TODO: draw(from: &mut Pixels, from_rectangle: RectangleI, to: &mut Pixels, to_rectangle: RectangleI)

    /// Draws a single pixel on the `Pixels` instance at the coordinates specified.
    /// If `Pixels` is on the GPU, this command will be done asynchronously, i.e.,
    /// the next time that `self.queue.submit(_)` is called.  This can happen when
    /// writing to the window.
    // TODO: double check that this does what we want with Color::TRANSPARENT.
    //       i think we want it to erase the pixel, but make sure that happens with GPU implementation.
    pub fn draw(&mut self, pixels: &mut Pixels, coordinates: Vector2i, color: Color) {
        let (width, height) = (pixels.width(), pixels.height());
        if coordinates.x < 0
            || coordinates.x >= width
            || coordinates.y < 0
            || coordinates.y >= height
        {
            return;
        }
        if pixels.synced.prefers_writing_to_cpu() {
            // Write to the CPU:
            pixels.array[coordinates.y as usize][coordinates.x as usize] = color;
            pixels.synced.cpu_was_updated();
        } else {
            let texture = pixels
                .texture
                .as_mut()
                .expect("should have a texture since Pixels prefer GPU writes");
            // Write to the GPU:
            // AFAICT there's not a better way to write single pixels to the texture.
            // That's probably ok, this isn't meant to be an efficient API.
            let color = [color];
            self.queue.write_texture(
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
            pixels.synced.gpu_was_updated();
        }
    }

    pub fn ensure_up_to_date_on_gpu(pixels: &mut Pixels) {
        if !pixels.synced.needs_gpu_update() {
            // Everything already up to date.
            return;
        }
        todo!();
    }

    pub fn ensure_up_to_date_on_cpu(pixels: &mut Pixels) {
        if !pixels.synced.needs_cpu_update() {
            // Everything already up to date.
            return;
        }
        // For GPU to CPU, see:
        // https://github.com/gfx-rs/wgpu/tree/trunk/examples/src/hello_compute
        todo!();
    }

    // TODO: shaders, consider https://github.com/EmbarkStudios/rust-gpu ???
}
