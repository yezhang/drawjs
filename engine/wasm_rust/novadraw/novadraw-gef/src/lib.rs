//! Novadraw GEF (Graphical Editing Framework)
//!
//! 编辑框架，提供 EditPart、Command、Tool 等核心概念。
//!
//! # 模块
//!
//! - [`command`] - 命令模式实现
//! - [`editpart`] - 编辑部件
//! - [`editpolicy`] - 编辑策略
//! - [`tool`] - 工具系统

#![allow(missing_docs)]

pub mod command;
pub mod editpart;
pub mod editpolicy;
pub mod tool;

pub use command::{Command, CommandResult, CommandStack};
pub use editpart::{EditPart, GraphicalEditPart, EditPartId};
pub use editpolicy::{EditPolicy, SelectionEditPolicy, DragEditPolicy, EditPolicyResponder};
pub use tool::{Tool, SelectionTool, MarqueeTool, RectangleCreationTool, ToolResult, ToolEvent, ToolState};
