#![allow(dead_code)]

use crate::gpu::*;

use wgpu::util::DeviceExt; // create_buffer_init

#[derive(Debug)]
pub struct Vertices<V: Variables> {
    /// The `Variables` in `V` must be settable, because we are specifying
    /// the values of each vertex (e.g., position, color, etc.).
    array: Vec<V>,
    /// Present if the vertices are on the GPU.
    pub(crate) buffer: Option<wgpu::Buffer>,
}

impl<V: Variables + bytemuck::Pod> Vertices<V> {
    pub fn new(array: Vec<V>) -> Self {
        Self {
            array,
            buffer: None,
        }
    }

    // TODO: probably should return the &buffer here.
    pub(crate) fn ensure_on_gpu(&mut self, gpu: &mut Gpu) {
        if self.buffer.is_some() {
            return;
        }
        self.buffer = Some(
            gpu.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: None, // TODO: consider adding `label` to Vertices struct and adding it here.
                    contents: bytemuck::cast_slice(&self.array[..]),
                    usage: wgpu::BufferUsages::VERTEX,
                }),
        );
    }
}
