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
pub struct Shader<VertexVariables: Variables, FragmentVariables: Variables, Globals: Variables> {
    // TODO
    vertex_data: PhantomData<VertexVariables>,
    fragment_data: PhantomData<FragmentVariables>,
    global_data: PhantomData<Globals>,
}

impl<V: Variables + bytemuck::Pod, F: Variables, G: Variables> Shader<V, F, G> {
    /// Returns a "Shading" of the vertices and fragments, i.e., something
    /// that is ready for the GPU to draw.
    pub fn shading<'a>(
        gpu: &mut Gpu,
        vertices: &'a mut Vertices<V>,
        fragments: &'a mut Fragments<F>,
        globals: G,
    ) -> Shading<'a, G> {
        vertices.ensure_on_gpu(gpu);
        fragments.ensure_on_gpu(gpu);
        Shading {
            vertices_buffer: vertices.buffer.as_ref().expect("ensured to be on the GPU"),
            fragments_buffer: fragments.buffer.as_ref().expect("ensured to be on the GPU"),
            globals,
        }
    }
}

/// A Shader with global values (uniforms) specified.
/// We erase the type information in Vertices/Fragments mostly for ergonomics,
/// but also because we don't want these to change at this stage (they have
/// been sent to the GPU).  Global variables *can* be modified here and this
/// shading can be drawn multiple times with different globals.
pub struct Shading<'a, Globals: Variables> {
    vertices_buffer: &'a wgpu::Buffer,
    fragments_buffer: &'a wgpu::Buffer,
    pub globals: Globals,
}

impl<'a, G: Variables> Shading<'a, G> {
    /// Draws to the specified `pixels` with a shader.
    pub fn draw(&self, _gpu: &mut Gpu, _pixels: &mut Pixels) {
        // TODO: we need some way of grabbing self.globals and uploading to GPU.
        //       we could have a method like `globals.upload(gpu)`,
        //       or if we have getters/setters we can do something like
        //       `globals.list().map(|x| globals.get(x))`
        todo!();
    }

    // TODO: option to draw only certain fragments, e.g., a range of fragments.
    // multiply by three when passing to render_pass.draw_indexed
}
