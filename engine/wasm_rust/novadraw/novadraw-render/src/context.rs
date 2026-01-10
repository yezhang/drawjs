//! 渲染上下文
//!
//! 参考 HTML5 Canvas API 设计，维护统一状态栈。

use glam::DVec2;
use novadraw_core::Color;
use novadraw_math::{Mat3, Transform, Vec2};

use crate::command::{LineCap, LineJoin, Path, RenderCommand, RenderCommandKind, NdStateSnapshot};

pub struct NdCanvas {
    commands: Vec<RenderCommand>,
    state_stack: Vec<NdState>,
}

impl NdCanvas {
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
            state_stack: vec![NdState::default()],
        }
    }

    fn current_state(&self) -> &NdState {
        self.state_stack.last().unwrap()
    }

    fn current_state_mut(&mut self) -> &mut NdState {
        self.state_stack.last_mut().unwrap()
    }

    fn create_command(&mut self, kind: RenderCommandKind) {
        let command = RenderCommand {
            kind,
            transform: self.current_state().transform,
        };
        self.commands.push(command);
    }

    pub fn save(&mut self) {
        let state = NdStateSnapshot {
            transform: self.current_state().transform,
            clip: self.current_state().clip,
            fill_color: self.current_state().fill_color,
            stroke_color: self.current_state().stroke_color,
            line_width: self.current_state().line_width,
            line_cap: self.current_state().line_cap,
            line_join: self.current_state().line_join,
        };
        self.create_command(RenderCommandKind::Save { state });
        self.state_stack.push(self.current_state().clone());
    }

    pub fn restore(&mut self) {
        if self.state_stack.len() > 1 {
            self.state_stack.pop();
        }
        self.create_command(RenderCommandKind::Restore);
    }

    pub fn push_transform(&mut self, transform: Transform) {
        self.current_state_mut().transform = self.current_state().transform * transform;
    }

    pub fn pop_transform(&mut self) {}

    pub fn translate(&mut self, x: f64, y: f64) {
        let t = Transform::from_translation(x, y);
        self.current_state_mut().transform = self.current_state().transform * t;
    }

    pub fn rotate(&mut self, angle: f64) {
        let t = Transform::from_rotation(angle);
        self.current_state_mut().transform = self.current_state().transform * t;
    }

    pub fn scale(&mut self, x: f64, y: f64) {
        let t = Transform::from_scale(x, y);
        self.current_state_mut().transform = self.current_state().transform * t;
    }

    pub fn transform(&mut self, a: f64, b: f64, c: f64, d: f64, e: f64, f: f64) {
        let matrix = Mat3::new(a, c, e, b, d, f, 0.0, 0.0, 1.0);
        let t = Transform::new(matrix, Vec2::ZERO);
        self.current_state_mut().transform = self.current_state().transform * t;
    }

    pub fn set_transform(&mut self, a: f64, b: f64, c: f64, d: f64, e: f64, f: f64) {
        let matrix = Mat3::new(a, c, e, b, d, f, 0.0, 0.0, 1.0);
        self.current_state_mut().transform = Transform::new(matrix, Vec2::ZERO);
    }

    pub fn reset_transform(&mut self) {
        self.current_state_mut().transform = Transform::IDENTITY;
    }

    pub fn set_fill_color(&mut self, color: Color) {
        self.current_state_mut().fill_color = Some(color);
    }

    pub fn set_stroke_color(&mut self, color: Color) {
        self.current_state_mut().stroke_color = Some(color);
    }

    pub fn set_line_width(&mut self, width: f64) {
        self.current_state_mut().line_width = width;
    }

    pub fn set_line_cap(&mut self, cap: LineCap) {
        self.current_state_mut().line_cap = cap;
    }

    pub fn set_line_join(&mut self, join: LineJoin) {
        self.current_state_mut().line_join = join;
    }

    pub fn clear_rect(&mut self, x: f64, y: f64, width: f64, height: f64) {
        let rect = [DVec2::new(x, y), DVec2::new(x + width, y + height)];
        self.create_command(RenderCommandKind::ClearRect { rect });
    }

    pub fn fill_rect(&mut self, x: f64, y: f64, width: f64, height: f64) {
        let rect = [DVec2::new(x, y), DVec2::new(x + width, y + height)];
        let fill_color = self.current_state().fill_color.unwrap_or(Color::BLACK);
        self.create_command(RenderCommandKind::FillRect { rect, fill_color });
    }

    pub fn stroke_rect(&mut self, x: f64, y: f64, width: f64, height: f64) {
        let rect = [DVec2::new(x, y), DVec2::new(x + width, y + height)];
        let stroke_color = self.current_state().stroke_color.unwrap_or(Color::BLACK);
        let width = self.current_state().line_width;
        self.create_command(RenderCommandKind::StrokeRect { rect, stroke_color, width });
    }

    pub fn begin_path(&mut self) {
        self.current_state_mut().current_path = Some(Path::new());
    }

    pub fn close_path(&mut self) {
        if let Some(path) = self.current_state_mut().current_path.as_mut() {
            path.close();
        }
    }

    pub fn move_to(&mut self, x: f64, y: f64) {
        if let Some(path) = self.current_state_mut().current_path.as_mut() {
            path.move_to(x, y);
        }
    }

    pub fn line_to(&mut self, x: f64, y: f64) {
        if let Some(path) = self.current_state_mut().current_path.as_mut() {
            path.line_to(x, y);
        }
    }

    pub fn rect_path(&mut self, x: f64, y: f64, width: f64, height: f64) {
        if let Some(path) = self.current_state_mut().current_path.as_mut() {
            path.rect(x, y, width, height);
        }
    }

    pub fn arc(&mut self, _x: f64, _y: f64, _radius: f64, _start_angle: f64, _end_angle: f64, _anticlockwise: bool) {}

    pub fn quadratic_curve_to(&mut self, _cpx: f64, _cpy: f64, _x: f64, _y: f64) {}

    pub fn bezier_curve_to(&mut self, _cp1x: f64, _cp1y: f64, _cp2x: f64, _cp2y: f64, _x: f64, _y: f64) {}

    pub fn fill(&mut self) {
        if let Some(path) = self.current_state_mut().current_path.take() {
            self.create_command(RenderCommandKind::FillPath(path));
        }
    }

    pub fn stroke(&mut self) {
        if let Some(path) = self.current_state_mut().current_path.take() {
            self.create_command(RenderCommandKind::StrokePath(path));
        }
    }

    pub fn clip(&mut self) {
        if let Some(rect) = self.current_state_mut().clip {
            self.create_command(RenderCommandKind::Clip { rect });
        }
    }

    pub fn clip_rect(&mut self, x: f64, y: f64, width: f64, height: f64) {
        let rect = [DVec2::new(x, y), DVec2::new(x + width, y + height)];
        self.current_state_mut().clip = Some(rect);
        self.create_command(RenderCommandKind::Clip { rect });
    }

    pub fn reset_clip(&mut self) {
        self.current_state_mut().clip = None;
    }

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

    pub fn line_cap(&mut self, _cap: LineCap) {}

    pub fn line_join(&mut self, _join: LineJoin) {}

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

    pub fn draw_image_with_size(&mut self, _image: &crate::command::ImageData, _x: f64, _y: f64, _width: f64, _height: f64) {}

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
        if self.current_state().clip.is_some() {
            1
        } else {
            0
        }
    }
}

#[derive(Clone, Debug)]
pub struct NdState {
    pub transform: Transform,
    pub clip: Option<[DVec2; 2]>,
    pub fill_color: Option<Color>,
    pub stroke_color: Option<Color>,
    pub line_width: f64,
    pub line_cap: LineCap,
    pub line_join: LineJoin,
    pub current_path: Option<Path>,
}

impl Default for NdState {
    fn default() -> Self {
        Self {
            transform: Transform::IDENTITY,
            clip: None,
            fill_color: None,
            stroke_color: None,
            line_width: 1.0,
            line_cap: LineCap::Butt,
            line_join: LineJoin::Miter,
            current_path: None,
        }
    }
}
