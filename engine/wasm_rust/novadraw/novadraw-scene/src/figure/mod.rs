//! Figure 渲染接口
//!
//! 定义图形渲染的通用接口，遵循 Eclipse Draw2D 设计模式。
//! Figure 只负责渲染，不包含状态（状态在 RuntimeBlock 中）。

mod basic;
pub use basic::EllipseFigure;
pub use basic::LineFigure;
pub use basic::RectangleFigure;

use novadraw_geometry::Rectangle;
use novadraw_render::NdCanvas;

/// Figure 渲染 trait
///
/// 所有图形对象都需要实现此 trait。
/// 只包含渲染相关方法，不包含状态（状态在 RuntimeBlock 中）。
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
pub trait Figure: Send + Sync {
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
    fn paint_figure(&self, _gc: &mut NdCanvas) {}

    /// ===== PaintChildren 相关方法 =====

    /// 绘制子元素
    ///
    /// 对应 d2 paintChildren(Graphics)
    /// 默认行为由渲染器调度 PaintChildren 任务
    fn paint_children(&self) {
        // 默认行为由渲染器处理
    }

    /// 是否使用本地坐标
    ///
    /// 对应 d2: useLocalCoordinates()
    /// true: 子元素使用 Figure 内部坐标（相对于 bounds 左上角）
    /// false: 子元素使用父节点坐标（默认）
    fn use_local_coordinates(&self) -> bool {
        false
    }

    /// ===== PaintBorder 阶段方法 =====

    /// 绘制边框
    ///
    /// 对应 d2: paintBorder(Graphics)
    fn paint_border(&self, _gc: &mut NdCanvas) {}

    /// ===== 基础方法 =====

    /// 获取图形边界
    fn bounds(&self) -> Rectangle;

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

    /// 设置图形边界
    ///
    /// 对应 d2: setBounds(Rectangle)
    /// 注意：本实现只更新 bounds 本身，不触发事件通知
    /// 事件通知（fireFigureMoved, repaint）由 RuntimeBlock 或 SceneGraph 处理
    fn set_bounds(&mut self, x: f64, y: f64, width: f64, height: f64) {
        if let Some(rect) = self.as_rectangle_mut() {
            rect.bounds = Rectangle::new(x, y, width, height);
        }
    }

    /// 作为不可变矩形图形获取
    fn as_rectangle(&self) -> Option<&RectangleFigure> {
        None
    }

    /// 作为可变矩形图形获取
    fn as_rectangle_mut(&mut self) -> Option<&mut RectangleFigure> {
        None
    }

    /// 获取名称（用于调试）
    fn name(&self) -> &'static str {
        "Figure"
    }

    /// ===== 辅助方法（可 override）=====

    /// 获取内边距 (top, left, bottom, right)
    fn insets(&self) -> (f64, f64, f64, f64) {
        (0.0, 0.0, 0.0, 0.0)
    }
}

/// 基础图形（默认实现）
///
/// 简单的图形实现，包含边界矩形。
/// 可用于创建占位图形或作为自定义图形的基础。
#[derive(Clone, Copy, Debug)]
pub struct BaseFigure {
    /// 边界矩形
    pub bounds: Rectangle,
}

impl BaseFigure {
    /// 创建新的基础图形
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self {
            bounds: Rectangle::new(x, y, width, height),
        }
    }

    /// 设置边界
    pub fn set_bounds(&mut self, x: f64, y: f64, width: f64, height: f64) {
        self.bounds = Rectangle::new(x, y, width, height);
    }
}

impl Figure for BaseFigure {
    fn bounds(&self) -> Rectangle {
        self.bounds
    }

    fn contains_point(&self, x: f64, y: f64) -> bool {
        let b = self.bounds;
        x >= b.x && x <= b.x + b.width && y >= b.y && y <= b.y + b.height
    }

    fn intersects(&self, rect: Rectangle) -> bool {
        let b = self.bounds;
        b.x < rect.x + rect.width
            && b.x + b.width > rect.x
            && b.y < rect.y + rect.height
            && b.y + b.height > rect.y
    }

    fn set_bounds(&mut self, x: f64, y: f64, width: f64, height: f64) {
        self.bounds = Rectangle::new(x, y, width, height);
    }

    fn name(&self) -> &'static str {
        "BaseFigure"
    }
}
