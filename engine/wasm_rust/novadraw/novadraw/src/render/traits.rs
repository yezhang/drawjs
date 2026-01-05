pub trait WindowProxy: Send + Sync {
    fn request_redraw(&self);
    fn scale_factor(&self) -> f64;
    fn width(&self) -> u32;
    fn height(&self) -> u32;
}

pub trait Renderer {
    type Window: WindowProxy;

    fn window(&self) -> &Self::Window;

    fn render(&mut self, commands: &[crate::render::RenderCommand]);

    fn resize(&mut self, width: u32, height: u32);
}
