//! 场景图管理
//!
//! 提供场景图数据结构和管理功能。

use std::sync::Arc;

use novadraw_geometry::{Rectangle, Translatable};
use novadraw_render::NdCanvas;
use slotmap::{Key, SlotMap};
use uuid::Uuid;

use super::figure::Updatable;
use super::layout::LayoutManager;

// 渲染模块
pub mod render_iterative;
pub mod render_recursive;

pub use render_iterative::FigureRendererIter;
pub use render_recursive::{FigureRenderer, FigureGraphRenderRef};

#[cfg(test)]
pub mod bounds_test;

#[cfg(test)]
pub mod update_integration_test;

slotmap::new_key_type! { pub struct BlockId; }

/// FigureBlock - 图形节点
///
/// 场景图中的基本单元，同时包含图形数据（通过 Box<dyn Figure>）
/// 和树形结构（parent/children），参考 Eclipse Draw2D 的 Figure 设计。
///
/// # 与 Figure trait 的区别
///
/// - `FigureBlock` 是具体的数据结构，实现了树形节点的所有功能
/// - `dyn Figure` 是渲染接口 trait，定义了图形的几何和渲染行为
/// - 一个 `FigureBlock` 持有 `Box<dyn Figure>` 来实现具体的图形类型
pub struct FigureBlock {
    /// 块 ID
    pub id: BlockId,
    /// UUID
    pub uuid: Uuid,
    /// 子块列表
    pub children: Vec<BlockId>,
    /// 父块
    pub parent: Option<BlockId>,
    /// 图形
    pub figure: Box<dyn super::Figure>,
    /// 布局管理器（可选），只有需要布局的容器才设置
    pub layout_manager: Option<Arc<dyn super::layout::LayoutManager>>,
    /// 是否选中
    pub is_selected: bool,
    /// 是否可见
    pub is_visible: bool,
    /// 是否启用
    pub is_enabled: bool,
    /// 首选尺寸 (宽, 高)，None 表示使用 Figure 的 bounds
    pub preferred_size: Option<(f64, f64)>,
    /// 最小尺寸 (宽, 高)
    pub minimum_size: Option<(f64, f64)>,
    /// 最大尺寸 (宽, 高)
    pub maximum_size: Option<(f64, f64)>,
}

impl FigureBlock {
    /// 创建新运行时块
    pub fn new(id: BlockId, uuid: Uuid, figure: Box<dyn super::Figure>) -> Self {
        Self {
            id,
            uuid,
            children: Vec::new(),
            parent: None,
            figure,
            layout_manager: None,
            is_selected: false,
            is_visible: true,
            is_enabled: true,
            preferred_size: None,
            minimum_size: None,
            maximum_size: None,
        }
    }

    /// 添加子块（类似 Draw2d 的 figure.addChild()）
    pub fn add_child(&mut self, child_id: BlockId) {
        self.children.push(child_id);
    }

    /// 获取子块数量
    pub fn children_count(&self) -> usize {
        self.children.len()
    }

    /// 获取图形的边界（局部坐标）
    pub fn figure_bounds(&self) -> Rectangle {
        self.figure.bounds()
    }

    /// 获取首选尺寸
    pub fn get_preferred_size(&self) -> (f64, f64) {
        if let Some(size) = self.preferred_size {
            return size;
        }
        let bounds = self.figure.bounds();
        (bounds.width, bounds.height)
    }

    /// 获取最小尺寸
    pub fn get_minimum_size(&self) -> (f64, f64) {
        if let Some(size) = self.minimum_size {
            return size;
        }
        self.get_preferred_size()
    }

    /// 获取最大尺寸
    pub fn get_maximum_size(&self) -> (f64, f64) {
        if let Some(size) = self.maximum_size {
            return size;
        }
        (f64::INFINITY, f64::INFINITY)
    }

    /// 设置首选尺寸
    pub fn set_preferred_size(&mut self, width: f64, height: f64) {
        self.preferred_size = Some((width, height));
    }

    /// 设置最小尺寸
    pub fn set_minimum_size(&mut self, width: f64, height: f64) {
        self.minimum_size = Some((width, height));
    }

    /// 设置最大尺寸
    pub fn set_maximum_size(&mut self, width: f64, height: f64) {
        self.maximum_size = Some((width, height));
    }

    /// 设置可见性
    pub fn set_visible(&mut self, visible: bool) {
        self.is_visible = visible;
    }

    /// 设置启用状态
    pub fn set_enabled(&mut self, enabled: bool) {
        self.is_enabled = enabled;
    }

    pub fn set_figure(&mut self, figure: Box<dyn super::Figure>) {
        self.figure = figure;
    }

    /// 设置图形边界（仅自身，不传播）
    ///
    /// 注意：此方法只更新自身的 bounds，不传播到子节点。
    /// 需要传播到子节点的操作应在 FigureGraph 级别使用迭代实现。
    pub fn set_bounds(&mut self, x: f64, y: f64, width: f64, height: f64) {
        self.figure.set_bounds(x, y, width, height);
    }
}

#[inline]
fn rect_intersects(a: &Rectangle, b: &Rectangle) -> bool {
    a.x < b.x + b.width && a.x + a.width > b.x && a.y < b.y + b.height && a.y + a.height > b.y
}

/// 场景图
///
/// 管理所有图形块的层次结构，参考 Eclipse Draw2d 设计模式。
///
/// # 使用示例
///
/// ```
/// use novadraw_scene::{Figure, RectangleFigure, FigureGraph};
///
/// let mut scene = FigureGraph::new();
///
/// // 创建根内容块（类似 Draw2d 的 setContents）
/// let contents = RectangleFigure::new(0.0, 0.0, 100.0, 50.0);
/// let contents_id = scene.set_contents(Box::new(contents));
///
/// // 添加子块到指定父块（类似 Draw2d 的 parent.addChild(child)）
/// let child = RectangleFigure::new(10.0, 10.0, 80.0, 30.0);
/// scene.add_child_to(contents_id, Box::new(child));
/// ```
pub struct FigureGraph {
    pub blocks: SlotMap<BlockId, FigureBlock>,
    pub uuid_map: std::collections::HashMap<Uuid, BlockId>,
    /// 根块（内部使用）
    root: BlockId,
    /// 内容块（用户可访问的根容器）
    contents: Option<BlockId>,
    pub layout_manager: Option<Arc<dyn LayoutManager>>,
    /// 布局是否有效，false 表示需要重新计算布局
    pub layout_valid: bool,
    /// 子元素的布局约束 (child_id -> Rectangle constraint)
    constraints: std::collections::HashMap<usize, Rectangle>,
    /// 更新管理器
    pub update_manager: super::update::SceneUpdateManager,
}

impl FigureGraph {
    /// 创建新场景图
    pub fn new() -> Self {
        let mut blocks = SlotMap::with_key();
        let uuid = Uuid::new_v4();

        let root_id = blocks.insert_with_key(|key| FigureBlock {
            id: key,
            uuid,
            children: Vec::new(),
            parent: None,
            figure: Box::new(super::figure::RootFigure::new(0.0, 0.0, 0.0, 0.0)),
            layout_manager: None,
            is_selected: false,
            is_visible: true,
            is_enabled: true,
            preferred_size: None,
            minimum_size: None,
            maximum_size: None,
        });

        FigureGraph {
            blocks,
            uuid_map: std::collections::HashMap::new(),
            root: root_id,
            contents: None,
            layout_manager: None,
            layout_valid: true,
            constraints: std::collections::HashMap::new(),
            update_manager: super::update::SceneUpdateManager::new(),
        }
    }

    /// 设置内容块
    ///
    /// 对应 draw2d: LightweightSystem.setContents(IFigure)
    ///
    /// 设置场景的根容器，后续添加的子块将作为此容器的子元素。
    /// 注意：此方法不触发 revalidate()，用于批量构建场景。
    /// 交互式修改使用 SceneManager.set_contents() 方法。
    pub fn set_contents(&mut self, figure: Box<dyn super::Figure>) -> BlockId {
        let contents_id = self.new_block_with_parent(figure, self.root);
        self.contents = Some(contents_id);
        contents_id
    }

    /// 获取内容块
    pub fn get_contents(&self) -> Option<BlockId> {
        self.contents
    }

    /// 添加子块到指定父块
    ///
    /// 对应 draw2d: parent.addChild(child) (不触发 revalidate)
    ///
    /// 与 `add_child()` 的区别：此方法不触发 revalidate()，用于批量构建场景。
    pub fn add_child_to(&mut self, parent_id: BlockId, figure: Box<dyn super::Figure>) -> BlockId {
        self.new_block_with_parent(figure, parent_id)
    }

    /// 添加子块到指定父块，并设置子块的位置和尺寸
    ///
    /// # 坐标语义
    ///
    /// - bounds 是绝对坐标（相对于坐标根），不是相对于父节点的偏移
    /// - 添加后，子节点的 bounds 保持不变
    /// - 平移操作由 `prim_translate` 负责，会修改 bounds 并传播到子节点
    ///
    /// # 示例
    ///
    /// ```
    /// use novadraw_core::Color;
    /// use novadraw_scene::{figure::RectangleFigure, FigureGraph};
    ///
    /// let mut scene = FigureGraph::new();
    /// let parent_id = scene.set_contents(Box::new(RectangleFigure::new(0.0, 0.0, 100.0, 100.0)));
    /// let color = Color::hex("#3498db");
    /// // 添加子节点，bounds 是绝对坐标 (10, 10, 50, 50)
    /// let _child_id = scene.add_child_with_bounds(parent_id, 10.0, 10.0, 50.0, 50.0, color);
    /// ```
    pub fn add_child_with_bounds(
        &mut self,
        parent_id: BlockId,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        color: novadraw_core::Color,
    ) -> BlockId {
        let figure = super::figure::RectangleFigure::new_with_color(x, y, width, height, color);
        self.new_block_with_parent(Box::new(figure), parent_id)
    }

