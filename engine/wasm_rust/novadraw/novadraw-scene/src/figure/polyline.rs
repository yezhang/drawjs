//! 折线图形

use novadraw_core::Color;
use novadraw_geometry::Rectangle;
use novadraw_render::NdCanvas;

use super::{Figure, Shape};

/// 折线图形
///
/// 参考 Eclipse Draw2D 的 Polyline 设计。
/// 使用点列表存储多个顶点，可以绘制任意折线。
/// bounds 是自动计算的，基于点列表并扩展线宽。
///
/// 注意：不能通过 set_bounds 定位，应该通过 add_point/set_points 操作点。
#[derive(Clone, Debug, PartialEq)]
pub struct PolylineFigure {
    /// 点列表
    points: Vec<novadraw_geometry::Vec2>,
    /// 线条颜色
    pub stroke_color: Color,
    /// 线条宽度
    pub stroke_width: f64,
    /// 线帽样式
    pub line_cap: novadraw_render::command::LineCap,
    /// 连接样式
    pub line_join: novadraw_render::command::LineJoin,
}

impl PolylineFigure {
    /// 创建两点折线（直线）
    ///
    /// 从 (x1, y1) 到 (x2, y2)
    pub fn new(x1: f64, y1: f64, x2: f64, y2: f64) -> Self {
        Self {
            points: vec![
                novadraw_geometry::Vec2::new(x1, y1),
                novadraw_geometry::Vec2::new(x2, y2),
            ],
            stroke_color: Color::hex("#2c3e50"),
            stroke_width: 2.0,
            line_cap: novadraw_render::command::LineCap::default(),
            line_join: novadraw_render::command::LineJoin::default(),
        }
    }

    /// 从点列表创建折线
    pub fn from_points(points: Vec<novadraw_geometry::Vec2>) -> Self {
        Self {
            points,
            stroke_color: Color::hex("#2c3e50"),
            stroke_width: 2.0,
            line_cap: novadraw_render::command::LineCap::default(),
            line_join: novadraw_render::command::LineJoin::default(),
        }
    }

    /// 创建指定颜色的折线
    pub fn new_with_color(x1: f64, y1: f64, x2: f64, y2: f64, color: Color) -> Self {
        Self {
            points: vec![
                novadraw_geometry::Vec2::new(x1, y1),
                novadraw_geometry::Vec2::new(x2, y2),
            ],
            stroke_color: color,
            stroke_width: 2.0,
            line_cap: novadraw_render::command::LineCap::default(),
            line_join: novadraw_render::command::LineJoin::default(),
        }
    }

    /// 添加点
    pub fn add_point(&mut self, x: f64, y: f64) {
        self.points.push(novadraw_geometry::Vec2::new(x, y));
    }

    /// 获取点列表（引用）
    pub fn get_points(&self) -> &[novadraw_geometry::Vec2] {
        &self.points
    }

    /// 设置点列表
    pub fn set_points(&mut self, points: Vec<novadraw_geometry::Vec2>) {
        self.points = points;
    }

    /// 获取起点
    pub fn start_point(&self) -> Option<novadraw_geometry::Vec2> {
        self.points.first().copied()
    }

    /// 获取终点
    pub fn end_point(&self) -> Option<novadraw_geometry::Vec2> {
        self.points.last().copied()
    }

    /// 获取点数量
    pub fn point_count(&self) -> usize {
        self.points.len()
    }

    /// 设置线条宽度
    pub fn with_width(mut self, width: f64) -> Self {
        self.stroke_width = width;
        self
    }

    /// 设置线帽样式
    pub fn with_cap(mut self, cap: novadraw_render::command::LineCap) -> Self {
        self.line_cap = cap;
        self
    }

    /// 设置连接样式
    pub fn with_join(mut self, join: novadraw_render::command::LineJoin) -> Self {
        self.line_join = join;
        self
    }

    /// 计算包含线宽的边界矩形
    fn calculate_bounds(&self) -> Rectangle {
        if self.points.is_empty() {
            return Rectangle::ZERO;
        }

        let mut min_x = f64::MAX;
        let mut min_y = f64::MAX;
        let mut max_x = f64::MIN;
        let mut max_y = f64::MIN;

        for point in &self.points {
            min_x = min_x.min(point.0.x);
            min_y = min_y.min(point.0.y);
            max_x = max_x.max(point.0.x);
            max_y = max_y.max(point.0.y);
        }

        // 扩展边界以包含描边宽度
        let half_stroke = self.stroke_width / 2.0;
        Rectangle::new(
            min_x - half_stroke,
            min_y - half_stroke,
            (max_x - min_x) + self.stroke_width,
            (max_y - min_y) + self.stroke_width,
        )
    }
}

impl Figure for PolylineFigure {
    fn bounds(&self) -> Rectangle {
        self.calculate_bounds()
    }

    fn name(&self) -> &'static str {
        "PolylineFigure"
    }
}

impl Shape for PolylineFigure {
    fn stroke_color(&self) -> Option<Color> {
        Some(self.stroke_color)
    }

    fn stroke_width(&self) -> f64 {
        self.stroke_width
    }

    fn fill_color(&self) -> Option<Color> {
        None // Polyline 不支持填充
    }

    fn line_cap(&self) -> novadraw_render::command::LineCap {
        self.line_cap
    }

    fn line_join(&self) -> novadraw_render::command::LineJoin {
        self.line_join
    }

    fn fill_enabled(&self) -> bool {
        false // Polyline 不支持填充
    }

    fn outline_enabled(&self) -> bool {
        true
    }

    fn fill_shape(&self, _gc: &mut NdCanvas) {
        // Polyline 不支持填充
    }

    fn outline_shape(&self, gc: &mut NdCanvas) {
        if self.points.len() < 2 {
            return;
        }

        let points: Vec<glam::DVec2> = self
            .points
            .iter()
            .map(|p| glam::DVec2::new(p.0.x, p.0.y))
            .collect();

        gc.polyline(
            &points,
            self.stroke_color,
            self.stroke_width,
            self.line_cap,
            self.line_join,
        );
    }
}

/// 直线图形（PolylineFigure 的别名，保持向后兼容）
pub type LineFigure = PolylineFigure;
