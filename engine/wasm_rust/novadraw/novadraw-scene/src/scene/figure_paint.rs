//! Trampoline 渲染任务系统
//!
//! 使用显式栈状态机实现 Figure 树的渲染遍历。
//! 参考 Eclipse Draw2D 的 paint() 方法调度逻辑，分为七阶段：
//! - InitProperties: 设置本地属性（颜色、字体）
//! - Prepare: 应用变换 + 准备上下文（save, translate, clip）
//! - PaintSelf: 绘制自身背景（paintFigure）
//! - PaintChildren: 遍历子元素
//! - PaintBorder: 绘制边框
//! - Cleanup: 清理（restore 状态）
//! - Finalize: 最终清理（popState）
//!
//! 核心原则：SceneGraph 不直接操作绘图状态，所有状态操作封装在 Figure Trait 方法中。
//! FigureRenderer 只负责任务调度，完全解耦。

use novadraw_render::NdCanvas;

use super::BlockId;

/// 绘制任务枚举
///
/// 七阶段遍历模式（参考 Draw2D Figure.paint）：
/// - InitProperties: 设置本地属性（颜色、字体）
/// - EnterState: pushState + prepare_context（保存状态 + 设置裁剪区）
/// - PaintSelf: 绘制自身背景（paintFigure）
/// - ResetState: restoreState（恢复 prepare_context 前的状态）
/// - PaintChildren: 遍历子元素
/// - PaintBorder: 绘制边框（paintBorder）
/// - ExitState: popState（恢复 EnterState 前的状态）
#[derive(Debug, Clone)]
pub enum PaintTask {
    /// 初始化属性阶段：设置本地颜色、字体等
    InitProperties { block_id: BlockId },

    /// 进入状态阶段：pushState + prepare_context
    /// 对应 d2: pushState() → prepare_context()
    EnterState { block_id: BlockId },

    /// 自绘阶段：绘制背景（paintFigure）
    /// 注意：不包含 restoreState，restoreState 在 ResetState 阶段单独处理
    PaintSelf { block_id: BlockId },

    /// 重置状态阶段：restoreState
    /// 对应 d2: restoreState()（在 paintFigure 和 paintChildren 之间）
    ResetState { block_id: BlockId },

    /// 绘制子元素阶段
    PaintChildren { block_id: BlockId },

    /// 绘制边框阶段
    PaintBorder { block_id: BlockId },

    /// 退出状态阶段：popState
    /// 对应 d2: popState()
    ExitState { block_id: BlockId },
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

impl<'a> Clone for SceneGraphRenderRef<'a> {
    fn clone(&self) -> Self {
        Self {
            blocks: self.blocks,
        }
    }
}

/// Figure 渲染器
///
/// 任务调度器，只负责：
/// 1. 从 Figure 树生成任务队列
/// 2. 主循环消费任务
/// 3. 调度 Figure 方法（所有 gc 操作封装在 Figure Trait 中）
pub struct FigureRenderer<'a> {
    scene: SceneGraphRenderRef<'a>,
    gc: &'a mut NdCanvas,
}

impl<'a> FigureRenderer<'a> {
    /// 创建渲染器
    pub fn new(scene: &SceneGraphRenderRef<'a>, gc: &'a mut NdCanvas) -> Self {
        Self {
            scene: SceneGraphRenderRef {
                blocks: scene.blocks,
            },
            gc,
        }
    }

