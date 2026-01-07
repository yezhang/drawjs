//! Figure 渲染接口
//!
//! 定义图形渲染的通用接口，遵循 Eclipse Draw2D 设计模式。
//! Figure 只负责渲染，不包含状态（状态在 RuntimeBlock 中）。

mod basic;
mod scalable;
pub use scalable::ScalableFigure;
pub use basic::Rectangle;
pub use basic::RectangleFigure;

use novadraw_render::RenderContext;

use crate::scene::{Point, Rect};

/// Figure 渲染 trait
///
/// 所有图形对象都需要实现此 trait。
/// 只包含渲染相关方法，不包含状态（状态在 RuntimeBlock 中）。
pub trait Figure: Send + Sync {
    /// 获取图形边界
    fn bounds(&self) -> Rect;

    /// 命中测试
    fn hit_test(&self, point: Point) -> bool {
        self.bounds().contains(point)
    }

    /// 绘制图形
    fn paint(&self, gc: &mut RenderContext);

    /// 绘制高亮选中状态
    fn paint_highlight(&self, gc: &mut RenderContext);

    /// 作为不可变矩形图形获取
    fn as_rectangle(&self) -> Option<&Rectangle> {
        None
    }

    /// 作为可变矩形图形获取
    fn as_rectangle_mut(&mut self) -> Option<&mut Rectangle> {
        None
    }

    /// 获取名称（用于调试）
    fn name(&self) -> &'static str {
        "Figure"
    }
}

/// 基础图形（默认实现）
///
/// 简单的图形实现，包含边界矩形。
/// 可用于创建占位图形或作为自定义图形的基础。
#[derive(Clone, Copy, Debug)]
pub struct BaseFigure {
    /// 边界矩形
    pub bounds: Rect,
}

impl BaseFigure {
    /// 创建新的基础图形
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self {
            bounds: Rect::new(x, y, width, height),
        }
    }

    /// 设置边界
    pub fn set_bounds(&mut self, x: f64, y: f64, width: f64, height: f64) {
        self.bounds = Rect::new(x, y, width, height);
    }
}

impl Figure for BaseFigure {
    fn bounds(&self) -> Rect {
        self.bounds
    }

    fn paint(&self, _gc: &mut RenderContext) {}

    fn paint_highlight(&self, _gc: &mut RenderContext) {}

    fn name(&self) -> &'static str {
        "BaseFigure"
    }
}
