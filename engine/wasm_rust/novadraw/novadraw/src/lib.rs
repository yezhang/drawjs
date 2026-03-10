//! Novadraw Main Library
//!
//! 此库作为所有子库的聚合入口，提供统一的 API。

pub use novadraw_core::Color;
pub use novadraw_geometry::Transform;

#[cfg(feature = "vello")]
pub use novadraw_render::{NdCanvas, RenderCommand, RenderCommandKind, RenderBackend, WindowProxy, command};

#[cfg(feature = "vello")]
pub use novadraw_render as render;

#[cfg(feature = "vello")]
pub use novadraw_render::backend;

#[cfg(feature = "vello")]
pub use novadraw_render::traits;

#[cfg(feature = "vello")]
pub use novadraw_scene::{Bounded, Border, BlockId, EllipseFigure, Figure, FigureRenderer, FillLayout, LayoutManager, LineBorder, MarginBorder, Point, PolygonFigure, PolylineFigure, Rectangle, RectangleBorder, RectangleFigure, RootFigure, RuntimeBlock, SceneGraph, Viewport, XYLayout};