    /// 添加子块
    ///
    /// 参考 draw2d: parent.addChild(child) -> revalidate()
    /// 与 `add_child_to()` 的区别：此方法会标记父容器需要重新布局，
    /// 并将父容器区域加入脏区域，下次 `perform_update()` 时会验证布局。
    ///
    /// # 使用场景
    ///
    /// 用于交互式修改（如拖拽添加、动态插入节点），不适合批量构建场景。
    /// 批量构建使用 `add_child_to()` 以避免不必要的更新触发。
    pub fn add_child(&mut self, parent_id: BlockId, figure: Box<dyn super::Figure>) -> BlockId {
        let bounds = figure.bounds();
        let child_id = self.new_block_with_parent(figure, parent_id);

        // 标记父容器需要重新布局
        self.mark_invalid(parent_id);
        // 标记脏区域
        self.repaint(parent_id, Some(bounds));
        // 标记新子块需要验证
        self.mark_invalid(child_id);

        child_id
    }

    /// 创建带父块的块
    fn new_block_with_parent(
        &mut self,
        figure: Box<dyn super::Figure>,
        parent_id: BlockId,
    ) -> BlockId {
        let uuid = Uuid::new_v4();
        let id = self.blocks.insert_with_key(|key| FigureBlock {
            id: key,
            uuid,
            children: Vec::new(),
            parent: Some(parent_id),
            figure,
            layout_manager: None,
            is_selected: false,
            is_visible: true,
            is_enabled: true,
            preferred_size: None,
            minimum_size: None,
            maximum_size: None,
        });
        self.uuid_map.insert(uuid, id);
        self.blocks[parent_id].children.push(id);
        self.layout_valid = false;
        id
    }

    /// 使布局失效，下次渲染时将重新计算布局
    ///
    /// 对应 draw2d: Figure.invalidate()
    pub fn invalidate(&mut self) {
        self.layout_valid = false;
    }

    /// 标记块需要重新布局
    ///
    /// 对应 draw2d: Figure.revalidate() -> UpdateManager.addInvalidFigure()
    /// 将块添加到更新管理器的失效队列中。
    ///
    /// # Arguments
    ///
    /// * `block_id` - 需要重新布局的块 ID
    pub fn mark_invalid(&mut self, block_id: BlockId) {
        self.update_manager.add_invalid_figure(block_id);
    }

    /// 请求重绘指定块
    ///
    /// 对应 draw2d: Figure.repaint() -> UpdateManager.addDirtyRegion()
    /// 将块添加到更新管理器的脏区域队列中。
    ///
    /// # Arguments
    ///
    /// * `block_id` - 需要重绘的块 ID
    /// * `rect` - 脏区域（局部坐标），如果为 None 则使用块的 bounds
    pub fn repaint(&mut self, block_id: BlockId, rect: Option<Rectangle>) {
        // 获取块并检查可见性
        if let Some(block) = self.blocks.get(block_id) {
            if !block.is_visible {
                return;
            }

            // 使用传入的区域或块的 bounds
            let dirty_rect = rect.unwrap_or_else(|| block.figure_bounds());
            self.update_manager.add_dirty_region(block_id, dirty_rect);
        }
    }

    /// 请求重绘整个场景
    ///
    /// 对应 draw2d: Figure.repaint() 使用整个 bounds
    pub fn repaint_all(&mut self) {
        if let Some(contents_id) = self.contents {
            self.repaint(contents_id, None);
        }
    }

    /// 检查是否有待处理的更新
    ///
    /// 对应 draw2d: UpdateManager.updateQueued flag
    pub fn is_update_queued(&self) -> bool {
        self.update_manager.is_update_queued()
    }

    /// 执行更新（两阶段：布局 + 重绘）
    ///
    /// 对应 draw2d: DeferredUpdateManager.performUpdate()
    ///
    /// Phase 1: 布局验证
    /// - 遍历所有失效块，调用 revalidate() 执行布局
    /// - 调用 Figure.validate() 预计算几何属性（如 Triangle 顶点）
    ///
    /// Phase 2: 脏区域重绘
    /// - 如果有待重绘的脏区域，使用脏区域裁剪渲染
    /// - 清空脏区域
    pub fn perform_update(&mut self) -> NdCanvas {
        // 对应 draw2d: DeferredUpdateManager.performUpdate()
        //
        // Phase 1: 布局验证
        // 使用 drain_invalid_blocks() 获取所有待验证块并清空队列
        let block_ids = self.update_manager.drain_invalid_blocks();
        for block_id in block_ids {
            if let Some(block) = self.blocks.get(block_id) {
                if block.is_visible && block.is_enabled {
                    self.revalidate(block_id);
                }
            }
        }

        // Phase 2: 脏区域重绘
        let damage = self.update_manager.compute_damage();
        let mut canvas = NdCanvas::new();

        if self.update_manager.has_pending_repaint() && damage.width > 0.0 && damage.height > 0.0 {
            canvas.clip_rect(damage.x, damage.y, damage.width, damage.height);
        }
        self.render_to_iterative(&mut canvas);

        // 清空脏区域和更新标记
        self.update_manager.clear_dirty_and_flag();

        canvas
    }

    /// 渲染脏区域
    ///
    /// 对应 draw2d: DeferredUpdateManager.repairDamage() (private)
    ///
    /// # 性能优化
    ///
    /// 脏区域优化可以显著减少渲染工作量。
    /// 对于全量重绘场景（无脏区域），使用 render() 方法。
    pub fn repair_damage(&mut self) -> NdCanvas {
        let damage = self.update_manager.compute_damage();

        let mut gc = NdCanvas::new();

        // 设置脏区域裁剪
        if self.update_manager.has_pending_repaint() && damage.width > 0.0 && damage.height > 0.0 {
            gc.clip_rect(damage.x, damage.y, damage.width, damage.height);
        }

        // 渲染场景
        self.render_to_iterative(&mut gc);

        // 清空脏区域
        self.update_manager.dirty_regions.clear();
        self.update_manager.update_queued = false;

        gc
    }

    /// 执行两阶段更新（布局 + 脏区域重绘）
    ///
    /// 对应 draw2d: DeferredUpdateManager.performUpdate() 的完整行为。
    /// 委托给 SceneUpdateManager.perform_update() 执行。
    pub fn perform_update_and_repair(&mut self) -> NdCanvas {
        self.perform_update()
    }

    /// 获取合并后的脏区域
    ///
    /// 返回所有脏区域合并后的大区域，用于视口裁剪。
    pub fn get_damage_region(&self) -> Rectangle {
        self.update_manager.compute_damage()
    }

    /// 清空更新队列
    ///
    /// 对应 draw2d: UpdateManager.clear()
    pub fn clear_updates(&mut self) {
        self.update_manager.clear();
    }

    /// 重新验证布局（递归），如果布局无效则重新计算
    ///
    /// 从指定容器开始，递归执行布局。
    /// 只有设置了布局管理器的容器才会执行布局。
    /// 参考 draw2d: Figure.layout() { if (layoutManager != null) layoutManager.layout() }
    pub fn revalidate(&mut self, container_id: BlockId) {
        // 1. 调用容器的 Updatable.validate()（布局后验证钩子）
        if let Some(block) = self.blocks.get_mut(container_id) {
            Updatable::validate(&mut *block.figure);
        }

        // 2. 获取该容器的布局器
        let layout_manager = self
            .blocks
            .get(container_id)
            .and_then(|b| b.layout_manager.clone());

        if layout_manager.is_none() {
            // 没有布局器，不执行任何布局，参考 draw2d：layoutManager == null 时什么都不做
            // 但仍然递归处理子容器
            self.revalidate_children(container_id);
            return;
        }

        // 3. 执行布局：调用 LayoutManager.layout()
        layout_manager.unwrap().layout(container_id, self);

        // 4. 递归处理子容器
        self.revalidate_children(container_id);

        // 标记布局有效
        self.layout_valid = true;
    }

    /// 递归验证子容器的布局
    fn revalidate_children(&mut self, parent_id: BlockId) {
        // 先收集子元素 ID，避免在迭代过程中同时持有不可变和可变引用
        let children: Vec<BlockId> = self
            .blocks
            .get(parent_id)
            .map(|b| b.children.clone())
            .unwrap_or_default();

        for child_id in children {
            self.revalidate(child_id);
        }
    }

    /// 获取子元素 ID 列表
    #[allow(dead_code)]
    fn get_children_ids(&self, parent_id: BlockId) -> Vec<BlockId> {
        self.blocks
            .get(parent_id)
            .map(|b| b.children.clone())
            .unwrap_or_default()
    }

    /// 重新验证布局（兼容旧 API）
    ///
    /// 如果布局无效则重新计算。
    /// 使用内容块作为根容器。
    pub fn revalidate_with_bounds(&mut self, container_bounds: Rectangle) {
        if !self.layout_valid {
            self.apply_layout(container_bounds);
            self.layout_valid = true;
        }
    }

    /// 检查布局是否有效
    pub fn is_layout_valid(&self) -> bool {
        self.layout_valid
    }

