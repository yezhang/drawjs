//! 基础图形实现
//!
//! 提供矩形、椭圆等基础图形实现。

use novadraw_core::Color;
use novadraw_render::RenderContext;

use super::{Figure, Point, Rect};

/// 矩形图形
///
/// 用于渲染矩形形状。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rectangle {
    /// X 坐标
    pub x: f64,
    /// Y 坐标
    pub y: f64,
    /// 宽度
    pub width: f64,
    /// 高度
    pub height: f64,
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
            x, y, width, height,
            fill_color: Color::hex("#3498db"),
            stroke_color: None,
            stroke_width: 0.0,
        }
    }

    /// 创建指定颜色的矩形
    pub fn new_with_color(x: f64, y: f64, width: f64, height: f64, color: Color) -> Self {
        Self {
            x, y, width, height,
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
        self.x += dx;
        self.y += dy;
    }
}

impl Figure for Rectangle {
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

    fn as_rectangle(&self) -> Option<&Rectangle> {
        Some(self)
    }

    fn as_rectangle_mut(&mut self) -> Option<&mut Rectangle> {
        Some(self)
    }
}

/// 矩形图形类型别名（兼容旧 API）
pub type RectangleFigure = Rectangle;
