use serde::{Deserialize, Serialize};
use slotmap::{SlotMap, new_key_type};
use std::collections::HashMap;
use uuid::Uuid;

use crate::color::Color;
use crate::render_ctx;
use glam::DVec2;

// 1. 定义运行时的高速 ID (SlotKey)
new_key_type! { pub struct BlockId; }

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

    pub fn contains(&self, point: DVec2) -> bool {
        point.x >= self.x && point.x <= self.x + self.width
            && point.y >= self.y && point.y <= self.y + self.height
    }

    pub fn center(&self) -> DVec2 {
        DVec2::new(self.x + self.width / 2.0, self.y + self.height / 2.0)
    }
}

pub trait Paint {
    fn bounds(&self) -> Rect;
    fn hit_test(&self, point: DVec2) -> bool {
        self.bounds().contains(point)
    }
    fn paint(&self, gc: &mut render_ctx::RenderContext);
    fn paint_highlight(&self, gc: &mut render_ctx::RenderContext) {
        let bounds = self.bounds();
        gc.set_fill_style(Color::rgba(0.0, 0.0, 0.0, 0.0));
        gc.set_stroke_style(Color::hex("#f39c12"), 2.0);
        gc.draw_stroke_rect(bounds.x, bounds.y, bounds.width, bounds.height);
    }
    fn as_rectangle_mut(&mut self) -> Option<&mut RectangleFigure> {
        None
    }
}

pub trait Interactive {
    fn on_mouse_enter(&mut self);
    fn on_mouse_leave(&mut self);
    fn on_mouse_move(&mut self, x: f64, y: f64);
    fn on_mouse_press(&mut self, x: f64, y: f64);
    fn on_mouse_release(&mut self, x: f64, y: f64);
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
            x,
            y,
            width,
            height,
            fill_color: Color::hex("#3498db"),
            stroke_color: None,
            stroke_width: 0.0,
        }
    }

    pub fn new_with_color(x: f64, y: f64, width: f64, height: f64, color: Color) -> Self {
        Self {
            x,
            y,
            width,
            height,
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
        gc.set_fill_style(self.fill_color);
        gc.draw_rect(self.x, self.y, self.width, self.height);

        if let Some(color) = self.stroke_color {
            gc.set_stroke_style(color, self.stroke_width);
            gc.draw_stroke_rect(self.x, self.y, self.width, self.height);
        }
    }

    fn paint_highlight(&self, gc: &mut render_ctx::RenderContext) {
        let bounds = self.bounds();
        gc.set_fill_style(Color::rgba(0.0, 0.0, 0.0, 0.0));
        gc.set_stroke_style(Color::hex("#f39c12"), 3.0);
        gc.draw_stroke_rect(bounds.x - 2.0, bounds.y - 2.0, bounds.width + 4.0, bounds.height + 4.0);
    }

    fn as_rectangle_mut(&mut self) -> Option<&mut RectangleFigure> {
        Some(self)
    }
}

pub struct RuntimeBlock {
    pub id: BlockId,
    pub uuid: Uuid,
    pub children: Vec<BlockId>,
    pub parent: Option<BlockId>,
    pub figure: Box<dyn Paint>,
    pub is_hovered: bool,
    pub is_selected: bool,
}

impl RuntimeBlock {
    fn paint(&self, gc: &mut render_ctx::RenderContext) {
        self.figure.paint(gc);
        if self.is_hovered || self.is_selected {
            self.figure.paint_highlight(gc);
        }
    }

    pub fn set_figure(&mut self, figure: Box<dyn Paint>) {
        self.figure = figure;
    }

    pub fn hit_test(&self, point: DVec2) -> bool {
        self.figure.hit_test(point)
    }

    pub fn translate(&mut self, dx: f64, dy: f64) {
        if let Some(rect) = self.figure.as_rectangle_mut() {
            rect.translate(dx, dy);
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
            children: Vec::new(),
            parent: None,
            figure: Box::new(NullFigure::new()),
            is_hovered: false,
            is_selected: false,
        });

        SceneGraph {
            blocks,
            uuid_map: HashMap::new(),
            root: root_id,
        }
    }

    pub fn render(&self) -> render_ctx::RenderContext {
        let mut gc = render_ctx::RenderContext::new();
        self.render_to_context(&mut gc);
        gc
    }

    fn render_to_context(&self, gc: &mut render_ctx::RenderContext) {
        self.traverse_dfs_stack(|block_id| {
            if let Some(runtime_block) = self.blocks.get(block_id) {
                runtime_block.paint(gc);
            }
        })
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
            children: Vec::new(),
            parent: parent_id,
            figure,
            is_hovered: false,
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

    pub fn hit_test(&self, point: DVec2) -> Option<BlockId> {
        let mut stack = Vec::new();

        // 先把 root 的所有子节点入栈
        if let Some(root) = self.blocks.get(self.root) {
            for &child_id in root.children.iter() {
                stack.push(child_id);
            }
        }

        while let Some(node_id) = stack.pop() {
            if let Some(node) = self.blocks.get(node_id) {
                // 先push子节点（后进先出，保证上层元素先被检查）
                for &child_id in node.children.iter() {
                    stack.push(child_id);
                }

                // 再检查当前节点
                if node.hit_test(point) {
                    return Some(node_id);
                }
            }
        }
        None
    }

    pub fn set_hovered(&mut self, block_id: Option<BlockId>) {
        // 清除所有 hover 状态
        for block in self.blocks.values_mut() {
            block.is_hovered = false;
        }
        // 设置新的 hover 状态
        if let Some(id) = block_id {
            if let Some(block) = self.blocks.get_mut(id) {
                block.is_hovered = true;
            }
        }
    }

    pub fn set_selected(&mut self, block_id: Option<BlockId>) {
        // 清除所有 selected 状态
        for block in self.blocks.values_mut() {
            block.is_selected = false;
        }
        // 设置新的 selected 状态
        if let Some(id) = block_id {
            if let Some(block) = self.blocks.get_mut(id) {
                block.is_selected = true;
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
}
