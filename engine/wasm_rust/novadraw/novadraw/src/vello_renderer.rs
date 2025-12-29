use std::sync::Arc;

use vello::kurbo::Affine;
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
}

type Scene = vello::Scene;

impl VelloRenderer {
    pub fn new() -> Self {
        VelloRenderer {
            render_context: RenderContext::new(),
            renderers: vec![],
            scene: Scene::new(),
        }
    }

    pub fn create_surface(
        &mut self,
        window: Arc<Window>,
        width: u32,
        height: u32,
    ) -> VelloSurface {
        let size = PhysicalSize::new(width, height);
        let surface_future = self.render_context.create_surface(
            Arc::clone(&window),
            size.width,
            size.height,
            vello::wgpu::PresentMode::AutoVsync,
        );
        let surface = pollster::block_on(surface_future).expect("Failed to create surface");

        self.renderers
            .resize_with(self.render_context.devices.len(), || None);
        self.renderers[surface.dev_id]
            .get_or_insert_with(|| create_renderer(&self.render_context, &surface));

        VelloSurface {
            window,
            surface: Box::new(surface),
            width,
            height,
            valid_surface: true,
        }
    }

    pub fn render(&mut self, surface: &mut VelloSurface, commands: &[RenderCommand]) {
        if !surface.valid_surface {
            return;
        }

        self.scene.reset();

        for cmd in commands {
            Self::render_command(&mut self.scene, cmd);
        }

        let device_handle = &self.render_context.devices[surface.surface.dev_id];
        let width = surface.surface.config.width;
        let height = surface.surface.config.height;

        self.renderers[surface.surface.dev_id]
            .as_mut()
            .unwrap()
            .render_to_texture(
                &device_handle.device,
                &device_handle.queue,
                &self.scene,
                &surface.surface.target_view,
                &vello::RenderParams {
                    base_color: Color::new([0.0, 0.0, 0.0, 0.0]),
                    width,
                    height,
                    antialiasing_method: AaConfig::Msaa16,
                },
            )
            .expect("Failed to render to texture");

        let surface_texture = surface
            .surface
            .surface
            .get_current_texture()
            .expect("Failed to get surface texture");

        let mut encoder = device_handle
            .device
            .create_command_encoder(&vello::wgpu::CommandEncoderDescriptor {
                label: Some("Surface Blit"),
            });

        surface.surface.blitter.copy(
            &device_handle.device,
            &mut encoder,
            &surface.surface.target_view,
            &surface_texture
                .texture
                .create_view(&vello::wgpu::TextureViewDescriptor::default()),
        );

        device_handle.queue.submit([encoder.finish()]);
        surface_texture.present();
    }

    fn render_command(scene: &mut Scene, cmd: &RenderCommand) {
        match cmd {
            RenderCommand::FillRect { rect, color, .. } => {
                let kurbo_rect = vello::kurbo::Rect::new(
                    rect[0].x, rect[0].y,
                    rect[1].x, rect[1].y,
                );

                let vello_color = color.map(|c| {
                    Color::new([c.r as f32, c.g as f32, c.b as f32, c.a as f32])
                }).unwrap_or_else(|| Color::new([0.2, 0.6, 0.86, 1.0]));

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

fn create_renderer(render_cx: &RenderContext, surface: &RenderSurface<'_>) -> Renderer {
    Renderer::new(
        &render_cx.devices[surface.dev_id].device,
        RendererOptions::default(),
    )
    .expect("Couldn't create renderer")
}

pub struct VelloSurface {
    pub window: Arc<Window>,
    pub surface: Box<RenderSurface<'static>>,
    pub width: u32,
    pub height: u32,
    pub valid_surface: bool,
}
