//! Figure 渲染接口
//!
//! 定义图形渲染的通用接口，遵循 Eclipse Draw2D 设计模式。
//! Figure 只负责渲染，不包含状态（状态在 RuntimeBlock 中）。
//!
//! # Trait 层级
//!
//! ```
//! Bounded        - 边界相关方法（bounds, set_bounds, name 等）
//!   │
//!   ▼
//! Figure         - 渲染接口（继承 Bounded）
//!   │
//!   ▼
//! Shape          - 描边/填充（继承 Figure）
//! ```

mod ellipse;
mod polygon;
mod polyline;
mod rectangle;
mod root;
mod rounded_rectangle;

pub use ellipse::EllipseFigure;
pub use polygon::PolygonFigure;
pub use polyline::PolylineFigure;
pub use rectangle::RectangleFigure;
pub use root::RootFigure;
pub use rounded_rectangle::RoundedRectangleFigure;

use novadraw_core::Color;
use novadraw_geometry::Rectangle;
use novadraw_render::NdCanvas;
use novadraw_render::command::{LineCap, LineJoin};

// ============================================================================
// Bounded Trait: 边界相关方法
// ============================================================================

/// 边界相关方法 trait
///
/// 包含图形的边界、名称、位置检测等基础方法。
/// 所有图形类型都需要实现此 trait。
pub trait Bounded: Send + Sync {
    /// 获取图形边界
    ///
    /// 默认实现返回零矩形，子类应覆盖
    fn bounds(&self) -> Rectangle;

    /// 设置图形边界
    ///
    /// 对应 d2: setBounds(Rectangle)
    /// 注意：本实现只更新 bounds 本身，不触发事件通知
    fn set_bounds(&mut self, x: f64, y: f64, width: f64, height: f64);

    /// 获取名称（用于调试）
    fn name(&self) -> &'static str;

    /// 检查点是否在图形边界内
    ///
    /// 对应 d2: containsPoint(int, int)
    fn contains_point(&self, x: f64, y: f64) -> bool {
        let b = self.bounds();
        x >= b.x && x <= b.x + b.width && y >= b.y && y <= b.y + b.height
    }

    /// 检查矩形是否与图形边界相交
    ///
    /// 对应 d2: intersects(Rectangle)
    fn intersects(&self, rect: Rectangle) -> bool {
        let b = self.bounds();
        b.x < rect.x + rect.width
            && b.x + b.width > rect.x
            && b.y < rect.y + rect.height
            && b.y + b.height > rect.y
    }

    /// 获取内边距 (top, left, bottom, right)
    fn insets(&self) -> (f64, f64, f64, f64) {
        (0.0, 0.0, 0.0, 0.0)
    }

    /// 是否使用本地坐标
    ///
    /// 对应 d2: useLocalCoordinates()
    /// true: 子元素使用 Figure 内部坐标（相对于 bounds 左上角）
    /// false: 子元素使用父节点坐标（默认）
    fn use_local_coordinates(&self) -> bool {
        false
    }
}

// ============================================================================
// Figure Trait: 渲染接口
// ============================================================================

/// Figure 渲染 trait
///
/// 所有图形对象都需要实现此 trait。
/// 只包含渲染相关方法，边界方法在 Bounded trait 中。
///
/// # 渲染流程（参考 Draw2D）
///
/// ```text
/// paint(Graphics) [模板方法]
///   ├─> setLocalBackgroundColor()  [InitProperties]
///   ├─> setLocalForegroundColor()  [InitProperties]
///   ├─> setLocalFont()             [InitProperties]
///   └─> paintFigure()              [PaintSelf]
///         ├─> paintClientArea()    [PaintChildren]
///         │     └─> paintChildren()
///         └─> paintBorder()        [PaintBorder]
/// ```
pub trait Figure: Bounded + Send + Sync {
    /// ===== 模板方法 =====

    /// 初始化本地属性
    ///
    /// 对应 d2: setLocalBackgroundColor/ForegroundColor/Font
    /// 设置图形的本地渲染属性（颜色、字体等）
    fn init_properties(&self, _gc: &mut NdCanvas) {
        // 默认空实现，子类可覆盖
    }

    /// ===== PaintSelf 阶段方法 =====

    /// 绘制自身（背景）
    ///
    /// 对应 d2: paintFigure(Graphics)
    /// 默认空实现，由 Shape trait 覆盖
    fn paint_figure(&self, _gc: &mut NdCanvas) {}

    /// ===== PaintChildren 相关方法 =====

    /// 绘制子元素
    ///
    /// 对应 d2 paintChildren(Graphics)
    /// 默认行为由渲染器调度 PaintChildren 任务
    fn paint_children(&self) {
        // 默认行为由渲染器处理
    }

