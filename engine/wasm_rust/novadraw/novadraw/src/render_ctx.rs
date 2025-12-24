use crate::render_ir::RenderCommand;

pub struct RenderContext {
    pub commands: Vec<RenderCommand>,
}

impl RenderContext {
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
        }
    }

    pub fn set_fill_style(&mut self, style: &String) {}

    pub fn draw_rect(&mut self, x: f64, y: f64, width: f64, height: f64) {
        println!("draw_rect");
    }
}
