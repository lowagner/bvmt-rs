#![allow(dead_code)]

use crate::dimensions::*;
use crate::pixels::*;

pub struct Gpu {
    device: wgpu::Device,
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
    // TODO: draw(pixels: &mut Pixels, coordinates: CoordinatesI, color: Color)

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
    CpuOnly,
    GpuOnly,
    CpuAhead,
    GpuAhead,
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
    }

    #[test]
    fn test_synced_on_gpu() {
        assert_eq!(Synced::CpuOnly.on_gpu(), false);
        assert_eq!(Synced::GpuOnly.on_gpu(), true);
        assert_eq!(Synced::CpuAhead.on_gpu(), true);
        assert_eq!(Synced::GpuAhead.on_gpu(), true);
    }

    #[test]
    fn test_synced_needs_gpu_update() {
        assert_eq!(Synced::CpuOnly.needs_gpu_update(), true);
        assert_eq!(Synced::GpuOnly.needs_gpu_update(), false);
        assert_eq!(Synced::CpuAhead.needs_gpu_update(), true);
        assert_eq!(Synced::GpuAhead.needs_gpu_update(), false);
    }

    #[test]
    fn test_synced_needs_cpu_update() {
        assert_eq!(Synced::CpuOnly.needs_cpu_update(), false);
        assert_eq!(Synced::GpuOnly.needs_cpu_update(), true);
        assert_eq!(Synced::CpuAhead.needs_cpu_update(), false);
        assert_eq!(Synced::GpuAhead.needs_cpu_update(), true);
    }
}
