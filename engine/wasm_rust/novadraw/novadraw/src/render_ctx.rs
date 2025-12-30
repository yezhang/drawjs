use crate::color::Color;
use crate::render_ir::RenderCommand;
use glam::DVec2;

pub struct RenderContext {
    pub commands: Vec<RenderCommand>,
    current_fill: Option<Color>,
    current_stroke: Option<(Color, f64)>,
}

impl RenderContext {
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
            current_fill: None,
            current_stroke: None,
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
}
