//! Novadraw 应用预导入模块
//!
//! 导入常用的类型和函数，方便快速开发。

pub use crate::{run_demo_app, AppBuilder, DemoApp};
pub use novadraw::{
    Color, EllipseFigure, Figure, LineFigure, Rectangle, RectangleFigure, SceneGraph,
};
pub use winit::event::ElementState;
