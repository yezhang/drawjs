use std::sync::Arc;

use vello::kurbo::{Affine, Stroke};
use vello::peniko::Color;
use vello::util::{RenderContext, RenderSurface};
use vello::{AaConfig, Renderer, RendererOptions};
use winit::dpi::PhysicalSize;
use winit::window::Window;

use crate::render_ir::RenderCommand;

pub struct VelloRenderer {
    render_context: RenderContext,
    renderers: Vec<Option<Renderer>>,
    scene: Scene,
    surface: RenderSurface<'static>,
    window: Arc<Window>,
}

type Scene = vello::Scene;

impl VelloRenderer {
    pub fn new(window: Arc<Window>, width: u32, height: u32) -> Self {
        let mut render_context = RenderContext::new();
        let size = PhysicalSize::new(width, height);
        let surface_future = render_context.create_surface(
            Arc::clone(&window),
            size.width,
            size.height,
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
            scene: Scene::new(),
            surface,
            window,
        }
    }

    pub fn render(&mut self, commands: &[RenderCommand]) {
        self.scene.reset();

        for cmd in commands {
            Self::render_command(&mut self.scene, cmd);
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
                    base_color: Color::new([0.0, 0.0, 0.0, 0.0]),
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

    fn render_command(scene: &mut Scene, cmd: &RenderCommand) {
        match cmd {
            RenderCommand::FillRect { rect, color, stroke_color, stroke_width } => {
                let kurbo_rect = vello::kurbo::Rect::new(
                    rect[0].x, rect[0].y,
                    rect[1].x, rect[1].y,
                );

                let vello_color = color.map(|c| {
                    Color::new([c.r as f32, c.g as f32, c.b as f32, c.a as f32])
                }).unwrap_or_else(|| Color::new([0.2, 0.6, 0.86, 1.0]));

                let alpha = vello_color.components[3];
                if alpha == 0.0 {
                    // 只绘制描边
                    let stroke_vello_color = stroke_color.map(|c| {
                        Color::new([c.r as f32, c.g as f32, c.b as f32, c.a as f32])
                    }).unwrap_or_else(|| Color::new([0.95, 0.61, 0.07, 1.0]));

                    let stroke = Stroke::new(*stroke_width as f64);

                    let stroke_width = *stroke_width;
                    if stroke_width > 0.0 {
                        let inset = stroke_width / 2.0;
                        let stroke_rect = vello::kurbo::Rect::new(
                            rect[0].x + inset, rect[0].y + inset,
                            rect[1].x - inset, rect[1].y - inset,
                        );
                        scene.stroke(
                            &Stroke::new(stroke_width as f64),
                            Affine::IDENTITY,
                            stroke_vello_color,
                            None,
                            &stroke_rect,
                        );
                    }
                } else {
                    scene.fill(
                        vello::peniko::Fill::NonZero,
                        Affine::IDENTITY,
                        vello_color,
                        None,
                        &kurbo_rect,
                    );
                }
            }
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.surface.config.width = width;
        self.surface.config.height = height;
    }

    pub fn window(&self) -> &Arc<Window> {
        &self.window
    }
}

fn create_renderer(render_cx: &RenderContext, surface: &RenderSurface<'_>) -> Renderer {
    Renderer::new(
        &render_cx.devices[surface.dev_id].device,
        RendererOptions::default(),
    )
    .expect("Couldn't create renderer")
}
