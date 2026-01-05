#[cfg(feature = "vello")]
pub mod vello;

#[cfg(feature = "vello")]
pub use vello::{VelloRenderer, WinitWindowProxy};

#[cfg(not(feature = "vello"))]
mod vello {
    use crate::render::traits::{Renderer, WindowProxy};
    use crate::render::RenderCommand;

    pub struct WinitWindowProxy;

    impl WindowProxy for WinitWindowProxy {
        fn request_redraw(&self) {}
        fn scale_factor(&self) -> f64 { 1.0 }
        fn width(&self) -> u32 { 0 }
        fn height(&self) -> u32 { 0 }
    }

    pub struct VelloRenderer;

    impl Renderer for VelloRenderer {
        type Window = WinitWindowProxy;

        fn window(&self) -> &Self::Window {
            &WinitWindowProxy
        }

        fn render(&mut self, _commands: &[RenderCommand]) {}
        fn resize(&mut self, _width: u32, _height: u32) {}
    }
}
