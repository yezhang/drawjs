//! Novadraw Main Library
//!
//! 此库作为所有子库的聚合入口，提供统一的 API。

pub use novadraw_core::Color;
pub use novadraw_geometry::Transform;

#[cfg(feature = "vello")]
pub use novadraw_render::{
    DamageSet, NdCanvas, RenderBackend, RenderCommand, RenderCommandKind, RenderSubmission,
    WindowProxy, command,
};

#[cfg(feature = "vello")]
pub use novadraw_render as render;

#[cfg(feature = "vello")]
pub use novadraw_render::backend;

#[cfg(feature = "vello")]
pub use novadraw_render::traits;

#[cfg(feature = "vello")]
pub use novadraw_scene::{
    BasicEventDispatcher, BlockId, Border, BorderLayout, BorderRegion, Bounded, Direction,
    DispatchContext, EllipseFigure, Event, EventDispatcher, Figure, FigureBlock, FigureGraph,
    FigureRenderer, FillLayout, FlowDirection, FlowLayout, LayoutManager, LineBorder,
    MarginBorder, MouseButton, MouseEvent, MouseEventKind, MutationContext, NovadrawContext,
    NovadrawSystem, PendingMutation, PendingMutations, Point, PolygonFigure, PolylineFigure,
    Rectangle, RectangleBorder, RectangleFigure, RootFigure, RoundedRectangleFigure, SceneHost,
    SceneUpdateManager, Shape, TriangleFigure, UpdateManager, Updatable, Viewport, XYLayout,
};

#[cfg(feature = "vello")]
pub mod border {
    pub use novadraw_scene::border::*;
}
