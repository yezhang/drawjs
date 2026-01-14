//! Figure 渲染接口
//!
//! 定义图形渲染的通用接口，遵循 Eclipse Draw2D 设计模式。
//! Figure 只负责渲染，不包含状态（状态在 RuntimeBlock 中）。

mod basic;
pub use basic::Rectangle;
pub use basic::RectangleFigure;

use novadraw_core::Color;
use novadraw_geometry::{Point, Rect};
use novadraw_render::NdCanvas;

use crate::scene::{BlockId, figure_paint::PaintTask};

/// Figure 渲染 trait
///
/// 所有图形对象都需要实现此 trait。
/// 只包含渲染相关方法，不包含状态（状态在 RuntimeBlock 中）。
///
/// # 渲染流程（参考 Draw2D）
///
/// ```text
/// paint(Graphics) [final]
///   ├─> setLocalBackgroundColor()  [InitProperties]
///   ├─> setLocalForegroundColor()  [InitProperties]
///   ├─> setLocalFont()             [InitProperties]
///   └─> pushState()
///         ├─> prepare_context()            [EnterState]
///         ├─> paintFigure()                [PaintSelf]
///         ├─> restoreState()               [ResetState]
///         ├─> paintClientArea()            [PaintChildren]
///         │     └─> paintChildren()        [PaintChildren]
///         ├─> paintBorder()                [PaintBorder]
///         └─> paintHighlight()             [PaintBorder]
///       popState()                         [ExitState]
/// ```
///
/// # 七阶段任务模型
///
/// - **InitProperties**: 设置本地属性（背景色、前景色、字体）
/// - **EnterState**: pushState + prepare_context（保存状态 + 设置裁剪区）
/// - **PaintSelf**: 绘制自身背景（paintFigure）
/// - **ResetState**: restoreState（恢复 prepare_context 前的状态）
/// - **PaintChildren**: 遍历子元素
/// - **PaintBorder**: 绘制边框（paintBorder）
/// - **ExitState**: popState（恢复 EnterState 前的状态）
pub trait Figure: Send + Sync {
    /// ===== 模板方法（final，不能覆盖）=====

    /// 生成绘制任务序列
    ///
    /// 模板方法，final。对应 d2: paint(Graphics) final
    fn paint(&self, block_id: BlockId) -> Vec<PaintTask> {
        vec![
            PaintTask::InitProperties { block_id },
            PaintTask::EnterState { block_id },
            PaintTask::PaintSelf { block_id },
            PaintTask::ResetState { block_id },
            PaintTask::PaintChildren { block_id },
            PaintTask::PaintBorder { block_id },
            PaintTask::ExitState { block_id },
        ]
    }

    /// ===== InitProperties 阶段方法 =====

    /// 初始化本地属性
    ///
    /// 对应 d2: setLocalBackgroundColor/ForegroundColor/Font
    /// 设置图形的本地渲染属性（颜色、字体等）
    fn init_properties(&self, _gc: &mut NdCanvas) {
        // 默认空实现，子类可覆盖
    }

    /// 获取本地背景色
    fn local_background_color(&self) -> Option<Color> {
        None
    }

    /// 获取本地前景色
    fn local_foreground_color(&self) -> Option<Color> {
        None
    }

    /// 获取本地字体
    fn local_font(&self) -> Option<String> {
        None
    }

    /// ===== EnterState 阶段方法 =====

    /// 进入状态
    ///
    /// 对应 d2: pushState()
    /// 保存当前渲染状态，子类可覆盖以添加额外的状态保存逻辑
    fn push_state(&self, gc: &mut NdCanvas) {
        gc.push_state();
    }

    /// 准备绘图上下文
    ///
    /// 对应 d2: prepare_context()
    /// 在已保存的状态基础上设置裁剪区和变换
    /// 注意：此方法不保存状态，状态保存由 push_state() 处理
    fn prepare_context(&self, gc: &mut NdCanvas, bounds: Rect) {
        gc.translate(bounds.x, bounds.y);
        gc.clip_rect(0.0, 0.0, bounds.width, bounds.height);
    }

    /// ===== PaintSelf 阶段方法 =====

    /// 绘制自身（背景）
    ///
    /// 对应 d2: paintFigure(Graphics)
    fn paint_figure(&self, _gc: &mut NdCanvas) {}

    /// ===== ResetState 阶段方法 =====

