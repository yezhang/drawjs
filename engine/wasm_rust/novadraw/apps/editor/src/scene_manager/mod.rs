use novadraw::{BlockId, Color, RectangleFigure, SceneGraph};

pub struct SceneManager {
    scene: SceneGraph,
    active_tool: Tool,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Tool {
    Select,
    Rectangle,
    Circle,
}

impl SceneManager {
    pub fn new() -> Self {
        let mut scene = SceneGraph::new();

        let bg = RectangleFigure::new_with_color(
            0.0, 0.0, 2000.0, 2000.0,
            Color::rgba(1.0, 1.0, 1.0, 1.0),
        );
        scene.new_block(None, Box::new(bg));

        Self {
            scene,
            active_tool: Tool::Select,
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

    pub fn set_tool(&mut self, tool: Tool) {
        self.active_tool = tool;
    }

    pub fn add_rectangle(&mut self, x: f64, y: f64, width: f64, height: f64, color: Color) -> BlockId {
        let rect = RectangleFigure::new_with_color(x, y, width, height, color);
        self.scene.new_block(None, Box::new(rect))
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