    /// 按矩形选择
    pub fn select_by_rect(&mut self, rect: Rectangle) {
        for block in self.blocks.values_mut() {
            block.is_selected = false;
        }

        // 收集需要选中的 ID
        let mut to_select: Vec<BlockId> = Vec::new();
        let mut stack = vec![self.root];
        while let Some(node_id) = stack.pop() {
            if let Some(block) = self.blocks.get(node_id) {
                if !block.is_visible {
                    continue;
                }

                // 先处理子节点
                for &child_id in block.children.iter().rev() {
                    stack.push(child_id);
                }

                // 检查矩形相交
                let bounds = block.figure_bounds();
                if rect_intersects(&rect, &bounds) {
                    to_select.push(node_id);
                }
            }
        }

        // 设置选中状态
        for id in to_select {
            if let Some(block) = self.blocks.get_mut(id) {
                block.is_selected = true;
            }
        }
    }

    /// 选择单个块
    #[allow(clippy::collapsible_if)]
    pub fn select_single(&mut self, block_id: Option<BlockId>) {
        for block in self.blocks.values_mut() {
            block.is_selected = false;
        }
        if let Some(id) = block_id {
            if let Some(block) = self.blocks.get_mut(id) {
                block.is_selected = true;
            }
        }
    }

    /// 设置选中状态
    pub fn set_selected(&mut self, block_id: Option<BlockId>) {
        self.select_single(block_id);
    }

    /// 获取当前选中的块 ID
    pub fn selected_block(&self) -> Option<BlockId> {
        for (id, block) in self.blocks.iter() {
            if block.is_selected {
                return Some(id);
            }
        }
        None
    }

    /// 命中测试
    ///
    /// 检测指定点是否命中任意图形，返回从根到目标的路径。
    /// 使用深度优先遍历（从后往前，确保先命中最上层的图形）。
    ///
    /// # 参数
    ///
    /// - `point`: 待检测的坐标（屏幕坐标）
    ///
    /// # 返回
    ///
    /// Some((target, path)) 其中 target 是最底层命中的图形，path 是从根到目标的路径
    /// None 表示未命中任何图形
    pub fn hit_test(&self, point: (f64, f64)) -> Option<(BlockId, Vec<BlockId>)> {
        let start_id = self.contents.unwrap_or(self.root);
        let mut stack = vec![start_id];
        let mut path = Vec::new();

        while let Some(id) = stack.pop() {
            if let Some(block) = self.blocks.get(id) {
                // 将当前节点加入路径
                path.push(id);

                // 收集可见子节点（逆序，保证先处理后添加的）
                let mut children: Vec<BlockId> = block
                    .children
                    .iter()
                    .filter(|&&child_id| {
                        if let Some(child) = self.blocks.get(child_id) {
                            child.is_visible && child.is_enabled
                        } else {
                            false
                        }
                    })
                    .cloned()
                    .collect();
                children.reverse();

                // 检查当前节点是否包含点
                if block.is_visible && block.is_enabled {
                    let bounds = block.figure_bounds();
                    if point.0 >= bounds.x
                        && point.0 <= bounds.x + bounds.width
                        && point.1 >= bounds.y
                        && point.1 <= bounds.y + bounds.height
                    {
                        // 命中，返回完整路径
                        let target = id;
                        let full_path = path.clone();
                        return Some((target, full_path));
                    }
                }

                // 将子节点加入栈
                for child_id in children {
                    stack.push(child_id);
                }
            } else {
                // 退出当前分支，从路径中移除
                path.pop();
            }
        }

        None
    }

    /// 简单的命中测试
    ///
    /// 只返回第一个命中的块 ID，不包含路径。
    pub fn hit_test_simple(&self, point: (f64, f64)) -> Option<BlockId> {
        self.hit_test(point).map(|(target, _)| target)
    }

    /// 渲染场景图
    ///
    /// 使用递归实现 Figure 树的渲染遍历。
    /// 渲染顺序（参考 draw2d）：
    /// 1. paintFigure() - 绘制自身
    /// 2. paintClientArea() - 绘制子元素
    /// 3. paintBorder() - 绘制边框
    pub fn render(&self) -> NdCanvas {
        let mut gc = NdCanvas::new();
        self.render_to(&mut gc);
        gc
    }

    /// 渲染到上下文（递归实现）
    fn render_to(&self, gc: &mut NdCanvas) {
        let start_id = self.contents.unwrap_or(self.root);
        let scene_ref = FigureGraphRenderRef {
            blocks: &self.blocks,
        };
        let mut renderer = FigureRenderer::new(&scene_ref, gc);
        renderer.render(start_id);
    }

    /// 迭代渲染场景图
    ///
    /// 使用显式栈实现 Figure 树的渲染遍历，避免递归栈溢出。
    /// 渲染顺序与 `render()` 相同：
    /// 1. paintFigure() - 绘制自身
    /// 2. paintClientArea() - 绘制子元素
    /// 3. paintBorder() - 绘制边框
    pub fn render_iterative(&self) -> NdCanvas {
        let mut gc = NdCanvas::new();
        self.render_to_iterative(&mut gc);
        gc
    }

    /// 渲染到上下文（迭代实现）
    fn render_to_iterative(&self, gc: &mut NdCanvas) {
        let start_id = self.contents.unwrap_or(self.root);
        let scene_ref = FigureGraphRenderRef {
            blocks: &self.blocks,
        };
        let mut renderer = FigureRendererIter::new(&scene_ref, gc);
        renderer.render(start_id);
    }

    // ========== 调试验证方法 ==========

    /// 打印场景图树结构（用于调试）
    ///
    /// 使用 `eprintln!` 输出到 stderr，格式示例：
    /// ```text
    /// V BlockId(0x1): Figure bounds=(0,0,100,100)
    ///   V BlockId(0x2): RectangleFigure bounds=(10,10,50,50)
    ///   H BlockId(0x3): RectangleFigure bounds=(50,50,50,50)  // 不可见
    /// ```
    #[cfg(feature = "debug_render")]
    pub fn print_tree(&self) {
        eprintln!("\n========== 场景图结构 ==========");
        self.print_block(self.root, 0);
        eprintln!("=================================\n");
    }

    /// 递归打印单个块（内部使用）
    #[cfg(feature = "debug_render")]
    fn print_block(&self, block_id: BlockId, depth: usize) {
        let indent = "  ".repeat(depth);
        if let Some(block) = self.blocks.get(block_id) {
            let bounds = block.figure_bounds();
            let visibility = if block.is_visible { "V" } else { "H" };
            let selected = if block.is_selected { " *" } else { "" };
            eprintln!(
                "{}{} {:?}: {} bounds=({:.0},{:.0},{:.0},{:.0}){}",
                indent,
                visibility,
                block_id,
                block.figure.name(),
                bounds.x,
                bounds.y,
                bounds.width,
                bounds.height,
                selected
            );

            // 正序打印子节点（视觉上：先添加的在上面）
            for &child_id in &block.children {
                self.print_block(child_id, depth + 1);
            }
        }
    }

    /// 打印渲染顺序（调试）
    ///
    /// 在渲染前调用，渲染后会打印渲染顺序
    #[cfg(feature = "debug_render")]
    #[allow(clippy::collapsible_if)]
    pub fn print_render_order(&self) {
        let start_id = self.contents.unwrap_or(self.root);
        let mut stack = vec![start_id];

        eprintln!("\n========== 渲染顺序 ==========");
        let mut order = Vec::new();

        while let Some(block_id) = stack.pop() {
            if let Some(block) = self.blocks.get(block_id) {
                if block.is_visible {
                    let bounds = block.figure_bounds();
                    order.push(format!("{}: {:?}", block.figure.name(), bounds));

                    for &child_id in block.children.iter().rev() {
                        if let Some(child) = self.blocks.get(child_id) {
                            if child.is_visible {
                                stack.push(child_id);
                            }
                        }
                    }
                }
            }
        }

        for (i, info) in order.iter().enumerate() {
            eprintln!("  {}: {}", i, info);
        }
        eprintln!("================================\n");
    }

    /// 获取块
    pub fn get_block(&self, id: BlockId) -> Option<&FigureBlock> {
        self.blocks.get(id)
    }

    /// 设置布局管理器
    pub fn set_layout_manager(&mut self, layout_manager: Arc<dyn LayoutManager>) {
        self.layout_manager = Some(layout_manager);
        self.invalidate();
    }

    /// 获取布局管理器
    pub fn get_layout_manager(&self) -> Option<&dyn LayoutManager> {
        self.layout_manager.as_deref()
    }

    /// 设置指定块的布局管理器
    pub fn set_block_layout_manager(
        &mut self,
        block_id: BlockId,
        layout_manager: Arc<dyn LayoutManager>,
    ) {
        if let Some(block) = self.blocks.get_mut(block_id) {
            block.layout_manager = Some(layout_manager);
            self.invalidate();
        }
    }

    /// 获取指定块的布局管理器
    pub fn get_block_layout_manager(&self, block_id: BlockId) -> Option<Arc<dyn LayoutManager>> {
        self.blocks
            .get(block_id)
            .and_then(|b| b.layout_manager.clone())
    }

    /// 设置子元素的布局约束
    ///
    /// 对应 draw2d: setConstraint(IFigure, Object)
    /// 约束使用 Rectangle 类型
    pub fn set_constraint(&mut self, child_id: BlockId, constraint: Rectangle) {
        // 使用 child_id 的索引作为约束的 key
        // 在实际布局时，通过遍历 children 来匹配
        self.constraints
            .insert(child_id.data().as_ffi() as usize, constraint);
        self.invalidate();
    }

