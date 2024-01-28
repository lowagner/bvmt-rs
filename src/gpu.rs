#![allow(dead_code)]

// Re-export GPU-related things for convenience.
pub use crate::color::Color;
pub use crate::fragments::Fragments;
pub use crate::options::NeedIt;
pub use crate::pixels::Pixels;
pub use crate::scene::{Scene, SceneDrawer};
pub use crate::shader::Shader;
pub use crate::variables::{
    BuiltIn, DefaultFragmentVariables, DefaultVertexVariables, Location, Metadata, Variable,
    Variables, VariablesDeclaration,
};
pub use crate::vertices::Vertices;

use crate::dimensions::*;
use crate::options::*;

use std::iter;

pub struct Gpu {
    pub(crate) device: wgpu::Device,
    pub(crate) queue: wgpu::Queue,
}

// TODO: how many commands actually need to modify device/queue?
//       we probably can get away with `&mut Gpu` -> `& Gpu` everywhere.
impl Gpu {
    /// Flushes any commands sent to the GPU, e.g., for writing pixels from CPU to GPU memory.
    /// If you don't need a GPU update immediately, then prefer waiting for the
    /// screen drawing algorithm, which will effectively flush the GPU commands
    /// each frame.
    pub fn flush(&mut self) {
        // TODO: see if this works.
        self.queue.submit(iter::empty());
    }

    // TODO: copy(&mut self, from: DataHere { pixels: &mut Pixels, box2: Box2i }, to: DataHere)

    /// Creates a `Pixels` instance with the given size; these pixels will
    /// only live on the GPU, unless their data is requested on the CPU at a later time.
    pub fn pixels(&mut self, size: Size2i) -> Pixels {
        let texture = self.create_texture(size);
        Pixels {
            size,
            synced: Synced::GpuOnly,
            array: vec![],
            texture: Some(texture),
            interpolated: false,
        }
    }

    pub(crate) fn create_texture(&mut self, size: Size2i) -> wgpu::Texture {
        let description = wgpu::TextureDescriptor {
            size: wgpu::Extent3d {
                width: size.width() as u32,
                height: size.height() as u32,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            // Need to keep texture format the same as the Color {r, g, b, a} memory layout.
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::all(),
            label: None,
            view_formats: &[],
        };
        return self.device.create_texture(&description);
    }
}
