//! 编辑策略
//!
//! EditPolicy 定义了 EditPart 的行为策略，如选择、移动、调整大小等。

use crate::editpart::{EditPart, EditPartId};
use crate::command::Command;
use novadraw_scene::SceneGraph;
use novadraw_math::Transform;
use glam::DVec2;

/// 编辑策略 trait
///
/// 定义 EditPart 的可编辑行为。
pub trait EditPolicy: Send + Sync {
    /// 获取策略标识
    fn get_key(&self) -> &str;
    /// 激活策略
    fn activate(&mut self, _part: &mut dyn EditPart, _scene: &mut SceneGraph) {}
    /// 停用策略
    fn deactivate(&mut self, _part: &mut dyn EditPart, _scene: &mut SceneGraph) {}
}

/// 选择策略
///
/// 处理部件的选中/取消选中。
#[derive(Debug, Clone)]
pub struct SelectionEditPolicy {
    key: &'static str,
    primary_handle: Option<DVec2>,
}

impl SelectionEditPolicy {
    pub fn new() -> Self {
        Self {
            key: "SelectionEditPolicy",
            primary_handle: None,
        }
    }

    pub fn with_handle(mut self, handle: DVec2) -> Self {
        self.primary_handle = Some(handle);
        self
    }
}

impl EditPolicy for SelectionEditPolicy {
    fn get_key(&self) -> &str {
        self.key
    }
}

/// 移动策略
///
/// 允许部件被拖动。
#[derive(Debug, Clone)]
pub struct DragEditPolicy {
    key: &'static str,
    original_position: Option<Transform>,
}

impl DragEditPolicy {
    pub fn new() -> Self {
        Self {
            key: "DragEditPolicy",
            original_position: None,
        }
    }
}

impl EditPolicy for DragEditPolicy {
    fn get_key(&self) -> &str {
        self.key
    }

    fn activate(&mut self, part: &mut dyn EditPart, scene: &mut SceneGraph) {
        if let Some(block) = scene.get_block(part.block_id()) {
            self.original_position = Some(block.transform);
        }
    }

    fn deactivate(&mut self, _part: &mut dyn EditPart, _scene: &mut SceneGraph) {
        self.original_position = None;
    }
}

/// 创建移动命令
pub fn create_move_command(
    scene: &SceneGraph,
    part: &dyn EditPart,
    dx: f64,
    dy: f64,
) -> Option<Box<dyn Command>> {
    if let Some(block) = scene.get_block(part.block_id()) {
        Some(Box::new(
            super::command::MoveCommand::new(
                part.block_id(),
                dx,
                dy,
                block.transform,
            )
        ))
    } else {
        None
    }
}

/// 响应策略
///
/// 响应其他策略或部件的变化。
pub trait EditPolicyResponder: Send + Sync {
    fn policy_changed(&mut self, _key: &str, _old_policy: Option<Box<dyn EditPolicy>>) {}
    fn part_moved(&mut self, _part: EditPartId, _dx: f64, _dy: f64) {}
    fn part_resized(&mut self, _part: EditPartId, _width: f64, _height: f64) {}
    fn part_added(&mut self, _parent: EditPartId, _child: EditPartId) {}
    fn part_removed(&mut self, _parent: EditPartId, _child: EditPartId) {}
}
