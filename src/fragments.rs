#![allow(dead_code)]

use crate::gpu::*;

use bytemuck::{Pod, Zeroable};
use wgpu::util::DeviceExt; // create_buffer_init

#[derive(Debug)]
pub struct Fragments<F: Variables> {
    array: Vec<Fragment>,
    /// The `Variables` here don't need to be settable, just self-descriptive.
    /// I.e., these variables are interpolated from the outputs of the vertex
    /// shader and *cannot* be set on each fragment, so they live here.
    pub(crate) variables: F,
    /// Present if these indices are on the GPU.
    pub(crate) buffer: Option<wgpu::Buffer>,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default, Pod, Zeroable)]
pub struct Fragment {
    index0: u16,
    index1: u16,
    index2: u16,
}

impl<F: Variables> Fragments<F> {
    pub fn new(variables: F, array: Vec<Fragment>) -> Self {
        Self {
            variables,
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
                    label: None, // TODO: consider adding `label` to Fragments struct and adding it here.
                    contents: bytemuck::cast_slice(&self.array[..]),
                    usage: wgpu::BufferUsages::INDEX,
                }),
        );
    }
}
