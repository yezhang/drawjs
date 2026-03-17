//! 迭代渲染实现
//!
//! 使用四状态栈迭代机制替代递归遍历，避免深度层次结构导致堆栈溢出。
//! 参考 Eclipse Draw2d 的 paint() 方法设计。

use novadraw_render::NdCanvas;

use super::{BlockId, SceneGraphRenderRef};
use crate::figure::Bounded;
use crate::{debug_render, trace_render};

/// 四状态渲染任务
///
/// 使用栈迭代实现 Figure 树的渲染遍历，将递归转换为显式栈操作。
/// 状态执行顺序：EnterFigure → EnterClientArea → ExitClientArea → ExitFigure
/// 与 d2 保持一致：paintChildren 中对每个子节点单独 clip 到其 bounds。
#[derive(Debug, Clone, Copy)]
enum RenderTask {
    /// 进入 Figure
    ///
    /// 执行：`push_state()` → `paint_figure()` → `restore_state()`
    /// 压栈：`ExitFigure` → `ExitClientArea` → `EnterClientArea`
    EnterFigure(BlockId),

    /// 进入子节点（设置子节点 clip）
    ///
    /// 执行：clip 到子节点 bounds
    /// 压栈：`ExitChild` → `EnterFigure`
    EnterChild(BlockId),

    /// 进入客户区域
    ///
    /// 执行：设置坐标系 (`translate` + `clip`)
    /// 压栈：`ExitClientArea` + 所有子节点的 `EnterChild`（逆序）
    EnterClientArea(BlockId),

    /// 退出客户区域
    ///
    /// 执行：`restore_state()` - 恢复到 EnterClientArea 前的状态
    ExitClientArea(BlockId),

    /// 退出子节点
    ///
    /// 执行：`restore_state()` - 恢复到 EnterChild 前的状态（parent client area）
    ExitChild(BlockId),

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
    /// 调试计数器
    counter: usize,
}

impl<'a> FigureRendererIter<'a> {
    /// 创建渲染器
    pub fn new(scene: &SceneGraphRenderRef<'a>, gc: &'a mut NdCanvas) -> Self {
        Self {
            scene: scene.clone(),
            gc,
            counter: 0,
        }
    }