    /// ===== PaintBorder 阶段方法 =====

    /// 绘制边框
    ///
    /// 对应 d2: paintBorder(Graphics)
    fn paint_border(&self, _gc: &mut NdCanvas) {}
}

// ============================================================================
// Shape Trait: 描边/填充
// ============================================================================

/// Shape 图形 trait
///
/// 参考 Eclipse Draw2D 的 Shape 类设计。
/// 提供描边、填充、透明度等图形通用属性。
///
/// # 渲染流程
///
/// ```
/// paint_figure()            [覆盖 Figure trait]
///   ├─> paint_fill()       [内部方法]
///   │     └─> fill_shape()    [抽象方法]
///   └─> paint_outline()    [内部方法]
///         └─> outline_shape() [抽象方法]
/// ```
pub trait Shape: Figure {
    /// ===== Shape 特有方法 =====

    /// 获取描边颜色
    fn stroke_color(&self) -> Option<Color>;

    /// 获取描边宽度
    fn stroke_width(&self) -> f64;

    /// 获取填充颜色
    fn fill_color(&self) -> Option<Color>;

    /// 获取线帽样式
    fn line_cap(&self) -> LineCap;

    /// 获取线连接样式
    fn line_join(&self) -> LineJoin;

    /// 是否启用填充
    fn fill_enabled(&self) -> bool {
        true
    }

    /// 是否启用描边
    fn outline_enabled(&self) -> bool {
        true
    }

    /// 获取透明度 (0.0 - 1.0)
    fn alpha(&self) -> f64 {
        1.0
    }

    /// ===== 渲染方法 =====

    /// 绘制自身（覆盖 Figure trait 的实现）
    ///
    /// 参考 d2: Shape.paintFigure()
    /// 调用 paint_fill() 和 paint_outline()
    fn paint_figure(&self, gc: &mut NdCanvas) {
        self.paint_fill(gc);
        self.paint_outline(gc);
    }

    /// 绘制填充
    ///
    /// 参考 d2: paintFill()
    /// 如果 fill_enabled() 为 true，调用 fill_shape()
    fn paint_fill(&self, gc: &mut NdCanvas) {
        if self.fill_enabled() {
            self.fill_shape(gc);
        }
    }

    /// 绘制描边
    ///
    /// 参考 d2: paintOutline()
    /// 如果 outline_enabled() 为 true，调用 outline_shape()
    fn paint_outline(&self, gc: &mut NdCanvas) {
        if self.outline_enabled() {
            self.outline_shape(gc);
        }
    }

    /// 填充形状（抽象方法）
    ///
    /// 对应 d2: fillShape(Graphics)
    /// 具体图形必须实现此方法
    fn fill_shape(&self, gc: &mut NdCanvas);

    /// 描边形状（抽象方法）
    ///
    /// 对应 d2: outlineShape(Graphics)
    /// 具体图形必须实现此方法
    fn outline_shape(&self, gc: &mut NdCanvas);
}

// ============================================================================
// Blanket Impl: 让所有实现 Bounds 的类型自动实现 Figure
// ============================================================================
//
// 设计原理：
// 1. Bounds trait 定义边界相关方法（bounds, set_bounds, name, use_local_coordinates 等）
// 2. Figure trait 继承 Bounds，定义渲染接口（paint_figure, paint_border 等）
// 3. Shape trait 继承 Figure，添加描边/填充属性和 fill_shape/outline_shape 抽象方法
// 4. 所有实现 Bounds 的类型自动获得 Figure 的实现
// 5. Shape 类型会覆盖 paint_figure 实现，调用 paint_fill 和 paint_outline
//
// 关键点：
// - 具体图形类型需要实现 Bounds 和 Shape
// - Blanket impl 让所有 Bounds 实现自动获得 Figure 实现
// - Shape 类型的 paint_figure 会覆盖默认实现

/// Blanket Impl：所有实现 Shape trait 的类型自动获得 Figure trait 的实现
///
/// 具体图形类型只需要实现 Shape，不需要显式实现 Figure。
/// Shape 继承 Figure，paint_figure 会自动覆盖默认实现。
///
/// RootFigure 只实现 Figure（不实现 Shape），所以需要显式 impl Figure。
impl<T: Shape> Figure for T where T: Bounded {
    /// 绘制自身：调用 Shape 的 paint_figure
    ///
    /// 当通过 Box<dyn Figure> 调用时，会正确分派到 Shape 的实现
    fn paint_figure(&self, gc: &mut NdCanvas) {
        Shape::paint_figure(self, gc);
    }
}
