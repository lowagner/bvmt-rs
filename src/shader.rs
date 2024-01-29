#![allow(dead_code)]

use crate::gpu::*;

use std::marker::PhantomData;

// TODO: consider using https://github.com/EmbarkStudios/rust-gpu for specifying shader code
/* TODO: i'm thinking about shaders like this:
    vertex_function(position, variables: VertexVariables, globals: Globals)
        -> {position: V4f, FragmentVariables}
and
    fragment_function(position: V4f, vertex_output: FragmentVariables, globals: Globals) -> {color: V4f}

can the globals go into both vertex and fragment functions?  or do we need separate globals for each shader part?
*/
#[derive(Debug, Default)]
pub struct Shader<V: Variables, F: Variables, G: Globals> {
    pub globals: G,
    vertex_data: PhantomData<V>,
    fragment_data: PhantomData<F>,
    shader_module: Option<wgpu::ShaderModule>,
}

impl<V: Variables + bytemuck::Pod, F: Variables, G: Globals> Shader<V, F, G> {
    /// Draws to the specified `pixels` with a shader.
    pub fn draw(
        &mut self,
        gpu: &mut Gpu,
        pixels: &mut Pixels,
        vertices: &mut Vertices<V>,
        fragments: &mut Fragments<F>,
    ) {
        let scene = Scene {
            background: Color::TRANSPARENT,
        };
        scene.draw_on(gpu, pixels, |drawer| unsafe {
            // `unsafe` because we can't pass `self` in like this:
            // drawer.draw(&mut self, vertices, fragments);
            self.draw_to_render_pass(
                &mut *drawer.gpu,
                &mut *drawer.render_pass,
                vertices,
                fragments,
            );
        });
    }

    pub(crate) fn draw_to_render_pass<'a>(
        &mut self,
        gpu: &mut Gpu,
        render_pass: &mut wgpu::RenderPass<'a>,
        vertices: &mut Vertices<V>,
        fragments: &mut Fragments<F>,
    ) {
        self.ensure_on_gpu(gpu);

        // TODO: use getters to upload all globals to the GPU.
        //       `globals.list().map(|x| globals.get(x.name()))`
        // TODO: we should create a shader ID and only set the pipeline
        //       if we haven't done it yet or it's a new shader.
        // TODO: render_pass.set_pipeline(shader.render_pipeline)
        // TODO: render_pass.set_bind_group(bind_global.group, bind_group { 0: pixel_view, 1: sampler }, &[])
        todo!();
    }

    // TODO: option to draw only certain fragments, e.g., a range of fragments.
    // multiply by three when passing to render_pass.draw_indexed
    pub(crate) fn write_globals(&mut self, gpu: &mut Gpu) {
        todo!();
    }

    fn ensure_on_gpu(&mut self, gpu: &mut Gpu) {
        if self.shader_module.is_some() {
            return;
        }
        /* TODO: style
        // vertex input values:
        struct Vertex {
            @location(0) position: vec3<f32>,
            // etc.
        }
        // vertex output values, interpolated for each triangle/fragment:
        struct Fragment {
            @builtin(position) clip_position: vec4<f32>,
            @location(0) uv: vec2<f32>,
            // etc.
        }
        // globals
        struct Globals {
            view_transform: mat4x4<f32>,
        }
        @group(0) @binding(0)
        var<uniform> globals: Globals;
        // TODO: is there a reason we can't just put these all into `globals`?
        // globals
        @group(1) @binding(0)
        var pixels_texture: texture_2d<f32>;
        @group(1) @binding(1)
        var pixels_sampler: sampler;
        // vertex shader
        @vertex
        fn vs_main(vertex: Vertex) -> Fragment {
            var fragment: Fragment;
            fragment.clip_position = ...;
            fragment.uv = ...;
            // Return fragment value for this vertex
            // to be interpolated in the triangle face.
            return fragment;
        }
        // fragment shader
        @fragment
        fn fs_main(in: Fragment) -> @location(0) vec4<f32> {
            // Return color to use for this pixel.
            return vec4<f32>(r, g, b, 1.0);
        }
        */
        let source = self.get_source();
        self.shader_module = Some(
            gpu.device
                .create_shader_module(wgpu::ShaderModuleDescriptor {
                    label: None, // TODO: maybe allow a label
                    source: wgpu::ShaderSource::Wgsl(source.into()),
                }),
        );

        // TODO: add pipeline_layout & render_pipeline
    }

    fn get_source(&self) -> String {
        // TODO:
        "".into()
    }
}
