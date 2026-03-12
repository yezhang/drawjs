//! 三角形图形

use novadraw_core::Color;
use novadraw_geometry::{Rectangle, Vec2};
use novadraw_render::NdCanvas;

use super::{Bounded, Shape};

/// 三角形图形
///
/// 参考 Eclipse Draw2D 的 TriangleShape 设计。
/// 使用三个顶点定义三角形。
#[derive(Clone, Debug, PartialEq)]
pub struct TriangleFigure {
    /// 顶点列表（3个点）
    points: Vec<Vec2>,
    /// 填充颜色
    fill_color: Color,
    /// 描边颜色
    stroke_color: Color,
    /// 描边宽度
    stroke_width: f64,
}

impl TriangleFigure {
    /// 创建等边三角形
    ///
    /// # Arguments
    ///
    /// * `x` - 左上角 x 坐标
    /// * `y` - 左上角 y 坐标
    /// * `size` - 三角形边长
    pub fn new(x: f64, y: f64, size: f64) -> Self {
        // 等边三角形三个顶点
        let points = vec![
            Vec2::new(x + size / 2.0, y),                     // 顶部顶点
            Vec2::new(x, y + size * 3.0_f64.sqrt() / 2.0),   // 左下顶点
            Vec2::new(x + size, y + size * 3.0_f64.sqrt() / 2.0), // 右下顶点
        ];
        Self {
            points,
            fill_color: Color::hex("#e74c3c"),
            stroke_color: Color::hex("#c0392b"),
            stroke_width: 2.0,
        }
    }

    /// 创建直角三角形
    ///
    /// # Arguments
    ///
    /// * `x` - 左上角 x 坐标
    /// * `y` - 左上角 y 坐标
    /// * `width` - 底边宽度
    /// * `height` - 垂直高度
    pub fn new_right(x: f64, y: f64, width: f64, height: f64) -> Self {
        let points = vec![
            Vec2::new(x, y),              // 左上顶点
            Vec2::new(x + width, y),      // 右上顶点
            Vec2::new(x, y + height),     // 左下顶点
        ];
        Self {
            points,
            fill_color: Color::hex("#3498db"),
            stroke_color: Color::hex("#2980b9"),
            stroke_width: 2.0,
        }
    }

    /// 从三个顶点创建三角形
    pub fn from_points(p1: Vec2, p2: Vec2, p3: Vec2) -> Self {
        Self {
            points: vec![p1, p2, p3],
            fill_color: Color::hex("#2ecc71"),
            stroke_color: Color::hex("#27ae60"),
            stroke_width: 2.0,
        }
    }

    /// 设置填充颜色
    pub fn with_fill_color(mut self, color: Color) -> Self {
        self.fill_color = color;
        self
    }

    /// 设置描边颜色
    pub fn with_stroke_color(mut self, color: Color) -> Self {
        self.stroke_color = color;
        self
    }

    /// 设置描边宽度
    pub fn with_stroke_width(mut self, width: f64) -> Self {
        self.stroke_width = width;
        self
    }

    /// 设置线条样式（填充+描边颜色）
    pub fn with_style(mut self, fill: Color, stroke: Color, stroke_width: f64) -> Self {
        self.fill_color = fill;
        self.stroke_color = stroke;
        self.stroke_width = stroke_width;
        self
    }

    /// 获取顶点列表
    pub fn get_points(&self) -> &[Vec2] {
        &self.points
    }
}

// 计算三角形的外接矩形
fn compute_bounds(points: &[Vec2]) -> Rectangle {
    let mut min_x = f64::MAX;
    let mut min_y = f64::MAX;
    let mut max_x = f64::MIN;
    let mut max_y = f64::MIN;

    for p in points {
        min_x = min_x.min(p.0.x);
        min_y = min_y.min(p.0.y);
        max_x = max_x.max(p.0.x);
        max_y = max_y.max(p.0.y);
    }

    if min_x == f64::MAX {
        Rectangle::new(0.0, 0.0, 0.0, 0.0)
    } else {
        Rectangle::new(min_x, min_y, max_x - min_x, max_y - min_y)
    }
}

impl Bounded for TriangleFigure {
    fn bounds(&self) -> Rectangle {
        compute_bounds(&self.points)
    }

    fn set_bounds(&mut self, x: f64, y: f64, width: f64, height: f64) {
        let current = compute_bounds(&self.points);
        if current.width == 0.0 || current.height == 0.0 {
            return;
        }

        let scale_x = width / current.width;
        let scale_y = height / current.height;
        let dx = x - current.x;
        let dy = y - current.y;

        self.points = self
            .points
            .iter()
            .map(|p| Vec2::new((p.0.x + dx) * scale_x, (p.0.y + dy) * scale_y))
            .collect();
    }

    fn use_local_coordinates(&self) -> bool {
        false
    }

    fn name(&self) -> &'static str {
        "TriangleFigure"
    }
}

impl Shape for TriangleFigure {
    fn stroke_color(&self) -> Option<Color> {
        Some(self.stroke_color)
    }

    fn stroke_width(&self) -> f64 {
        self.stroke_width
    }

    fn fill_color(&self) -> Option<Color> {
        Some(self.fill_color)
    }

    fn line_cap(&self) -> novadraw_render::command::LineCap {
        novadraw_render::command::LineCap::Butt
    }

    fn line_join(&self) -> novadraw_render::command::LineJoin {
        novadraw_render::command::LineJoin::Miter
    }

    fn fill_enabled(&self) -> bool {
        true
    }

    fn outline_enabled(&self) -> bool {
        true
    }

    fn fill_shape(&self, gc: &mut NdCanvas) {
        if self.points.len() < 3 {
            return;
        }

        gc.begin_path();
        if let Some(first) = self.points.first() {
            gc.move_to(first.0.x, first.0.y);
        }
        for point in self.points.iter().skip(1) {
            gc.line_to(point.0.x, point.0.y);
        }
        gc.close_path();

        gc.fill_style(self.fill_color);
        gc.fill();
    }

    fn outline_shape(&self, gc: &mut NdCanvas) {
        if self.points.len() < 3 {
            return;
        }

        // 参考 d2 Triangle.outlineShape: 直接绘制多边形
        // 不做 inset 缩放，让渲染器处理描边对齐
        gc.begin_path();
        if let Some(first) = self.points.first() {
            gc.move_to(first.0.x, first.0.y);
        }
        for point in self.points.iter().skip(1) {
            gc.line_to(point.0.x, point.0.y);
        }
        gc.close_path();

        gc.stroke_style(self.stroke_color);
        gc.line_width(self.stroke_width);
        gc.line_cap(self.line_cap());
        gc.line_join(self.line_join());
        gc.stroke();
    }
}
