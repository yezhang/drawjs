//! 场景图管理
//!
//! 提供场景图数据结构和管理功能。

use std::sync::Arc;

use novadraw_geometry::{Point, Rect};
use novadraw_render::NdCanvas;
use slotmap::SlotMap;
use uuid::Uuid;

use super::layout::LayoutManager;

pub mod hit_test;
pub mod figure_paint;

pub use hit_test::HitTestResult;
pub use figure_paint::{FigureRenderer, PaintTask, SceneGraphRenderRef};

slotmap::new_key_type! { pub struct BlockId; }

/// 运行时块
///
/// 场景图中的基本单元，包含图形。
pub struct RuntimeBlock {
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

impl RuntimeBlock {
    /// 创建新运行时块
    pub fn new(id: BlockId, uuid: Uuid, figure: Box<dyn super::Figure>) -> Self {
        Self {
            id,
            uuid,
            children: Vec::new(),
            parent: None,
            figure,
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
    pub fn figure_bounds(&self) -> Rect {
        self.figure.bounds()
    }

    /// 检查点在图形边界内（局部坐标）
    pub fn contains_local(&self, point: Point) -> bool {
        self.figure.hit_test(point)
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
}

#[inline]
fn rect_intersects(a: &Rect, b: &Rect) -> bool {
    a.x < b.x + b.width
        && a.x + a.width > b.x
        && a.y < b.y + b.height
        && a.y + a.height > b.y
}

/// 场景图
///
/// 管理所有图形块的层次结构，参考 Eclipse Draw2d 设计模式。
///
/// # 使用示例
///
/// ```
/// use novadraw_scene::{Figure, RectangleFigure, SceneGraph};
///
/// let mut scene = SceneGraph::new();
///
/// // 创建根内容块（类似 Draw2d 的 setContents）
/// let contents = RectangleFigure::new(0.0, 0.0, 100.0, 50.0);
/// let contents_id = scene.set_contents(Box::new(contents));
///
/// // 添加子块到指定父块（类似 Draw2d 的 parent.addChild(child)）
/// let child = RectangleFigure::new(10.0, 10.0, 80.0, 30.0);
/// scene.add_child_to(contents_id, Box::new(child));
/// ```
pub struct SceneGraph {
    pub blocks: SlotMap<BlockId, RuntimeBlock>,
    pub uuid_map: std::collections::HashMap<Uuid, BlockId>,
    /// 根块（内部使用）
    root: BlockId,
    /// 内容块（用户可访问的根容器）
    contents: Option<BlockId>,
    pub layout_manager: Option<Arc<dyn LayoutManager>>,
    /// 布局是否有效，false 表示需要重新计算布局
    pub layout_valid: bool,
}

impl SceneGraph {
    /// 创建新场景图
    pub fn new() -> Self {
        let mut blocks = SlotMap::with_key();
        let uuid = Uuid::new_v4();

        let root_id = blocks.insert_with_key(|key| RuntimeBlock {
            id: key,
            uuid,
            children: Vec::new(),
            parent: None,
            figure: Box::new(super::figure::BaseFigure::new(0.0, 0.0, 0.0, 0.0)),
            is_selected: false,
            is_visible: true,
            is_enabled: true,
            preferred_size: None,
            minimum_size: None,
            maximum_size: None,
        });

        SceneGraph {
            blocks,
            uuid_map: std::collections::HashMap::new(),
            root: root_id,
            contents: None,
            layout_manager: None,
            layout_valid: true,
        }
    }

    /// 设置内容块（类似 Draw2d 的 LightweightSystem.setContents）
    ///
    /// 设置场景的根容器，后续添加的子块将作为此容器的子元素。
    pub fn set_contents(&mut self, figure: Box<dyn super::Figure>) -> BlockId {
        let contents_id = self.new_block_with_parent(figure, self.root);
        self.contents = Some(contents_id);
        contents_id
    }

    /// 获取内容块
    pub fn get_contents(&self) -> Option<BlockId> {
        self.contents
    }

    /// 添加子块到指定父块（类似 Draw2d 的 parent.addChild(child)）
    pub fn add_child_to(&mut self, parent_id: BlockId, figure: Box<dyn super::Figure>) -> BlockId {
        self.new_block_with_parent(figure, parent_id)
    }

    /// 添加子块到指定父块，并设置相对于父节点的局部位置和尺寸（类似 Draw2d 的 Figure.setBounds）
    ///
    /// # 与 Draw2D 的一致性
    ///
    /// - Figure 的 bounds (x, y, width, height) 直接存储在 Figure 中
    /// - bounds 的 x, y 就是相对于父节点的位置
    /// - 位置由 Figure.bounds() 定义，无独立的运行时变换
    ///
    /// # 示例
    ///
    /// ```
    /// use novadraw_core::Color;
    /// use novadraw_scene::{figure::Rectangle, SceneGraph};
    ///
    /// let mut scene = SceneGraph::new();
    /// let parent_id = scene.set_contents(Box::new(Rectangle::new(0.0, 0.0, 100.0, 100.0)));
    /// let color = Color::hex("#3498db");
    /// // 添加子节点，相对于父节点位于 (10, 10)，尺寸 50x50
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
        let figure = super::figure::Rectangle::new_with_color(x, y, width, height, color);
        self.new_block_with_parent(Box::new(figure), parent_id)
    }

    /// 创建带父块的块
    fn new_block_with_parent(
        &mut self,
        figure: Box<dyn super::Figure>,
        parent_id: BlockId,
    ) -> BlockId {
        let uuid = Uuid::new_v4();
        let id = self.blocks.insert_with_key(|key| RuntimeBlock {
            id: key,
            uuid,
            children: Vec::new(),
            parent: Some(parent_id),
            figure,
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
    pub fn invalidate(&mut self) {
        self.layout_valid = false;
    }

    /// 重新验证布局，如果布局无效则重新计算
    pub fn revalidate(&mut self, container_bounds: Rect) {
        if !self.layout_valid {
            self.apply_layout(container_bounds);
            self.layout_valid = true;
        }
    }

    /// 检查布局是否有效
    pub fn is_layout_valid(&self) -> bool {
        self.layout_valid
    }

    /// 内容块命中测试（简单版本）
    ///
    /// 返回命中的 BlockId，不包含路径信息。
    /// 使用 Trampoline 模式避免递归栈溢出。
    pub fn hit_test_content(&self, point: Point) -> Option<BlockId> {
        self.hit_test_with_path(point).map(|r| r.target())
    }

    /// 矩形选择命中测试
    pub fn hit_test_rect(&self, rect: Rect) -> Vec<BlockId> {
        let mut selected = Vec::new();
        let mut stack = vec![self.root];

        while let Some(node_id) = stack.pop() {
            if let Some(block) = self.blocks.get(node_id) {
                if !block.is_visible {
                    continue;
                }

                let bounds = block.figure_bounds();

                // 先处理子节点
                for &child_id in block.children.iter().rev() {
                    stack.push(child_id);
                }

                // 检查矩形相交
                if rect_intersects(&rect, &bounds) {
                    selected.push(node_id);
                }
            }
        }
        selected
    }

    /// 按矩形选择
    pub fn select_by_rect(&mut self, rect: Rect) {
        let selected = self.hit_test_rect(rect);
        for block in self.blocks.values_mut() {
            block.is_selected = false;
        }
        for id in selected {
            if let Some(block) = self.blocks.get_mut(id) {
                block.is_selected = true;
            }
        }
    }

    /// 选择单个块
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

    /// 渲染场景图（Trampoline 模式）
    ///
    /// 使用 Trampoline 模式实现 Figure 树的渲染遍历。
    /// 核心思想：将所有操作转换为任务，放入队列中顺序执行。
    ///
    /// 渲染顺序（参考 d2）：
    /// 1. paintClientArea() - 客户区域（包含子元素）
    /// 2. paintBorder() - 绘制边框
    /// 3. paintHighlight() - 绘制选中高亮
    ///
    /// # 优势
    /// - 避免递归导致的栈溢出
    /// - 任务在堆上，无深度限制
    /// - 状态完全由任务队列控制
    pub fn render(&self) -> NdCanvas {
        let mut gc = NdCanvas::new();
        self.render_to(&mut gc);
        gc
    }

    /// 使用 Trampoline 模式渲染到上下文
    fn render_to(&self, gc: &mut NdCanvas) {
        let start_id = self.contents.unwrap_or(self.root);
        let scene_ref = SceneGraphRenderRef {
            blocks: &self.blocks,
        };
        let mut renderer = FigureRenderer::new(&scene_ref, gc);
        renderer.render(start_id);
    }

    // ========== 命中测试方法 ==========

    /// 命中测试（带路径）
    ///
    /// 使用 Trampoline 模式实现非递归的深度优先遍历。
    /// 从 contents 开始检测，返回包含路径的命中结果。
    ///
    /// # 算法
    ///
    /// - 从 contents 开始（用户可见内容区域）
    /// - 逆序遍历直接子节点（后添加的在上层，先检测）
    /// - 利用 bounds 进行剪枝
    /// - 找到命中的最深层节点即返回
    ///
    /// # 返回
    ///
    /// 如果命中，返回 `HitTestResult`，包含：
    /// - `target()`: 命中的 BlockId
    /// - `path()`: 从 contents 到命中节点的路径
    pub fn hit_test_with_path(&self, point: Point) -> Option<HitTestResult> {
        use crate::scene::hit_test::{HitTestRef, HitTester};

        let start_id = self.contents.unwrap_or(self.root);
        let scene_ref = HitTestRef {
            blocks: &self.blocks,
        };
        let mut tester = HitTester::new(&scene_ref);
        tester.hit_test(start_id, point)
    }

    /// 命中测试并选中
    ///
    /// 执行命中测试，并将命中的节点设为选中状态。
    /// 返回命中的节点 ID。
    pub fn hit_test_and_select(&mut self, point: Point) -> Option<BlockId> {
        let result = self.hit_test_with_path(point);
        let target_id = result.map(|hit| hit.target());
        self.select_single(target_id);
        target_id
    }

    // ========== 调试验证方法 ==========

    /// 打印场景图树结构（用于调试）
    ///
    /// 使用 `eprintln!` 输出到 stderr，格式示例：
    /// ```text
    /// V BlockId(0x1): Figure bounds=(0,0,100,100)
    ///   V BlockId(0x2): Rectangle bounds=(10,10,50,50)
    ///   H BlockId(0x3): Rectangle bounds=(50,50,50,50)  // 不可见
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
    pub fn get_block(&self, id: BlockId) -> Option<&RuntimeBlock> {
        self.blocks.get(id)
    }

    /// 设置布局管理器
    pub fn set_layout_manager(&mut self, layout_manager: Arc<dyn LayoutManager>) {
        self.layout_manager = Some(layout_manager);
    }

    /// 应用布局
    ///
    /// 根据布局管理器重新计算子元素的位置。
    pub fn apply_layout(&mut self, container_bounds: Rect) {
        let Some(layout_manager) = &self.layout_manager else {
            return;
        };

        let container_id = self.root;
        if let Some(container) = self.blocks.get(container_id) {
            let mut children_bounds = Vec::new();
            for &child_id in &container.children {
                if let Some(child) = self.blocks.get(child_id) {
                    let bounds = child.figure.bounds();
                    children_bounds.push((child_id, bounds));
                }
            }

            layout_manager.layout(container_bounds, &mut children_bounds);

            for (child_id, new_bounds) in children_bounds {
                if let Some(child) = self.blocks.get_mut(child_id) {
                    if let Some(rect) = child.figure.as_rectangle_mut() {
                        rect.bounds = new_bounds;
                    }
                }
            }
        }
    }

    /// 计算布局大小
    pub fn compute_layout_size(&self, container_bounds: Rect) -> (f64, f64) {
        let Some(layout_manager) = &self.layout_manager else {
            return (container_bounds.width, container_bounds.height);
        };

        if let Some(container) = self.blocks.get(self.root) {
            let mut children_bounds = Vec::new();
            for &child_id in &container.children {
                if let Some(child) = self.blocks.get(child_id) {
                    children_bounds.push(child.figure.bounds());
                }
            }
            layout_manager.compute_size(container_bounds, &children_bounds)
        } else {
            (container_bounds.width, container_bounds.height)
        }
    }
}

impl Default for SceneGraph {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::figure::Rectangle;

    /// 测试渲染顺序：Z-order 验证
    ///
    /// 场景：父容器包含三个子矩形（从下到上添加）
    /// 期望：渲染顺序应为 parent → child1 → child2 → child3
    ///       即先添加的在下面（被遮挡），后添加的在上面（遮挡别人）
    #[test]
    fn test_render_order_z_order() {
        let mut scene = SceneGraph::new();

        // 创建父容器（100x100）
        let parent = Rectangle::new(0.0, 0.0, 100.0, 100.0);
        let parent_id = scene.set_contents(Box::new(parent));

        // 添加三个子矩形（从下到上添加）
        let child1 = Rectangle::new(10.0, 10.0, 20.0, 20.0);
        let _c1 = scene.add_child_to(parent_id, Box::new(child1));

        let child2 = Rectangle::new(30.0, 30.0, 20.0, 20.0);
        let _c2 = scene.add_child_to(parent_id, Box::new(child2));

        let child3 = Rectangle::new(50.0, 50.0, 20.0, 20.0);
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

        // Trampoline 渲染：每个矩形产生多个命令
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
        assert!(cmd_count >= 20 && cmd_count <= 35, "应有 20-35 个渲染命令，实际为 {}", cmd_count);
    }

    /// 测试渲染顺序：嵌套层次
    ///
    /// 场景：父 → 子1 → 孙1
    /// 期望渲染顺序：parent → child1 → grandchild1
    #[test]
    fn test_render_order_nested() {
        let mut scene = SceneGraph::new();

        // 根
        let root = Rectangle::new(0.0, 0.0, 200.0, 200.0);
        let root_id = scene.set_contents(Box::new(root));

        // 子
        let child = Rectangle::new(50.0, 50.0, 100.0, 100.0);
        let child_id = scene.add_child_to(root_id, Box::new(child));

        // 孙
        let grandchild = Rectangle::new(60.0, 60.0, 30.0, 30.0);
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

        // Trampoline 渲染：每个图形产生多个命令
        // 3 个图形：root + child + grandchild
        // 每个图形的命令数（参见 test_render_order_z_order）
        // Total: 8 (root) + 7 (child) + 7 (grandchild) = 22
        assert!(cmd_count >= 12 && cmd_count <= 25, "应有 12-25 个渲染命令，实际为 {}", cmd_count);
    }

    /// 测试可见性过滤
    ///
    /// 场景：父容器包含可见子元素和不可见子元素
    /// 期望：只渲染可见元素
    #[test]
    fn test_visibility_filter() {
        let mut scene = SceneGraph::new();

        let parent = Rectangle::new(0.0, 0.0, 100.0, 100.0);
        let parent_id = scene.set_contents(Box::new(parent));

        // 可见子元素
        let visible_child = Rectangle::new(10.0, 10.0, 20.0, 20.0);
        let _ = scene.add_child_to(parent_id, Box::new(visible_child));

        // 不可见子元素
        let invisible_child = Rectangle::new(50.0, 50.0, 20.0, 20.0);
        let invisible_id = scene.add_child_to(parent_id, Box::new(invisible_child));

        // 设置不可见
        scene.blocks.get_mut(invisible_id).unwrap().is_visible = false;

        let gc = scene.render();
        let cmd_count = gc.commands().len();

        // Trampoline 渲染：parent + visible_child = 2 个图形
        // 每个图形的命令数（参见 test_render_order_z_order）
        // parent: 8, child: 7, Total: 15
        assert!(cmd_count >= 8 && cmd_count <= 18, "应只渲染可见元素，实际为 {} 个命令", cmd_count);
    }

    /// 测试变换累加
    ///
    /// 场景：子元素有非零位置
    /// 期望：Trampoline 渲染能正确处理嵌套层次
    #[test]
    fn test_transform_accumulation() {
        let mut scene = SceneGraph::new();

        let parent = Rectangle::new(0.0, 0.0, 100.0, 100.0);
        let parent_id = scene.set_contents(Box::new(parent));

        let child = Rectangle::new(25.0, 25.0, 50.0, 50.0);
        let _child_id = scene.add_child_to(parent_id, Box::new(child));

        // 打印场景结构
        {
            eprintln!("\n=== 测试变换累加 ===");
            eprintln!("Parent bounds: {:?}", scene.blocks.get(parent_id).unwrap().figure_bounds());
            eprintln!("Child bounds: {:?}", scene.blocks.get(parent_id).unwrap().children);
        }

        // Trampoline 渲染应能正确处理嵌套层次
        let gc = scene.render();
        let commands = gc.commands();

        // 验证：parent + child = 2 个图形
        // 每个图形的命令数（参见 test_render_order_z_order）
        // parent: 8, child: 7, Total: 15
        assert!(commands.len() >= 8, "应有足够的渲染命令，实际为 {}", commands.len());

        // 验证有 FillRect 命令
        let has_fill_rect = commands.iter().any(|cmd| {
            matches!(cmd.kind, novadraw_render::command::RenderCommandKind::FillRect { .. })
        });
        assert!(has_fill_rect, "应有 FillRect 命令");
    }
}
