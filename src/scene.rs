#![allow(dead_code)]

use crate::gpu::*;
use crate::shader::ShadingAllBorrowed;

pub struct Scene<'a, G: Variables + 'a, Shadings: IntoIterator<Item = ShadingAllBorrowed<'a, G>>> {
    /// Color to use before drawing anything.
    pub background: Color,
    pub shadings: Shadings,
}

impl<'a, G: Variables + 'a, Shadings: IntoIterator<Item = ShadingAllBorrowed<'a, G>>>
    Scene<'a, G, S>
{
    /// Draws this `Scene` to the specified `Pixels`.
    pub fn draw(&self, gpu: &mut Gpu, pixels: &mut Pixels) {
        // Technically we need the pixels *for this frame* but the pixels will be
        // updated before other GPU commands are run with `gpu.queue.submit()` later.
        // TODO: verify
        pixels.ensure_up_to_date_on_gpu(gpu, NeedIt::Later);
        // TODO: add name for Pixels as a label.
        self.draw_to_texture(gpu, pixels.texture.as_mut().expect("ensured on GPU"), None);
    }

    pub(crate) fn draw_to_texture(
        &self,
        gpu: &mut Gpu,
        texture: &mut wgpu::Texture,
        label: Option<&str>,
    ) {
        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut gpu_commands = gpu
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label });

        {
            let mut render_pass = gpu_commands.begin_render_pass(&wgpu::RenderPassDescriptor {
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

            for shading in self.shadings {
                shading.draw_to_render_pass(gpu, &mut render_pass);
            }
        }

        gpu.queue.submit(std::iter::once(gpu_commands.finish()));
    }
}
