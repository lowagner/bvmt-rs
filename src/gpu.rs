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
    // TODO: ensure(pixels: &mut Pixels, on: On)
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum On {
    Cpu,
    Gpu,
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
}
