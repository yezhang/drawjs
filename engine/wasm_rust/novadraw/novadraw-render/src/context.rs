//! 渲染上下文
//!
//! 参考 HTML5 Canvas API 设计，直接生成命令（无状态栈）。
//! 状态管理由 VelloRenderer 执行器负责。

use glam::DVec2;
use novadraw_core::Color;
use novadraw_geometry::Transform;

use crate::command::{RenderCommand, RenderCommandKind};

pub struct NdCanvas {
    commands: Vec<RenderCommand>,
}

impl NdCanvas {
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
        }
    }

    fn create_command(&mut self, kind: RenderCommandKind) {
        let command = RenderCommand { kind };
        self.commands.push(command);
    }

    /// 保存当前状态（压栈）
    ///
    /// 对应 Draw2D: Graphics.pushState()
    /// 将当前状态复制并压入状态栈
    pub fn push_state(&mut self) {
        self.create_command(RenderCommandKind::PushState);
    }

    /// 恢复到最近一次 pushState 的状态（不弹出栈）
    ///
    /// 对应 Draw2D: Graphics.restoreState()
    /// 用于在 paintFigure 之后、paintChildren 之前恢复裁剪区
    pub fn restore_state(&mut self) {
        self.create_command(RenderCommandKind::RestoreState);
    }

    /// 弹出并恢复状态
    ///
    /// 对应 Draw2D: Graphics.popState()
    /// 用于在所有绘制完成后恢复 pushState 前的状态
    pub fn pop_state(&mut self) {
        self.create_command(RenderCommandKind::PopState);
    }

    /// 平移
    ///
    /// 生成 ConcatTransform 命令
    pub fn translate(&mut self, x: f64, y: f64) {
        let t = Transform::from_translation(x, y);
        self.create_command(RenderCommandKind::ConcatTransform { matrix: t });
    }

    /// 旋转
    ///
    /// 生成 ConcatTransform 命令
    pub fn rotate(&mut self, angle: f64) {
        let t = Transform::from_rotation(angle);
        self.create_command(RenderCommandKind::ConcatTransform { matrix: t });
    }

    /// 缩放
    ///
    /// 生成 ConcatTransform 命令
    pub fn scale(&mut self, x: f64, y: f64) {
        let t = Transform::from_scale(x, y);
        self.create_command(RenderCommandKind::ConcatTransform { matrix: t });
    }

    pub fn transform(&mut self, a: f64, b: f64, c: f64, d: f64, e: f64, f: f64) {
        let t = Transform::new(a, b, c, d, e, f);
        self.create_command(RenderCommandKind::ConcatTransform { matrix: t });
    }

    pub fn set_transform(&mut self, a: f64, b: f64, c: f64, d: f64, e: f64, f: f64) {
        let t = Transform::new(a, b, c, d, e, f);
        self.create_command(RenderCommandKind::ConcatTransform { matrix: t });
    }

    pub fn reset_transform(&mut self) {
        let t = Transform::IDENTITY;
        self.create_command(RenderCommandKind::ConcatTransform { matrix: t });
    }

    pub fn clear_rect(&mut self, x: f64, y: f64, width: f64, height: f64, color: Color) {
        let rect = [DVec2::new(x, y), DVec2::new(x + width, y + height)];
        self.create_command(RenderCommandKind::ClearRect { rect, color });
    }

    pub fn fill_rect(&mut self, x: f64, y: f64, width: f64, height: f64, color: Color) {
        let rect = [DVec2::new(x, y), DVec2::new(x + width, y + height)];
        self.create_command(RenderCommandKind::FillRect { rect, color });
    }

    pub fn stroke_rect(&mut self, x: f64, y: f64, width: f64, height: f64, color: Color, stroke_width: f64) {
        let rect = [DVec2::new(x, y), DVec2::new(x + width, y + height)];
        self.create_command(RenderCommandKind::StrokeRect { rect, color, width: stroke_width });
    }

    pub fn begin_path(&mut self) {}

    pub fn close_path(&mut self) {}

    pub fn move_to(&mut self, _x: f64, _y: f64) {}

    pub fn line_to(&mut self, _x: f64, _y: f64) {}

    pub fn rect_path(&mut self, _x: f64, _y: f64, _width: f64, _height: f64) {}

    pub fn arc(
        &mut self,
        _x: f64,
        _y: f64,
        _radius: f64,
        _start_angle: f64,
        _end_angle: f64,
        _anticlockwise: bool,
    ) {
    }

    pub fn quadratic_curve_to(&mut self, _cpx: f64, _cpy: f64, _x: f64, _y: f64) {}

    pub fn bezier_curve_to(
        &mut self,
        _cp1x: f64,
        _cp1y: f64,
        _cp2x: f64,
        _cp2y: f64,
        _x: f64,
        _y: f64,
    ) {
    }

    pub fn fill(&mut self) {}

    pub fn stroke(&mut self) {}

    pub fn clip_rect(&mut self, x: f64, y: f64, width: f64, height: f64) {
        let rect = [DVec2::new(x, y), DVec2::new(x + width, y + height)];
        self.create_command(RenderCommandKind::Clip { rect });
    }

    pub fn reset_clip(&mut self) {}

    pub fn clear_commands(&mut self) {
        self.commands.clear();
    }

    pub fn commands(&self) -> &Vec<RenderCommand> {
        &self.commands
    }

    pub fn commands_mut(&mut self) -> &mut Vec<RenderCommand> {
        &mut self.commands
    }

    pub fn fill_style(&mut self, _color: Color) {}

    pub fn stroke_style(&mut self, _color: Color) {}

    pub fn line_width(&mut self, _width: f64) {}

    pub fn line_cap(&mut self, _cap: crate::command::LineCap) {}

    pub fn line_join(&mut self, _join: crate::command::LineJoin) {}

    pub fn line_dash_offset(&mut self, _offset: f64) {}

    pub fn set_line_dash(&mut self, _dash: &[f64]) {}

    pub fn miter_limit(&mut self, _limit: f64) {}

    pub fn font(&mut self, _font: &str) {}

    pub fn text_align(&mut self, _align: &str) {}

    pub fn text_baseline(&mut self, _baseline: &str) {}

    pub fn fill_text(&mut self, _text: &str, _x: f64, _y: f64) {}

    pub fn stroke_text(&mut self, _text: &str, _x: f64, _y: f64) {}

    pub fn measure_text(&mut self, _text: &str) -> f64 {
        0.0
    }

    pub fn draw_image(&mut self, _image: &crate::command::ImageData, _x: f64, _y: f64) {}

    pub fn draw_image_with_size(
        &mut self,
        _image: &crate::command::ImageData,
        _x: f64,
        _y: f64,
        _width: f64,
        _height: f64,
    ) {
    }

    pub fn global_alpha(&mut self, _alpha: f64) {}

    pub fn global_composite_operation(&mut self, _op: &str) {}

    pub fn shadow_color(&mut self, _color: Color) {}

    pub fn shadow_blur(&mut self, _blur: f64) {}

    pub fn shadow_offset_x(&mut self, _offset: f64) {}

    pub fn shadow_offset_y(&mut self, _offset: f64) {}

    pub fn is_point_in_path(&mut self, _x: f64, _y: f64) -> bool {
        false
    }

    pub fn is_point_in_stroke(&mut self, _x: f64, _y: f64) -> bool {
        false
    }

    pub fn clip_depth(&self) -> usize {
        0
    }
}
