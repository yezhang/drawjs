//! Novadraw Main Library
//!
//! 此库作为所有子库的聚合入口，提供统一的 API。

pub use novadraw_core::Color;
pub use novadraw_math::Transform;

#[cfg(feature = "vello")]
pub use novadraw_render::{NdCanvas, NdState, NdStateSnapshot, RenderCommand, RenderCommandKind, Renderer, WindowProxy};

#[cfg(feature = "vello")]
pub use novadraw_render as render;

#[cfg(feature = "vello")]
pub use novadraw_render::backend;

#[cfg(feature = "vello")]
pub use novadraw_render::traits;

#[cfg(feature = "vello")]
pub use novadraw_scene::{BaseFigure, BlockId, Figure, FigureRenderer, FillLayout, LayoutManager, PaintTask, Point, Rectangle, RectangleFigure, Rect, RuntimeBlock, SceneGraph, Viewport, ViewportFigure, XYLayout};
