//! Novadraw Math Library
//!
//! 纯数学库，提供基础数学类型。
//!
//! # 模块
//!
//! - [`vec3`] - 3D 向量类型
//! - [`mat3`] - 3x3 矩阵类型

#![deny(missing_docs)]

pub mod vec3;
pub mod mat3;

pub use vec3::Vec3;
pub use mat3::Mat3;
