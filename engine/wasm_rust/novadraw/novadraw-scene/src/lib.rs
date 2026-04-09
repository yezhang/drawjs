//! Novadraw Scene Library
//!
//! 场景图库，提供 Figure 渲染和场景图管理。
//!
//! # 模块
//!
//! - [`figure`] - Figure 渲染接口和实现
//! - [`border`] - Border 边框系统
//! - [`scene`] - 场景图管理
//! - [`viewport`] - 视口管理
//! - [`update`] - 更新管理器

#![allow(missing_docs)]

pub mod border;
pub mod context;
pub mod event;
pub mod figure;
pub mod layout;
pub mod log;
pub mod mutation;
pub mod scene;
pub mod scene_host;
pub mod system;
pub mod update;
pub mod viewport;

pub use border::{Border, LineBorder, MarginBorder, RectangleBorder};
pub use context::NovadrawContext;
pub use event::{
    BasicEventDispatcher, DispatchContext, Event, EventDispatcher, MouseButton, MouseEvent,
    MouseEventKind,
};
pub use figure::{
    Bounded, Direction, EllipseFigure, Figure, PolygonFigure, PolylineFigure, RectangleFigure,
    RootFigure, RoundedRectangleFigure, Shape, TriangleFigure, Updatable,
};
pub use mutation::{MutationContext, PendingMutation, PendingMutations};
pub use layout::{
    BorderLayout, BorderRegion, FillLayout, FlowDirection, FlowLayout, LayoutManager, XYLayout,
};
pub use novadraw_geometry::{Point, Rectangle};
pub use scene::{BlockId, FigureBlock, FigureRenderer, FigureGraph};
pub use scene_host::SceneHost;
pub use system::NovadrawSystem;
pub use update::{SceneUpdateManager, UpdateEvent, UpdateListener, UpdateManager};
pub use viewport::Viewport;
