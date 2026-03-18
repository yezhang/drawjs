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
///
/// # 顶点计算
///
/// 顶点在 `validate()` 阶段计算并缓存，参考 d2: Figure.validate()。
/// 渲染时使用缓存的顶点，避免重复计算。
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
    /// 缓存的顶点（validate 后有效）
    cached_points: Option<[(f64, f64); 3]>,
    /// 缓存的 bounds（用于检测是否需要重新计算）
    cached_bounds: Option<Rectangle>,
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
            cached_points: None,
            cached_bounds: None,
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
            cached_points: None,
            cached_bounds: None,
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
    /// - r.shrink(getInsets()) - 收缩 insets (novadraw 暂不支持 insets)
    /// - r.resize(-1, -1) - 向内收缩，为描边预留空间
    /// - r.y/r.x += (height/width - size) / 2 - 居中调整
    ///
    /// 关键点：
    /// 1. 描边向内收缩，填充区域缩小
    /// 2. 主尖角完整，不被裁剪（在收缩后的边界内，考虑描边向外扩展）
    /// 3. 底部两角的裁剪由渲染器的 line join 行为自然产生
    fn compute_points(&self) -> [(f64, f64); 3] {
        let mut r = self.bounds.clone();

        if r.width <= 0.0 || r.height <= 0.0 {
            return [(0.0, 0.0), (0.0, 0.0), (0.0, 0.0)];
        }

        // 向内收缩 1px (对应 d2 的 r.resize(-1, -1))
        r.x += 1.0;
        r.y += 1.0;
        r.width = (r.width - 2.0).max(0.0);
        r.height = (r.height - 2.0).max(0.0);

        if r.width <= 0.0 || r.height <= 0.0 {
            return [(0.0, 0.0), (0.0, 0.0), (0.0, 0.0)];
        }

        // 计算 size 并居中 (对应 d2 的逻辑)
        // 使用 f64::min 明确指定类型
        let size: f64 = match self.direction {
            Direction::North | Direction::South => {
                // 垂直方向：size = min(height, width / 2)
                let half_width = r.width / 2.0;
                let size = if r.height < half_width { r.height } else { half_width };
                // 垂直居中：r.y += (r.height - size) / 2
                r.y += (r.height - size) / 2.0;
                size
            }
            Direction::West | Direction::East => {
                // 水平方向：size = min(height / 2, width)
                let half_height = r.height / 2.0;
                let size = if half_height < r.width { half_height } else { r.width };
                // 水平居中：r.x += (r.width - size) / 2
                r.x += (r.width - size) / 2.0;
                size
            }
        };

        // 最小尺寸保护 (对应 d2 的 Math.max(size, 1))
        let size = size.max(1.0);

        match self.direction {
            Direction::North => {
                // 顶点朝上
                let head_x = r.x + r.width / 2.0;
                let head_y = r.y;
                let p2_x = head_x - size;
                let p3_x = head_x + size;
                let bottom_y = head_y + size;

                [(head_x, head_y), (p2_x, bottom_y), (p3_x, bottom_y)]
            }
            Direction::South => {
                // 顶点朝下
                let head_x = r.x + r.width / 2.0;
                let head_y = r.y + size;
                let p2_x = head_x - size;
                let p3_x = head_x + size;
                let bottom_y = head_y - size;

                [(head_x, head_y), (p2_x, bottom_y), (p3_x, bottom_y)]
            }
            Direction::West => {
                // 顶点朝左
                let head_x = r.x;
                let head_y = r.y + r.height / 2.0;
                let right_x = head_x + size;
                let p2_y = head_y - size;
                let p3_y = head_y + size;

                [(head_x, head_y), (right_x, p2_y), (right_x, p3_y)]
            }
            Direction::East => {
                // 顶点朝右
                let head_x = r.x + size;
                let head_y = r.y + r.height / 2.0;
                let left_x = head_x - size;
                let p2_y = head_y - size;
                let p3_y = head_y + size;

                [(head_x, head_y), (left_x, p2_y), (left_x, p3_y)]
            }
        }
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
        // 返回原始 bounds，与 d2 保持一致
        // d2 中 Figure.bounds 保持为用户设置的原始值，不受 validate() 中收缩的影响
        // paintChildren() 使用原始 bounds 作为裁剪区域
        self.bounds
    }

    fn set_bounds(&mut self, x: f64, y: f64, width: f64, height: f64) {
        self.bounds = Rectangle::new(x, y, width, height);
    }

    fn name(&self) -> &'static str {
        "TriangleFigure"
    }
}

impl Shape for TriangleFigure {
    /// 布局验证：计算并缓存顶点
    ///
    /// 对应 d2: Triangle.validate()
    /// 在布局完成后被调用，预计算三角形的顶点位置。
    fn validate(&mut self) {
        // 检查 bounds 是否变化，变化则重新计算顶点
        if self.cached_bounds != Some(self.bounds.clone()) {
            self.cached_points = Some(self.compute_points());
            self.cached_bounds = Some(self.bounds.clone());
        }
    }

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
        // 使用缓存的顶点，如果没有缓存则计算
        let points = self.cached_points.unwrap_or_else(|| self.compute_points());

        gc.begin_path();
        gc.move_to(points[0].0, points[0].1);
        gc.line_to(points[1].0, points[1].1);
        gc.line_to(points[2].0, points[2].1);
        gc.close_path();

        gc.fill_style(self.fill_color);
        gc.fill();
    }

    fn outline_shape(&self, gc: &mut NdCanvas) {
        // 使用缓存的顶点，如果没有缓存则计算
        let points = self.cached_points.unwrap_or_else(|| self.compute_points());

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
