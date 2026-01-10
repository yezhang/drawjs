//! Vello 渲染器实现

use std::sync::Arc;

use vello::kurbo::Stroke;
use vello::peniko::Color as VelloColor;
use vello::util::{RenderContext, RenderSurface};
use vello::{AaConfig, Renderer, RendererOptions};

use crate::command::RenderCommand;
use crate::traits::{Renderer as RendererTrait, WindowProxy};

pub mod winit;
pub use winit::{WinitWindowProxy, WinitWindowProxyInner};

pub struct VelloRenderer {
    render_context: RenderContext,
    renderers: Vec<Option<Renderer>>,
    scene: vello::Scene,
    surface: RenderSurface<'static>,
    window: Arc<WinitWindowProxy>,
    scale_factor: f64,
}

impl VelloRenderer {
    pub fn new(window: Arc<WinitWindowProxy>, logical_width: f64, logical_height: f64) -> Self {
        let scale_factor = window.scale_factor();
        let width = (logical_width * scale_factor) as u32;
        let height = (logical_height * scale_factor) as u32;

        let mut render_context = RenderContext::new();
        let surface_future = render_context.create_surface(
            window.window().clone(),
            width,
            height,
            vello::wgpu::PresentMode::AutoVsync,
        );
        let surface = pollster::block_on(surface_future).expect("Failed to create surface");

        let mut renderers = vec![];
        renderers.resize_with(render_context.devices.len(), || None);
        renderers[surface.dev_id]
            .get_or_insert_with(|| create_renderer(&render_context, &surface));

        VelloRenderer {
            render_context,
            renderers,
            scene: vello::Scene::new(),
            surface,
            window,
            scale_factor,
        }
    }

    fn render_command(scene: &mut vello::Scene, cmd: &RenderCommand, scale_factor: f64) {
        let transform = cmd.transform();
        let matrix = transform.matrix();

        let matrix_arr = matrix.to_array();
        let a = matrix_arr[0][0];
        let b = matrix_arr[1][0];
        let c = matrix_arr[0][1];
        let d = matrix_arr[1][1];

        let e = matrix_arr[0][2];
        let f = matrix_arr[1][2];

        let affine = vello::kurbo::Affine::new([
            a, b,
            c, d,
            e * scale_factor, f * scale_factor,
        ]);

        match &cmd.kind {
            crate::command::RenderCommandKind::ClearRect { rect } => {
                let x0 = rect[0].x * scale_factor;
                let y0 = rect[0].y * scale_factor;
                let x1 = rect[1].x * scale_factor;
                let y1 = rect[1].y * scale_factor;
                let kurbo_rect = vello::kurbo::Rect::new(x0, y0, x1, y1);
                let vello_color = VelloColor::new([1.0, 1.0, 1.0, 1.0]);
                scene.fill(vello::peniko::Fill::NonZero, affine, vello_color, None, &kurbo_rect);
            }

            crate::command::RenderCommandKind::FillRect { rect, fill_color } => {
                let x0 = rect[0].x * scale_factor;
                let y0 = rect[0].y * scale_factor;
                let x1 = rect[1].x * scale_factor;
                let y1 = rect[1].y * scale_factor;
                let kurbo_rect = vello::kurbo::Rect::new(x0, y0, x1, y1);
                let vello_color = VelloColor::new([
                    fill_color.r as f32,
                    fill_color.g as f32,
                    fill_color.b as f32,
                    fill_color.a as f32,
                ]);
                scene.fill(vello::peniko::Fill::NonZero, affine, vello_color, None, &kurbo_rect);
            }

            crate::command::RenderCommandKind::StrokeRect { rect, stroke_color, width } => {
                let x0 = rect[0].x * scale_factor;
                let y0 = rect[0].y * scale_factor;
                let x1 = rect[1].x * scale_factor;
                let y1 = rect[1].y * scale_factor;
                let kurbo_rect = vello::kurbo::Rect::new(x0, y0, x1, y1);
                let vello_color = VelloColor::new([
                    stroke_color.r as f32,
                    stroke_color.g as f32,
                    stroke_color.b as f32,
                    stroke_color.a as f32,
                ]);
                scene.stroke(
                    &Stroke::new(*width * scale_factor),
                    affine,
                    vello_color,
                    None,
                    &kurbo_rect,
                );
            }

            crate::command::RenderCommandKind::Clear { color } => {
                let _ = scene;
                let _ = color;
            }

            crate::command::RenderCommandKind::Line { p1, p2, color, width, .. } => {
                let v1 = vello::kurbo::Point::new(p1.x * scale_factor, p1.y * scale_factor);
                let v2 = vello::kurbo::Point::new(p2.x * scale_factor, p2.y * scale_factor);
                let vello_color = VelloColor::new([color.r as f32, color.g as f32, color.b as f32, color.a as f32]);

                scene.stroke(
                    &Stroke::new(*width * scale_factor),
                    affine,
                    vello_color,
                    None,
                    &vello::kurbo::Line::new(v1, v2),
                );
            }

            _ => {
                // 其他命令暂未实现
            }
        }
    }
}

impl RendererTrait for VelloRenderer {
    type Window = WinitWindowProxy;

    fn window(&self) -> &Self::Window {
        &self.window
    }

    fn render(&mut self, commands: &[RenderCommand]) {
        self.scene.reset();

        let scale_factor = self.scale_factor;
        for cmd in commands {
            Self::render_command(&mut self.scene, cmd, scale_factor);
        }

        let device_handle = &self.render_context.devices[self.surface.dev_id];
        let width = self.surface.config.width;
        let height = self.surface.config.height;

        self.renderers[self.surface.dev_id]
            .as_mut()
            .unwrap()
            .render_to_texture(
                &device_handle.device,
                &device_handle.queue,
                &self.scene,
                &self.surface.target_view,
                &vello::RenderParams {
                    base_color: VelloColor::new([1.0, 1.0, 1.0, 1.0]),
                    width,
                    height,
                    antialiasing_method: AaConfig::Msaa16,
                },
            )
            .expect("Failed to render to texture");

        let surface_texture = self
            .surface
            .surface
            .get_current_texture()
            .expect("Failed to get surface texture");

        let mut encoder = device_handle
            .device
            .create_command_encoder(&vello::wgpu::CommandEncoderDescriptor {
                label: Some("Surface Blit"),
            });

        self.surface.blitter.copy(
            &device_handle.device,
            &mut encoder,
            &self.surface.target_view,
            &surface_texture
                .texture
                .create_view(&vello::wgpu::TextureViewDescriptor::default()),
        );

        device_handle.queue.submit([encoder.finish()]);
        surface_texture.present();
    }

    fn resize(&mut self, width: u32, height: u32) {
        let pixel_width = (width as f64 * self.scale_factor) as u32;
        let pixel_height = (height as f64 * self.scale_factor) as u32;
        self.render_context.resize_surface(&mut self.surface, pixel_width, pixel_height);
    }
}

fn create_renderer(render_cx: &RenderContext, surface: &RenderSurface<'_>) -> Renderer {
    Renderer::new(
        &render_cx.devices[surface.dev_id].device,
        RendererOptions::default(),
    )
    .expect("Couldn't create renderer")
}
