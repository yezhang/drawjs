use crate::core::color::Color;
use crate::render::RenderContext;
use crate::scene::{Figure, Point, Rect};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RectangleFigure {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub fill_color: Color,
    pub stroke_color: Option<Color>,
    pub stroke_width: f64,
}

impl RectangleFigure {
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self {
            x, y, width, height,
            fill_color: Color::hex("#3498db"),
            stroke_color: None,
            stroke_width: 0.0,
        }
    }

    pub fn new_with_color(x: f64, y: f64, width: f64, height: f64, color: Color) -> Self {
        Self {
            x, y, width, height,
            fill_color: color,
            stroke_color: None,
            stroke_width: 0.0,
        }
    }

    pub fn with_stroke(mut self, color: Color, width: f64) -> Self {
        self.stroke_color = Some(color);
        self.stroke_width = width;
        self
    }

    pub fn translate(&mut self, dx: f64, dy: f64) {
        self.x += dx;
        self.y += dy;
    }
}

impl Figure for RectangleFigure {
    fn bounds(&self) -> Rect {
        Rect::new(self.x, self.y, self.width, self.height)
    }

    fn paint(&self, gc: &mut RenderContext) {
        let origin = gc.transform_point(Point::new(self.x, self.y));
        gc.set_fill_style(self.fill_color);
        gc.draw_rect(origin.x, origin.y, self.width, self.height);

        if let Some(color) = self.stroke_color {
            gc.set_stroke_style(color, self.stroke_width);
            gc.draw_stroke_rect(origin.x, origin.y, self.width, self.height);
        }
    }

    fn paint_highlight(&self, gc: &mut RenderContext) {
        let bounds = self.bounds();
        let origin = gc.transform_point(Point::new(bounds.x, bounds.y));
        gc.set_fill_style(Color::rgba(0.0, 0.0, 0.0, 0.0));
        gc.set_stroke_style(Color::hex("#f39c12"), 2.0);
        gc.draw_stroke_rect(origin.x - 2.0, origin.y - 2.0, bounds.width + 4.0, bounds.height + 4.0);
    }

    fn as_rectangle_mut(&mut self) -> Option<&mut RectangleFigure> {
        Some(self)
    }
}
