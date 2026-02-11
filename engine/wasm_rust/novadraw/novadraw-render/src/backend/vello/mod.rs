//! Vello 渲染器实现
//!
//! 实现 RenderCommand 解释器，维护独立的状态栈。
//! 状态管理从 NdCanvas 移到本模块（参考 skia/Flutter DisplayList 设计）。

use std::sync::Arc;

use glam::DVec2;
use novadraw_geometry::Transform;
use vello::kurbo::Stroke;
use vello::peniko::Color as VelloColor;
use vello::util::{RenderContext, RenderSurface};
use vello::{AaConfig, Renderer, RendererOptions};

use crate::command::RenderCommand;
use crate::traits::{RenderBackend, WindowProxy};

pub mod winit;
pub use winit::{WinitWindowProxy, WinitWindowProxyInner};

/// 渲染状态
#[derive(Clone, Debug, Default)]
struct RenderState {
    /// 当前变换矩阵
    transform: Transform,
    /// 当前裁剪区域
    clip: Option<[DVec2; 2]>,
}

pub struct VelloRenderer {
    render_context: RenderContext,
    renderers: Vec<Option<Renderer>>,
    scene: vello::Scene,
    surface: RenderSurface<'static>,
    window: Arc<WinitWindowProxy>,
    scale_factor: f64,
    /// 状态栈
    state_stack: Vec<RenderState>,
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
        renderers[surface.dev_id].get_or_insert_with(|| create_renderer(&render_context, &surface));

        VelloRenderer {
            render_context,
            renderers,
            scene: vello::Scene::new(),
            surface,
            window,
            scale_factor,
            state_stack: vec![RenderState::default()],
        }
    }

    /// 获取当前状态
    fn current_state(&self) -> &RenderState {
        self.state_stack.last().unwrap()
    }

    /// 获取可变当前状态
    fn current_state_mut(&mut self) -> &mut RenderState {
        self.state_stack.last_mut().unwrap()
    }

    /// 处理单个渲染命令
    fn render_command(&mut self, cmd: &RenderCommand) {
        match &cmd.kind {
            // ===== 状态管理命令 =====
            crate::command::RenderCommandKind::PushState => {
                // 保存当前状态到栈
                self.state_stack.push(self.current_state().clone());

                // 应用裁剪区域
                if let Some(clip) = self.current_state().clip {
                    self.push_clip_layer(&clip);
                }
            }

            crate::command::RenderCommandKind::RestoreState => {
                // 恢复到最近保存状态，不弹出
                // 恢复到栈顶-2（即最近一次 pushState 保存的状态）
                if self.state_stack.len() >= 2 {
                    let last_idx = self.state_stack.len() - 1;
                    let saved_idx = self.state_stack.len() - 2;
                    // 检查当前状态是否有 clip（需要弹出）
                    let current_has_clip = self.state_stack[last_idx].clip.is_some();
                    // 只有当当前状态有 clip 时，才弹出层
                    if current_has_clip {
                        self.scene.pop_layer();
                    }
                    self.state_stack[last_idx] = self.state_stack[saved_idx].clone();
                }
            }

            crate::command::RenderCommandKind::PopState => {
                // 弹出并恢复状态
                if self.state_stack.len() > 1 {
                    if self.current_state().clip.is_some() {
                        // 弹出裁剪层
                        self.scene.pop_layer();
                    }
                    self.state_stack.pop();
                }
            }

            crate::command::RenderCommandKind::ConcatTransform { matrix } => {
                // 叠加变换
                let new_transform = self.current_state().transform.then_transform(*matrix);

                self.current_state_mut().transform = new_transform;
            }

            crate::command::RenderCommandKind::Clip { rect } => {
                self.current_state_mut().clip = Some(*rect);
                self.push_clip_layer(rect);
            }

            // ===== 绘制命令 =====
            crate::command::RenderCommandKind::ClearRect { rect, color } => {
                let affine =
                    Self::transform_to_affine(&self.current_state().transform, self.scale_factor);
                let x0 = rect[0].x * self.scale_factor;
                let y0 = rect[0].y * self.scale_factor;
                let x1 = rect[1].x * self.scale_factor;
                let y1 = rect[1].y * self.scale_factor;
                let kurbo_rect = vello::kurbo::Rect::new(x0, y0, x1, y1);
                let vello_color = VelloColor::new([
                    color.r as f32,
                    color.g as f32,
                    color.b as f32,
                    color.a as f32,
                ]);
                self.scene.fill(
                    vello::peniko::Fill::NonZero,
                    affine,
                    vello_color,
                    None,
                    &kurbo_rect,
                );
            }

            crate::command::RenderCommandKind::FillRect { rect, color } => {
                let affine =
                    Self::transform_to_affine(&self.current_state().transform, self.scale_factor);
                let x0 = rect[0].x * self.scale_factor;
                let y0 = rect[0].y * self.scale_factor;
                let x1 = rect[1].x * self.scale_factor;
                let y1 = rect[1].y * self.scale_factor;
                let kurbo_rect = vello::kurbo::Rect::new(x0, y0, x1, y1);
                let vello_color = VelloColor::new([
                    color.r as f32,
                    color.g as f32,
                    color.b as f32,
                    color.a as f32,
                ]);
                self.scene.fill(
                    vello::peniko::Fill::NonZero,
                    affine,
                    vello_color,
                    None,
                    &kurbo_rect,
                );
            }

            crate::command::RenderCommandKind::StrokeRect { rect, color, width } => {
                let affine =
                    Self::transform_to_affine(&self.current_state().transform, self.scale_factor);
                let x0 = rect[0].x * self.scale_factor;
                let y0 = rect[0].y * self.scale_factor;
                let x1 = rect[1].x * self.scale_factor;
                let y1 = rect[1].y * self.scale_factor;
                let kurbo_rect = vello::kurbo::Rect::new(x0, y0, x1, y1);
                let vello_color = VelloColor::new([
                    color.r as f32,
                    color.g as f32,
                    color.b as f32,
                    color.a as f32,
                ]);
                self.scene.stroke(
                    &Stroke::new(*width * self.scale_factor),
                    affine,
                    vello_color,
                    None,
                    &kurbo_rect,
                );
            }

            crate::command::RenderCommandKind::Clear { color: _ } => {
                // 未实现
            }

            crate::command::RenderCommandKind::Line {
                p1,
                p2,
                color,
                width,
                cap: _,
                join: _,
            } => {
                let affine =
                    Self::transform_to_affine(&self.current_state().transform, self.scale_factor);
                let v1 =
                    vello::kurbo::Point::new(p1.x * self.scale_factor, p1.y * self.scale_factor);
                let v2 =
                    vello::kurbo::Point::new(p2.x * self.scale_factor, p2.y * self.scale_factor);
                let vello_color = VelloColor::new([
                    color.r as f32,
                    color.g as f32,
                    color.b as f32,
                    color.a as f32,
                ]);

                self.scene.stroke(
                    &Stroke::new(*width * self.scale_factor),
                    affine,
                    vello_color,
                    None,
                    &vello::kurbo::Line::new(v1, v2),
                );
            }

            crate::command::RenderCommandKind::Ellipse {
                cx,
                cy,
                rx,
                ry,
                fill_color,
                stroke_color,
                stroke_width,
            } => {
                let affine =
                    Self::transform_to_affine(&self.current_state().transform, self.scale_factor);
                let center = vello::kurbo::Point::new(cx * self.scale_factor, cy * self.scale_factor);
                let radii = vello::kurbo::Vec2::new(*rx * self.scale_factor, *ry * self.scale_factor);
                let ellipse = vello::kurbo::Ellipse::new(center, radii, 0.0);

                // 填充椭圆
                if let Some(color) = fill_color {
                    let vello_color = VelloColor::new([
                        color.r as f32,
                        color.g as f32,
                        color.b as f32,
                        color.a as f32,
                    ]);
                    self.scene.fill(
                        vello::peniko::Fill::NonZero,
                        affine,
                        vello_color,
                        None,
                        &ellipse,
                    );
                }

                // 描边椭圆
                if let Some(color) = stroke_color {
                    let vello_color = VelloColor::new([
                        color.r as f32,
                        color.g as f32,
                        color.b as f32,
                        color.a as f32,
                    ]);
                    self.scene.stroke(
                        &Stroke::new(*stroke_width * self.scale_factor),
                        affine,
                        vello_color,
                        None,
                        &ellipse,
                    );
                }
            }

            // 其他命令暂未实现
            _ => {}
        }
    }

    /// 将 Transform 转换为 vello Affine
    fn transform_to_affine(transform: &Transform, scale_factor: f64) -> vello::kurbo::Affine {
        let coeffs = transform.coeffs();
        // coeffs = [a, b, c, d, e, f]
        // Apply scale factor to translation components (e, f)
        vello::kurbo::Affine::new([
            coeffs[0],
            coeffs[1],
            coeffs[2],
            coeffs[3],
            coeffs[4] * scale_factor,
            coeffs[5] * scale_factor,
        ])
    }

    /// 推送裁剪层到场景
    fn push_clip_layer(&mut self, rect: &[DVec2; 2]) {
        let affine =
            Self::transform_to_affine(&self.current_state().transform, self.scale_factor);
        let x0 = rect[0].x * self.scale_factor;
        let y0 = rect[0].y * self.scale_factor;
        let x1 = rect[1].x * self.scale_factor;
        let y1 = rect[1].y * self.scale_factor;
        let kurbo_rect = vello::kurbo::Rect::new(x0, y0, x1, y1);
        self.scene
            .push_clip_layer(vello::peniko::Fill::NonZero, affine, &kurbo_rect);
    }
}

