#![allow(dead_code)]

use crate::gpu::*;

use std::marker::PhantomData;

// TODO: consider using https://github.com/EmbarkStudios/rust-gpu for specifying shader code
/* TODO: i'm thinking about shaders like this:
    vertex_function(position, variables: VertexVariables, globals: GlobalVariables)
        -> {position: V4f, FragmentVariables}
and
    fragment_function(position: V4f, vertex_output: FragmentVariables, globals: GlobalVariables) -> {color: V4f}

can the globals go into both vertex and fragment functions?  or do we need separate globals for each shader part?
*/
pub struct Shader<
    VertexVariables: Variables,
    FragmentVariables: Variables,
    GlobalVariables: Variables,
> {
    // TODO
    vertex_data: PhantomData<VertexVariables>,
    fragment_data: PhantomData<FragmentVariables>,
    global_data: PhantomData<GlobalVariables>,
}

impl<V: Variables, F: Variables, G: Variables> Shader<V, F, G> {
    // TODO: pub fn shading(v: Vertices, f: Fragments, g: G) -> Shading
}

/// A Shader with global values (uniforms) specified.
pub struct Shading<
    VertexVariables: Variables,
    FragmentVariables: Variables,
    GlobalVariables: Variables,
> {
    vertices: Vertices<VertexVariables>,
    fragments: Fragments<FragmentVariables>,
    pub global_variables: GlobalVariables,
}

impl<V: Variables, F: Variables, G: Variables> Shading<V, F, G> {
    /// Draws to the specified `pixels` with a shader.
    pub fn draw(&mut self, _gpu: &mut Gpu, _pixels: &mut Pixels) {
        todo!();
    }
}
