//! Figure 渲染接口
//!
//! 定义图形渲染的通用接口，遵循 Eclipse Draw2D 设计模式。
//! Figure 只负责渲染，不包含状态（状态在 RuntimeBlock 中）。

mod basic;
mod scalable;
mod viewport_figure;
pub use scalable::ScalableFigure;
pub use basic::Rectangle;
pub use basic::RectangleFigure;
pub use viewport_figure::ViewportFigure;

use novadraw_core::Color;
use novadraw_render::NdCanvas;

use crate::scene::{BlockId, Point, Rect};

/// Figure 渲染 trait
///
/// 所有图形对象都需要实现此 trait。
/// 只包含渲染相关方法，不包含状态（状态在 RuntimeBlock 中）。
pub trait Figure: Send + Sync {
    /// 获取图形边界
    fn bounds(&self) -> Rect;

    /// 命中测试（默认使用 bounds）
    fn hit_test(&self, point: Point) -> bool {
        self.bounds().contains(point)
    }

    /// 是否为视口容器
    ///
    /// 视口容器会在 paint_figure() 中应用变换并自行处理子元素渲染。
    /// 对应 Draw2D 的 Viewport 行为。
    fn is_viewport_container(&self) -> bool {
        false
    }

    /// 绘制图形
    fn paint(&self, gc: &mut NdCanvas);

    /// 绘制自身（背景）
    ///
    /// 对应 d2: paintFigure()
    /// 默认调用 paint()，可被子类重写
    fn paint_figure(&self, gc: &mut NdCanvas) {
        self.paint(gc);
    }

    /// 绘制边框
    ///
    /// 对应 d2: paintBorder()
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

    /// 是否不透明（用于裁剪优化）
    ///
    /// 对应 d2: isOpaque()
    /// 不透明图形可跳过子元素裁剪（如果无边框）
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

    /// 作为可变视口图形获取
    fn as_viewport_figure_mut(&mut self) -> Option<&mut super::ViewportFigure> {
        None
    }

    /// 获取名称（用于调试）
    fn name(&self) -> &'static str {
        "Figure"
    }

    /// ===== Trampoline 任务生成方法 =====
    ///
    /// 这些方法定义"绘制流程"（how），返回任务队列而非直接调用 gc。
    /// 子类可 override 自定义行为。

    /// 生成完整绘制任务队列（模板方法，final 不能覆盖）
    ///
    /// 对应 draw2d: paint() final
    fn generate_paint_tasks(
        &self,
        renderer: &FigureRendererRef<'_>,
        block_id: BlockId,
    ) -> Vec<PaintTask> {
        let mut tasks = Vec::new();

        tasks.extend(self.paint_client_area(renderer, block_id));

        tasks.push(PaintTask::PaintFigure { block_id });
        tasks.push(PaintTask::PaintBorder { block_id });

        if renderer.block(block_id).map(|b| b.is_selected).unwrap_or(false) {
            tasks.push(PaintTask::PaintHighlight { block_id });
        }

        tasks
    }

    /// 绘制客户区域（包含子元素）
    ///
    /// 对应 draw2d: paintClientArea()
    /// 子类可 override 自定义流程（如 Viewport 添加滚动平移）
    fn paint_client_area(
        &self,
        renderer: &FigureRendererRef<'_>,
        block_id: BlockId,
    ) -> Vec<PaintTask> {
        let mut tasks = Vec::new();
        let block = match renderer.block(block_id) {
            Some(b) if b.is_visible => b,
            _ => return tasks,
        };

        tasks.push(PaintTask::Save);

        let bounds = self.bounds();
        let trans = block.transform.translation();
        tasks.push(PaintTask::Translate {
            x: trans.x() + bounds.x,
            y: trans.y() + bounds.y,
        });

        if !self.optimize_clip() {
            let area = self.client_area();
            tasks.push(PaintTask::Clip {
                x: area.x,
                y: area.y,
                w: area.width,
                h: area.height,
            });
        }

        tasks.extend(self.paint_children(renderer, block_id));

        tasks.push(PaintTask::Restore);

        tasks
    }

    /// 绘制子元素
    ///
    /// 对应 draw2d: paintChildren()
    /// 子类可 override 自定义子元素遍历逻辑
    fn paint_children(
        &self,
        renderer: &FigureRendererRef<'_>,
        block_id: BlockId,
    ) -> Vec<PaintTask> {
        let mut tasks = Vec::new();
        let block = match renderer.block(block_id) {
            Some(b) if b.is_visible => b,
            _ => return tasks,
        };

        for &child_id in block.children.iter() {
            if let Some(child) = renderer.block(child_id) {
                if child.is_visible {
                    tasks.extend(child.figure.generate_paint_tasks(renderer, child_id));
                }
            }
        }

        tasks
    }

    /// ===== 辅助方法（可 override） =====

    /// 是否使用局部坐标
    ///
    /// 为 true 时，绘制方法使用局部坐标（相对于 bounds 原点）。
    /// 为 false 时，绘制方法使用世界坐标（已应用变换）。
    fn use_local_coordinates(&self) -> bool {
        false
    }

    /// 是否优化裁剪
    ///
    /// 为 true 时，跳过裁剪任务（提高性能）。
    /// 为 false 时，添加裁剪任务。
    fn optimize_clip(&self) -> bool {
        true
    }

    /// 获取客户区域
    ///
    /// 默认返回 bounds，可被子类 override 定义实际可绘制区域。
    fn client_area(&self) -> Rect {
        self.bounds()
    }

    /// 获取内边距 (top, left, bottom, right)
    fn insets(&self) -> (f64, f64, f64, f64) {
        (0.0, 0.0, 0.0, 0.0)
    }
}

use crate::scene::trampoline::PaintTask;

pub use crate::scene::trampoline::FigureRenderer as FigureRendererRef;

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

    fn paint(&self, _gc: &mut NdCanvas) {}

    fn paint_highlight(&self, _gc: &mut NdCanvas) {}

    fn name(&self) -> &'static str {
        "BaseFigure"
    }
}
