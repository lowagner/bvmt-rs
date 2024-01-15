#![allow(dead_code)]

use crate::color::*;
use crate::dimensions::*;
use crate::pixels::*;

pub struct Gpu {
    device: wgpu::Device,
    queue: wgpu::Queue,
}

impl Gpu {
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
            pixels.array[coordinates.y as usize][coordinates.x as usize] = color;
        } else {
            // Write to GPU
            todo!();
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
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub(crate) enum Synced {
    /// Not synced, the data lives only in the CPU.
    CpuOnly,
    /// Not synced, the data lives only in the GPU.
    GpuOnly,
    /// Not synced, the CPU data is ahead.
    CpuAhead,
    /// Not synced, the GPU data is ahead.
    GpuAhead,
    /// Synced, but prefer making updates to the CPU version of the data.
    ButCpuPreferred,
    /// Synced, but prefer making updates to the GPU version of the data.
    ButGpuPreferred,
}

impl Synced {
    pub(crate) fn on_cpu(&self) -> bool {
        *self != Synced::GpuOnly
    }

    pub(crate) fn on_gpu(&self) -> bool {
        *self != Synced::CpuOnly
    }

    pub(crate) fn needs_gpu_update(&self) -> bool {
        matches!(self, Synced::CpuOnly | Synced::CpuAhead)
    }

    pub(crate) fn needs_cpu_update(&self) -> bool {
        matches!(self, Synced::GpuOnly | Synced::GpuAhead)
    }

    pub(crate) fn prefers_writing_to_cpu(&self) -> bool {
        // If already ahead on the CPU, then just keep writing there.
        matches!(
            self,
            Synced::CpuOnly | Synced::CpuAhead | Synced::ButCpuPreferred
        )
    }

    pub(crate) fn prefers_writing_to_gpu(&self) -> bool {
        // If already ahead on the GPU, then just keep writing there.
        matches!(
            self,
            Synced::GpuOnly | Synced::GpuAhead | Synced::ButGpuPreferred
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_synced_on_cpu() {
        assert_eq!(Synced::CpuOnly.on_cpu(), true);
        assert_eq!(Synced::GpuOnly.on_cpu(), false);
        assert_eq!(Synced::CpuAhead.on_cpu(), true);
        assert_eq!(Synced::GpuAhead.on_cpu(), true);
        assert_eq!(Synced::ButCpuPreferred.on_cpu(), true);
        assert_eq!(Synced::ButGpuPreferred.on_cpu(), true);
    }

    #[test]
    fn test_synced_on_gpu() {
        assert_eq!(Synced::CpuOnly.on_gpu(), false);
        assert_eq!(Synced::GpuOnly.on_gpu(), true);
        assert_eq!(Synced::CpuAhead.on_gpu(), true);
        assert_eq!(Synced::GpuAhead.on_gpu(), true);
        assert_eq!(Synced::ButCpuPreferred.on_gpu(), true);
        assert_eq!(Synced::ButGpuPreferred.on_gpu(), true);
    }

    #[test]
    fn test_synced_needs_gpu_update() {
        assert_eq!(Synced::CpuOnly.needs_gpu_update(), true);
        assert_eq!(Synced::GpuOnly.needs_gpu_update(), false);
        assert_eq!(Synced::CpuAhead.needs_gpu_update(), true);
        assert_eq!(Synced::GpuAhead.needs_gpu_update(), false);
        assert_eq!(Synced::ButCpuPreferred.needs_gpu_update(), false);
        assert_eq!(Synced::ButGpuPreferred.needs_gpu_update(), false);
    }

    #[test]
    fn test_synced_needs_cpu_update() {
        assert_eq!(Synced::CpuOnly.needs_cpu_update(), false);
        assert_eq!(Synced::GpuOnly.needs_cpu_update(), true);
        assert_eq!(Synced::CpuAhead.needs_cpu_update(), false);
        assert_eq!(Synced::GpuAhead.needs_cpu_update(), true);
        assert_eq!(Synced::ButCpuPreferred.needs_cpu_update(), false);
        assert_eq!(Synced::ButGpuPreferred.needs_cpu_update(), false);
    }

    #[test]
    fn test_synced_prefers_writing_to_cpu() {
        assert_eq!(Synced::CpuOnly.prefers_writing_to_cpu(), true);
        assert_eq!(Synced::GpuOnly.prefers_writing_to_cpu(), false);
        assert_eq!(Synced::CpuAhead.prefers_writing_to_cpu(), true);
        assert_eq!(Synced::GpuAhead.prefers_writing_to_cpu(), false);
        assert_eq!(Synced::ButCpuPreferred.prefers_writing_to_cpu(), true);
        assert_eq!(Synced::ButGpuPreferred.prefers_writing_to_cpu(), false);
    }

    #[test]
    fn test_synced_prefers_writing_to_gpu() {
        assert_eq!(Synced::CpuOnly.prefers_writing_to_gpu(), false);
        assert_eq!(Synced::GpuOnly.prefers_writing_to_gpu(), true);
        assert_eq!(Synced::CpuAhead.prefers_writing_to_gpu(), false);
        assert_eq!(Synced::GpuAhead.prefers_writing_to_gpu(), true);
        assert_eq!(Synced::ButCpuPreferred.prefers_writing_to_gpu(), false);
        assert_eq!(Synced::ButGpuPreferred.prefers_writing_to_gpu(), true);
    }
}