    /// 迭代渲染
    ///
    /// 使用显式栈替代递归，实现四状态遍历机制。
    /// 与 d2 paintChildren 保持一致：对每个子节点单独 clip 到其 bounds。
    pub fn render(&mut self, root_id: BlockId) {
        let mut stack: Vec<RenderTask> = vec![RenderTask::EnterFigure(root_id)];

        while let Some(task) = stack.pop() {
            match task {
                RenderTask::EnterFigure(block_id) => {
                    self.handle_enter_figure(block_id, &mut stack);
                }

                RenderTask::EnterChild(block_id) => {
                    self.handle_enter_child(block_id, &mut stack);
                }

                RenderTask::EnterClientArea(block_id) => {
                    self.handle_enter_client_area(block_id, &mut stack);
                }

                RenderTask::ExitClientArea(block_id) => {
                    self.handle_exit_client_area(block_id);
                }

                RenderTask::ExitChild(block_id) => {
                    self.handle_exit_child(block_id);
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
    /// 3. 裁剪到子元素 bounds（对应 d2 paintChildren 中的 clipRect）
    /// 4. `paint_figure()`
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

        self.counter += 1;
        let id = self.counter;
        let bounds = block.figure.bounds();
        debug_render!("[ITER] #{:02} EnterFigure bounds={:?}", id, bounds);

        // 1. 初始化本地属性
        #[allow(clippy::needless_borrow)]
        block.figure.init_properties(&mut self.gc);

        // 2. 保存状态
        self.gc.push_state();

        // 3. 绘制自身（对应 d2 paintFigure）
        // 注意：不应用 clip，让描边可以超出 bounds
        // clip 只在绘制子元素时应用（对应 d2 paintClientArea）
        #[allow(clippy::needless_borrow)]
        block.figure.paint_figure(&mut self.gc);

        debug_render!("[ITER] #{:02}   paint_figure done, restore_state", id);
        self.gc.restore_state();

        // 压入后续状态（逆序压栈，确保正确执行顺序）
        stack.push(RenderTask::ExitFigure(block_id));

        stack.push(RenderTask::EnterClientArea(block_id));
    }

    /// 处理进入客户区域状态
    ///
    /// 执行：
    /// 1. 设置坐标系（`translate` + `clip`）
    /// 2. 收集子节点并压栈（每个子节点先 EnterChild clip 到 bounds，再 EnterFigure）
    fn handle_enter_client_area(&mut self, block_id: BlockId, stack: &mut Vec<RenderTask>) {
        let block = match self.scene.get(block_id) {
            Some(b) if b.is_visible => b,
            None | Some(_) => return,
        };

        self.counter += 1;
        let id = self.counter;

        // 1. 设置坐标系
        if Bounded::use_local_coordinates(block.figure.as_ref()) {
            let bounds = Bounded::bounds(block.figure.as_ref());
            let (top, left, bottom, right) = Bounded::insets(block.figure.as_ref());
            debug_render!("[ITER] #{:02} EnterClientArea use_local=true, translate({}, {}) clip(0,0,{},{})",
                id, bounds.x + left, bounds.y + top, bounds.width - left - right, bounds.height - top - bottom);
            self.gc.translate(bounds.x + left, bounds.y + top);
            // clip 到 client area = bounds - insets
            self.gc.clip_rect(
                0.0,
                0.0,
                bounds.width - left - right,
                bounds.height - top - bottom,
            );
        } else {
            let bounds = Bounded::bounds(block.figure.as_ref());
            debug_render!("[ITER] #{:02} EnterClientArea use_local=false, clip({},{},{},{})",
                id, bounds.x, bounds.y, bounds.width, bounds.height);
            self.gc
                .clip_rect(bounds.x, bounds.y, bounds.width, bounds.height);
        }

        // 2. 收集可见子节点（需要获取 bounds 用于 clip）
        let children_info: Vec<(BlockId, _)> = block
            .children
            .iter()
            .filter_map(|&child_id| {
                if let Some(child) = self.scene.get(child_id) {
                    if child.is_visible {
                        let bounds = child.figure.bounds();
                        return Some((child_id, bounds));
                    }
                }
                None
            })
            .collect();

        debug_render!("[ITER] #{:02}   children count: {}", id, children_info.len());

        stack.push(RenderTask::ExitClientArea(block_id));

        // 逆序压栈，确保先添加的子节点先渲染
        // 与 d2 paintChildren 一致：每个子节点 clip 到其 bounds
        for (child_id, _) in children_info.into_iter().rev() {
            stack.push(RenderTask::EnterChild(child_id));
        }
    }

    /// 处理进入子节点状态
    ///
    /// 执行：clip 到子节点 bounds（对应 d2 paintChildren 中的 clipRect）
    /// 压栈：`ExitChild` → `EnterFigure`
    fn handle_enter_child(&mut self, block_id: BlockId, stack: &mut Vec<RenderTask>) {
        let block = match self.scene.get(block_id) {
            Some(b) if b.is_visible => b,
            None | Some(_) => return,
        };

        self.counter += 1;
        let id = self.counter;
        let bounds = block.figure.bounds();
        debug_render!("[ITER] #{:02} EnterChild clip to bounds={:?}", id, bounds);

        // clip 到子节点 bounds（对应 d2 paintChildren 中的 clipRect）
        self.gc.clip_rect(bounds.x, bounds.y, bounds.width, bounds.height);

        // 压入后续状态
        stack.push(RenderTask::ExitChild(block_id));
        stack.push(RenderTask::EnterFigure(block_id));
    }

    /// 处理退出子节点状态
    ///
    /// 执行：`restore_state()` - 恢复到 EnterChild 前的状态（parent client area）
    fn handle_exit_child(&mut self, block_id: BlockId) {
        self.counter += 1;
        let id = self.counter;
        debug_render!("[ITER] #{:02} ExitChild restore_state", id);
        self.gc.restore_state();
    }

    /// 处理退出客户区域状态
    ///
    /// 执行：`restore_state()` - 恢复到 EnterClientArea 前的状态
    fn handle_exit_client_area(&mut self, block_id: BlockId) {
        self.counter += 1;
        let id = self.counter;
        debug_render!("[ITER] #{:02} ExitClientArea restore_state", id);
        self.gc.restore_state();
    }

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

        self.counter += 1;
        let id = self.counter;
        debug_render!("[ITER] #{:02} ExitFigure paint_border + pop_state", id);

        // 绘制边框
        #[allow(clippy::needless_borrow)]
        block.figure.paint_border(&mut self.gc);

        // 恢复初始状态
        self.gc.pop_state();
    }
}
