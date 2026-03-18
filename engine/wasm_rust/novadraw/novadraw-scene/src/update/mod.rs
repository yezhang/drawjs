//! Update Manager - 更新管理
//!
//! 管理场景图的更新流程，包括：
//! - 脏区域（dirty region）跟踪：记录需要重绘的区域
//! - 失效块（invalid block）队列：记录需要重新布局的块
//! - 两阶段更新：先布局（validation），再重绘（repaint）
//!
//! 参考 Eclipse Draw2D 的 DeferredUpdateManager 设计。
//!
//! # 更新流程
//!
//! ```text
//! repaint() ──────► add_dirty_region() ──► 脏区域队列
//!                                                          │
//! revalidate() ──► add_invalid_figure() ──► 失效块队列   │
//!                                                          ▼
//!                                             perform_update()
//!                                                  │
//!                    ┌──────────────────────────────┼──────────────────────────────┐
//!                    ▼                              ▼                              ▼
//!              Phase 1: Layout            Phase 2: Union Dirty Regions    Phase 3: Repaint
//!              (布局失效的块)              (合并所有脏区域)                (重绘脏区域)
//! ```

mod deferred;
mod listener;

pub use deferred::SceneUpdateManager;
pub use listener::{UpdateEvent, UpdateListener};
