use std::sync::Arc;

use novadraw::{BlockId, Color, FillLayout, RectangleFigure, SceneGraph, Viewport};

pub struct SceneManager {
    pub scene: SceneGraph,
    pub viewport: Viewport,
    pub background_id: Option<BlockId>,
}

impl SceneManager {
    pub fn new() -> Self {
        let mut scene = SceneGraph::new();

        scene.set_layout_manager(Arc::new(FillLayout::new()));

        let bg = RectangleFigure::new_with_color(
            0.0, 0.0, 800.0, 600.0,
            Color::rgba(1.0, 1.0, 1.0, 1.0),
        );
        let bg_id = scene.new_ui_block(Box::new(bg));

        Self {
            scene,
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
        let container_bounds = novadraw::Rect::new(0.0, 0.0, width, height);
        self.scene.apply_layout(container_bounds);
    }

    pub fn scene(&self) -> &SceneGraph {
        &self.scene
    }

    pub fn scene_mut(&mut self) -> &mut SceneGraph {
        &mut self.scene
    }

    pub fn add_rectangle(&mut self, x: f64, y: f64, width: f64, height: f64, color: Color) -> BlockId {
        println!("[SceneManager::add_rectangle] x={}, y={}, w={}, h={}", x, y, width, height);
        let rect = RectangleFigure::new_with_color(x, y, width, height, color);
        self.scene.new_content_block(Box::new(rect))
    }

    #[allow(dead_code)]
    pub fn debug_background_size(&self) -> (f64, f64) {
        if let Some(bg_id) = self.background_id {
            if let Some(block) = self.scene.blocks.get(bg_id) {
                if let Some(rect) = block.figure.as_rectangle() {
                    return (rect.width, rect.height);
                }
            }
        }
        (0.0, 0.0)
    }

    pub fn add_rectangle_at_center(&mut self, center_x: f64, center_y: f64, width: f64, height: f64, color: Color) -> BlockId {
        let x = center_x - width / 2.0;
        let y = center_y - height / 2.0;
        self.add_rectangle(x, y, width, height, color)
    }
}

impl Default for SceneManager {
    fn default() -> Self {
        Self::new()
    }
}