    /// 重置状态
    ///
    /// 对应 d2: restoreState()
    /// 恢复 prepare_context() 之前的渲染状态
    fn restore_state(&self, gc: &mut NdCanvas) {
        gc.restore_state();
    }

    /// ===== PaintChildren 阶段方法 =====

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

    /// 是否优化裁剪
    ///
    /// 对应 d2: optimizeClip()
    /// 返回 true 表示不裁剪（边框不透明时优化）
    fn optimize_clip(&self) -> bool {
        false
    }

    /// 获取裁剪策略
    ///
    /// 对应 d2: getClippingStrategy()
    /// 返回 None 使用默认裁剪策略
    fn clipping_strategy(&self) -> Option<&dyn ClippingStrategy> {
        None
    }

    /// ===== PaintBorder 阶段方法 =====

    /// 绘制边框
    ///
    /// 对应 d2: paintBorder(Graphics)
    fn paint_border(&self, _gc: &mut NdCanvas) {}

    /// 绘制选中高亮
    fn paint_highlight(&self, gc: &mut NdCanvas) {
        let bounds = self.bounds();
        gc.set_stroke_color(Color::hex("#f39c12"));
        gc.set_line_width(2.0);
        gc.stroke_rect(
            bounds.x - 2.0,
            bounds.y - 2.0,
            bounds.width + 4.0,
            bounds.height + 4.0,
        );
    }

    /// ===== ExitState 阶段方法 =====

    /// 退出状态
    ///
    /// 对应 d2: popState()
    /// 恢复 pushState() 之前的渲染状态
    fn pop_state(&self, gc: &mut NdCanvas) {
        gc.pop_state();
    }

    /// ===== 基础方法 =====

    /// 获取图形边界
    fn bounds(&self) -> Rect;

    /// 命中测试（默认使用 bounds）
    fn hit_test(&self, point: Point) -> bool {
        self.bounds().contains(point)
    }

    /// 是否不透明（用于裁剪优化）
    ///
    /// 对应 d2: isOpaque()
    fn is_opaque(&self) -> bool {
        false
    }

    /// 作为不可变矩形图形获取
    fn as_rectangle(&self) -> Option<&Rectangle> {
        None
    }

    /// 作为可变矩形图形获取
    fn as_rectangle_mut(&mut self) -> Option<&mut Rectangle> {
        None
    }

    /// 获取名称（用于调试）
    fn name(&self) -> &'static str {
        "Figure"
    }

    /// ===== 辅助方法（可 override）=====

    /// 获取客户区域
    fn client_area(&self) -> Rect {
        self.bounds()
    }

    /// 获取内边距 (top, left, bottom, right)
    fn insets(&self) -> (f64, f64, f64, f64) {
        (0.0, 0.0, 0.0, 0.0)
    }
}

/// 裁剪策略 trait
///
/// 负责计算子元素的裁剪区域。
/// 对应 d2: IClippingStrategy
pub trait ClippingStrategy {
    /// 获取子元素的裁剪区域
    ///
    /// 返回子元素可以绘制的裁剪区域数组
    fn get_clip(&self, child: &dyn Figure) -> Vec<Rect>;
}

/// 默认裁剪策略
///
/// 使用子元素的 bounds 作为裁剪区域
pub struct DefaultClippingStrategy;

impl ClippingStrategy for DefaultClippingStrategy {
    fn get_clip(&self, child: &dyn Figure) -> Vec<Rect> {
        vec![child.bounds()]
    }
}

/// 基础图形（默认实现）
///
/// 简单的图形实现，包含边界矩形。
/// 可用于创建占位图形或作为自定义图形的基础。
#[derive(Clone, Copy, Debug)]
pub struct BaseFigure {
    /// 边界矩形
    pub bounds: Rect,
}

impl BaseFigure {
    /// 创建新的基础图形
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self {
            bounds: Rect::new(x, y, width, height),
        }
    }

    /// 设置边界
    pub fn set_bounds(&mut self, x: f64, y: f64, width: f64, height: f64) {
        self.bounds = Rect::new(x, y, width, height);
    }
}

impl Figure for BaseFigure {
    fn bounds(&self) -> Rect {
        self.bounds
    }

    fn paint_highlight(&self, _gc: &mut NdCanvas) {}

    fn name(&self) -> &'static str {
        "BaseFigure"
    }
}
