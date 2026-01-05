use crate::core::color::Color;

#[derive(Clone, Debug)]
pub struct RenderCommand {
    pub kind: RenderCommandKind,
}

#[derive(Clone, Debug)]
pub enum RenderCommandKind {
    FillRect {
        rect: [glam::DVec2; 2],
        color: Option<Color>,
        stroke_color: Option<Color>,
        stroke_width: f64,
    },
}

impl RenderCommand {
    pub fn new_fill_rect(rect: [glam::DVec2; 2], color: Option<Color>) -> Self {
        Self {
            kind: RenderCommandKind::FillRect {
                rect,
                color,
                stroke_color: None,
                stroke_width: 0.0,
            },
        }
    }

    pub fn with_stroke(mut self, stroke_color: Color, stroke_width: f64) -> Self {
        if let RenderCommandKind::FillRect { stroke_color: sc, stroke_width: sw, .. } = &mut self.kind {
            *sc = Some(stroke_color);
            *sw = stroke_width;
        }
        self
    }
}
