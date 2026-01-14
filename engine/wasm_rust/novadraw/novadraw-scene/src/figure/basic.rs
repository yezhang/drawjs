//! 基础图形实现
//!
//! 提供矩形、椭圆等基础图形实现。

use novadraw_core::Color;
use novadraw_geometry::Rect;
use novadraw_render::NdCanvas;

use super::Figure;

/// 矩形图形
///
/// 用于渲染矩形形状。
/// 遵循 d2 设计：使用 `bounds: Rect` 统一管理边界，而非独立 x/y/width/height 字段。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rectangle {
    /// 边界矩形（包含 x, y, width, height）
    pub bounds: Rect,
    /// 填充颜色
    pub fill_color: Color,
    /// 边框颜色
    pub stroke_color: Option<Color>,
    /// 边框宽度
    pub stroke_width: f64,
}

impl Rectangle {
    /// 创建矩形
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self {
            bounds: Rect::new(x, y, width, height),
            fill_color: Color::hex("#3498db"),
            stroke_color: None,
            stroke_width: 0.0,
        }
    }

    /// 从 Rect 创建矩形
    pub fn from_bounds(bounds: Rect) -> Self {
        Self {
            bounds,
            fill_color: Color::hex("#3498db"),
            stroke_color: None,
            stroke_width: 0.0,
        }
    }

    /// 创建指定颜色的矩形
    pub fn new_with_color(x: f64, y: f64, width: f64, height: f64, color: Color) -> Self {
        Self {
            bounds: Rect::new(x, y, width, height),
            fill_color: color,
            stroke_color: None,
            stroke_width: 0.0,
        }
    }

    /// 添加边框
    pub fn with_stroke(mut self, color: Color, width: f64) -> Self {
        self.stroke_color = Some(color);
        self.stroke_width = width;
        self
    }

    /// 平移
    pub fn translate(&mut self, dx: f64, dy: f64) {
        self.bounds.x += dx;
        self.bounds.y += dy;
    }
}

impl Figure for Rectangle {
    fn bounds(&self) -> Rect {
        self.bounds
    }

    fn paint_figure(&self, gc: &mut NdCanvas) {
        gc.set_fill_color(self.fill_color);
        gc.fill_rect(self.bounds.x, self.bounds.y, self.bounds.width, self.bounds.height);

        if let Some(color) = self.stroke_color {
            gc.set_stroke_color(color);
            gc.set_line_width(self.stroke_width);
            gc.stroke_rect(self.bounds.x, self.bounds.y, self.bounds.width, self.bounds.height);
        }
    }

    // paint_highlight 使用 Figure trait 默认实现

    fn as_rectangle(&self) -> Option<&Rectangle> {
        Some(self)
    }

    fn as_rectangle_mut(&mut self) -> Option<&mut Rectangle> {
        Some(self)
    }
}

/// 矩形图形类型别名（兼容旧 API）
pub type RectangleFigure = Rectangle;
