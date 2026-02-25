//! 根图形

use novadraw_geometry::Rectangle;
use novadraw_render::NdCanvas;

use super::Figure;

/// 根图形（内部使用）
///
/// 参考 d2 的 LightweightSystem.RootFigure 设计。
/// 用于表示 SceneGraph 内部的根容器，与用户设置的图形根区分。
///
/// 特点：
/// - 透明（不渲染自身）
/// - 使用 trait 默认的本地坐标模式（false）
/// - 不需要填充/描边属性
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RootFigure {
    /// 边界矩形
    bounds: Rectangle,
}

impl RootFigure {
    /// 创建新的根图形
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self {
            bounds: Rectangle::new(x, y, width, height),
        }
    }

    /// 设置边界
    pub fn set_bounds(&mut self, x: f64, y: f64, width: f64, height: f64) {
        self.bounds = Rectangle::new(x, y, width, height);
    }
}

impl Figure for RootFigure {
    fn bounds(&self) -> Rectangle {
        self.bounds
    }

    /// 根图形不渲染自身（透明）
    fn paint_figure(&self, _gc: &mut NdCanvas) {
        // 空实现：根图形透明，不渲染自身
    }

    fn set_bounds(&mut self, x: f64, y: f64, width: f64, height: f64) {
        self.bounds = Rectangle::new(x, y, width, height);
    }

    fn name(&self) -> &'static str {
        "RootFigure"
    }
}
