//! Novadraw Math Library
//!
//! 纯数学库，提供 2D 图形所需的数学类型。
//!
//! # 模块
//!
//! - [`vec2`] - 2D 向量类型
//! - [`vec3`] - 3D 向量类型
//! - [`mat3`] - 3x3 矩阵类型
//! - [`rect`] - 矩形、点、尺寸类型
//! - [`transform`] - 仿射变换

#![deny(missing_docs)]

pub mod vec2;
pub mod vec3;
pub mod mat3;
pub mod rect;
pub mod transform;

pub use vec2::Vec2;
pub use vec3::Vec3;
pub use mat3::Mat3;
pub use rect::{Point, Rect, Size};
pub use transform::{AffineTransform, Transform};
