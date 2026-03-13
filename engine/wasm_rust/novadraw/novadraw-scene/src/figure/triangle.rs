//! 三角形图形
//!
//! 参考 Eclipse Draw2D 的 Triangle 设计。
//! 预定义的等腰三角形，主要用于箭头指向标（Connection Anchors）。

use novadraw_core::Color;
use novadraw_geometry::Rectangle;
use novadraw_render::NdCanvas;

use super::{Bounded, Shape};

/// 三角形方向
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum Direction {
    /// 指向上方
    North,
    /// 指向下方
    South,
    /// 指向左侧
    West,
    /// 指向右侧
    #[default]
    East,
}

/// 三角形图形
///
/// 参考 Eclipse Draw2D 的 Triangle 设计。
/// 预定义的等腰三角形，根据 bounds 和 direction 动态计算顶点。
/// 主要用于箭头指向标（Connection Anchors）。
#[derive(Clone, Debug, PartialEq)]
pub struct TriangleFigure {
    /// 边界矩形
    pub bounds: Rectangle,
    /// 填充颜色
    pub fill_color: Color,
    /// 描边颜色
    pub stroke_color: Color,
    /// 描边宽度
    pub stroke_width: f64,
    /// 方向
    pub direction: Direction,
    /// 线帽样式
    pub line_cap: novadraw_render::command::LineCap,
    /// 连接样式
    pub line_join: novadraw_render::command::LineJoin,
}

impl TriangleFigure {
    /// 创建三角形
    ///
    /// # Arguments
    ///
    /// * `x` - 左上角 x 坐标
    /// * `y` - 左上角 y 坐标
    /// * `width` - 宽度
    /// * `height` - 高度
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self {
            bounds: Rectangle::new(x, y, width, height),
            fill_color: Color::hex("#e74c3c"),
            stroke_color: Color::hex("#c0392b"),
            stroke_width: 1.0,
            direction: Direction::North,
            line_cap: novadraw_render::command::LineCap::Butt,
            line_join: novadraw_render::command::LineJoin::Miter,
        }
    }

    /// 从 Rectangle 创建三角形
    pub fn from_bounds(bounds: Rectangle) -> Self {
        Self::new(bounds.x, bounds.y, bounds.width, bounds.height)
    }

    /// 创建指定方向的三角形
    pub fn new_with_direction(x: f64, y: f64, width: f64, height: f64, direction: Direction) -> Self {
        Self {
            bounds: Rectangle::new(x, y, width, height),
            fill_color: Color::hex("#e74c3c"),
            stroke_color: Color::hex("#c0392b"),
            stroke_width: 1.0,
            direction,
            line_cap: novadraw_render::command::LineCap::Butt,
            line_join: novadraw_render::command::LineJoin::Miter,
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

    /// 设置方向
    pub fn with_direction(mut self, direction: Direction) -> Self {
        self.direction = direction;
        self
    }

    /// 设置线条样式
    pub fn with_style(mut self, fill: Color, stroke: Color, stroke_width: f64) -> Self {
        self.fill_color = fill;
        self.stroke_color = stroke;
        self.stroke_width = stroke_width;
        self
    }

    /// 计算三角形的三个顶点
    ///
    /// 参考 d2 Triangle.validate():
    /// - r.shrink(getInsets()) - 收缩 insets
    /// - r.resize(-1, -1) - 向外扩展 1px，为描边预留空间
    fn compute_points(&self) -> [(f64, f64); 3] {
        // 参考 d2: 先收缩 insets，再向外扩展 1px
        let mut r = self.bounds.clone();

        // 向外扩展 1px，为描边预留空间 (d2: r.resize(-1, -1))
        let inset = 1.0_f64.max(self.stroke_width) / 2.0;
        r.x -= inset;
        r.y -= inset;
        r.width += inset * 2.0;
        r.height += inset * 2.0;

        if r.width <= 0.0 || r.height <= 0.0 {
            return [(0.0, 0.0), (0.0, 0.0), (0.0, 0.0)];
        }

        let (head_x, head_y, size, p2_base, p3_base) = match self.direction {
            Direction::North => {
                // 顶点朝上
                let size = r.height.min(r.width / 2.0);
                let head_x = r.x + r.width / 2.0;
                let head_y = r.y;
                (head_x, head_y, size, (head_x - size, head_y + size), (head_x + size, head_y + size))
            }
            Direction::South => {
                // 顶点朝下
                let size = r.height.min(r.width / 2.0);
                let head_x = r.x + r.width / 2.0;
                let head_y = r.y + r.height;
                (head_x, head_y, size, (head_x - size, head_y - size), (head_x + size, head_y - size))
            }
            Direction::West => {
                // 顶点朝左
                let size = r.width.min(r.height / 2.0);
                let head_x = r.x;
                let head_y = r.y + r.height / 2.0;
                (head_x, head_y, size, (head_x + size, head_y - size), (head_x + size, head_y + size))
            }
            Direction::East => {
                // 顶点朝右
                let size = r.width.min(r.height / 2.0);
                let head_x = r.x + r.width;
                let head_y = r.y + r.height / 2.0;
                (head_x, head_y, size, (head_x - size, head_y - size), (head_x - size, head_y + size))
            }
        };

        [(head_x, head_y), p2_base, p3_base]
    }
}

/// 计算三角形的外接矩形（不含描边扩展）
fn compute_bounds(points: &[(f64, f64)]) -> Rectangle {
    let mut min_x = f64::MAX;
    let mut min_y = f64::MAX;
    let mut max_x = f64::MIN;
    let mut max_y = f64::MIN;

    for (x, y) in points {
        min_x = min_x.min(*x);
        min_y = min_y.min(*y);
        max_x = max_x.max(*x);
        max_y = max_y.max(*y);
    }

    if min_x == f64::MAX {
        Rectangle::new(0.0, 0.0, 0.0, 0.0)
    } else {
        Rectangle::new(min_x, min_y, max_x - min_x, max_y - min_y)
    }
}

impl Bounded for TriangleFigure {
    fn bounds(&self) -> Rectangle {
        let points = self.compute_points();
        let points_ref: Vec<(f64, f64)> = points.to_vec();
        compute_bounds(&points_ref)
    }

    fn set_bounds(&mut self, x: f64, y: f64, width: f64, height: f64) {
        self.bounds = Rectangle::new(x, y, width, height);
    }

    fn name(&self) -> &'static str {
        "TriangleFigure"
    }
}

