use serde::{Deserialize, Serialize};
use slotmap::{SlotMap, new_key_type};
use std::collections::HashMap;
use uuid::Uuid;

use crate::color::Color;
use crate::render_ctx;
use crate::transform::Transform;
use glam::DVec2;

new_key_type! { pub struct BlockId; }

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

pub trait Paint {
    fn bounds(&self) -> Rect;
    fn hit_test(&self, point: Point) -> bool {
        self.bounds().contains(point)
    }
    fn paint(&self, gc: &mut render_ctx::RenderContext);
    fn paint_highlight(&self, gc: &mut render_ctx::RenderContext) {
        let bounds = self.bounds();
        let origin = gc.transform_point(Point::new(bounds.x, bounds.y));
        gc.set_fill_style(Color::rgba(0.0, 0.0, 0.0, 0.0));
        gc.set_stroke_style(Color::hex("#f39c12"), 2.0);
        gc.draw_stroke_rect(origin.x - 2.0, origin.y - 2.0, bounds.width + 4.0, bounds.height + 4.0);
    }
    fn as_rectangle_mut(&mut self) -> Option<&mut RectangleFigure> {
        None
    }
}

pub struct NullFigure;

impl NullFigure {
    pub fn new() -> Self {
        NullFigure {}
    }
}

impl Paint for NullFigure {
    fn bounds(&self) -> Rect {
        Rect::new(0.0, 0.0, 0.0, 0.0)
    }
    fn paint(&self, _gc: &mut render_ctx::RenderContext) {}
}

pub struct RectangleFigure {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub fill_color: Color,
    pub stroke_color: Option<Color>,
    pub stroke_width: f64,
}

impl RectangleFigure {
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self {
            x, y, width, height,
            fill_color: Color::hex("#3498db"),
            stroke_color: None,
            stroke_width: 0.0,
        }
    }

    pub fn new_with_color(x: f64, y: f64, width: f64, height: f64, color: Color) -> Self {
        Self {
            x, y, width, height,
            fill_color: color,
            stroke_color: None,
            stroke_width: 0.0,
        }
    }

    pub fn with_stroke(mut self, color: Color, width: f64) -> Self {
        self.stroke_color = Some(color);
        self.stroke_width = width;
        self
    }

    pub fn translate(&mut self, dx: f64, dy: f64) {
        self.x += dx;
        self.y += dy;
    }
}

impl Paint for RectangleFigure {
    fn bounds(&self) -> Rect {
        Rect::new(self.x, self.y, self.width, self.height)
    }

    fn paint(&self, gc: &mut render_ctx::RenderContext) {
        let origin = gc.transform_point(Point::new(self.x, self.y));
        gc.set_fill_style(self.fill_color);
        gc.draw_rect(origin.x, origin.y, self.width, self.height);

        if let Some(color) = self.stroke_color {
            gc.set_stroke_style(color, self.stroke_width);
            gc.draw_stroke_rect(origin.x, origin.y, self.width, self.height);
        }
    }

    fn as_rectangle_mut(&mut self) -> Option<&mut RectangleFigure> {
        Some(self)
    }
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
    pub figure: Box<dyn Paint>,
    pub transform: Transform,
    pub is_selected: bool,
}

impl RuntimeBlock {
    fn paint(&self, gc: &mut render_ctx::RenderContext) {
        gc.push_transform(self.transform);
        self.figure.paint(gc);
        if self.is_selected && self.block_type == BlockType::Content {
            self.figure.paint_highlight(gc);
        }
        gc.pop_transform();
    }

