//! 场景图管理
//!
//! 提供场景图数据结构和管理功能。

use std::sync::Arc;

use glam::DVec2;
use novadraw_math::{Transform, Vec2};
use novadraw_render::RenderContext;
use slotmap::SlotMap;
use uuid::Uuid;

use super::layout::LayoutManager;

slotmap::new_key_type! { pub struct BlockId; }

/// 2D 点类型
pub type Point = DVec2;

/// 矩形区域
///
/// 用于表示二维空间中的矩形区域。
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Rect {
    /// X 坐标
    pub x: f64,
    /// Y 坐标
    pub y: f64,
    /// 宽度
    pub width: f64,
    /// 高度
    pub height: f64,
}

impl Rect {
    /// 创建矩形
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self { x, y, width, height }
    }

    /// 从两个角点创建矩形
    pub fn from_corners(corner1: Point, corner2: Point) -> Self {
        let x = corner1.x.min(corner2.x);
        let y = corner1.y.min(corner2.y);
        let width = (corner2.x - corner1.x).abs();
        let height = (corner2.y - corner1.y).abs();
        Self { x, y, width, height }
    }

    /// 检查点是否在矩形内
    pub fn contains(&self, point: Point) -> bool {
        point.x >= self.x
            && point.x <= self.x + self.width
            && point.y >= self.y
            && point.y <= self.y + self.height
    }

    /// 获取中心点
    pub fn center(&self) -> Point {
        Point::new(self.x + self.width / 2.0, self.y + self.height / 2.0)
    }
}

fn rect_intersects(a: &Rect, b: &Rect) -> bool {
    a.x < b.x + b.width && a.x + a.width > b.x && a.y < b.y + b.height && a.y + a.height > b.y
}

/// 运行时块
///
/// 场景图中的基本单元，包含图形和变换。
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
    /// 变换
    pub transform: Transform,
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
            transform: Transform::IDENTITY,
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

    fn paint(&self, gc: &mut RenderContext) {
        if !self.is_visible {
            return;
        }
        gc.push_transform(self.transform);
        self.figure.paint(gc);
        if self.is_selected {
            self.figure.paint_highlight(gc);
        }
        gc.pop_transform();
    }

    pub fn set_figure(&mut self, figure: Box<dyn super::Figure>) {
        self.figure = figure;
    }

    pub fn set_transform(&mut self, transform: Transform) {
        self.transform = transform;
    }

    pub fn hit_test(&self, point: Point) -> bool {
        if !self.is_visible || !self.is_enabled {
            return false;
        }
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

    pub fn translate(&mut self, dx: f64, dy: f64) {
        if let Some(rect) = self.figure.as_rectangle_mut() {
            rect.translate(dx, dy);
            return;
        }
        let translate = Transform::from_translation(dx, dy);
        self.transform = self.transform * translate;
    }
}

#[inline]
fn dvec2_to_vec2(v: DVec2) -> Vec2 {
    Vec2::new(v.x, v.y)
}

