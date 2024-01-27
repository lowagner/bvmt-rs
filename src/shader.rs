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
pub struct Shader<VertexVariables: Variables, FragmentVariables: Variables, Globals: Variables> {
    pub globals: Globals,
    vertex_data: PhantomData<VertexVariables>,
    fragment_data: PhantomData<FragmentVariables>,
    shader_module: Option<wgpu::ShaderModule>,
}

impl<V: Variables + bytemuck::Pod, F: Variables, G: Variables> Shader<V, F, G> {
    /// Draws to the specified `pixels` with a shader.
    pub fn draw(
        &mut self,
        gpu: &mut Gpu,
        pixels: &mut Pixels,
        vertices: &mut Vertices<V>,
        fragments: &mut Fragments<F>,
    ) {
        // Technically we need the pixels *for this frame* but the pixels will be
        // updated before other GPU commands are run with `gpu.queue.submit()` later.
        // TODO: verify
        pixels.ensure_up_to_date_on_gpu(gpu, NeedIt::Later);

        let texture = pixels.texture.as_mut().expect("ensured on GPU");
        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut gpu_commands = gpu
            .device
            // TODO: add a label from pixels??
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        let mut render_pass = gpu_commands.begin_render_pass(&wgpu::RenderPassDescriptor {
            // TODO: add a label from pixels??
            label: None,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &texture_view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load, // load existing pixels into texture
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            occlusion_query_set: None,
            timestamp_writes: None,
        });

        self.draw_to_render_pass(gpu, &mut render_pass, vertices, fragments);
    }

    pub(crate) fn draw_to_render_pass<'a>(
        &mut self,
        gpu: &mut Gpu,
        render_pass: &mut wgpu::RenderPass<'a>,
        vertices: &mut Vertices<V>,
        fragments: &mut Fragments<F>,
    ) {
        self.ensure_on_gpu(gpu);

        // TODO: we need some way of grabbing self.globals and uploading to GPU.
        //       we could have a method like `globals.upload(gpu)`,
        //       or if we have getters/setters we can do something like
        //       `globals.list().map(|x| globals.get(x))`
        // TODO: we should create a shader ID and only set the pipeline
        //       if we haven't done it yet or it's a new shader.
        // TODO: render_pass.set_pipeline(shader.render_pipeline)
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
        let mut source = ""; // TODO
        self.shader_module = Some(
            gpu.device
                .create_shader_module(wgpu::ShaderModuleDescriptor {
                    label: None, // TODO: maybe allow a label
                    source: wgpu::ShaderSource::Wgsl(source.into()),
                }),
        );

        // TODO: add pipeline_layout & render_pipeline
    }
}
