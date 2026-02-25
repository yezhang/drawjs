//! 椭圆图形

use novadraw_core::Color;
use novadraw_geometry::Rectangle;
use novadraw_render::NdCanvas;

use super::{Figure, Shape};

/// 椭圆图形
///
/// 用于渲染椭圆形状。
/// 椭圆外切于 bounds 矩形。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EllipseFigure {
    /// 边界矩形（外切矩形）
    pub bounds: Rectangle,
    /// 填充颜色
    pub fill_color: Color,
    /// 边框颜色
    pub stroke_color: Option<Color>,
    /// 边框宽度
    pub stroke_width: f64,
    /// 线帽样式
    pub line_cap: novadraw_render::command::LineCap,
    /// 连接样式
    pub line_join: novadraw_render::command::LineJoin,
}

impl EllipseFigure {
    /// 创建椭圆
    ///
    /// 椭圆外切于指定的 bounds 矩形
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self {
            bounds: Rectangle::new(x, y, width, height),
            fill_color: Color::hex("#e74c3c"),
            stroke_color: None,
            stroke_width: 0.0,
            line_cap: novadraw_render::command::LineCap::default(),
            line_join: novadraw_render::command::LineJoin::default(),
        }
    }

    /// 从 Rectangle 创建椭圆
    pub fn from_bounds(bounds: Rectangle) -> Self {
        Self {
            bounds,
            fill_color: Color::hex("#e74c3c"),
            stroke_color: None,
            stroke_width: 0.0,
            line_cap: novadraw_render::command::LineCap::default(),
            line_join: novadraw_render::command::LineJoin::default(),
        }
    }

    /// 创建指定颜色的椭圆
    pub fn new_with_color(x: f64, y: f64, width: f64, height: f64, color: Color) -> Self {
        Self {
            bounds: Rectangle::new(x, y, width, height),
            fill_color: color,
            stroke_color: None,
            stroke_width: 0.0,
            line_cap: novadraw_render::command::LineCap::default(),
            line_join: novadraw_render::command::LineJoin::default(),
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

    /// 设置边界
    pub fn set_bounds(&mut self, x: f64, y: f64, width: f64, height: f64) {
        self.bounds = Rectangle::new(x, y, width, height);
    }

    /// 获取椭圆中心 x
    pub fn cx(&self) -> f64 {
        self.bounds.x + self.bounds.width / 2.0
    }

    /// 获取椭圆中心 y
    pub fn cy(&self) -> f64 {
        self.bounds.y + self.bounds.height / 2.0
    }

    /// 获取 x 轴半径
    pub fn rx(&self) -> f64 {
        self.bounds.width / 2.0
    }

    /// 获取 y 轴半径
    pub fn ry(&self) -> f64 {
        self.bounds.height / 2.0
    }
}

impl Figure for EllipseFigure {
    fn bounds(&self) -> Rectangle {
        self.bounds
    }

    fn paint_figure(&self, gc: &mut NdCanvas) {
        // 使用 Shape trait 的渲染逻辑
        // Shape: Figure 会调用 paint_fill() 和 paint_outline()
        // 注意：通过 self 调用以确保使用正确的 trait 方法
        self.paint_fill(gc);
        self.paint_outline(gc);
    }

    fn set_bounds(&mut self, x: f64, y: f64, width: f64, height: f64) {
        self.bounds = Rectangle::new(x, y, width, height);
    }

    fn name(&self) -> &'static str {
        "EllipseFigure"
    }
}

impl super::Shape for EllipseFigure {
    fn stroke_color(&self) -> Option<Color> {
        self.stroke_color
    }

    fn stroke_width(&self) -> f64 {
        self.stroke_width
    }

    fn fill_color(&self) -> Option<Color> {
        Some(self.fill_color)
    }

    fn line_cap(&self) -> novadraw_render::command::LineCap {
        self.line_cap
    }

    fn line_join(&self) -> novadraw_render::command::LineJoin {
        self.line_join
    }

    fn fill_enabled(&self) -> bool {
        true
    }

    fn outline_enabled(&self) -> bool {
        self.stroke_color.is_some()
    }

    fn fill_shape(&self, gc: &mut NdCanvas) {
        let cx = self.bounds.x + self.bounds.width / 2.0;
        let cy = self.bounds.y + self.bounds.height / 2.0;
        let rx = (self.bounds.width - self.stroke_width) / 2.0;
        let ry = (self.bounds.height - self.stroke_width) / 2.0;

        gc.ellipse(
            cx,
            cy,
            rx.max(0.0),
            ry.max(0.0),
            Some(self.fill_color),
            None,
            0.0,
            self.line_cap,
            self.line_join,
        );
    }

    fn outline_shape(&self, gc: &mut NdCanvas) {
        if let Some(color) = self.stroke_color {
            let cx = self.bounds.x + self.bounds.width / 2.0;
            let cy = self.bounds.y + self.bounds.height / 2.0;
            let rx = (self.bounds.width - self.stroke_width) / 2.0;
            let ry = (self.bounds.height - self.stroke_width) / 2.0;

            gc.ellipse(
                cx,
                cy,
                rx.max(0.0),
                ry.max(0.0),
                None,
                Some(color),
                self.stroke_width,
                self.line_cap,
                self.line_join,
            );
        }
    }
}
