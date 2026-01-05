pub mod figure;

pub use figure::{Figure, RectangleFigure};

use slotmap::SlotMap;
use uuid::Uuid;

use crate::core::transform::Transform;
use crate::render::RenderContext;
use glam::DVec2;

slotmap::new_key_type! { pub struct BlockId; }

pub type Point = DVec2;

#[derive(Clone, Copy, Debug)]
pub struct Rect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl Rect {
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self { x, y, width, height }
    }

    pub fn from_corners(corner1: Point, corner2: Point) -> Self {
        let x = corner1.x.min(corner2.x);
        let y = corner1.y.min(corner2.y);
        let width = (corner2.x - corner1.x).abs();
        let height = (corner2.y - corner1.y).abs();
        Self { x, y, width, height }
    }

    pub fn contains(&self, point: Point) -> bool {
        point.x >= self.x
            && point.x <= self.x + self.width
            && point.y >= self.y
            && point.y <= self.y + self.height
    }

    pub fn center(&self) -> Point {
        Point::new(self.x + self.width / 2.0, self.y + self.height / 2.0)
    }
}

fn rect_intersects(a: &Rect, b: &Rect) -> bool {
    a.x < b.x + b.width && a.x + a.width > b.x && a.y < b.y + b.height && a.y + a.height > b.y
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BlockType {
    Root,
    Background,
    Content,
    UILayer,
}

pub struct RuntimeBlock {
    pub id: BlockId,
    pub uuid: Uuid,
    pub block_type: BlockType,
    pub children: Vec<BlockId>,
    pub parent: Option<BlockId>,
    pub figure: Box<dyn Figure>,
    pub transform: Transform,
    pub is_selected: bool,
}

impl RuntimeBlock {
    fn paint(&self, gc: &mut RenderContext) {
        gc.push_transform(self.transform);
        self.figure.paint(gc);
        if self.is_selected && self.block_type == BlockType::Content {
            self.figure.paint_highlight(gc);
        }
        gc.pop_transform();
    }

    pub fn set_figure(&mut self, figure: Box<dyn Figure>) {
        self.figure = figure;
    }

    pub fn set_transform(&mut self, transform: Transform) {
        self.transform = transform;
    }

    pub fn hit_test(&self, point: Point, parent_transform: &Transform) -> bool {
        if self.block_type != BlockType::Content {
            return false;
        }
        let cumulative_transform = *parent_transform * self.transform;
        let local_point = cumulative_transform.inverse().multiply_point_2d(point);
        self.figure.hit_test(local_point)
    }

    pub fn translate(&mut self, dx: f64, dy: f64) {
        if let Some(rect) = self.figure.as_rectangle_mut() {
            rect.translate(dx, dy);
        } else {
            let translate = Transform::from_translation_2d(dx, dy);
            self.transform = self.transform * translate;
        }
    }
}

pub struct SceneGraph {
    pub blocks: SlotMap<BlockId, RuntimeBlock>,
    pub uuid_map: std::collections::HashMap<Uuid, BlockId>,
    pub root: BlockId,
}

impl SceneGraph {
    pub fn new() -> Self {
        let mut blocks = SlotMap::with_key();
        let uuid = Uuid::new_v4();

        let root_id = blocks.insert_with_key(|key| RuntimeBlock {
            id: key,
            uuid,
            block_type: BlockType::Root,
            children: Vec::new(),
            parent: None,
            figure: Box::new(figure::NullFigure::new()),
            transform: Transform::identity(),
            is_selected: false,
        });

        SceneGraph {
            blocks,
            uuid_map: std::collections::HashMap::new(),
            root: root_id,
        }
    }

    pub fn new_content_block(&mut self, figure: Box<dyn Figure>) -> BlockId {
        self.new_content_block_with_transform(figure, Transform::identity())
    }

    pub fn new_content_block_with_transform(&mut self, figure: Box<dyn Figure>, transform: Transform) -> BlockId {
        let uuid = Uuid::new_v4();
        let id = self.blocks.insert_with_key(|key| RuntimeBlock {
            id: key,
            uuid,
            block_type: BlockType::Content,
            children: Vec::new(),
            parent: Some(self.root),
            figure,
            transform,
            is_selected: false,
        });
        self.uuid_map.insert(uuid, id);
        self.blocks[self.root].children.push(id);
        id
    }

    pub fn new_ui_block(&mut self, figure: Box<dyn Figure>) -> BlockId {
        let uuid = Uuid::new_v4();
        let id = self.blocks.insert_with_key(|key| RuntimeBlock {
            id: key,
            uuid,
            block_type: BlockType::UILayer,
            children: Vec::new(),
            parent: Some(self.root),
            figure,
            transform: Transform::identity(),
            is_selected: false,
        });
        self.uuid_map.insert(uuid, id);
        self.blocks[self.root].children.push(id);
        id
    }

    pub fn promote_ui_block_to_content(&mut self, ui_id: BlockId) -> Option<BlockId> {
        if let Some(ui_block) = self.blocks.get_mut(ui_id) {
            if ui_block.block_type == BlockType::UILayer {
                ui_block.block_type = BlockType::Content;
                return Some(ui_id);
            }
        }
        None
    }

    pub fn hit_test(&self, point: Point) -> Option<BlockId> {
        self.hit_test_content(point)
    }

    fn hit_test_with_transform(&self, point: Point, parent_transform: Transform) -> Option<BlockId> {
        let mut stack = Vec::new();

        if let Some(root) = self.blocks.get(self.root) {
            for &child_id in root.children.iter() {
                if let Some(block) = self.blocks.get(child_id) {
                    if block.block_type != BlockType::Content {
                        continue;
                    }
                    stack.push((child_id, Transform::identity()));
                }
            }
        }

        while let Some((node_id, parent_t)) = stack.pop() {
            if let Some(node) = self.blocks.get(node_id) {
                if node.block_type == BlockType::Content {
                    let cumulative_transform = parent_t * node.transform;

                    for &child_id in node.children.iter() {
                        if let Some(child) = self.blocks.get(child_id) {
                            if child.block_type == BlockType::Content {
                                stack.push((child_id, cumulative_transform));
                            }
                        }
                    }
                    if node.hit_test(point, &parent_t) {
                        return Some(node_id);
                    }
                }
            }
        }
        None
    }

    pub fn hit_test_content(&self, point: Point) -> Option<BlockId> {
        self.hit_test_with_transform(point, Transform::identity())
    }

    pub fn hit_test_rect(&self, rect: Rect) -> Vec<BlockId> {
        self.hit_test_rect_with_transform(rect, Transform::identity())
    }

    fn hit_test_rect_with_transform(&self, rect: Rect, parent_transform: Transform) -> Vec<BlockId> {
        let mut selected = Vec::new();
        let mut stack = Vec::new();

        if let Some(root) = self.blocks.get(self.root) {
            for &child_id in root.children.iter() {
                if let Some(block) = self.blocks.get(child_id) {
                    if block.block_type == BlockType::Content {
                        stack.push((child_id, Transform::identity()));
                    }
                }
            }
        }

        while let Some((node_id, parent_t)) = stack.pop() {
            if let Some(node) = self.blocks.get(node_id) {
                if node.block_type == BlockType::Content {
                    let cumulative_transform = parent_t * node.transform;

                    for &child_id in node.children.iter() {
                        if let Some(child) = self.blocks.get(child_id) {
                            if child.block_type == BlockType::Content {
                                stack.push((child_id, cumulative_transform));
                            }
                        }
                    }

                    let bounds = node.figure.bounds();
                    let corners = [
                        cumulative_transform.multiply_point_2d(DVec2::new(bounds.x, bounds.y)),
                        cumulative_transform.multiply_point_2d(DVec2::new(bounds.x + bounds.width, bounds.y)),
                        cumulative_transform.multiply_point_2d(DVec2::new(bounds.x + bounds.width, bounds.y + bounds.height)),
                        cumulative_transform.multiply_point_2d(DVec2::new(bounds.x, bounds.y + bounds.height)),
                    ];
                    let transformed_bounds = Rect::from_corners(corners[0], corners[2]);
                    if rect_intersects(&rect, &transformed_bounds) {
                        selected.push(node_id);
                    }
                }
            }
        }
        selected
    }

    pub fn select_by_rect(&mut self, rect: Rect) {
        let selected = self.hit_test_rect(rect);
        for block in self.blocks.values_mut() {
            if block.block_type == BlockType::Content {
                block.is_selected = false;
            }
        }
        for id in selected {
            if let Some(block) = self.blocks.get_mut(id) {
                block.is_selected = true;
            }
        }
    }

    pub fn select_single(&mut self, block_id: Option<BlockId>) {
        for block in self.blocks.values_mut() {
            if block.block_type == BlockType::Content {
                block.is_selected = false;
            }
        }
        if let Some(id) = block_id {
            if let Some(block) = self.blocks.get_mut(id) {
                block.is_selected = true;
            }
        }
    }

    pub fn set_selected(&mut self, block_id: Option<BlockId>) {
        self.select_single(block_id);
    }

    pub fn render(&self) -> RenderContext {
        let mut gc = RenderContext::new();
        self.render_to_context(&mut gc);
        gc
    }

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
        let mut viewport_stack = Vec::new();

        viewport_stack.push(false);

        stack.push(self.root);

        while let Some(node_id) = stack.pop() {
            if let Some(runtime_block) = self.blocks.get(node_id) {
                let apply_viewport = viewport_stack.last() == Some(&true)
                    || runtime_block.block_type == BlockType::Content;

                if apply_viewport {
                    gc.push_transform(viewport_transform);
                } else {
                    gc.push_transform(Transform::identity());
                }

                viewport_stack.push(runtime_block.block_type == BlockType::Content);

                runtime_block.paint(gc);

                gc.pop_transform();
                viewport_stack.pop();

                for &child_id in runtime_block.children.iter().rev() {
                    stack.push(child_id);
                }
            }
        }
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

    pub fn get_block(&self, id: BlockId) -> Option<&RuntimeBlock> {
        self.blocks.get(id)
    }

    pub fn translate(&mut self, id: BlockId, dx: f64, dy: f64) {
        if let Some(block) = self.blocks.get_mut(id) {
            block.translate(dx, dy);
        }
    }

    pub fn set_block_transform(&mut self, id: BlockId, transform: Transform) {
        if let Some(block) = self.blocks.get_mut(id) {
            block.set_transform(transform);
        }
    }
}
