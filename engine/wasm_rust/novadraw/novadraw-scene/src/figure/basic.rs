//! 基础图形实现
//!
//! 提供矩形、椭圆等基础图形实现。

use novadraw_core::Color;
use novadraw_geometry::Rectangle;
use novadraw_render::NdCanvas;

use super::Figure;

/// 矩形图形
///
/// 用于渲染矩形形状。
/// 遵循 d2 设计：使用 `bounds: Rectangle` 统一管理边界，而非独立 x/y/width/height 字段。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RectangleFigure {
    /// 边界矩形（包含 x, y, width, height）
    pub bounds: Rectangle,
    /// 填充颜色
    pub fill_color: Color,
    /// 边框颜色
    pub stroke_color: Option<Color>,
    /// 边框宽度
    pub stroke_width: f64,
    /// 是否使用本地坐标模式
    use_local_coordinates: bool,
}

impl RectangleFigure {
    /// 创建矩形
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self {
            bounds: Rectangle::new(x, y, width, height),
            fill_color: Color::hex("#3498db"),
            stroke_color: None,
            stroke_width: 0.0,
            use_local_coordinates: false,
        }
    }

    /// 从 Rectangle 创建矩形
    pub fn from_bounds(bounds: Rectangle) -> Self {
        Self {
            bounds,
            fill_color: Color::hex("#3498db"),
            stroke_color: None,
            stroke_width: 0.0,
            use_local_coordinates: false,
        }
    }

    /// 创建指定颜色的矩形
    pub fn new_with_color(x: f64, y: f64, width: f64, height: f64, color: Color) -> Self {
        Self {
            bounds: Rectangle::new(x, y, width, height),
            fill_color: color,
            stroke_color: None,
            stroke_width: 0.0,
            use_local_coordinates: false,
        }
    }

    /// 添加边框
    pub fn with_stroke(mut self, color: Color, width: f64) -> Self {
        self.stroke_color = Some(color);
        self.stroke_width = width;
        self
    }

    /// 设置坐标模式
    ///
    /// `true`: 使用本地坐标（子元素相对于 bounds 左上角定位）
    /// `false`: 使用绝对坐标（子元素使用全局坐标）
    pub fn with_local_coordinates(mut self, enable: bool) -> Self {
        self.use_local_coordinates = enable;
        self
    }

    /// 平移
    pub fn translate(&mut self, dx: f64, dy: f64) {
        self.bounds.x += dx;
        self.bounds.y += dy;
    }

    /// 设置边界（对应 d2: setBounds）
    pub fn set_bounds(&mut self, x: f64, y: f64, width: f64, height: f64) {
        self.bounds = Rectangle::new(x, y, width, height);
    }
}

impl Figure for RectangleFigure {
    fn bounds(&self) -> Rectangle {
        self.bounds
    }

    fn use_local_coordinates(&self) -> bool {
        self.use_local_coordinates
    }

    fn paint_figure(&self, gc: &mut NdCanvas) {
        if self.use_local_coordinates() {
            // 本地坐标模式：prepare_context 已 translate，从 (0,0) 绘制
            gc.fill_rect(0.0, 0.0, self.bounds.width, self.bounds.height, self.fill_color);
            if let Some(color) = self.stroke_color {
                gc.stroke_rect(0.0, 0.0, self.bounds.width, self.bounds.height, color, self.stroke_width);
            }
        } else {
            // 绝对坐标模式：直接使用 bounds 的绝对坐标绘制（参考 d2 Figure.paintFigure）
            gc.fill_rect(self.bounds.x, self.bounds.y, self.bounds.width, self.bounds.height, self.fill_color);
            if let Some(color) = self.stroke_color {
                gc.stroke_rect(self.bounds.x, self.bounds.y, self.bounds.width, self.bounds.height, color, self.stroke_width);
            }
        }
    }

    fn as_rectangle(&self) -> Option<&RectangleFigure> {
        Some(self)
    }

    fn as_rectangle_mut(&mut self) -> Option<&mut RectangleFigure> {
        Some(self)
    }
}
