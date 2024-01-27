#![allow(dead_code)]

use crate::gpu::*;

pub struct Scene {
    pub background: Color,
}

impl Scene {
    pub fn draw_on<'a, F: FnOnce(&mut SceneDrawer<'a>)>(
        &self,
        gpu: &mut Gpu,
        pixels: &'a mut Pixels,
        draw_callback: F,
    ) {
        // Technically we need the pixels *for this frame* but the pixels will be
        // updated before other GPU commands are run with `gpu.queue.submit()` later.
        // TODO: verify
        pixels.ensure_up_to_date_on_gpu(gpu, NeedIt::Later);

        self.draw_on_texture(
            gpu,
            pixels.texture.as_mut().expect("ensured on GPU"),
            None,
            draw_callback,
        )
    }

    pub(crate) fn draw_on_texture<'a, F: FnOnce(&mut SceneDrawer<'a>)>(
        &self,
        gpu: &mut Gpu,
        texture: &'a mut wgpu::Texture,
        label: Option<&str>,
        draw_callback: F,
    ) {
        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut gpu_commands = gpu
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label });

        {
            let render_pass = gpu_commands.begin_render_pass(&wgpu::RenderPassDescriptor {
                label,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &texture_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: if self.background.is_opaque() {
                            wgpu::LoadOp::Clear(self.background.into())
                        } else {
                            // TODO: if background.a < 255 but not == 0, then we need to
                            // render a translucent rectangle before drawing.
                            // This could be a cool "hit head" effect, since previous frames
                            // will stick around a bit.  Would need to depend on fps.
                            wgpu::LoadOp::Load
                        },
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });
            let mut drawer = SceneDrawer {
                render_pass: &render_pass,
                gpu,
            };
            draw_callback(&mut drawer);
        }
        gpu.queue.submit(std::iter::once(gpu_commands.finish()));
    }
}

struct SceneDrawer<'a> {
    render_pass: &'a wgpu::RenderPass<'a>,
    gpu: &'a mut Gpu,
}

impl<'a> SceneDrawer<'a> {
    pub fn draw<V: Variables + bytemuck::Pod, F: Variables, G: Variables>(
        &mut self,
        shader: &mut Shader<V, F, G>,
        vertices: &mut Vertices<V>,
        fragments: &mut Fragments<F>,
    ) {
        shader.draw_to_render_pass(&mut self.gpu, &mut self.render_pass, vertices, fragments);
    }
}
