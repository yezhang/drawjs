use glam::{DVec2, DVec3};

pub enum RenderCommand {
    FillRect {
        rect: [DVec2; 2], // min, max
        //color: Color,
        corner_radius: f64,
    }
}
