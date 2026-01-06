//! 可缩放图形
//!
//! 提供 ScalableFigure 装饰器，支持运行时缩放。

use novadraw_render::RenderContext;
use novadraw_math::Transform;

use super::{Figure, Point, Rect};

/// 可缩放图形
///
/// 包装任意 Figure，添加运行时缩放能力。
/// 遵循 Draw2D 的 ScalableFigure 设计模式。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ScalableFigure<F: Figure> {
    inner: F,
    scale: f64,
}

impl<F: Figure> ScalableFigure<F> {
    /// 创建新的可缩放图形
    pub fn new(inner: F) -> Self {
        Self {
            inner,
            scale: 1.0,
        }
    }

    /// 获取缩放比例
    pub fn scale(&self) -> f64 {
        self.scale
    }

    /// 设置缩放比例
    pub fn set_scale(&mut self, scale: f64) {
        self.scale = scale.max(0.0);
    }

    /// 获取内部图形引用
    pub fn inner(&self) -> &F {
        &self.inner
    }

    /// 获取内部图形可变引用
    pub fn inner_mut(&mut self) -> &mut F {
        &mut self.inner
    }

    /// 缩放并返回自身
    pub fn with_scale(mut self, scale: f64) -> Self {
        self.set_scale(scale);
        self
    }
}

impl<F: Figure> Figure for ScalableFigure<F> {
    fn bounds(&self) -> Rect {
        let b = self.inner.bounds();
        Rect::new(b.x, b.y, b.width * self.scale, b.height * self.scale)
    }

    fn hit_test(&self, point: Point) -> bool {
        let scaled_point = Point::new(
            point.x / self.scale,
            point.y / self.scale,
        );
        self.inner.hit_test(scaled_point)
    }

    fn paint(&self, gc: &mut RenderContext) {
        if self.scale == 1.0 {
            self.inner.paint(gc);
        } else {
            let transform = Transform::from_scale(self.scale, self.scale);
            gc.push_transform(transform);
            self.inner.paint(gc);
            gc.pop_transform();
        }
    }

    fn paint_highlight(&self, gc: &mut RenderContext) {
        let transform = Transform::from_scale(self.scale, self.scale);
        gc.push_transform(transform);
        self.inner.paint_highlight(gc);
        gc.pop_transform();
    }

    fn as_rectangle(&self) -> Option<&super::Rectangle> {
        self.inner.as_rectangle()
    }

    fn as_rectangle_mut(&mut self) -> Option<&mut super::Rectangle> {
        self.inner.as_rectangle_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::figure::Rectangle;

    #[test]
    fn test_bounds_scaled() {
        let rect = Rectangle::new(0.0, 0.0, 100.0, 50.0);
        let scalable = ScalableFigure::new(rect).with_scale(2.0);
        let bounds = scalable.bounds();
        assert_eq!(bounds.width, 200.0);
        assert_eq!(bounds.height, 100.0);
    }

    #[test]
    fn test_bounds_unscaled() {
        let rect = Rectangle::new(10.0, 20.0, 100.0, 50.0);
        let scalable = ScalableFigure::new(rect);
        let bounds = scalable.bounds();
        assert_eq!(bounds.x, 10.0);
        assert_eq!(bounds.y, 20.0);
        assert_eq!(bounds.width, 100.0);
        assert_eq!(bounds.height, 50.0);
    }
}