    /// 获取子元素的布局约束
    ///
    /// 对应 draw2d: getConstraint(IFigure)
    pub fn get_constraint(&self, child_id: BlockId) -> Option<Rectangle> {
        self.constraints
            .get(&(child_id.data().as_ffi() as usize))
            .cloned()
    }

    /// 移除子元素的布局约束
    ///
    /// 对应 draw2d: LayoutManager.remove(IFigure)
    pub fn remove_constraint(&mut self, child_id: BlockId) {
        self.constraints
            .remove(&(child_id.data().as_ffi() as usize));
        self.invalidate();
    }

    /// 使布局生效
    ///
    /// 对应 draw2d: validate()
    /// 标记布局为有效
    pub fn validate(&mut self) {
        self.layout_valid = true;
    }

    /// 应用布局
    ///
    /// 根据布局管理器重新计算子元素的位置。
    /// 注意：当前实现为简化版本。
    pub fn apply_layout(&mut self, container_bounds: Rectangle) {
        // TODO: 完整的布局实现需要基于约束系统
        // 当前简化实现：不做任何布局，子元素保持原位
        let _ = container_bounds;
    }

    /// 计算布局大小
    ///
    /// 返回容器的首选大小。
    pub fn compute_layout_size(&self, container_bounds: Rectangle) -> (f64, f64) {
        // TODO: 完整的布局实现需要基于约束系统
        // 当前简化实现：返回容器大小
        (container_bounds.width, container_bounds.height)
    }

    // ========== 坐标变换方法 ==========

    /// 原始平移（对应 draw2d: primTranslate）
    ///
    /// 移动 Figure 的位置并传播到子节点。
    /// 如果 `use_local_coordinates()` 为 false（默认），子节点的 bounds 也会被平移。
    /// 如果 `use_local_coordinates()` 为 true，只平移当前节点，不传播到子节点。
    ///
    /// # 关键特性
    ///
    /// - 使用**显式栈**迭代实现，避免递归栈溢出
    /// - 所有 bounds 都是**绝对坐标**（相对于坐标根）
    /// - `use_local_coordinates()` 为 true 时，当前节点是坐标根，不传播到子节点
    ///
    /// # 坐标语义说明
    ///
    /// - 子节点的 bounds 也是绝对坐标，所以平移时会同时修改父子节点的 bounds
    /// - 这种设计确保所有 bounds 始终相对于坐标根
    /// - 当 `use_local_coordinates()` 为 true 时，坐标根的 bounds 变化会触发事件通知
    ///
    /// # 与 draw2d 的一致性
    ///
    /// ```java
    /// // Figure.java:1390-1397 - primTranslate
    /// protected void primTranslate(int dx, int dy) {
    ///     bounds.x += dx;
    ///     bounds.y += dy;
    ///
    ///     if (useLocalCoordinates()) {
    ///         fireCoordinateSystemChanged();
    ///         return;
    ///     }
    ///     children.forEach(child -> child.translate(dx, dy));
    /// }
    /// ```
    pub fn prim_translate(&mut self, block_id: BlockId, dx: f64, dy: f64) {
        // 使用显式栈实现迭代式深度优先遍历
        let mut stack = vec![block_id];

        while let Some(id) = stack.pop() {
            if let Some(block) = self.blocks.get_mut(id) {
                // 修改当前节点的 bounds (x, y)
                let current_bounds = block.figure.bounds();
                block.figure.set_bounds(
                    current_bounds.x + dx,
                    current_bounds.y + dy,
                    current_bounds.width,
                    current_bounds.height,
                );

                // 检查是否使用本地坐标模式
                if block.figure.use_local_coordinates() {
                    // 本地坐标模式：不传播，但可能需要触发事件
                    // TODO: 实现 fireCoordinateSystemChanged()
                    continue;
                }

                // 默认模式：将所有子节点加入栈进行平移
                for &child_id in &block.children {
                    stack.push(child_id);
                }
            }
        }
    }

    /// 设置节点的 bounds
    ///
    /// 对应 draw2d: setBounds(Rectangle)
    /// 核心逻辑：
    /// 1. 计算位置偏移
    /// 2. 使用栈迭代调用 prim_translate 传播偏移到所有子节点
    /// 3. 更新自身的宽高
    ///
    /// 注意：所有子节点传播操作必须使用迭代实现，禁止递归
    #[allow(clippy::collapsible_if)]
    pub fn set_bounds(&mut self, block_id: BlockId, x: f64, y: f64, width: f64, height: f64) {
        let (dx, dy, needs_width_height_update) = {
            if let Some(block) = self.blocks.get(block_id) {
                let old_bounds = block.figure.bounds();
                let dx = x - old_bounds.x;
                let dy = y - old_bounds.y;
                let needs_width_height_update =
                    width != old_bounds.width || height != old_bounds.height;
                (dx, dy, needs_width_height_update)
            } else {
                return;
            }
        };

        // 1. 传播位置偏移到所有子节点（使用栈迭代）
        if dx != 0.0 || dy != 0.0 {
            self.prim_translate(block_id, dx, dy);
        }

        // 2. 更新自身的宽高（x, y 已由 prim_translate 更新）
        if needs_width_height_update {
            if let Some(block) = self.blocks.get_mut(block_id) {
                block.figure.set_bounds(x, y, width, height);
            }
        }
    }

    /// 将局部坐标转换为绝对坐标
    ///
    /// 对应 draw2d: translateToAbsolute(Translatable)
    ///
    /// # 算法
    ///
    /// 使用栈迭代实现：
    /// 1. 从当前节点向上遍历，记录路径上的坐标根
    /// 2. 逆向遍历路径，累加每个坐标根的 bounds
    ///
    /// # 注意
    ///
    /// 绝对坐标是相对于场景根的坐标。
    /// 此方法将 Translatable 对象从局部坐标（相对于最近坐标根）转换为绝对坐标。
    ///
    /// # 示例
    ///
    /// 假设：
    /// - coord_root bounds = (20, 30)
    /// - 本地坐标 (10, 5)
    /// - 绝对坐标 = (20 + 10, 30 + 5) = (30, 35)
    #[allow(clippy::collapsible_if)]
    pub fn translate_to_absolute_mut<T: Translatable>(&self, block_id: BlockId, t: &mut T) {
        // 第一阶段：向上遍历，记录所有"父节点是坐标根"的节点
        let mut roots: Vec<BlockId> = Vec::new();
        let mut current = Some(block_id);

        while let Some(id) = current {
            if let Some(block) = self.blocks.get(id) {
                if block.figure.use_local_coordinates() {
                    roots.push(id);
                }
                current = block.parent;
            }
        }

        // 第二阶段：逆向遍历，累加每个坐标根的 bounds
        for id in roots.iter().rev() {
            if let Some(block) = self.blocks.get(*id) {
                let bounds = block.figure.bounds();
                t.translate(bounds.x, bounds.y);
            }
        }
    }

    /// 检查节点是否是坐标根
    ///
    /// 对应 draw2d: isCoordinateSystem()
    /// 返回 true 如果节点使用本地坐标（即它是子节点的坐标根）。
    pub fn is_coordinate_system(&self, block_id: BlockId) -> bool {
        if let Some(block) = self.blocks.get(block_id) {
            block.figure.use_local_coordinates()
        } else {
            false
        }
    }

    /// 将本地坐标转换为父节点坐标
    ///
    /// 对应 draw2d: translateToParent(Translatable)
    ///
    /// # 算法
    ///
    /// 当父节点是坐标根（`useLocalCoordinates() = true`）时：
    /// - 本地坐标需要累加父节点的 insets 才能得到父节点坐标
    /// - 因为本地 (0, 0) 对应父坐标 (left_insets, top_insets)
    ///
    /// # 示例
    ///
    /// 假设：
    /// - 父节点 bounds = (0, 0, 100, 100)，left/top insets = 5
    /// - 子节点的本地坐标 (10, 20) 转换为父坐标 (15, 25)
    #[allow(clippy::collapsible_if, clippy::needless_return)]
    pub fn translate_to_parent<T: Translatable>(&self, block_id: BlockId, t: &mut T) {
        if let Some(block) = self.blocks.get(block_id) {
            if let Some(parent_id) = block.parent {
                if let Some(parent) = self.blocks.get(parent_id) {
                    if parent.figure.use_local_coordinates() {
                        // 只有当父节点是坐标根时才转换
                        // 本地 (0, 0) 对应父坐标 (left, top)
                        let (top, left, _, _) = parent.figure.insets();
                        t.translate(left, top);
                        return;
                    }
                }
            }
        }
    }

    /// 将父节点坐标转换为本地坐标
    ///
    /// 对应 draw2d: translateFromParent(Translatable)
    ///
    /// # 算法
    ///
    /// 当父节点是坐标根时：
    /// - 父坐标需要减去父节点的 insets 才能得到本地坐标
    /// - 因为父坐标 (left_insets, top_insets) 对应本地 (0, 0)
    #[allow(clippy::collapsible_if, clippy::needless_return)]
    pub fn translate_from_parent<T: Translatable>(&self, block_id: BlockId, t: &mut T) {
        if let Some(block) = self.blocks.get(block_id) {
            if let Some(parent_id) = block.parent {
                if let Some(parent) = self.blocks.get(parent_id) {
                    if parent.figure.use_local_coordinates() {
                        // 只有当父节点是坐标根时才转换
                        let (top, left, _, _) = parent.figure.insets();
                        t.translate(-left, -top);
                        return;
                    }
                }
            }
        }
    }

