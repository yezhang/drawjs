//! Vello 渲染器实现
//!
//! 使用 Vello GPU 渲染引擎的渲染器。

use std::sync::Arc;

use vello::kurbo::Stroke;
use vello::peniko::Color as VelloColor;
use vello::util::{RenderContext, RenderSurface};
use vello::{AaConfig, Renderer, RendererOptions};

use crate::command::RenderCommand;
use crate::traits::{Renderer as RendererTrait, WindowProxy};

/// Vello 模块
pub mod winit;
pub use winit::{WinitWindowProxy, WinitWindowProxyInner};

/// Vello 渲染器
///
/// 使用 Vello GPU 渲染引擎实现渲染接口。
pub struct VelloRenderer {
    render_context: RenderContext,
    renderers: Vec<Option<Renderer>>,
    scene: vello::Scene,
    surface: RenderSurface<'static>,
    window: Arc<WinitWindowProxy>,
    scale_factor: f64,
}

impl VelloRenderer {
    /// 创建新的 Vello 渲染器
    ///
    /// # 参数
    ///
    /// * `window` - 窗口代理
    /// * `logical_width` - 逻辑宽度
    /// * `logical_height` - 逻辑高度
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
        match &cmd.kind {
            crate::command::RenderCommandKind::FillRect { rect, color, stroke_color, stroke_width } => {
                let x0 = rect[0].x * scale_factor;
                let y0 = rect[0].y * scale_factor;
                let x1 = rect[1].x * scale_factor;
                let y1 = rect[1].y * scale_factor;
                let kurbo_rect = vello::kurbo::Rect::new(x0, y0, x1, y1);

                let vello_color = color.map(|c| {
                    VelloColor::new([c.r as f32, c.g as f32, c.b as f32, c.a as f32])
                }).unwrap_or_else(|| VelloColor::new([0.2, 0.6, 0.86, 1.0]));

                let alpha = vello_color.components[3];
                if alpha == 0.0 {
                    let stroke_vello_color = stroke_color.map(|c| {
                        VelloColor::new([c.r as f32, c.g as f32, c.b as f32, c.a as f32])
                    }).unwrap_or_else(|| VelloColor::new([0.95, 0.61, 0.07, 1.0]));

                    let stroke_width = *stroke_width * scale_factor;
                    if stroke_width > 0.0 {
                        let inset = stroke_width / 2.0;
                        let stroke_rect = vello::kurbo::Rect::new(
                            x0 + inset, y0 + inset,
                            x1 - inset, y1 - inset,
                        );
                        scene.stroke(
                            &Stroke::new(stroke_width as f64),
                            vello::kurbo::Affine::IDENTITY,
                            stroke_vello_color,
                            None,
                            &stroke_rect,
                        );
                    }
                } else {
                    scene.fill(
                        vello::peniko::Fill::NonZero,
                        vello::kurbo::Affine::IDENTITY,
                        vello_color,
                        None,
                        &kurbo_rect,
                    );
                }
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
                    base_color: VelloColor::new([1.0, 1.0, 1.0, 1.0]),  // 白色背景清除
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
