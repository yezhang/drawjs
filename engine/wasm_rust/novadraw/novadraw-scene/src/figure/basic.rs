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
        // Figure 始终在绝对坐标绘制（参考 d2 Figure.paintFigure）
        // use_local_coordinates 仅影响子元素的坐标系统，不影响 Figure 自身的绘制位置
        gc.fill_rect(
            self.bounds.x,
            self.bounds.y,
            self.bounds.width,
            self.bounds.height,
            self.fill_color,
        );
        if let Some(color) = self.stroke_color {
            gc.stroke_rect(
                self.bounds.x,
                self.bounds.y,
                self.bounds.width,
                self.bounds.height,
                color,
                self.stroke_width,
            );
        }
    }

    fn as_rectangle(&self) -> Option<&RectangleFigure> {
        Some(self)
    }

    fn as_rectangle_mut(&mut self) -> Option<&mut RectangleFigure> {
        Some(self)
    }
}

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
    /// 是否使用本地坐标模式
    use_local_coordinates: bool,
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
            use_local_coordinates: false,
        }
    }

    /// 从 Rectangle 创建椭圆
    pub fn from_bounds(bounds: Rectangle) -> Self {
        Self {
            bounds,
            fill_color: Color::hex("#e74c3c"),
            stroke_color: None,
            stroke_width: 0.0,
            use_local_coordinates: false,
        }
    }

    /// 创建指定颜色的椭圆
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
    pub fn with_local_coordinates(mut self, enable: bool) -> Self {
        self.use_local_coordinates = enable;
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

    fn use_local_coordinates(&self) -> bool {
        self.use_local_coordinates
    }

    fn paint_figure(&self, gc: &mut NdCanvas) {
        // 计算椭圆参数，确保描边完全在 bounds 内
        // stroke 向外延伸，所以 rx/ry 需要向内收缩 stroke_width
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
            self.stroke_color,
            self.stroke_width,
        );
    }

    fn name(&self) -> &'static str {
        "EllipseFigure"
    }
}

/// 直线图形
///
/// 用于渲染直线。
/// bounds 定义起点 (x, y) 和终点偏移 (width, height)。
/// 直线从 (bounds.x, bounds.y) 到 (bounds.x + width, bounds.y + height)。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LineFigure {
    /// 边界矩形：x,y=起点, width/height=终点偏移
    pub bounds: Rectangle,
    /// 线条颜色
    pub stroke_color: Color,
    /// 线条宽度
    pub stroke_width: f64,
    /// 线帽样式
    pub line_cap: novadraw_render::command::LineCap,
    /// 连接样式
    pub line_join: novadraw_render::command::LineJoin,
}

impl LineFigure {
    /// 创建直线
    ///
    /// 从 (x1, y1) 到 (x2, y2)
    pub fn new(x1: f64, y1: f64, x2: f64, y2: f64) -> Self {
        Self {
            bounds: Rectangle::new(x1.min(x2), y1.min(y2), (x2 - x1).abs(), (y2 - y1).abs()),
            stroke_color: Color::hex("#2c3e50"),
            stroke_width: 2.0,
            line_cap: novadraw_render::command::LineCap::default(),
            line_join: novadraw_render::command::LineJoin::default(),
        }
    }

    /// 创建指定颜色的直线
    pub fn new_with_color(x1: f64, y1: f64, x2: f64, y2: f64, color: Color) -> Self {
        Self {
            bounds: Rectangle::new(x1.min(x2), y1.min(y2), (x2 - x1).abs(), (y2 - y1).abs()),
            stroke_color: color,
            stroke_width: 2.0,
            line_cap: novadraw_render::command::LineCap::default(),
            line_join: novadraw_render::command::LineJoin::default(),
        }
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

    /// 获取起点
    pub fn p1(&self) -> (f64, f64) {
        // 如果直线是反向定义的，需要调整
        if self.bounds.width == 0.0 && self.bounds.height == 0.0 {
            (self.bounds.x, self.bounds.y)
        } else {
            // 对于斜线，这里简化为 bounds 左上角
            // 实际渲染时 p1 和 p2 需要根据原始坐标计算
            (self.bounds.x, self.bounds.y)
        }
    }

    /// 获取终点
    pub fn p2(&self) -> (f64, f64) {
        (self.bounds.x + self.bounds.width, self.bounds.y + self.bounds.height)
    }
}

impl Figure for LineFigure {
    fn bounds(&self) -> Rectangle {
        self.bounds
    }

    fn paint_figure(&self, gc: &mut NdCanvas) {
        gc.line(
            glam::DVec2::new(self.bounds.x, self.bounds.y),
            glam::DVec2::new(self.bounds.x + self.bounds.width, self.bounds.y + self.bounds.height),
            self.stroke_color,
            self.stroke_width,
            self.line_cap,
            self.line_join,
        );
    }

    fn name(&self) -> &'static str {
        "LineFigure"
    }
}
