use crate::color::Color;
use crate::render_ir::RenderCommand;
use glam::DVec2;

pub struct RenderContext {
    pub commands: Vec<RenderCommand>,
    current_fill: Option<Color>,
}

impl RenderContext {
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
            current_fill: None,
        }
    }

    pub fn set_fill_style(&mut self, color: Color) {
        self.current_fill = Some(color);
    }

    pub fn draw_rect(&mut self, x: f64, y: f64, width: f64, height: f64) {
        let color = self.current_fill.take();

        self.commands.push(RenderCommand::FillRect {
            rect: [DVec2::new(x, y), DVec2::new(x + width, y + height)],
            color,
            corner_radius: 0.0,
        });
    }
}
