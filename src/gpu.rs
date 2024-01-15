#![allow(dead_code)]

pub struct Gpu {}

impl Gpu {
    // TODO: draw(from: &mut Pixels, from_rectangle: RectangleI, to: &mut Pixels, to_rectangle: RectangleI)
    // TODO: draw(pixels: &mut Pixels, coordinates: CoordinatesI, color: Color)
    // TODO: ensure(pixels: &mut Pixels, on: On)
}

pub enum On {
    Cpu,
    Gpu,
}

pub(crate) enum Synced {
    CpuOnly,
    GpuOnly,
    CpuAhead,
    GpuAhead,
}
