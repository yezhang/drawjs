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

/// 空图形（用于占位）
pub struct NullFigure;

impl NullFigure {
    /// 创建新的空图形
    pub fn new() -> Self {
        NullFigure {}
    }
}

impl Figure for NullFigure {
    fn bounds(&self) -> Rect {
        Rect::new(0.0, 0.0, 0.0, 0.0)
    }

    fn paint(&self, _gc: &mut RenderContext) {}

    fn paint_highlight(&self, _gc: &mut RenderContext) {}

    fn name(&self) -> &'static str {
        "NullFigure"
    }
}
