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

#![allow(missing_docs)]

pub mod border;
pub mod figure;
pub mod layout;
pub mod scene;
pub mod viewport;

pub use border::{Border, LineBorder, MarginBorder, RectangleBorder};
pub use figure::{Bounded, EllipseFigure, Figure, PolygonFigure, PolylineFigure, RectangleFigure, RootFigure, RoundedRectangleFigure};
pub use layout::{FillLayout, LayoutManager, XYLayout};
pub use novadraw_geometry::{Point, Rectangle};
pub use scene::{BlockId, FigureRenderer, RuntimeBlock, SceneGraph};
pub use viewport::Viewport;
