use std::sync::Arc;

use vello::kurbo::{Affine, Stroke};
use vello::peniko::Color as VelloColor;
use vello::util::{RenderContext, RenderSurface};
use vello::{AaConfig, Renderer, RendererOptions};

use crate::engine::{Renderer as RendererTrait, WindowProxy};
use crate::render_ir::RenderCommand;

pub struct WinitWindowProxy {
    window: Arc<winit::window::Window>,
}

impl WinitWindowProxy {
    pub fn new(window: Arc<winit::window::Window>) -> Self {
        Self { window }
    }

    pub fn window(&self) -> &Arc<winit::window::Window> {
        &self.window
    }

    pub fn clone_window(&self) -> Arc<winit::window::Window> {
        Arc::clone(&self.window)
    }
}

impl WindowProxy for WinitWindowProxy {
    fn request_redraw(&self) {
        self.window.request_redraw();
    }

    fn scale_factor(&self) -> f64 {
        self.window.scale_factor()
    }

    fn width(&self) -> u32 {
        self.window.inner_size().width
    }

    fn height(&self) -> u32 {
        self.window.inner_size().height
    }
}

type Scene = vello::Scene;

pub struct VelloRenderer {
    render_context: RenderContext,
    renderers: Vec<Option<Renderer>>,
    scene: Scene,
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
            scene: Scene::new(),
            surface,
            window,
            scale_factor,
        }
    }

    fn render_command(scene: &mut Scene, cmd: &RenderCommand, scale: f64) {
        match cmd {
            RenderCommand::FillRect { rect, color, stroke_color, stroke_width } => {
                let x0 = rect[0].x * scale;
                let y0 = rect[0].y * scale;
                let x1 = rect[1].x * scale;
                let y1 = rect[1].y * scale;
                let kurbo_rect = vello::kurbo::Rect::new(x0, y0, x1, y1);

                let vello_color = color.map(|c| {
                    VelloColor::new([c.r as f32, c.g as f32, c.b as f32, c.a as f32])
                }).unwrap_or_else(|| VelloColor::new([0.2, 0.6, 0.86, 1.0]));

                let alpha = vello_color.components[3];
                if alpha == 0.0 {
                    let stroke_vello_color = stroke_color.map(|c| {
                        VelloColor::new([c.r as f32, c.g as f32, c.b as f32, c.a as f32])
                    }).unwrap_or_else(|| VelloColor::new([0.95, 0.61, 0.07, 1.0]));

                    let stroke_width = *stroke_width;
                    if stroke_width > 0.0 {
                        let inset = stroke_width / 2.0;
                        let stroke_rect = vello::kurbo::Rect::new(
                            x0 + inset, y0 + inset,
                            x1 - inset, y1 - inset,
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
}

impl RendererTrait for VelloRenderer {
    type Window = WinitWindowProxy;

    fn window(&self) -> &Self::Window {
        &self.window
    }

    fn render(&mut self, commands: &[RenderCommand]) {
        self.scene.reset();

        let scale = self.scale_factor;
        for cmd in commands {
            Self::render_command(&mut self.scene, cmd, scale);
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
                    base_color: VelloColor::new([0.0, 0.0, 0.0, 0.0]),
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
        self.render_context.resize_surface(&mut self.surface, width, height);
    }
}

fn create_renderer(render_cx: &RenderContext, surface: &RenderSurface<'_>) -> Renderer {
    Renderer::new(
        &render_cx.devices[surface.dev_id].device,
        RendererOptions::default(),
    )
    .expect("Couldn't create renderer")
}
