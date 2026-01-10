//! Trampoline 渲染任务系统
//!
//! 使用 Trampoline 模式实现 Figure 树的渲染遍历，
//! 解决递归导致的栈溢出问题，同时保持与 draw2d 相同的扩展能力。

use novadraw_render::NdCanvas;

use super::BlockId;

/// 绘制任务枚举
///
/// 所有绘制操作都通过任务表示。任务在堆上队列中顺序执行，
/// 避免深度层次结构导致的栈溢出。
#[derive(Debug, Clone)]
pub enum PaintTask {
    /// 保存状态 - gc.save()
    Save,

    /// 恢复状态 - gc.restore()
    Restore,

    /// 平移变换 - gc.translate(x, y)
    Translate {
        x: f64,
        y: f64,
    },

    /// 缩放变换 - gc.scale(x, y)
    Scale {
        x: f64,
        y: f64,
    },

    /// 裁剪矩形 - gc.clip_rect(x, y, w, h)
    Clip {
        x: f64,
        y: f64,
        w: f64,
        h: f64,
    },

    /// 绘制自身 - block.figure.paint_figure(gc)
    PaintFigure {
        block_id: BlockId,
    },

    /// 绘制边框 - block.figure.paint_border(gc)
    PaintBorder {
        block_id: BlockId,
    },

    /// 绘制高亮 - block.figure.paint_highlight(gc)
    PaintHighlight {
        block_id: BlockId,
    },

    /// 空任务（占位）
    Nop,
}

/// 场景图引用（用于渲染）
pub struct SceneGraphRenderRef<'a> {
    pub(crate) blocks: &'a slotmap::SlotMap<BlockId, super::RuntimeBlock>,
}

impl<'a> SceneGraphRenderRef<'a> {
    /// 获取块
    pub fn get(&self, id: BlockId) -> Option<&super::RuntimeBlock> {
        self.blocks.get(id)
    }
}

/// Figure 渲染器
///
/// 任务调度器，负责：
/// 1. 从 Figure 树生成任务队列
/// 2. 从队列取任务并执行
pub struct FigureRenderer<'a> {
    scene: SceneGraphRenderRef<'a>,
    gc: &'a mut NdCanvas,
    tasks: Vec<PaintTask>,
}

impl<'a> FigureRenderer<'a> {
    /// 创建渲染器
    pub fn new(scene: &SceneGraphRenderRef<'a>, gc: &'a mut NdCanvas) -> Self {
        Self {
            scene: SceneGraphRenderRef {
                blocks: scene.blocks,
            },
            gc,
            tasks: Vec::new(),
        }
    }

    /// 从根元素渲染
    pub fn render(&mut self, root_id: BlockId) {
        self.tasks.clear();

        if let Some(root) = self.scene.get(root_id) {
            self.tasks = root.figure.generate_paint_tasks(self, root_id);
        }

        while let Some(task) = self.tasks.pop() {
            self.execute(task);
        }
    }

    /// 执行任务
    fn execute(&mut self, task: PaintTask) {
        match task {
            PaintTask::Save => self.gc.save(),
            PaintTask::Restore => self.gc.restore(),
            PaintTask::Translate { x, y } => self.gc.translate(x, y),
            PaintTask::Scale { x, y } => self.gc.scale(x, y),
            PaintTask::Clip { x, y, w, h } => self.gc.clip_rect(x, y, w, h),
            PaintTask::PaintFigure { block_id } => {
                if let Some(block) = self.scene.get(block_id) {
                    block.figure.paint_figure(self.gc);
                }
            }
            PaintTask::PaintBorder { block_id } => {
                if let Some(block) = self.scene.get(block_id) {
                    block.figure.paint_border(self.gc);
                }
            }
            PaintTask::PaintHighlight { block_id } => {
                if let Some(block) = self.scene.get(block_id) {
                    block.figure.paint_highlight(self.gc);
                }
            }
            PaintTask::Nop => {}
        }
    }

    /// 获取块
    pub fn block(&self, id: BlockId) -> Option<&super::RuntimeBlock> {
        self.scene.get(id)
    }

    /// 获取 GC
    pub fn gc(&mut self) -> &mut NdCanvas {
        self.gc
    }
}
