//! 根图形

use novadraw_core::Color;
use novadraw_geometry::Rectangle;
use novadraw_render::NdCanvas;

use super::{Bounded, Shape};

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

// 实现 Bounded trait
impl Bounded for RootFigure {
    fn bounds(&self) -> Rectangle {
        self.bounds
    }

    fn set_bounds(&mut self, x: f64, y: f64, width: f64, height: f64) {
        self.bounds = Rectangle::new(x, y, width, height);
    }

    fn name(&self) -> &'static str {
        "RootFigure"
    }
}

// 实现 Shape trait：根图形透明，不渲染自身
impl Shape for RootFigure {
    fn stroke_color(&self) -> Option<Color> {
        None
    }

    fn stroke_width(&self) -> f64 {
        0.0
    }

    fn fill_color(&self) -> Option<Color> {
        None
    }

    fn line_cap(&self) -> novadraw_render::command::LineCap {
        novadraw_render::command::LineCap::default()
    }

    fn line_join(&self) -> novadraw_render::command::LineJoin {
        novadraw_render::command::LineJoin::default()
    }

    fn fill_enabled(&self) -> bool {
        false
    }

    fn outline_enabled(&self) -> bool {
        false
    }

    fn fill_shape(&self, _gc: &mut NdCanvas) {}

    fn outline_shape(&self, _gc: &mut NdCanvas) {}
}
