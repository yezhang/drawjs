pub mod basic;

pub use basic::RectangleFigure;

pub trait Figure: Send + Sync {
    fn bounds(&self) -> super::Rect;
    fn hit_test(&self, point: super::Point) -> bool {
        self.bounds().contains(point)
    }
    fn paint(&self, gc: &mut crate::render::RenderContext);
    fn paint_highlight(&self, gc: &mut crate::render::RenderContext);
    fn as_rectangle_mut(&mut self) -> Option<&mut basic::RectangleFigure> {
        None
    }
}

pub struct NullFigure;

impl NullFigure {
    pub fn new() -> Self {
        NullFigure {}
    }
}

impl Figure for NullFigure {
    fn bounds(&self) -> super::Rect {
        super::Rect::new(0.0, 0.0, 0.0, 0.0)
    }

    fn paint(&self, _gc: &mut crate::render::RenderContext) {}
    fn paint_highlight(&self, _gc: &mut crate::render::RenderContext) {}
}
