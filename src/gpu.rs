#![allow(dead_code)]

pub struct Gpu {}

impl Gpu {
    // TODO: draw(from: &mut Pixels, from_rectangle: RectangleI, to: &mut Pixels, to_rectangle: RectangleI)
    // TODO: draw(pixels: &mut Pixels, coordinates: CoordinatesI, color: Color)
    // TODO: ensure(pixels: &mut Pixels, on: On)

    // TODO: `pixels(size: Size2i) -> Pixels` -> creating pixels on the GPU only
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
