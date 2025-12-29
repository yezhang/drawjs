use glam::DVec2;

use crate::color::Color;

pub enum RenderCommand {
    FillRect {
        rect: [DVec2; 2],
        color: Option<Color>,
        corner_radius: f64,
    },
}
