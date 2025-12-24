mod block;
mod color;
mod render_ctx;
mod render_ir;
mod vello_renderer;

// API 重新导出
pub use block::SceneGraph;
pub use render_ctx::RenderContext;

pub use vello_renderer::VelloRenderer as Renderer;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
