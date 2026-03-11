//! 多边形图形

use novadraw_core::Color;
use novadraw_geometry::Rectangle;
use novadraw_render::NdCanvas;

use super::{Bounded, PolylineFigure, Shape};

/// 多边形图形
///
/// 参考 Eclipse Draw2D 的 Polygon 设计。
/// 继承自 PolylineFigure，但支持填充（闭合路径）。
#[derive(Clone, Debug, PartialEq)]
pub struct PolygonFigure {
    /// 内部使用 PolylineFigure 存储点
    polyline: PolylineFigure,
    /// 填充颜色
    fill_color: Color,
}

impl PolygonFigure {
    /// 创建多边形（从点列表）
    pub fn from_points(points: Vec<novadraw_geometry::Vec2>) -> Self {
        Self {
            polyline: PolylineFigure::from_points(points),
            fill_color: Color::hex("#3498db"),
        }
    }

    /// 添加点
    pub fn add_point(&mut self, x: f64, y: f64) {
        self.polyline.add_point(x, y);
    }

    /// 获取点列表
    pub fn get_points(&self) -> &[novadraw_geometry::Vec2] {
        self.polyline.get_points()
    }

    /// 设置填充颜色
    pub fn with_fill_color(mut self, color: Color) -> Self {
        self.fill_color = color;
        self
    }

    /// 设置线条样式
    pub fn with_stroke(mut self, color: Color, width: f64) -> Self {
        self.polyline.stroke_color = color;
        self.polyline.stroke_width = width;
        self
    }
}

// 实现 Bounded trait
impl Bounded for PolygonFigure {
    fn bounds(&self) -> Rectangle {
        Bounded::bounds(&self.polyline)
    }

    fn set_bounds(&mut self, x: f64, y: f64, width: f64, height: f64) {
        // 多边形通过点定义，set_bounds 需要重新计算点位置
        // 这里简化处理：平移现有点
        let current_bounds = Bounded::bounds(&self.polyline);
        if current_bounds.width == 0.0 || current_bounds.height == 0.0 {
            return;
        }
        let scale_x = width / current_bounds.width;
        let scale_y = height / current_bounds.height;
        let dx = x - current_bounds.x;
        let dy = y - current_bounds.y;

        let new_points: Vec<novadraw_geometry::Vec2> = self
            .polyline
            .get_points()
            .iter()
            .map(|p| novadraw_geometry::Vec2::new((p.0.x + dx) * scale_x, (p.0.y + dy) * scale_y))
            .collect();
        self.polyline.set_points(new_points);
    }

    fn use_local_coordinates(&self) -> bool {
        Bounded::use_local_coordinates(&self.polyline)
    }

    fn name(&self) -> &'static str {
        "PolygonFigure"
    }
}

// 实现 Shape trait
impl Shape for PolygonFigure {
    fn stroke_color(&self) -> Option<Color> {
        self.polyline.stroke_color()
    }

    fn stroke_width(&self) -> f64 {
        self.polyline.stroke_width()
    }

    fn fill_color(&self) -> Option<Color> {
        Some(self.fill_color)
    }

    fn line_cap(&self) -> novadraw_render::command::LineCap {
        self.polyline.line_cap()
    }

    fn line_join(&self) -> novadraw_render::command::LineJoin {
        self.polyline.line_join()
    }

    fn fill_enabled(&self) -> bool {
        true
    }

    fn outline_enabled(&self) -> bool {
        true
    }

    fn fill_shape(&self, gc: &mut NdCanvas) {
        let points = self.polyline.get_points();
        if points.len() < 3 {
            return;
        }

        // 使用 path API 构建闭合路径
        gc.begin_path();
        if let Some(first) = points.first() {
            gc.move_to(first.0.x, first.0.y);
        }
        for point in points.iter().skip(1) {
            gc.line_to(point.0.x, point.0.y);
        }
        gc.close_path();

        // 设置填充颜色并填充
        gc.fill_style(self.fill_color);
        gc.fill();
    }

    fn outline_shape(&self, gc: &mut NdCanvas) {
        self.polyline.outline_shape(gc);
    }
}