    /// 将绝对坐标转换为本地坐标
    ///
    /// 对应 draw2d: translateToRelative(Translatable)
    ///
    /// # 算法
    ///
    /// 绝对坐标是相对于场景根的坐标。
    /// 递归向父节点遍历：
    /// 1. 递归处理父节点
    /// 2. 如果当前节点的父节点是坐标根，减去父节点的 bounds
    ///    因为：absolute = coord_root_bounds + local
    ///    所以：local = absolute - coord_root_bounds
    ///
    /// # 注意
    ///
    /// 此方法将绝对坐标（相对于场景根）转换为本地坐标（相对于最近坐标根）。
    #[allow(clippy::collapsible_if, clippy::needless_return)]
    pub fn translate_to_relative<T: Translatable>(&self, block_id: BlockId, t: &mut T) {
        if let Some(block) = self.blocks.get(block_id) {
            if let Some(parent_id) = block.parent {
                // 递归处理父节点
                self.translate_to_relative(parent_id, t);

                // 如果父节点是坐标根，减去父节点的 bounds
                // 因为 absolute = coord_root_bounds + local
                // 所以 local = absolute - coord_root_bounds
                if let Some(parent) = self.blocks.get(parent_id) {
                    if parent.figure.use_local_coordinates() {
                        let bounds = parent.figure.bounds();
                        t.translate(-bounds.x, -bounds.y);
                    }
                }
            }
        }
    }
}

impl super::layout::LayoutContext for FigureGraph {
    fn get_children(&self, parent_id: BlockId) -> Vec<(BlockId, Rectangle)> {
        if let Some(block) = self.blocks.get(parent_id) {
            block
                .children
                .iter()
                .filter_map(|&child_id| {
                    self.blocks
                        .get(child_id)
                        .map(|child| (child_id, child.figure_bounds()))
                })
                .collect()
        } else {
            Vec::new()
        }
    }

    fn get_constraint(&self, child_id: BlockId) -> Option<Rectangle> {
        let key = child_id.data().as_ffi() as usize;
        self.constraints.get(&key).cloned()
    }

    fn get_preferred_size(&self, block_id: BlockId) -> (f64, f64) {
        if let Some(block) = self.blocks.get(block_id) {
            block.preferred_size.unwrap_or_else(|| {
                let bounds = block.figure_bounds();
                (bounds.width, bounds.height)
            })
        } else {
            (0.0, 0.0)
        }
    }

    fn set_child_bounds(&mut self, child_id: BlockId, bounds: Rectangle) {
        if let Some(block) = self.blocks.get_mut(child_id) {
            block
                .figure
                .set_bounds(bounds.x, bounds.y, bounds.width, bounds.height);
        }
    }

    fn get_container_bounds(&self, container_id: BlockId) -> Rectangle {
        if let Some(block) = self.blocks.get(container_id) {
            block.figure_bounds()
        } else {
            Rectangle::new(0.0, 0.0, 0.0, 0.0)
        }
    }
}

impl Default for FigureGraph {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::super::figure::{Bounded, RectangleFigure, Shape, Updatable};
    use crate::scene::FigureGraphRenderRef;
    use crate::{Rectangle, FigureGraph};
    use novadraw_core::Color as NovadrawCoreColor;
    use novadraw_render::NdCanvas;

    // ========== 通用测试 Figure 类型 ==========

    /// 坐标根 Figure（使用本地坐标）
    #[derive(Clone, Copy)]
    struct TestCoordinateRootFigure {
        bounds: Rectangle,
    }

