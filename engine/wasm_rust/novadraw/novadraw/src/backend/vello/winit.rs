use std::sync::Arc;
use winit::window::Window;

use crate::render::traits::WindowProxy;

pub struct WinitWindowProxy {
    window: Arc<Window>,
}

impl WinitWindowProxy {
    pub fn new(window: Arc<Window>) -> Self {
        Self { window }
    }

    pub fn window(&self) -> &Arc<Window> {
        &self.window
    }

    pub fn clone_window(&self) -> Arc<Window> {
        Arc::clone(&self.window)
    }
}

pub type WinitWindowProxyInner = WinitWindowProxy;

impl WindowProxy for WinitWindowProxy {
    fn request_redraw(&self) {
        self.window.request_redraw();
    }

    fn scale_factor(&self) -> f64 {
        self.window.scale_factor()
    }

    fn width(&self) -> u32 {
        self.window.inner_size().width
    }

    fn height(&self) -> u32 {
        self.window.inner_size().height
    }
}