impl RenderBackend for VelloRenderer {
    type Window = WinitWindowProxy;

    fn window(&self) -> &Self::Window {
        &self.window
    }

    fn render(&mut self, commands: &[RenderCommand]) {
        self.scene.reset();
        // self.scene = vello::Scene::new();

        // 重置状态栈
        self.state_stack.clear();
        self.state_stack.push(RenderState::default());

        // 按顺序解释执行命令
        for cmd in commands {
            self.render_command(cmd);
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

        let mut encoder =
            device_handle
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

    fn resize(&mut self, pixel_width: u32, pixel_height: u32, scale_factor: f64) {
        self.scale_factor = scale_factor;
        self.render_context
            .resize_surface(&mut self.surface, pixel_width, pixel_height);
    }
}

impl VelloRenderer {
    /// 重新创建 surface（用于 resize 时确保配置更新）
    pub fn recreate_surface(&mut self, pixel_width: u32, pixel_height: u32) {
        let surface_future = self.render_context.create_surface(
            self.window.window().clone(),
            pixel_width,
            pixel_height,
            vello::wgpu::PresentMode::AutoVsync,
        );
        self.surface = pollster::block_on(surface_future).expect("Failed to recreate surface");
    }
}

fn create_renderer(render_cx: &RenderContext, surface: &RenderSurface<'_>) -> Renderer {
    Renderer::new(
        &render_cx.devices[surface.dev_id].device,
        RendererOptions::default(),
    )
    .expect("Couldn't create renderer")
}