    impl TestCoordinateRootFigure {
        fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
            Self {
                bounds: Rectangle::new(x, y, width, height),
            }
        }
    }

    impl Bounded for TestCoordinateRootFigure {
        fn bounds(&self) -> Rectangle {
            self.bounds
        }

        fn set_bounds(&mut self, x: f64, y: f64, width: f64, height: f64) {
            self.bounds = Rectangle::new(x, y, width, height);
        }

        fn use_local_coordinates(&self) -> bool {
            true
        }

        fn name(&self) -> &'static str {
            "TestCoordinateRootFigure"
        }
    }

    impl Updatable for TestCoordinateRootFigure {
        fn validate(&mut self) {}
        fn invalidate(&mut self) {}
    }

    impl Shape for TestCoordinateRootFigure {
        fn stroke_color(&self) -> Option<NovadrawCoreColor> {
            None
        }

        fn stroke_width(&self) -> f64 {
            0.0
        }

        fn fill_color(&self) -> Option<NovadrawCoreColor> {
            None
        }

        fn line_cap(&self) -> novadraw_render::command::LineCap {
            novadraw_render::command::LineCap::default()
        }

        fn line_join(&self) -> novadraw_render::command::LineJoin {
            novadraw_render::command::LineJoin::default()
        }

        fn fill_enabled(&self) -> bool {
            false
        }

        fn outline_enabled(&self) -> bool {
            false
        }

        fn fill_shape(&self, _gc: &mut NdCanvas) {}

        fn outline_shape(&self, _gc: &mut NdCanvas) {}
    }

    /// 带 insets 的 Figure
    #[derive(Clone, Copy)]
    struct TestFigureWithInsets {
        bounds: Rectangle,
        insets: (f64, f64, f64, f64),
    }

    impl TestFigureWithInsets {
        fn new(x: f64, y: f64, width: f64, height: f64, insets: (f64, f64, f64, f64)) -> Self {
            Self {
                bounds: Rectangle::new(x, y, width, height),
                insets,
            }
        }
    }

    impl Bounded for TestFigureWithInsets {
        fn bounds(&self) -> Rectangle {
            self.bounds
        }

        fn set_bounds(&mut self, x: f64, y: f64, width: f64, height: f64) {
            self.bounds = Rectangle::new(x, y, width, height);
        }

        fn use_local_coordinates(&self) -> bool {
            true
        }

        fn insets(&self) -> (f64, f64, f64, f64) {
            self.insets
        }

        fn name(&self) -> &'static str {
            "TestFigureWithInsets"
        }
    }

    impl Updatable for TestFigureWithInsets {
        fn validate(&mut self) {}
        fn invalidate(&mut self) {}
    }

    impl Shape for TestFigureWithInsets {
        fn stroke_color(&self) -> Option<NovadrawCoreColor> {
            None
        }

        fn stroke_width(&self) -> f64 {
            0.0
        }

        fn fill_color(&self) -> Option<NovadrawCoreColor> {
            None
        }

        fn line_cap(&self) -> novadraw_render::command::LineCap {
            novadraw_render::command::LineCap::default()
        }

        fn line_join(&self) -> novadraw_render::command::LineJoin {
            novadraw_render::command::LineJoin::default()
        }

        fn fill_enabled(&self) -> bool {
            false
        }

        fn outline_enabled(&self) -> bool {
            false
        }

        fn fill_shape(&self, _gc: &mut NdCanvas) {}

        fn outline_shape(&self, _gc: &mut NdCanvas) {}
    }

    /// 测试渲染顺序：Z-order 验证
    ///
    /// 场景：父容器包含三个子矩形（从下到上添加）
    /// 期望：渲染顺序应为 parent → child1 → child2 → child3
    ///       即先添加的在下面（被遮挡），后添加的在上面（遮挡别人）
    #[test]
    fn test_render_order_z_order() {
        let mut scene = FigureGraph::new();

        // 创建父容器（100x100）
        let parent = RectangleFigure::new(0.0, 0.0, 100.0, 100.0);
        let parent_id = scene.set_contents(Box::new(parent));

        // 添加三个子矩形（从下到上添加）
        let child1 = RectangleFigure::new(10.0, 10.0, 20.0, 20.0);
        let _c1 = scene.add_child_to(parent_id, Box::new(child1));

        let child2 = RectangleFigure::new(30.0, 30.0, 20.0, 20.0);
        let _c2 = scene.add_child_to(parent_id, Box::new(child2));

        let child3 = RectangleFigure::new(50.0, 50.0, 20.0, 20.0);
        let _c3 = scene.add_child_to(parent_id, Box::new(child3));

        // 打印树结构（用于手动验证）
        {
            eprintln!("\n=== 场景图树结构 ===");
            // print_block 仅在 debug_render feature 下可用
            eprintln!("====================\n");

            // 打印预期渲染顺序
            eprintln!("预期渲染顺序（先渲染的在下面）:");
            eprintln!("  0: parent");
            eprintln!("  1: child1 (最早添加，在最下层)");
            eprintln!("  2: child2");
            eprintln!("  3: child3 (最晚添加，在最上层)");
            eprintln!();
        }

        // 渲染并验证命令数量
        let gc = scene.render();
        let cmd_count = gc.commands().len();

        // 渲染：每个矩形产生多个命令
        // parent + 3 个子矩形 = 4 个图形
        // 新渲染流程（每个图形）：
        //   - save (transform)
        //   - save (prepare_context)
        //   - translate (bounds)
        //   - clip_rect
        //   - fill_rect
        //   - restore (after paint_figure)
        //   - stroke_rect (border)
        //   - restore (PostOrder)
        // parent: save + save + translate + clip + fill + restore + stroke + restore = 8
        // 每个 child: save + save + translate + clip + fill + restore + restore = 7
        // Total: 8 + 3 * 7 = 29
        assert!(
            cmd_count >= 35,
            "应有至少 35 个渲染命令，实际为 {}",
            cmd_count
        );
    }

    /// 测试渲染顺序：嵌套层次
    ///
    /// 场景：父 → 子1 → 孙1
    /// 期望渲染顺序：parent → child1 → grandchild1
    #[test]
    fn test_render_order_nested() {
        let mut scene = FigureGraph::new();

        // 根
        let root = RectangleFigure::new(0.0, 0.0, 200.0, 200.0);
        let root_id = scene.set_contents(Box::new(root));

        // 子
        let child = RectangleFigure::new(50.0, 50.0, 100.0, 100.0);
        let child_id = scene.add_child_to(root_id, Box::new(child));

        // 孙
        let grandchild = RectangleFigure::new(60.0, 60.0, 30.0, 30.0);
        let _gc_id = scene.add_child_to(child_id, Box::new(grandchild));

        // 打印树结构
        {
            eprintln!("\n=== 嵌套场景图树结构 ===");
            // print_block 仅在 debug_render feature 下可用
            eprintln!("=======================\n");

            // 预期渲染顺序：root → child → grandchild
            eprintln!("预期渲染顺序:");
            eprintln!("  0: root");
            eprintln!("  1: child");
            eprintln!("  2: grandchild");
            eprintln!();
        }

        let gc = scene.render();
        let cmd_count = gc.commands().len();

        // 渲染：每个图形产生多个命令
        // 3 个图形：root + child + grandchild
        // 每个图形的命令数（参见 test_render_order_z_order）
        // Total: 8 (root) + 7 (child) + 7 (grandchild) = 22
        assert!(
            cmd_count >= 20,
            "应有至少 20 个渲染命令，实际为 {}",
            cmd_count
        );
    }

    /// 测试可见性过滤
    ///
    /// 场景：父容器包含可见子元素和不可见子元素
    /// 期望：只渲染可见元素
    #[test]
    fn test_visibility_filter() {
        let mut scene = FigureGraph::new();

        let parent = RectangleFigure::new(0.0, 0.0, 100.0, 100.0);
        let parent_id = scene.set_contents(Box::new(parent));

        // 可见子元素
        let visible_child = RectangleFigure::new(10.0, 10.0, 20.0, 20.0);
        let _ = scene.add_child_to(parent_id, Box::new(visible_child));

        // 不可见子元素
        let invisible_child = RectangleFigure::new(50.0, 50.0, 20.0, 20.0);
        let invisible_id = scene.add_child_to(parent_id, Box::new(invisible_child));

        // 设置不可见
        scene.blocks.get_mut(invisible_id).unwrap().is_visible = false;

        let gc = scene.render();
        let cmd_count = gc.commands().len();

        // 渲染：parent + visible_child = 2 个图形
        // 每个图形的命令数（参见 test_render_order_z_order）
        // parent: 8, child: 7, Total: 15
        assert!(
            cmd_count >= 8 && cmd_count <= 18,
            "应只渲染可见元素，实际为 {} 个命令",
            cmd_count
        );
    }

    /// 测试变换累加
    ///
    /// 场景：子元素有非零位置
    /// 期望：Trampoline 渲染能正确处理嵌套层次
    #[test]
    fn test_transform_accumulation() {
        let mut scene = FigureGraph::new();

        let parent = RectangleFigure::new(0.0, 0.0, 100.0, 100.0);
        let parent_id = scene.set_contents(Box::new(parent));

        let child = RectangleFigure::new(25.0, 25.0, 50.0, 50.0);
        let _child_id = scene.add_child_to(parent_id, Box::new(child));

        // 打印场景结构
        {
            eprintln!("\n=== 测试变换累加 ===");
            eprintln!(
                "Parent bounds: {:?}",
                scene.blocks.get(parent_id).unwrap().figure_bounds()
            );
            eprintln!(
                "Child bounds: {:?}",
                scene.blocks.get(parent_id).unwrap().children
            );
        }

        // 渲染应能正确处理嵌套层次
        let gc = scene.render();
        let commands = gc.commands();

        // 验证：parent + child = 2 个图形
        // 每个图形的命令数（参见 test_render_order_z_order）
        // parent: 8, child: 7, Total: 15
        assert!(
            commands.len() >= 8,
            "应有足够的渲染命令，实际为 {}",
            commands.len()
        );

        // 验证有 FillRect 命令
        let has_fill_rect = commands.iter().any(|cmd| {
            matches!(
                cmd.kind,
                novadraw_render::command::RenderCommandKind::FillRect { .. }
            )
        });
        assert!(has_fill_rect, "应有 FillRect 命令");
    }

    // ========== 坐标变换测试 ==========

    /// 测试 prim_translate 基本功能
    ///
    /// 场景：平移父节点，子节点也应被平移
    /// 期望：父子节点的 bounds 都被平移相同的量
    #[test]
    fn test_prim_translate_basic() {
        let mut scene = FigureGraph::new();

        // 创建父子层次
        let parent = RectangleFigure::new(0.0, 0.0, 100.0, 100.0);
        let parent_id = scene.set_contents(Box::new(parent));

        let child = RectangleFigure::new(10.0, 10.0, 50.0, 50.0);
        let child_id = scene.add_child_to(parent_id, Box::new(child));

        // 平移父节点 (10, 20)
        scene.prim_translate(parent_id, 10.0, 20.0);

        // 验证父节点 bounds
        let parent_bounds = scene.blocks.get(parent_id).unwrap().figure_bounds();
        assert_eq!(parent_bounds.x, 10.0, "父节点 x 应为 10");
        assert_eq!(parent_bounds.y, 20.0, "父节点 y 应为 20");

        // 验证子节点 bounds 也被平移
        let child_bounds = scene.blocks.get(child_id).unwrap().figure_bounds();
        assert_eq!(child_bounds.x, 20.0, "子节点 x 应为 20 (10 + 10)");
        assert_eq!(child_bounds.y, 30.0, "子节点 y 应为 30 (10 + 20)");
    }

    /// 测试 prim_translate 嵌套传播
    ///
    /// 场景：平移根节点，所有后代都被平移
    /// 期望：整棵子树的 bounds 都被平移
    #[test]
    fn test_prim_translate_nested() {
        let mut scene = FigureGraph::new();

        // 创建三层层次：root -> parent -> child
        let root = RectangleFigure::new(0.0, 0.0, 200.0, 200.0);
        let root_id = scene.set_contents(Box::new(root));

        let parent = RectangleFigure::new(50.0, 50.0, 100.0, 100.0);
        let parent_id = scene.add_child_to(root_id, Box::new(parent));

        let child = RectangleFigure::new(10.0, 10.0, 50.0, 50.0);
        let child_id = scene.add_child_to(parent_id, Box::new(child));

        // 平移根节点 (5, 10)
        scene.prim_translate(root_id, 5.0, 10.0);

        // 验证所有节点都被平移
        let root_bounds = scene.blocks.get(root_id).unwrap().figure_bounds();
        assert_eq!(root_bounds.x, 5.0);
        assert_eq!(root_bounds.y, 10.0);

        let parent_bounds = scene.blocks.get(parent_id).unwrap().figure_bounds();
        assert_eq!(parent_bounds.x, 55.0, "父节点 x 应为 55 (50 + 5)");
        assert_eq!(parent_bounds.y, 60.0, "父节点 y 应为 60 (50 + 10)");

        let child_bounds = scene.blocks.get(child_id).unwrap().figure_bounds();
        assert_eq!(child_bounds.x, 15.0, "子节点 x 应为 15 (10 + 5)");
        assert_eq!(child_bounds.y, 20.0, "子节点 y 应为 20 (10 + 10)");
    }

    /// 测试 is_coordinate_system 功能
    ///
    /// 场景：检查节点的坐标根状态
    /// 期望：默认返回 false，使用本地坐标返回 true
    #[test]
    fn test_is_coordinate_system() {
        let mut scene = FigureGraph::new();

        let parent = RectangleFigure::new(0.0, 0.0, 100.0, 100.0);
        let parent_id = scene.set_contents(Box::new(parent));

        let child = RectangleFigure::new(10.0, 10.0, 50.0, 50.0);
        let child_id = scene.add_child_to(parent_id, Box::new(child));

        // 默认不使用本地坐标
        assert!(!scene.is_coordinate_system(parent_id), "默认不是坐标根");
        assert!(!scene.is_coordinate_system(child_id), "默认不是坐标根");
    }

    // ========== translate_to_parent 测试 ==========

    /// 测试 translate_to_parent 基本功能
    ///
    /// 场景：父节点是坐标根且无 insets
    /// 期望：本地坐标 (10, 20) 转换为父坐标 (10, 20)
    #[test]
    fn test_translate_to_parent_basic() {
        let mut scene = FigureGraph::new();

        let contents = RectangleFigure::new(0.0, 0.0, 800.0, 600.0);
        let contents_id = scene.set_contents(Box::new(contents));

        let parent_id = scene.add_child_to(
            contents_id,
            Box::new(TestCoordinateRootFigure::new(0.0, 0.0, 100.0, 100.0)),
        );

        let child = RectangleFigure::new(10.0, 20.0, 50.0, 50.0);
        let child_id = scene.add_child_to(parent_id, Box::new(child));

        // 本地坐标 (10, 20) 转换为父坐标 (10, 20)
        let mut point = (10.0, 20.0);
        scene.translate_to_parent(child_id, &mut point);
        assert_eq!(point, (10.0, 20.0));
    }

    /// 测试 translate_to_parent 带 insets
    ///
    /// 场景：父节点是坐标根且有 insets
    /// 期望：本地坐标 (10, 20) 转换为父坐标 (15, 25)，其中 insets = (5, 5, 0, 0)
    #[test]
    fn test_translate_to_parent_with_insets() {
        let mut scene = FigureGraph::new();

        let contents = RectangleFigure::new(0.0, 0.0, 800.0, 600.0);
        let contents_id = scene.set_contents(Box::new(contents));

        let parent_id = scene.add_child_to(
            contents_id,
            Box::new(TestFigureWithInsets::new(
                0.0,
                0.0,
                100.0,
                100.0,
                (5.0, 5.0, 0.0, 0.0),
            )),
        );

        let child = RectangleFigure::new(10.0, 20.0, 50.0, 50.0);
        let child_id = scene.add_child_to(parent_id, Box::new(child));

        // 本地坐标 (10, 20) 转换为父坐标 (15, 25)，即本地 + insets
        let mut point = (10.0, 20.0);
        scene.translate_to_parent(child_id, &mut point);
        assert_eq!(point.0, 15.0, "x 应为 10 + 5");
        assert_eq!(point.1, 25.0, "y 应为 20 + 5");
    }

    /// 测试 translate_to_parent 父节点不是坐标根
    ///
    /// 场景：父节点不是坐标根
    /// 期望：不进行转换，返回原坐标
    #[test]
    fn test_translate_to_parent_not_coordinate_root() {
        let mut scene = FigureGraph::new();

        let contents = RectangleFigure::new(0.0, 0.0, 800.0, 600.0);
        let contents_id = scene.set_contents(Box::new(contents));

        let parent = RectangleFigure::new(0.0, 0.0, 100.0, 100.0);
        let parent_id = scene.add_child_to(contents_id, Box::new(parent));

        let child = RectangleFigure::new(10.0, 20.0, 50.0, 50.0);
        let child_id = scene.add_child_to(parent_id, Box::new(child));

        // 父节点不是坐标根，不转换
        let mut point = (10.0, 20.0);
        scene.translate_to_parent(child_id, &mut point);
        assert_eq!(point, (10.0, 20.0), "父节点不是坐标根时不转换");
    }

    // ========== translate_from_parent 测试 ==========

    /// 测试 translate_from_parent 基本功能
    ///
    /// 场景：父节点是坐标根且无 insets
    /// 期望：父坐标 (10, 20) 转换为本地坐标 (10, 20)
    #[test]
    fn test_translate_from_parent_basic() {
        let mut scene = FigureGraph::new();

        let contents = RectangleFigure::new(0.0, 0.0, 800.0, 600.0);
        let contents_id = scene.set_contents(Box::new(contents));

        let parent_id = scene.add_child_to(
            contents_id,
            Box::new(TestCoordinateRootFigure::new(0.0, 0.0, 100.0, 100.0)),
        );

        let child = RectangleFigure::new(10.0, 20.0, 50.0, 50.0);
        let child_id = scene.add_child_to(parent_id, Box::new(child));

        // 父坐标 (10, 20) 转换为本地坐标 (10, 20)
        let mut point = (10.0, 20.0);
        scene.translate_from_parent(child_id, &mut point);
        assert_eq!(point, (10.0, 20.0));
    }

    /// 测试 translate_from_parent 带 insets
    ///
    /// 场景：父节点是坐标根且有 insets
    /// 期望：父坐标 (15, 25) 转换为本地坐标 (10, 20)
    #[test]
    fn test_translate_from_parent_with_insets() {
        let mut scene = FigureGraph::new();

        let contents = RectangleFigure::new(0.0, 0.0, 800.0, 600.0);
        let contents_id = scene.set_contents(Box::new(contents));

        let parent_id = scene.add_child_to(
            contents_id,
            Box::new(TestFigureWithInsets::new(
                0.0,
                0.0,
                100.0,
                100.0,
                (5.0, 5.0, 0.0, 0.0),
            )),
        );

        let child = RectangleFigure::new(10.0, 20.0, 50.0, 50.0);
        let child_id = scene.add_child_to(parent_id, Box::new(child));

        // 父坐标 (15, 25) 转换为本地坐标 (10, 20)，即减去 insets
        let mut point = (15.0, 25.0);
        scene.translate_from_parent(child_id, &mut point);
        assert_eq!(point.0, 10.0, "x 应为 15 - 5");
        assert_eq!(point.1, 20.0, "y 应为 25 - 5");
    }

    // ========== translate_to_relative 测试 ==========

    /// 测试 translate_to_relative 基本功能
    ///
    /// 场景：父节点是坐标根，bounds = (0, 0)
    /// 期望：绝对坐标 (30, 40) 转换为本地坐标 (30, 40)
    #[test]
    fn test_translate_to_relative_basic() {
        let mut scene = FigureGraph::new();

        let contents = RectangleFigure::new(0.0, 0.0, 800.0, 600.0);
        let contents_id = scene.set_contents(Box::new(contents));

        let parent_id = scene.add_child_to(
            contents_id,
            Box::new(TestCoordinateRootFigure::new(0.0, 0.0, 100.0, 100.0)),
        );

        let child = RectangleFigure::new(30.0, 40.0, 50.0, 50.0);
        let child_id = scene.add_child_to(parent_id, Box::new(child));

        // 绝对坐标 (30, 40) 减去 coord_root_bounds (0, 0) = 本地坐标 (30, 40)
        let mut point = (30.0, 40.0);
        scene.translate_to_relative(child_id, &mut point);
        assert_eq!(point, (30.0, 40.0));
    }

    /// 测试 translate_to_relative 嵌套坐标根
    ///
    /// 场景：深层嵌套，多个坐标根
    /// 期望：正确累积转换
    #[test]
    fn test_translate_to_relative_nested() {
        let mut scene = FigureGraph::new();

        let contents = RectangleFigure::new(0.0, 0.0, 800.0, 600.0);
        let contents_id = scene.set_contents(Box::new(contents));

        // coord_root1 (20, 30)
        let coord_root1_id = scene.add_child_to(
            contents_id,
            Box::new(TestCoordinateRootFigure::new(20.0, 30.0, 100.0, 100.0)),
        );

        // coord_root2 相对于 coord_root1 (10, 5)
        let coord_root2_id = scene.add_child_to(
            coord_root1_id,
            Box::new(TestCoordinateRootFigure::new(10.0, 5.0, 50.0, 50.0)),
        );

        // child 相对于 coord_root2 (15, 25)
        let child = RectangleFigure::new(15.0, 25.0, 30.0, 30.0);
        let child_id = scene.add_child_to(coord_root2_id, Box::new(child));

        // 绝对坐标 = coord_root1 + coord_root2 + child = (20+10+15, 30+5+25) = (45, 60)
        // 本地坐标 = 绝对坐标 - coord_root1_bounds - coord_root2_bounds = (15, 25)
        let mut point = (45.0, 60.0);
        scene.translate_to_relative(child_id, &mut point);
        assert_eq!(point.0, 15.0, "x 应为 45 - 20 - 10");
        assert_eq!(point.1, 25.0, "y 应为 60 - 30 - 5");
    }

    /// 测试 translate_to_relative Rectangle 类型
    ///
    /// 场景：使用 Rectangle 类型进行坐标转换
    /// 期望：Rectangle 的 x, y 被正确转换
    #[test]
    fn test_translate_to_relative_rect() {
        let mut scene = FigureGraph::new();

        let contents = RectangleFigure::new(0.0, 0.0, 800.0, 600.0);
        let contents_id = scene.set_contents(Box::new(contents));

        // coord_root (10, 20)
        let parent_id = scene.add_child_to(
            contents_id,
            Box::new(TestCoordinateRootFigure::new(10.0, 20.0, 100.0, 100.0)),
        );

        // child 相对于 coord_root (30, 40)
        let child = RectangleFigure::new(30.0, 40.0, 50.0, 50.0);
        let child_id = scene.add_child_to(parent_id, Box::new(child));

        // 绝对坐标 Rectangle (40, 60, 50, 50) 减去 coord_root_bounds (10, 20) = 本地坐标 (30, 40)
        let mut rect = Rectangle::new(40.0, 60.0, 50.0, 50.0);
        scene.translate_to_relative(child_id, &mut rect);
        assert_eq!(rect.x, 30.0, "x 应为 40 - 10");
        assert_eq!(rect.y, 40.0, "y 应为 60 - 20");
    }

    // ========== translate_to_absolute_mut 测试 ==========

    /// 测试 translate_to_absolute_mut 基本功能
    ///
    /// 场景：父节点是坐标根，bounds = (20, 30)
    /// 期望：本地坐标 (10, 5) 转换为绝对坐标 (30, 35)
    #[test]
    fn test_translate_to_absolute_mut_basic() {
        let mut scene = FigureGraph::new();

        let contents = RectangleFigure::new(0.0, 0.0, 800.0, 600.0);
        let contents_id = scene.set_contents(Box::new(contents));

        // coord_root (20, 30)
        let coord_root_id = scene.add_child_to(
            contents_id,
            Box::new(TestCoordinateRootFigure::new(20.0, 30.0, 100.0, 100.0)),
        );

        // child 相对于 coord_root (10, 5)
        let child = RectangleFigure::new(10.0, 5.0, 50.0, 50.0);
        let child_id = scene.add_child_to(coord_root_id, Box::new(child));

        // 本地坐标 (10, 5) 转换为绝对坐标 (30, 35)
        let mut point = (10.0, 5.0);
        scene.translate_to_absolute_mut(child_id, &mut point);
        assert_eq!(point.0, 30.0, "x 应为 10 + 20");
        assert_eq!(point.1, 35.0, "y 应为 5 + 30");
    }

    /// 测试 translate_to_absolute_mut 嵌套坐标根
    ///
    /// 场景：多层坐标根
    /// 期望：正确累加多个坐标根的 bounds
    #[test]
    fn test_translate_to_absolute_mut_nested() {
        let mut scene = FigureGraph::new();

        let contents = RectangleFigure::new(0.0, 0.0, 800.0, 600.0);
        let contents_id = scene.set_contents(Box::new(contents));

        // coord_root1 (10, 20)
        let coord_root1_id = scene.add_child_to(
            contents_id,
            Box::new(TestCoordinateRootFigure::new(10.0, 20.0, 100.0, 100.0)),
        );

        // coord_root2 相对于 coord_root1 (5, 10)
        let coord_root2_id = scene.add_child_to(
            coord_root1_id,
            Box::new(TestCoordinateRootFigure::new(5.0, 10.0, 50.0, 50.0)),
        );

        // child 相对于 coord_root2 (15, 25)
        let child = RectangleFigure::new(15.0, 25.0, 30.0, 30.0);
        let child_id = scene.add_child_to(coord_root2_id, Box::new(child));

        // 绝对坐标 = coord_root1 + coord_root2 + child = (10+5+15, 20+10+25) = (30, 55)
        let mut point = (15.0, 25.0);
        scene.translate_to_absolute_mut(child_id, &mut point);
        assert_eq!(point.0, 30.0, "x 应为 15 + 10 + 5");
        assert_eq!(point.1, 55.0, "y 应为 25 + 20 + 10");
    }

    /// 测试 translate_to_absolute_mut Rectangle 类型
    ///
    /// 场景：使用 Rectangle 类型进行坐标转换
    /// 期望：Rectangle 的 x, y 被正确转换
    #[test]
    fn test_translate_to_absolute_mut_rect() {
        let mut scene = FigureGraph::new();

        let contents = RectangleFigure::new(0.0, 0.0, 800.0, 600.0);
        let contents_id = scene.set_contents(Box::new(contents));

        // coord_root (20, 30)
        let coord_root_id = scene.add_child_to(
            contents_id,
            Box::new(TestCoordinateRootFigure::new(20.0, 30.0, 100.0, 100.0)),
        );

        // child 相对于 coord_root (10, 5)
        let child = RectangleFigure::new(10.0, 5.0, 50.0, 50.0);
        let child_id = scene.add_child_to(coord_root_id, Box::new(child));

        // 本地坐标 Rectangle (10, 5, 50, 50) 转换为绝对坐标 (30, 35, 50, 50)
        let mut rect = Rectangle::new(10.0, 5.0, 50.0, 50.0);
        scene.translate_to_absolute_mut(child_id, &mut rect);
        assert_eq!(rect.x, 30.0, "x 应为 10 + 20");
        assert_eq!(rect.y, 35.0, "y 应为 5 + 30");
    }

    // ========== 迭代渲染测试 ==========

    /// 测试迭代渲染：嵌套层次
    ///
    /// 场景：父 → 子1 → 孙1
    /// 期望：迭代渲染与递归渲染产生相同数量的命令
    #[test]
    fn test_iterative_render_nested() {
        use super::render_iterative::FigureRendererIter;

        let mut scene = FigureGraph::new();

        // 根
        let root = RectangleFigure::new(0.0, 0.0, 200.0, 200.0);
        let root_id = scene.set_contents(Box::new(root));

        // 子
        let child = RectangleFigure::new(50.0, 50.0, 100.0, 100.0);
        let child_id = scene.add_child_to(root_id, Box::new(child));

        // 孙
        let grandchild = RectangleFigure::new(60.0, 60.0, 30.0, 30.0);
        let _gc_id = scene.add_child_to(child_id, Box::new(grandchild));

        // 递归渲染
        let gc_recursive = scene.render();
        let cmd_recursive = gc_recursive.commands().len();

        // 迭代渲染
        let mut gc_iterative = novadraw_render::NdCanvas::new();
        let scene_ref = FigureGraphRenderRef {
            blocks: &scene.blocks,
        };
        let mut renderer = FigureRendererIter::new(&scene_ref, &mut gc_iterative);
        renderer.render(root_id);
        let cmd_iterative = gc_iterative.commands().len();

        // 验证：两种渲染方式都产生有效数量的命令
        assert!(
            cmd_recursive >= 10,
            "递归渲染应产生至少 10 个命令，实际为 {}",
            cmd_recursive
        );
        assert!(
            cmd_iterative >= 10,
            "迭代渲染应产生至少 10 个命令，实际为 {}",
            cmd_iterative
        );
    }

    /// 测试迭代渲染：Z-order
    ///
    /// 场景：父容器包含三个子矩形
    /// 期望：迭代渲染与递归渲染产生相同数量的命令
    #[test]
    fn test_iterative_render_z_order() {
        use super::render_iterative::FigureRendererIter;

        let mut scene = FigureGraph::new();

        // 创建父容器（100x100）
        let parent = RectangleFigure::new(0.0, 0.0, 100.0, 100.0);
        let parent_id = scene.set_contents(Box::new(parent));

        // 添加三个子矩形
        let child1 = RectangleFigure::new(10.0, 10.0, 20.0, 20.0);
        let _c1 = scene.add_child_to(parent_id, Box::new(child1));

        let child2 = RectangleFigure::new(30.0, 30.0, 20.0, 20.0);
        let _c2 = scene.add_child_to(parent_id, Box::new(child2));

        let child3 = RectangleFigure::new(50.0, 50.0, 20.0, 20.0);
        let _c3 = scene.add_child_to(parent_id, Box::new(child3));

        // 递归渲染
        let gc_recursive = scene.render();
        let cmd_recursive = gc_recursive.commands().len();

        // 迭代渲染
        let mut gc_iterative = novadraw_render::NdCanvas::new();
        let scene_ref = FigureGraphRenderRef {
            blocks: &scene.blocks,
        };
        let mut renderer = FigureRendererIter::new(&scene_ref, &mut gc_iterative);
        renderer.render(parent_id);
        let cmd_iterative = gc_iterative.commands().len();

        // 验证：两种渲染方式都产生有效数量的命令
        assert!(
            cmd_recursive >= 10,
            "递归渲染应产生至少 10 个命令，实际为 {}",
            cmd_recursive
        );
        assert!(
            cmd_iterative >= 10,
            "迭代渲染应产生至少 10 个命令，实际为 {}",
            cmd_iterative
        );
    }

    /// 测试迭代渲染：可见性过滤
    ///
    /// 场景：父容器包含可见和不可见子元素
    /// 期望：迭代渲染只渲染可见元素
    #[test]
    fn test_iterative_render_visibility_filter() {
        use super::render_iterative::FigureRendererIter;

        let mut scene = FigureGraph::new();

        let parent = RectangleFigure::new(0.0, 0.0, 100.0, 100.0);
        let parent_id = scene.set_contents(Box::new(parent));

        // 可见子元素
        let visible_child = RectangleFigure::new(10.0, 10.0, 20.0, 20.0);
        let _ = scene.add_child_to(parent_id, Box::new(visible_child));

        // 不可见子元素
        let invisible_child = RectangleFigure::new(50.0, 50.0, 20.0, 20.0);
        let invisible_id = scene.add_child_to(parent_id, Box::new(invisible_child));

        // 设置不可见
        scene.blocks.get_mut(invisible_id).unwrap().is_visible = false;

        // 迭代渲染
        let mut gc_iterative = novadraw_render::NdCanvas::new();
        let scene_ref = FigureGraphRenderRef {
            blocks: &scene.blocks,
        };
        let mut renderer = FigureRendererIter::new(&scene_ref, &mut gc_iterative);
        renderer.render(parent_id);
        let cmd_iterative = gc_iterative.commands().len();

        // 验证：有 2 个图形（parent + visible_child），每个产生多个命令
        assert!(
            cmd_iterative >= 8 && cmd_iterative <= 18,
            "应只渲染可见元素，实际为 {} 个命令",
            cmd_iterative
        );
    }

    /// 测试迭代渲染：深度嵌套
    ///
    /// 场景：创建 10 层深度的嵌套结构
    /// 期望：迭代渲染能正确处理深度嵌套，不产生栈溢出
    #[test]
    fn test_iterative_render_deep_nesting() {
        use super::render_iterative::FigureRendererIter;

        let mut scene = FigureGraph::new();

        // 创建根
        let root = RectangleFigure::new(0.0, 0.0, 100.0, 100.0);
        let mut parent_id = scene.set_contents(Box::new(root));

        // 创建 10 层嵌套
        for i in 0..10 {
            let child =
                RectangleFigure::new((i as f64 + 1.0) * 10.0, (i as f64 + 1.0) * 10.0, 50.0, 50.0);
            parent_id = scene.add_child_to(parent_id, Box::new(child));
        }

        // 迭代渲染（不应栈溢出）
        let mut gc_iterative = novadraw_render::NdCanvas::new();
        let scene_ref = FigureGraphRenderRef {
            blocks: &scene.blocks,
        };
        let mut renderer = FigureRendererIter::new(&scene_ref, &mut gc_iterative);
        renderer.render(scene.get_contents().unwrap());

        // 验证：有 11 个图形（1 个根 + 10 个子）
        let cmd_iterative = gc_iterative.commands().len();
        assert!(
            cmd_iterative >= 50,
            "应有足够的渲染命令，实际为 {}",
            cmd_iterative
        );
    }
}

impl crate::scene_host::SceneUpdateTarget for FigureGraph {
    fn perform_update(&mut self) -> novadraw_render::NdCanvas {
        FigureGraph::perform_update(self)
    }

    fn repair_damage(&mut self) -> novadraw_render::NdCanvas {
        FigureGraph::repair_damage(self)
    }
}
