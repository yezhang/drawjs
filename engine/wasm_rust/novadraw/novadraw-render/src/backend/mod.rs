//! Vello 渲染后端
//!
//! 使用 Vello GPU 渲染器实现渲染接口。
//!
//! # 示例
//!
//! ```ignore
//! use novadraw_render::backend::vello::VelloRenderer;
//! ```

#[cfg(feature = "vello")]
pub mod vello;
