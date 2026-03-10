//! MarginBorder 边距边框
//!
//! 参考 Eclipse Draw2D 的 MarginBorder 实现。
//!
//! 主要用于提供内边距（insets），影响子元素布局。
//! paint() 方法为空实现，不绘制任何可见内容。

use novadraw_core::Color;
use novadraw_geometry::Rectangle;
use novadraw_render::NdCanvas;

use super::{Border, BorderStyle, DEFAULT_BORDER_WIDTH};

/// 边距边框
///
/// 参考 d2: MarginBorder 提供内边距，不绘制可见内容。
///
/// # 与 d2 的差异
///
/// d2 中 MarginBorder 的 paint() 为空，仅用于布局（提供 insets）。
/// novadraw 当前未实现布局系统，所以 insets 效果不会体现。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MarginBorder {
    /// 边框颜色
    pub color: Color,
    /// 边框宽度
    pub width: f64,
    /// 边框样式
    pub style: super::BorderStyle,
    /// 上边距（内边距）
    pub top: f64,
    /// 左边距（内边距）
    pub left: f64,
    /// 下边距（内边距）
    pub bottom: f64,
    /// 右边距（内边距）
    pub right: f64,
}

impl MarginBorder {
    /// 创建边距边框
    pub fn new(color: Color, width: f64) -> Self {
        Self {
            color,
            width,
            style: BorderStyle::Solid,
            top: 0.0,
            left: 0.0,
            bottom: 0.0,
            right: 0.0,
        }
    }

    /// 创建默认边距边框
    pub fn default_border() -> Self {
        Self::new(Color::rgba(0.0, 0.0, 0.0, 1.0), DEFAULT_BORDER_WIDTH)
    }

    /// 设置上边距
    pub fn with_top(mut self, top: f64) -> Self {
        self.top = top;
        self
    }

    /// 设置左边距
    pub fn with_left(mut self, left: f64) -> Self {
        self.left = left;
        self
    }

    /// 设置下边距
    pub fn with_bottom(mut self, bottom: f64) -> Self {
        self.bottom = bottom;
        self
    }

    /// 设置右边距
    pub fn with_right(mut self, right: f64) -> Self {
        self.right = right;
        self
    }

    /// 设置所有边距
    pub fn with_margins(mut self, top: f64, left: f64, bottom: f64, right: f64) -> Self {
        self.top = top;
        self.left = left;
        self.bottom = bottom;
        self.right = right;
        self
    }

    /// 设置边框样式
    pub fn with_style(mut self, style: BorderStyle) -> Self {
        self.style = style;
        self
    }
}

impl Border for MarginBorder {
    fn get_insets(&self) -> (f64, f64, f64, f64) {
        (self.top, self.left, self.bottom, self.right)
    }

    /// 绘制边框
    ///
    /// 参考 d2: MarginBorder.paint() 为空实现，不绘制任何可见内容。
    /// 主要用于提供内边距（insets），影响子元素布局。
    fn paint(&self, _figure_bounds: Rectangle, _gc: &mut NdCanvas) {
        // 空实现：MarginBorder 不绘制可见内容
        // insets 由 get_insets() 提供，用于布局
    }

    fn get_color(&self) -> Color {
        self.color
    }

    fn get_width(&self) -> f64 {
        self.width
    }
}