#[inline]
fn vec2_to_dvec2(v: Vec2) -> DVec2 {
    DVec2::new(v.x(), v.y())
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
            transform: Transform::IDENTITY,
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

    /// 创建带父块的块
    fn new_block_with_parent(&mut self, figure: Box<dyn super::Figure>, parent_id: BlockId) -> BlockId {
        let uuid = Uuid::new_v4();
        let id = self.blocks.insert_with_key(|key| RuntimeBlock {
            id: key,
            uuid,
            children: Vec::new(),
            parent: Some(parent_id),
            figure,
            transform: Transform::IDENTITY,
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

    /// 创建内容块
    pub fn new_content_block(&mut self, figure: Box<dyn super::Figure>) -> BlockId {
        self.new_content_block_with_transform(figure, Transform::IDENTITY)
    }

    /// 创建带变换的内容块
    pub fn new_content_block_with_transform(&mut self, figure: Box<dyn super::Figure>, transform: Transform) -> BlockId {
        let uuid = Uuid::new_v4();
        let id = self.blocks.insert_with_key(|key| RuntimeBlock {
            id: key,
            uuid,
            children: Vec::new(),
            parent: Some(self.root),
            figure,
            transform,
            is_selected: false,
            is_visible: true,
            is_enabled: true,
            preferred_size: None,
            minimum_size: None,
            maximum_size: None,
        });
        self.uuid_map.insert(uuid, id);
        self.blocks[self.root].children.push(id);
        self.layout_valid = false;
        id
    }

    /// 创建 UI 层块
    pub fn new_ui_block(&mut self, figure: Box<dyn super::Figure>) -> BlockId {
        self.new_ui_block_with_transform(figure)
    }

    /// 创建带变换的 UI 层块
    pub fn new_ui_block_with_transform(&mut self, figure: Box<dyn super::Figure>) -> BlockId {
        let uuid = Uuid::new_v4();
        let id = self.blocks.insert_with_key(|key| RuntimeBlock {
            id: key,
            uuid,
            children: Vec::new(),
            parent: Some(self.root),
            figure,
            transform: Transform::IDENTITY,
            is_selected: false,
            is_visible: true,
            is_enabled: true,
            preferred_size: None,
            minimum_size: None,
            maximum_size: None,
        });
        self.uuid_map.insert(uuid, id);
        self.blocks[self.root].children.push(id);
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

    /// 命中测试
    pub fn hit_test(&self, point: Point) -> Option<BlockId> {
        self.hit_test_content(point)
    }

    fn hit_test_from(&self, start_id: BlockId, point: Point) -> Option<BlockId> {
        #[derive(Clone, Copy)]
        enum StackItem {
            Node(BlockId),
            Check(BlockId),
        }

        let mut stack: Vec<StackItem> = Vec::new();
        stack.push(StackItem::Node(start_id));

        while let Some(item) = stack.pop() {
            match item {
                StackItem::Node(node_id) => {
                    if let Some(node) = self.blocks.get(node_id) {
                        stack.push(StackItem::Check(node_id));
                        for &child_id in node.children.iter().rev() {
                            stack.push(StackItem::Node(child_id));
                        }
                    }
                }
                StackItem::Check(node_id) => {
                    if let Some(node) = self.blocks.get(node_id) {
                        if node.hit_test(point) {
                            return Some(node_id);
                        }
                    }
                }
            }
        }
        None
    }

    /// 内容块命中测试
    pub fn hit_test_content(&self, point: Point) -> Option<BlockId> {
        if let Some(contents) = self.contents {
            self.hit_test_from(contents, point)
        } else {
            self.hit_test_from(self.root, point)
        }
    }

    /// 矩形选择命中测试
    pub fn hit_test_rect(&self, rect: Rect) -> Vec<BlockId> {
        self.hit_test_rect_with_transform(rect, Transform::IDENTITY)
    }

    fn hit_test_rect_with_transform(&self, rect: Rect, _parent_transform: Transform) -> Vec<BlockId> {
        let mut selected = Vec::new();
        let mut stack = Vec::new();

        if let Some(root) = self.blocks.get(self.root) {
            for &child_id in root.children.iter() {
                if let Some(_block) = self.blocks.get(child_id) {
                    stack.push((child_id, Transform::IDENTITY));
                }
            }
        }

        while let Some((node_id, parent_t)) = stack.pop() {
            if let Some(node) = self.blocks.get(node_id) {
                let cumulative_transform = parent_t * node.transform;

                for &child_id in node.children.iter() {
                    stack.push((child_id, cumulative_transform));
                }

                let bounds = node.figure.bounds();
                let corners = [
                    vec2_to_dvec2(cumulative_transform.transform_point(dvec2_to_vec2(DVec2::new(bounds.x, bounds.y)))),
                    vec2_to_dvec2(cumulative_transform.transform_point(dvec2_to_vec2(DVec2::new(bounds.x + bounds.width, bounds.y)))),
                    vec2_to_dvec2(cumulative_transform.transform_point(dvec2_to_vec2(DVec2::new(bounds.x + bounds.width, bounds.y + bounds.height)))),
                    vec2_to_dvec2(cumulative_transform.transform_point(dvec2_to_vec2(DVec2::new(bounds.x, bounds.y + bounds.height)))),
                ];
                let transformed_bounds = Rect::from_corners(corners[0], corners[2]);
                if rect_intersects(&rect, &transformed_bounds) {
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

    /// 渲染场景图
    pub fn render(&self) -> RenderContext {
        let mut gc = RenderContext::new();
        self.render_to_context(&mut gc);
        gc
    }

    /// 使用视口变换渲染
    pub fn render_with_viewport(&self, viewport_transform: Transform) -> RenderContext {
        let mut gc = RenderContext::new();
        self.render_to_context_with_viewport(&mut gc, viewport_transform);
        gc
    }

    fn render_to_context(&self, gc: &mut RenderContext) {
        self.traverse_dfs_stack(|block_id| {
            if let Some(runtime_block) = self.blocks.get(block_id) {
                runtime_block.paint(gc);
            }
        })
    }

    fn render_to_context_with_viewport(&self, gc: &mut RenderContext, viewport_transform: Transform) {
        let mut stack = Vec::new();

        gc.push_transform(viewport_transform);

        stack.push(self.root);

        while let Some(node_id) = stack.pop() {
            if let Some(runtime_block) = self.blocks.get(node_id) {
                runtime_block.paint(gc);

                for &child_id in runtime_block.children.iter().rev() {
                    stack.push(child_id);
                }
            }
        }

        gc.pop_transform();
    }

    fn traverse_dfs_stack<F>(&self, mut visitor: F)
    where
        F: FnMut(BlockId),
    {
        let mut stack = Vec::new();
        stack.push(self.root);

        while let Some(node_id) = stack.pop() {
            visitor(node_id);

            if let Some(node) = self.blocks.get(node_id) {
                for &child_id in node.children.iter().rev() {
                    stack.push(child_id);
                }
            }
        }
    }

    /// 获取块
    pub fn get_block(&self, id: BlockId) -> Option<&RuntimeBlock> {
        self.blocks.get(id)
    }

    /// 平移块
    pub fn translate(&mut self, id: BlockId, dx: f64, dy: f64) {
        if let Some(block) = self.blocks.get_mut(id) {
            block.translate(dx, dy);
        }
    }

    /// 设置块变换
    pub fn set_block_transform(&mut self, id: BlockId, transform: Transform) {
        if let Some(block) = self.blocks.get_mut(id) {
            block.set_transform(transform);
        }
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
                        rect.x = new_bounds.x;
                        rect.y = new_bounds.y;
                        rect.width = new_bounds.width;
                        rect.height = new_bounds.height;
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
