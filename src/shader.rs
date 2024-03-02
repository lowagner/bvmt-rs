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
    pub label: Option<String>,
    pub additional_definitions: String,
    vertex_data: PhantomData<V>,
    fragment_data: PhantomData<F>,
    globals: PhantomData<G>,
    wgpu_shader: Option<WgpuShader>,
}

#[derive(Debug)]
struct WgpuShader {
    module: wgpu::ShaderModule,
    pipeline: wgpu::RenderPipeline,
    bind_group_layouts: Vec<wgpu::BindGroupLayout>,
}

impl<V: Variables + bytemuck::Pod, F: Variables, G: Globals> Shader<V, F, G> {
    pub fn labeled(label: String) -> Self {
        Self {
            label: Some(label),
            additional_definitions: "".to_string(),
            vertex_data: PhantomData,
            fragment_data: PhantomData,
            globals: PhantomData,
            wgpu_shader: None,
        }
    }

    /// Draws to the specified `pixels` with a shader.
    pub fn draw(
        &mut self,
        gpu: &mut Gpu,
        pixels: &mut Pixels,
        vertices: &mut Vertices<V>,
        fragments: &mut Fragments<F>,
        globals: &G,
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
                globals,
            );
        });
    }

    pub(crate) fn draw_to_render_pass<'a>(
        &mut self,
        gpu: &mut Gpu,
        render_pass: &mut wgpu::RenderPass<'a>,
        vertices: &mut Vertices<V>,
        fragments: &mut Fragments<F>,
        globals: &G,
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

    fn ensure_on_gpu(&mut self, gpu: &mut Gpu) {
        if self.wgpu_shader.is_some() {
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
        fn vs_main(input: Vertex) -> Fragment {
            var fragment: Fragment;
            fragment.clip_position = ...;
            fragment.uv = ...;
            // Return fragment value for this vertex
            // to be interpolated in the triangle face.
            return fragment;
        }
        // fragment shader
        @fragment
        fn fs_main(input: Fragment) -> @location(0) vec4<f32> {
            // Return color to use for this pixel.
            return vec4<f32>(r, g, b, 1.0);
        }
        */
        let source = self.get_source();
        let module = gpu
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None, // TODO: maybe allow a label
                source: wgpu::ShaderSource::Wgsl(source.into()),
            });

        let bind_group_layouts = vec![];

        let pipeline_layout = gpu
            .device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: self.label.as_ref().map(|l| l.as_str()),
                bind_group_layouts: &bind_group_layouts.iter().collect::<Vec<_>>()[..],
                push_constant_ranges: &[],
            });

        let (array_stride, vertex_attributes) = Self::get_vertex_attributes();
        let vertex_buffer_layout = wgpu::VertexBufferLayout {
            array_stride,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &vertex_attributes,
        };

        let pipeline = gpu
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: self.label.as_ref().map(|l| l.as_str()),
                layout: Some(&pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &module,
                    entry_point: "vs_main",
                    buffers: &[vertex_buffer_layout],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &module,
                    entry_point: "fs_main",
                    targets: &[Some(wgpu::ColorTargetState {
                        format: wgpu::TextureFormat::Rgba8Unorm,
                        blend: Some(wgpu::BlendState {
                            color: wgpu::BlendComponent::REPLACE,
                            alpha: wgpu::BlendComponent::REPLACE,
                        }),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                }),
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: Some(wgpu::Face::Back),
                    polygon_mode: wgpu::PolygonMode::Fill,
                    unclipped_depth: false,
                    conservative: false,
                },
                depth_stencil: Some(wgpu::DepthStencilState {
                    format: wgpu::TextureFormat::Depth32Float,
                    depth_write_enabled: true,
                    depth_compare: wgpu::CompareFunction::Less,
                    stencil: wgpu::StencilState::default(),
                    bias: wgpu::DepthBiasState::default(),
                }),
                multisample: wgpu::MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
                multiview: None,
            });

        self.wgpu_shader = Some(WgpuShader {
            module,
            bind_group_layouts,
            pipeline,
        });
    }

    fn get_source(&self) -> String {
        let vertex_input = VariablesStruct {
            name: "Vertex".into(),
            variables: V::list(),
        };
        let fragment_input = VariablesStruct {
            name: "Fragment".into(),
            variables: F::list(),
        };
        // TODO: globals
        // TODO: use indoc here for nicer formatting
        indoc::formatdoc!(
            r"
            {}
            {}
            {}
            @vertex fn vs_main(input: Vertex) -> Fragment {}
            @fragment fn fs_main(input: Fragment) -> @location(0) vec4<f32> {}
            ",
            vertex_input,
            fragment_input,
            self.additional_definitions,
            V::main(),
            F::main()
        )
    }

    /// Returns the stride and the vertex attributes.
    fn get_vertex_attributes<'a>() -> (wgpu::BufferAddress, Vec<wgpu::VertexAttribute>) {
        let mut attributes = vec![];
        let mut offset: u64 = 0;
        // Index into the shader for the next attribute.
        let mut shader_location: u32 = 0;

        for variable in V::list() {
            assert_eq!(
                shader_location,
                variable
                    .index()
                    .expect("only indexed locations belong in the vertex shader")
                    .into()
            );

            let (variable_bytes, variable_format) = variable.bytes_format();
            attributes.push(wgpu::VertexAttribute {
                offset,
                shader_location,
                format: variable_format,
            });

            shader_location += 1;
            offset += variable_bytes as u64;
        }

        assert_eq!(offset as usize, std::mem::size_of::<V>());
        (offset as wgpu::BufferAddress, attributes)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_default_shader() {
        let shader: Shader<DefaultVertexVariables, DefaultFragmentVariables, DefaultGlobals> =
            Shader {
                label: Some("hello-shader".to_string()),
                additional_definitions:
                    "fn my_func() -> vec3<f32> { return vec3<f32>(0.1, 0.2, 0.3); }".to_string(),
                vertex_data: PhantomData,
                fragment_data: PhantomData,
                globals: PhantomData,
                wgpu_shader: None,
            };

        assert_eq!(
            shader.get_source(),
            indoc::indoc! {"
                struct Vertex {
                    @location(0) position: vec3<f32>,
                    @location(1) color: vec3<f32>,
                }
                struct Fragment {
                    @builtin(position) clip_position: vec4<f32>,
                    @location(0) color: vec3<f32>,
                }
                fn my_func() -> vec3<f32> { return vec3<f32>(0.1, 0.2, 0.3); }
                @vertex fn vs_main(input: Vertex) -> Fragment {
                    var output: Fragment;
                    output.color = input.color;
                    output.clip_position = vec4<f32>(input.position, 1.0);
                    return output;
                }
                @fragment fn fs_main(input: Fragment) -> @location(0) vec4<f32> {
                    return vec4<f32>(input.color, 1.0);
                }
            "}
            .to_string()
        );
    }
}
