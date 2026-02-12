//! Novadraw 演示应用公共库
//!
//! 提供通用的应用框架，简化演示应用的开发。
//!
//! # 使用示例
//!
//! ```rust
//! use novadraw_apps::{run_demo_app, scene_creator};
//!
//! #[scene_creator]
//! fn create_scene() -> novadraw::SceneGraph {
//!     let mut scene = novadraw::SceneGraph::new();
//!     // 创建场景...
//!     scene
//! }
//!
//! fn main() {
//!     run_demo_app("My App", vec![
//!         ("Scene 1", create_scene),
//!     ]);
//! }
//! ```

pub mod app;
pub mod prelude;

pub use app::{run_demo_app, AppBuilder, DemoApp};
pub use prelude::*;
