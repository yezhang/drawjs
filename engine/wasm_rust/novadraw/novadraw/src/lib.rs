//! Novadraw Main Library
//!
//! 此库作为所有子库的聚合入口，提供统一的 API。

pub use novadraw_core::Color;
pub use novadraw_geometry::Transform;

#[cfg(feature = "vello")]
pub use novadraw_render::{NdCanvas, RenderCommand, RenderCommandKind, RenderBackend, WindowProxy};

#[cfg(feature = "vello")]
pub use novadraw_render as render;

#[cfg(feature = "vello")]
pub use novadraw_render::backend;

#[cfg(feature = "vello")]
pub use novadraw_render::traits;

#[cfg(feature = "vello")]
pub use novadraw_scene::{BaseFigure, BlockId, Figure, FigureRenderer, FillLayout, LayoutManager, Point, Rectangle, RectangleFigure, RuntimeBlock, SceneGraph, Viewport, XYLayout};
