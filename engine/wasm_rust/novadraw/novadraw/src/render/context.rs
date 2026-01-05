use crate::core::color::Color;
use crate::core::transform::Transform;
use crate::render::command::RenderCommandKind;
use crate::render::RenderCommand;
use glam::DVec2;

pub struct RenderContext {
    pub commands: Vec<RenderCommand>,
    transform_stack: TransformStack,
}

struct TransformStack {
    stack: Vec<Transform>,
}

impl TransformStack {
    fn new() -> Self {
        Self {
            stack: vec![Transform::identity()],
        }
    }

    fn push(&mut self, transform: Transform) {
        let current = self.stack.last().unwrap().clone();
        self.stack.push(current.multiply(&transform));
    }

    fn pop(&mut self) {
        if self.stack.len() > 1 {
            self.stack.pop();
        }
    }

    fn current(&self) -> &Transform {
        self.stack.last().unwrap()
    }
}

impl RenderContext {
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
            transform_stack: TransformStack::new(),
        }
    }

    pub fn push_transform(&mut self, transform: Transform) {
        self.transform_stack.push(transform);
    }

    pub fn pop_transform(&mut self) {
        self.transform_stack.pop();
    }

    pub fn set_fill_style(&mut self, color: Color) {
        let transform = self.transform_stack.current().clone();
        self.commands.push(RenderCommand::new_fill_rect(
            [DVec2::new(0.0, 0.0), DVec2::new(0.0, 0.0)],
            Some(color),
        ));
    }

    pub fn set_stroke_style(&mut self, color: Color, width: f64) {
        if let Some(last_cmd) = self.commands.last_mut() {
            *last_cmd = last_cmd.clone().with_stroke(color, width);
        }
    }

    pub fn draw_rect(&mut self, x: f64, y: f64, width: f64, height: f64) {
        if let Some(last_cmd) = self.commands.last_mut() {
            if let RenderCommandKind::FillRect { rect, .. } = &mut last_cmd.kind {
                rect[0] = DVec2::new(x, y);
                rect[1] = DVec2::new(x + width, y + height);
            }
        }
    }

    pub fn draw_stroke_rect(&mut self, x: f64, y: f64, width: f64, height: f64) {
        if let Some(last_cmd) = self.commands.last_mut() {
            if let RenderCommandKind::FillRect { rect, .. } = &mut last_cmd.kind {
                rect[0] = DVec2::new(x, y);
                rect[1] = DVec2::new(x + width, y + height);
            }
        }
    }

    pub fn transform_point(&self, point: DVec2) -> DVec2 {
        self.transform_stack.current().multiply_point_2d(point)
    }
}
