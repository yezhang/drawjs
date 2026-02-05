//! Novadraw Geometry Library
//!
//! 几何库，提供 2D 图形所需的几何类型。
//!
//! # 模块
//!
//! - [`vec2`] - 2D 向量类型
//! - [`rect`] - 矩形、点、尺寸类型
//! - [`transform`] - 仿射变换

#![deny(missing_docs)]

pub mod vec2;
pub mod rect;
pub mod transform;

pub use vec2::Vec2;
pub use rect::{Point, Rectangle, Size};
pub use transform::Transform;
