//! 递归渲染实现
//!
//! 直接递归实现 Figure 树的渲染遍历，参考 Eclipse Draw2D 的 paint() 方法。

use novadraw_render::NdCanvas;

use super::BlockId;

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

/// Figure 渲染器（递归模式）
///
/// 直接递归实现，简洁直观。
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

    /// 递归渲染
    ///
    /// 对应 d2 Figure.paint() final。
    pub fn render(&mut self, root_id: BlockId) {
        self.paint(root_id);
    }

    /// 绘制 Figure
    ///
    /// 对应 d2 Figure.paint()：
    /// ```text
    /// paint(Graphics)
    ///   ├─> setLocalBackgroundColor()
    ///   ├─> setLocalForegroundColor()
    ///   ├─> setLocalFont()
    ///   └─> pushState()
    ///         ├─> paintFigure()
    ///         ├─> restoreState()
    ///         ├─> paintClientArea()
    ///         │     └─> paintChildren() + restoreState()
    ///         ├─> paintBorder()
    ///         └─> popState()
    /// ```
    fn paint(&mut self, block_id: BlockId) {
        // 获取 block
        let block = match self.scene.get(block_id) {
            Some(b) if b.is_visible => b,
            _ => return,
        };

        // 1. 设置本地属性
        block.figure.init_properties(self.gc);

        // 2. 保存状态 → 直接调用 gc
        self.gc.push_state();

        // 3. 绘制自身
        block.figure.paint_figure(self.gc);

        // 4. 恢复上下文状态 → 直接调用 gc
        self.gc.restore_state();

        // 5. 绘制子元素区域（paintClientArea 负责 translate + clip）
        self.paint_client_area(block_id);

        // 6. 绘制边框
        // 注意：block 借用在此结束，可以安全重新获取
        let block = match self.scene.get(block_id) {
            Some(b) if b.is_visible => b,
            _ => return,
        };
        block.figure.paint_border(self.gc);

        // 7. 恢复初始状态 → 直接调用 gc
        self.gc.pop_state();
    }

    /// 绘制子元素区域
    ///
    /// 对应 d2 Figure.paintClientArea()：
    /// ```text
    /// paintClientArea(Graphics)
    ///   if (useLocalCoordinates) {
    ///     translate(x + left, y + top);
    ///     clipRect(0, 0, w - left - right, h - top - bottom);
    ///   } else {
    ///     clipRect(x, y, w, h);
    ///   }
    ///   paintChildren(graphics);
    /// ```
    fn paint_client_area(&mut self, block_id: BlockId) {
        let block = match self.scene.get(block_id) {
            Some(b) if b.is_visible => b,
            _ => return,
        };

        if block.figure.use_local_coordinates() {
            let bounds = block.figure.bounds();
            let (top, left, _, _) = block.figure.insets();
            self.gc.translate(bounds.x + left, bounds.y + top);
            self.gc
                .clip_rect(0.0, 0.0, bounds.width - left, bounds.height - top);
        } else {
            self.gc.clip_rect(
                block.figure.bounds().x,
                block.figure.bounds().y,
                block.figure.bounds().width,
                block.figure.bounds().height,
            );
        }

        self.gc.push_state();
        self.paint_children(block_id);
        self.gc.pop_state();

        // 恢复 paintClientArea 设置的裁剪区域
        self.gc.restore_state();
    }

    /// 绘制子元素
    ///
    /// 对应 d2 Figure.paintChildren()。
    /// 为每个子节点设置裁剪 + 绘制 + 恢复
    fn paint_children(&mut self, block_id: BlockId) {
        let children: Vec<BlockId> = {
            let block = match self.scene.get(block_id) {
                Some(b) if b.is_visible => b,
                _ => return,
            };
            block.children.to_vec()
        };

        // 正序遍历（与 d2 一致）
        for &child_id in &children {
            // 获取子节点 bounds
            let child_block = match self.scene.get(child_id) {
                Some(b) if b.is_visible => b,
                _ => continue,
            };
            let child_bounds = child_block.figure.bounds();

            // clipRect(child_bounds) + paint(child) + restoreState()
            self.gc.clip_rect(
                child_bounds.x,
                child_bounds.y,
                child_bounds.width,
                child_bounds.height,
            );
            self.paint(child_id);
            self.gc.restore_state();
        }
    }
}
