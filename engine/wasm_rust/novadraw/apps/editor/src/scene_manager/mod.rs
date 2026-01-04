use novadraw::{BlockId, Color, RectangleFigure, SceneGraph, Viewport};
use glam::DVec2;

pub struct SceneManager {
    scene: SceneGraph,
    pub active_tool: Tool,
    selection_box: Option<BlockId>,
    hovered_block: Option<BlockId>,
    viewport: Viewport,
    background_id: Option<BlockId>,
}

#[derive(Clone, Copy, PartialEq)]
#[allow(dead_code)]
pub enum Tool {
    Select,
    Rectangle,
    Circle,
}

impl SceneManager {
    pub fn new() -> Self {
        let mut scene = SceneGraph::new();

        let bg = RectangleFigure::new_with_color(
            0.0, 0.0, 800.0, 600.0,
            Color::rgba(1.0, 1.0, 1.0, 1.0),
        );
        let bg_id = scene.new_ui_block(Box::new(bg));

        Self {
            scene,
            active_tool: Tool::Select,
            selection_box: None,
            hovered_block: None,
            viewport: Viewport::new(),
            background_id: Some(bg_id),
        }
    }

    pub fn viewport(&self) -> &Viewport {
        &self.viewport
    }

    pub fn viewport_mut(&mut self) -> &mut Viewport {
        &mut self.viewport
    }

    pub fn set_window_size(&mut self, width: f64, height: f64) {
        if let Some(bg_id) = self.background_id {
            if let Some(block) = self.scene.blocks.get_mut(bg_id) {
                if let Some(rect) = block.figure.as_rectangle_mut() {
                    rect.width = width;
                    rect.height = height;
                }
            }
        }
    }

    pub fn scene(&self) -> &SceneGraph {
        &self.scene
    }

    pub fn scene_mut(&mut self) -> &mut SceneGraph {
        &mut self.scene
    }

    pub fn active_tool(&self) -> Tool {
        self.active_tool
    }

    #[allow(dead_code)]
    pub fn set_tool(&mut self, tool: Tool) {
        self.active_tool = tool;
    }

    #[allow(dead_code)]
    pub fn hovered_block(&self) -> Option<BlockId> {
        self.hovered_block
    }

    pub fn set_hovered(&mut self, block_id: Option<BlockId>) {
        self.hovered_block = block_id;
    }

    pub fn add_rectangle(&mut self, x: f64, y: f64, width: f64, height: f64, color: Color) -> BlockId {
        let rect = RectangleFigure::new_with_color(x, y, width, height, color);
        self.scene.new_content_block(Box::new(rect))
    }

    pub fn add_rectangle_at_center(&mut self, center_x: f64, center_y: f64, width: f64, height: f64, color: Color) -> BlockId {
        let x = center_x - width / 2.0;
        let y = center_y - height / 2.0;
        self.add_rectangle(x, y, width, height, color)
    }

    pub fn start_selection_box(&mut self, pos: DVec2) {
        let rect = RectangleFigure::new_with_color(
            pos.x, pos.y, 0.0, 0.0,
            Color::rgba(0.2, 0.6, 0.86, 0.3),
        ).with_stroke(Color::rgba(0.2, 0.6, 0.86, 1.0), 1.0);
        let id = self.scene.new_ui_block(Box::new(rect));
        self.selection_box = Some(id);
    }

    pub fn update_selection_box(&mut self, end: DVec2) {
        if let Some(id) = self.selection_box {
            if let Some(block) = self.scene.blocks.get_mut(id) {
                if let Some(rect) = block.figure.as_rectangle_mut() {
                    let start = DVec2::new(rect.x, rect.y);
                    let x = start.x.min(end.x);
                    let y = start.y.min(end.y);
                    let width = (end.x - start.x).abs();
                    let height = (end.y - start.y).abs();
                    rect.x = x;
                    rect.y = y;
                    rect.width = width;
                    rect.height = height;
                }
            }
        }
    }

    pub fn end_selection_box(&mut self, end: DVec2) {
        let start = if let Some(id) = self.selection_box {
            if let Some(block) = self.scene.blocks.get_mut(id) {
                if let Some(rect) = block.figure.as_rectangle_mut() {
                    Some(DVec2::new(rect.x, rect.y))
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        };

        if let Some(start_pos) = start {
            let x = start_pos.x.min(end.x);
            let y = start_pos.y.min(end.y);
            let width = (end.x - start_pos.x).abs();
            let height = (end.y - start_pos.y).abs();

            self.scene.select_by_rect(novadraw::Rect::new(x, y, width, height));
        }

        self.remove_selection_box();
    }

    pub fn remove_selection_box(&mut self) {
        if let Some(id) = self.selection_box {
            self.scene.blocks.remove(id);
            self.selection_box = None;
        }
    }

    pub fn create_temp_rectangle(&mut self, pos: DVec2) -> BlockId {
        let rect = RectangleFigure::new_with_color(
            pos.x, pos.y, 0.0, 0.0,
            Color::rgba(0.2, 0.6, 0.86, 0.5),
        );
        let id = self.scene.new_ui_block(Box::new(rect));
        id
    }

    pub fn update_temp_rectangle(&mut self, id: BlockId, start: DVec2, end: DVec2) {
        if let Some(block) = self.scene.blocks.get_mut(id) {
            if let Some(rect) = block.figure.as_rectangle_mut() {
                let x = start.x.min(end.x);
                let y = start.y.min(end.y);
                let width = (end.x - start.x).abs();
                let height = (end.y - start.y).abs();
                rect.x = x;
                rect.y = y;
                rect.width = width;
                rect.height = height;
            }
        }
    }

    pub fn finalize_temp_rectangle(&mut self, id: BlockId) {
        if let Some(block) = self.scene.blocks.get_mut(id) {
            if let Some(rect) = block.figure.as_rectangle_mut() {
                rect.fill_color = Color::hex("#3498db");
            }
        }
    }
}

impl Default for SceneManager {
    fn default() -> Self {
        Self::new()
    }
}