impl Shape for TriangleFigure {
    fn stroke_color(&self) -> Option<Color> {
        if self.stroke_color.a > 0.0 {
            Some(self.stroke_color)
        } else {
            None
        }
    }

    fn stroke_width(&self) -> f64 {
        self.stroke_width
    }

    fn fill_color(&self) -> Option<Color> {
        if self.fill_color.a > 0.0 {
            Some(self.fill_color)
        } else {
            None
        }
    }

    fn line_cap(&self) -> novadraw_render::command::LineCap {
        self.line_cap
    }

    fn line_join(&self) -> novadraw_render::command::LineJoin {
        self.line_join
    }

    fn fill_enabled(&self) -> bool {
        self.fill_color.a > 0.0
    }

    fn outline_enabled(&self) -> bool {
        self.stroke_color.a > 0.0 && self.stroke_width > 0.0
    }

    fn fill_shape(&self, gc: &mut NdCanvas) {
        let points = self.compute_points();

        gc.begin_path();
        gc.move_to(points[0].0, points[0].1);
        gc.line_to(points[1].0, points[1].1);
        gc.line_to(points[2].0, points[2].1);
        gc.close_path();

        gc.fill_style(self.fill_color);
        gc.fill();
    }

    fn outline_shape(&self, gc: &mut NdCanvas) {
        let points = self.compute_points();

        gc.begin_path();
        gc.move_to(points[0].0, points[0].1);
        gc.line_to(points[1].0, points[1].1);
        gc.line_to(points[2].0, points[2].1);
        gc.close_path();

        gc.stroke_style(self.stroke_color);
        gc.line_width(self.stroke_width);
        gc.line_cap(self.line_cap);
        gc.line_join(self.line_join);
        gc.stroke();
    }
}
