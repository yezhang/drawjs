//! Novadraw Scene Library
//!
//! 场景图库，提供 Figure 渲染和场景图管理。
//!
//! # 模块
//!
//! - [`figure`] - Figure 渲染接口和实现
//! - [`scene`] - 场景图管理
//! - [`viewport`] - 视口管理

#![allow(missing_docs)]

pub mod figure;
pub mod layout;
pub mod scene;
pub mod viewport;

pub use figure::{BaseFigure, Figure, Rectangle, RectangleFigure, ViewportFigure};
pub use layout::{FillLayout, LayoutManager, XYLayout};
pub use scene::{BlockId, FigureRenderer, PaintTask, Point, Rect, RuntimeBlock, SceneGraph};
pub use viewport::Viewport;