    /// 从根元素渲染
    ///
    /// 模板方法，final
    /// 对应 d2: paint(Graphics) final
    ///
    /// # Draw2D 渲染流程参考
    ///
    /// ```text
    /// paint(Graphics)
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
    /// # 职责分离
    ///
    /// - FigureRenderer: 任务调度、遍历、可见性过滤
    /// - Figure: 所有绘图状态操作（save/restore/translate/clip 等）
    pub fn render(&mut self, root_id: BlockId) {
        // 1. 初始化显式任务栈
        let mut task_stack: Vec<PaintTask> = Vec::new();

        // 2. 将根节点的任务序列压入栈中
        self.push_figure_tasks(&mut task_stack, root_id);

        // 3. 蹦床循环 (Trampoline Loop) - 纯调度逻辑
        while let Some(task) = task_stack.pop() {
            match task {
                PaintTask::InitProperties { block_id } => {
                    // 初始化本地属性（颜色、字体）
                    if let Some(block) = self.scene.get(block_id) {
                        block.figure.init_properties(self.gc);
                    }
                }
                PaintTask::EnterState { block_id } => {
                    // EnterState: pushState + prepare_context
                    let bounds = {
                        if let Some(block) = self.scene.get(block_id) {
                            block.figure.bounds()
                        } else {
                            continue;
                        }
                    };

                    if let Some(block) = self.scene.get(block_id) {
                        let figure = &block.figure;
                        figure.push_state(self.gc); // pushState

                        // 根据 useLocalCoordinates 决定是否需要 translate
                        // d2 逻辑：
                        // - useLocalCoordinates = false（默认）：不 translate，使用全局坐标
                        // - useLocalCoordinates = true：translate 到 bounds 位置，使用本地坐标
                        if figure.use_local_coordinates() {
                            // 使用本地坐标：translate 到 bounds 位置
                            let (top, left, _, _) = figure.insets();
                            figure.prepare_context(self.gc, bounds, left, top);
                        } else {
                            // 不使用本地坐标：设置裁剪区，但不 translate
                            // 所有 bounds 都是绝对坐标，直接在全局坐标系中绘制
                            figure.prepare_context_no_translate(self.gc, bounds);
                        }
                    }
                }
                PaintTask::PaintSelf { block_id } => {
                    // PaintSelf: 绘制自身背景（不包含 restoreState）
                    if let Some(block) = self.scene.get(block_id) {
                        block.figure.paint_figure(self.gc);
                    }
                }
                PaintTask::ResetState { block_id } => {
                    // ResetState: restoreState
                    if let Some(block) = self.scene.get(block_id) {
                        block.figure.restore_state(self.gc);
                    }
                }
                PaintTask::PaintChildren { block_id } => {
                    if let Some(block) = self.scene.get(block_id) {
                        // 遍历子节点并逆序压栈，考虑到 z-order
                        // 特性，后加入场景树的节点会在栈底层，后渲染，视觉上在上层。
                        // 这符合预期。
                        for &child_id in block.children.iter().rev() {
                            self.push_figure_tasks(&mut task_stack, child_id);
                        }
                    }
                }
                PaintTask::PaintBorder { block_id } => {
                    if let Some(block) = self.scene.get(block_id) {
                        let figure = &block.figure;

                        figure.paint_border(self.gc);

                        // 高亮由 Selection 机制处理，不在 Figure 职责范围内
                        // 参考 d2 Figure.paint() 不包含 highlight 绘制
                    }
                }
                PaintTask::ExitState { block_id } => {
                    // ExitState: popState
                    if let Some(block) = self.scene.get(block_id) {
                        block.figure.pop_state(self.gc);
                    }
                }
            }
        }
    }

    /// 辅助方法：获取 Figure 的任务流并逆序压入全局任务栈
    fn push_figure_tasks(&self, stack: &mut Vec<PaintTask>, id: BlockId) {
        let block = match self.scene.get(id) {
            Some(b) if b.is_visible => b,
            _ => return,
        };
        let tasks = block.figure.paint(id);
        // 逆序压栈，确保 tasks[0] 在栈顶被最先执行
        for task in tasks.into_iter().rev() {
            stack.push(task);
        }
    }

    /// 获取块
    pub fn block(&self, id: BlockId) -> Option<&super::RuntimeBlock> {
        self.scene.get(id)
    }

    /// 获取 GC
    #[deprecated(since = "0.1.0", note = "FigureRenderer 应完全解耦，不直接操作 gc")]
    pub fn gc(&mut self) -> &mut NdCanvas {
        self.gc
    }
}
