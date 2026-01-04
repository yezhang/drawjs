use crate::color::Color;
use crate::render_ir::RenderCommand;
use crate::transform::{Transform, TransformStack};
use glam::DVec2;

pub struct RenderContext {
    pub commands: Vec<RenderCommand>,
    current_fill: Option<Color>,
    current_stroke: Option<(Color, f64)>,
    transform_stack: TransformStack,
}

impl RenderContext {
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
            current_fill: None,
            current_stroke: None,
            transform_stack: TransformStack::new(),
        }
    }

    pub fn set_fill_style(&mut self, color: Color) {
        self.current_fill = Some(color);
    }

    pub fn set_stroke_style(&mut self, color: Color, width: f64) {
        self.current_stroke = Some((color, width));
    }

    pub fn draw_rect(&mut self, x: f64, y: f64, width: f64, height: f64) {
        let color = self.current_fill.take();
        let stroke = self.current_stroke.take();

        self.commands.push(RenderCommand::FillRect {
            rect: [DVec2::new(x, y), DVec2::new(x + width, y + height)],
            color,
            stroke_color: stroke.map(|s| s.0),
            stroke_width: stroke.map(|s| s.1).unwrap_or(0.0),
        });
    }

    pub fn draw_stroke_rect(&mut self, x: f64, y: f64, width: f64, height: f64) {
        let stroke = self.current_stroke.take();
        self.commands.push(RenderCommand::FillRect {
            rect: [DVec2::new(x, y), DVec2::new(x + width, y + height)],
            color: Some(Color::rgba(0.0, 0.0, 0.0, 0.0)),
            stroke_color: stroke.map(|s| s.0),
            stroke_width: stroke.map(|s| s.1).unwrap_or(0.0),
        });
    }

    pub fn push_transform(&mut self, transform: Transform) {
        self.transform_stack.push(transform);
    }

    pub fn pop_transform(&mut self) {
        self.transform_stack.pop();
    }

    pub fn current_transform(&self) -> Transform {
        self.transform_stack.current()
    }

    pub fn transform_point(&self, point: DVec2) -> DVec2 {
        self.transform_stack.current().multiply_point_2d(point)
    }
}
