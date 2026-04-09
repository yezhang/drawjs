use novadraw_render::{NdCanvas, RenderBackend};

use crate::{EventDispatcher, FigureGraph, UpdateManager};

pub trait NovadrawSystem: Send + Sync {
    fn scene(&mut self) -> &mut FigureGraph;
    fn update_manager(&mut self) -> &mut dyn UpdateManager;
    fn dispatcher(&mut self) -> &mut dyn EventDispatcher;
    fn render(&mut self, renderer: &mut impl RenderBackend) -> NdCanvas;
    fn viewport_size(&self) -> (f64, f64);
    fn request_update(&self);
}
