use std::sync::Arc;

use crate::render_ir::RenderCommand;
use vello::util::{RenderContext, RenderSurface};
use vello::{
    kurbo::{Affine, Circle},
    peniko::{Color, Fill},
    *,
};
use winit::window::{Window, WindowAttributes};

pub struct VelloRenderer {
    cached_window: Option<Arc<Window>>,
}

impl VelloRenderer {
    pub fn new() -> Self {
        VelloRenderer {
            cached_window: None,
        }
    }

    pub fn submit_commands(&self, cmds: &[RenderCommand]) {
        // let render_cx = RenderContext::new();
        // let size = winit::dpi::PhysicalSize::from_logical::<_, f64>((width, height), scale_factor);
        // _ = window.request_inner_size(size);
        // let surface = render_cx.create_surface(window, width, height, present_mode)
        println!("执行了渲染")
    }
}
