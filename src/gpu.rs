#![allow(dead_code)]

// Re-export GPU-related things for convenience.
pub use crate::binds::{Bind, UniformStruct};
pub use crate::color::Color;
pub use crate::defaults::{DefaultFragmentVariables, DefaultGlobals, DefaultVertexVariables};
pub use crate::fragments::Fragments;
pub use crate::globals::Globals;
pub use crate::options::NeedIt;
pub use crate::pixels::Pixels;
pub use crate::scene::{Scene, SceneDrawer};
pub use crate::shader::Shader;
pub use crate::variables::{
    built_in, BuiltIn, Location, Metadata, Value, Variable, Variables, VariablesStruct,
};
pub use crate::vertices::Vertices;

use crate::dimensions::*;
use crate::options::*;

use strum::{EnumCount, IntoEnumIterator};
use strum_macros::{EnumCount, EnumIter};

use std::iter;

#[derive(Debug, EnumCount, EnumIter)]
pub enum Sampler {
    /** Grabs the nearest pixel; i.e., non-interpolated. */
    Nearest,
    /** Linear interpolation between pixels. */
    Interpolate,
    // TODO: RepeatedNearest
    // TODO: RepeatedInterpolated
}

pub struct Gpu {
    pub(crate) device: wgpu::Device,
    pub(crate) queue: wgpu::Queue,
    pub(crate) samplers: Vec<wgpu::Sampler>,
}

// TODO: how many commands actually need to modify device/queue?
//       we probably can get away with `&mut Gpu` -> `& Gpu` everywhere.
impl Gpu {
    pub(crate) fn new(device: wgpu::Device, queue: wgpu::Queue) -> Self {
        let samplers: Vec<wgpu::Sampler> = Sampler::iter()
            .map(|sampler| match sampler {
                Sampler::Nearest => device.create_sampler(&wgpu::SamplerDescriptor {
                    mag_filter: wgpu::FilterMode::Nearest,
                    ..Default::default()
                }),
                Sampler::Interpolate => device.create_sampler(&wgpu::SamplerDescriptor {
                    mag_filter: wgpu::FilterMode::Linear,
                    ..Default::default()
                }),
            })
            .collect();
        assert_eq!(samplers.len(), Sampler::COUNT);
        Self {
            device,
            queue,
            samplers,
        }
    }

    /// Flushes any commands sent to the GPU, e.g., for writing pixels from CPU to GPU memory.
    /// If you don't need a GPU update immediately, then prefer waiting for the
    /// screen drawing algorithm, which will effectively flush the GPU commands
    /// each frame.
    pub fn flush(&mut self) {
        // TODO: see if this works.
        self.queue.submit(iter::empty());
    }

    // TODO: copy(&mut self, from: DataHere { pixels: &mut Pixels, box2: Box2i }, to: DataHere)

    /// Creates a `Pixels` instance with the given size;
    /// these pixels will start on the GPU but can be moved to the CPU later.
    pub fn pixels(&mut self, size: Size2i) -> Pixels {
        let texture = self.create_texture(size);
        Pixels {
            size,
            synced: Synced::GpuOnly,
            array: vec![],
            texture: Some(texture),
            interpolated: false,
            label: None,
        }
    }

    /// Creates a `Pixels` instance with the given label and size; these pixels will
    /// only live on the GPU, unless their data is requested on the CPU at a later time.
    pub fn pixels_labeled(&mut self, label: String, size: Size2i) -> Pixels {
        let texture = self.create_texture(size);
        Pixels {
            size,
            synced: Synced::GpuOnly,
            array: vec![],
            texture: Some(texture),
            interpolated: false,
            label: Some(label),
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
