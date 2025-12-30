use glam::DVec2;

use crate::color::Color;

#[derive(Debug)]
pub enum RenderCommand {
    FillRect {
        rect: [DVec2; 2],
        color: Option<Color>,
        stroke_color: Option<Color>,
        stroke_width: f64,
    },
}
