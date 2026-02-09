//! 迭代渲染实现
//!
//! 使用四状态栈迭代机制替代递归遍历，避免深度层次结构导致堆栈溢出。
//! 参考 Eclipse Draw2d 的 paint() 方法设计。

use novadraw_render::NdCanvas;

use super::{BlockId, SceneGraphRenderRef};

/// 四状态渲染任务
///
/// 使用栈迭代实现 Figure 树的渲染遍历，将递归转换为显式栈操作。
/// 状态执行顺序：EnterFigure → EnterClientArea → ExitClientArea → ExitFigure
#[derive(Debug, Clone, Copy)]
enum RenderTask {
    /// 进入 Figure
    ///
    /// 执行：`push_state()` → `paint_figure()`
    /// 压栈：`ExitFigure` → `ExitClientArea` → `EnterClientArea`
    EnterFigure(BlockId),

    /// 进入客户区域
    ///
    /// 执行：设置坐标系 (`translate` + `clip`)
    /// 压栈：`ExitClientArea` + 所有子节点的 `EnterFigure`（逆序）
    EnterClientArea(BlockId),

    /// 退出客户区域
    ///
    /// 执行：`restore_state()` - 恢复到 EnterClientArea 前的状态
    ExitClientArea(BlockId),

    /// 退出 Figure
    ///
    /// 执行：`paint_border()` → `pop_state()`
    ExitFigure(BlockId),
}

/// Figure 渲染器（迭代模式）
///
/// 使用显式栈实现迭代式深度优先遍历，避免递归栈溢出。
pub struct FigureRendererIter<'a> {
    scene: SceneGraphRenderRef<'a>,
    gc: &'a mut NdCanvas,
}

impl<'a> FigureRendererIter<'a> {
    /// 创建渲染器
    pub fn new(scene: &SceneGraphRenderRef<'a>, gc: &'a mut NdCanvas) -> Self {
        Self {
            scene: scene.clone(),
            gc,
        }
    }

    /// 迭代渲染
    ///
    /// 使用显式栈替代递归，实现四状态遍历机制。
    pub fn render(&mut self, root_id: BlockId) {
        let mut stack: Vec<RenderTask> = vec![RenderTask::EnterFigure(root_id)];

        while let Some(task) = stack.pop() {
            match task {
                RenderTask::EnterFigure(block_id) => {
                    self.handle_enter_figure(block_id, &mut stack);
                }

                RenderTask::EnterClientArea(block_id) => {
                    self.handle_enter_client_area(block_id, &mut stack);
                }

                RenderTask::ExitClientArea(block_id) => {
                    self.handle_exit_client_area(block_id);
                }

                RenderTask::ExitFigure(block_id) => {
                    self.handle_exit_figure(block_id);
                }
            }
        }
    }

    /// 处理进入 Figure 状态
    ///
    /// 执行：
    /// 1. 初始化本地属性
    /// 2. `push_state()`
    /// 3. `paint_figure()`
    ///
    /// 压栈顺序（后进先出，确保正确执行顺序）：
    /// - `ExitFigure` - 最后执行
    /// - `ExitClientArea` - 倒数第二
    /// - `EnterClientArea` - 最早执行
    fn handle_enter_figure(&mut self, block_id: BlockId, stack: &mut Vec<RenderTask>) {
        let block = match self.scene.get(block_id) {
            Some(b) if b.is_visible => b,
            None | Some(_) => return,
        };

        // 1. 初始化本地属性
        block.figure.init_properties(&mut self.gc);

        // 2. 保存状态
        self.gc.push_state();

        // 3. 绘制自身（背景）
        block.figure.paint_figure(&mut self.gc);

        self.gc.restore_state();

        // 压入后续状态（逆序压栈，确保正确执行顺序）
        stack.push(RenderTask::ExitFigure(block_id));

        stack.push(RenderTask::EnterClientArea(block_id));
    }

    /// 处理进入客户区域状态
    ///
    /// 执行：
    /// 1. 设置坐标系（`translate` + `clip`）
    /// 2. 收集子节点并压栈
    fn handle_enter_client_area(&mut self, block_id: BlockId, stack: &mut Vec<RenderTask>) {
        let block = match self.scene.get(block_id) {
            Some(b) if b.is_visible => b,
            None | Some(_) => return,
        };

        // 1. 设置坐标系
        if block.figure.use_local_coordinates() {
            let bounds = block.figure.bounds();
            let (top, left, _, _) = block.figure.insets();
            self.gc.translate(bounds.x + left, bounds.y + top);
            self.gc
                .clip_rect(0.0, 0.0, bounds.width - left, bounds.height - top);
        } else {
            let bounds = block.figure.bounds();
            self.gc
                .clip_rect(bounds.x, bounds.y, bounds.width, bounds.height);
        }

        // 2. 收集可见子节点（逆序压栈，保持正序遍历）
        let children: Vec<BlockId> = block
            .children
            .iter()
            .filter(|&&child_id| {
                if let Some(child) = self.scene.get(child_id) {
                    child.is_visible
                } else {
                    false
                }
            })
            .copied()
            .collect();

        stack.push(RenderTask::ExitClientArea(block_id));

        // 逆序压栈，确保先添加的子节点先渲染
        for child_id in children.into_iter().rev() {
            stack.push(RenderTask::EnterFigure(child_id));
        }
    }

    /// 处理退出客户区域状态
    ///
    /// 执行：`restore_state()` - 恢复到 EnterClientArea 前的状态
    fn handle_exit_client_area(&mut self, _block_id: BlockId) {}

    /// 处理退出 Figure 状态
    ///
    /// 执行：
    /// 1. `paint_border()` - 绘制边框
    /// 2. `pop_state()` - 恢复初始状态
    fn handle_exit_figure(&mut self, block_id: BlockId) {
        let block = match self.scene.get(block_id) {
            Some(b) if b.is_visible => b,
            None | Some(_) => return,
        };

        // 绘制边框
        block.figure.paint_border(&mut self.gc);

        // 恢复初始状态
        self.gc.pop_state();
    }
}