    pub fn set_figure(&mut self, figure: Box<dyn Paint>) {
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

    pub fn as_rectangle_mut(&mut self) -> Option<&mut RectangleFigure> {
        self.figure.as_rectangle_mut()
    }
}

#[derive(Serialize, Deserialize)]
struct SerializedBlock {
    uuid: Uuid,
    children: Vec<Uuid>,
}

pub struct SceneGraph {
    pub blocks: SlotMap<BlockId, RuntimeBlock>,
    pub uuid_map: HashMap<Uuid, BlockId>,
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
            figure: Box::new(NullFigure::new()),
            transform: Transform::identity(),
            is_selected: false,
        });

        SceneGraph {
            blocks,
            uuid_map: HashMap::new(),
            root: root_id,
        }
    }

    pub fn new_content_block(&mut self, figure: Box<dyn Paint>) -> BlockId {
        self.new_content_block_with_transform(figure, Transform::identity())
    }

    pub fn new_content_block_with_transform(&mut self, figure: Box<dyn Paint>, transform: Transform) -> BlockId {
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

    pub fn new_content_block_with_parent(
        &mut self,
        parent_id: BlockId,
        figure: Box<dyn Paint>,
    ) -> BlockId {
        let uuid = Uuid::new_v4();
        let id = self.blocks.insert_with_key(|key| RuntimeBlock {
            id: key,
            uuid,
            block_type: BlockType::Content,
            children: Vec::new(),
            parent: Some(parent_id),
            figure,
            transform: Transform::identity(),
            is_selected: false,
        });
        self.uuid_map.insert(uuid, id);
        if let Some(parent) = self.blocks.get_mut(parent_id) {
            parent.children.push(id);
        }
        id
    }

    pub fn new_ui_block(&mut self, figure: Box<dyn Paint>) -> BlockId {
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

    pub fn hit_test_content(&self, point: Point) -> Option<BlockId> {
        self.hit_test_with_transform(point, Transform::identity())
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
                    let center = cumulative_transform.multiply_point_2d(bounds.center());
                    let world_rect = Rect::new(
                        center.x - bounds.width / 2.0,
                        center.y - bounds.height / 2.0,
                        bounds.width,
                        bounds.height,
                    );
                    if rect_intersects(&rect, &world_rect) {
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

    pub fn render(&self) -> render_ctx::RenderContext {
        let mut gc = render_ctx::RenderContext::new();
        self.render_to_context(&mut gc);
        gc
    }

    pub fn render_with_viewport(&self, viewport_transform: Transform) -> render_ctx::RenderContext {
        let mut gc = render_ctx::RenderContext::new();
        self.render_to_context_with_viewport(&mut gc, viewport_transform);
        gc
    }

    fn render_to_context(&self, gc: &mut render_ctx::RenderContext) {
        self.traverse_dfs_stack(|block_id| {
            if let Some(runtime_block) = self.blocks.get(block_id) {
                runtime_block.paint(gc);
            }
        })
    }

    fn render_to_context_with_viewport(&self, gc: &mut render_ctx::RenderContext, _viewport_transform: Transform) {
        let mut stack = Vec::new();
        let mut viewport_stack = Vec::new();

        viewport_stack.push(false);

        stack.push(self.root);

        while let Some(node_id) = stack.pop() {
            if let Some(runtime_block) = self.blocks.get(node_id) {
                let apply_viewport = viewport_stack.last() == Some(&true)
                    || runtime_block.block_type == BlockType::Content;

                if apply_viewport {
                    gc.push_transform(Transform::identity());
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

    pub fn traverse_dfs_stack<F>(&self, mut visitor: F)
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

    pub fn new_block(&mut self, parent_id: Option<BlockId>, figure: Box<dyn Paint>) -> BlockId {
        let uuid = Uuid::new_v4();

        let id = self.blocks.insert_with_key(|key| RuntimeBlock {
            id: key,
            uuid,
            block_type: BlockType::Content,
            children: Vec::new(),
            parent: parent_id,
            figure,
            transform: Transform::identity(),
            is_selected: false,
        });

        self.uuid_map.insert(uuid, id);

        match parent_id {
            Some(pid) => {
                if let Some(parent) = self.blocks.get_mut(pid) {
                    parent.children.push(id);
                }
            }
            None => {
                self.blocks[self.root].children.push(id);
            }
        }

        id
    }

    pub fn hit_test(&self, point: Point) -> Option<BlockId> {
        self.hit_test_content(point)
    }

    pub fn set_selected(&mut self, block_id: Option<BlockId>) {
        self.select_single(block_id);
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
