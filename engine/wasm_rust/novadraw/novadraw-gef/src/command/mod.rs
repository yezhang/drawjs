//! 命令模式实现
//!
//! 提供撤销/重做功能的命令系统。

use std::fmt;
use std::sync::Arc;
use novadraw_scene::{SceneGraph, BlockId};

/// 命令执行结果
#[derive(Debug, Clone, PartialEq)]
pub enum CommandResult {
    /// 命令成功执行
    Success,
    /// 命令被取消
    Cancelled,
    /// 命令执行失败
    Failed(String),
}

/// 命令 trait
///
/// 所有编辑操作都实现此 trait。
pub trait Command: Send + Sync {
    /// 执行命令
    fn execute(&mut self, scene: &mut SceneGraph) -> CommandResult;
    /// 撤销命令
    fn undo(&mut self, scene: &mut SceneGraph) -> CommandResult;
    /// 获取命令描述
    fn label(&self) -> &str;
    /// 检查命令是否可以合并（用于批量操作）
    fn can_merge_with(&self, _other: &dyn Command) -> bool {
        false
    }
}

/// 基础命令实现
#[derive(Debug)]
pub struct BasicCommand<F>
where
    F: Fn(&mut SceneGraph) -> CommandResult + Send + Sync + 'static,
{
    execute_fn: F,
    undo_fn: F,
    label: Arc<str>,
}

impl<F> BasicCommand<F>
where
    F: Fn(&mut SceneGraph) -> CommandResult + Send + Sync + 'static,
{
    pub fn new(execute_fn: F, undo_fn: F, label: impl Into<Arc<str>>) -> Self {
        Self {
            execute_fn,
            undo_fn,
            label: label.into(),
        }
    }
}

impl<F> Command for BasicCommand<F>
where
    F: Fn(&mut SceneGraph) -> CommandResult + Send + Sync + 'static,
{
    fn execute(&mut self, scene: &mut SceneGraph) -> CommandResult {
        (self.execute_fn)(scene)
    }

    fn undo(&mut self, scene: &mut SceneGraph) -> CommandResult {
        (self.undo_fn)(scene)
    }

    fn label(&self) -> &str {
        &self.label
    }
}

impl fmt::Debug for dyn Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Command")
            .field("label", &self.label())
            .finish()
    }
}

/// 创建矩形的命令
#[derive(Debug, Clone)]
pub struct CreateRectangleCommand {
    block_id: Option<BlockId>,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    color: novadraw_core::Color,
}

impl CreateRectangleCommand {
    pub fn new(x: f64, y: f64, width: f64, height: f64, color: novadraw_core::Color) -> Self {
        Self {
            block_id: None,
            x,
            y,
            width,
            height,
            color,
        }
    }
}

impl Command for CreateRectangleCommand {
    fn execute(&mut self, scene: &mut SceneGraph) -> CommandResult {
        let rect = novadraw_scene::Rectangle::new_with_color(
            self.x, self.y, self.width, self.height, self.color,
        );
        let id = scene.new_content_block(Box::new(rect));
        self.block_id = Some(id);
        CommandResult::Success
    }

    fn undo(&mut self, _scene: &mut SceneGraph) -> CommandResult {
        // GEF 模式下通常不真正删除，而是隐藏或标记为已删除
        // 这里简化处理
        CommandResult::Success
    }

    fn label(&self) -> &str {
        "Create Rectangle"
    }
}

/// 移动图形的命令
#[derive(Debug, Clone)]
pub struct MoveCommand {
    block_id: BlockId,
    dx: f64,
    dy: f64,
    start_transform: novadraw_math::Transform,
}

impl MoveCommand {
    pub fn new(block_id: BlockId, dx: f64, dy: f64, start_transform: novadraw_math::Transform) -> Self {
        Self {
            block_id,
            dx,
            dy,
            start_transform,
        }
    }
}

impl Command for MoveCommand {
    fn execute(&mut self, scene: &mut SceneGraph) -> CommandResult {
        if let Some(block) = scene.blocks.get(self.block_id) {
            let translate = novadraw_math::Transform::from_translation(self.dx, self.dy);
            let new_transform = block.transform * translate;
            scene.set_block_transform(self.block_id, new_transform);
        }
        CommandResult::Success
    }

    fn undo(&mut self, scene: &mut SceneGraph) -> CommandResult {
        scene.set_block_transform(self.block_id, self.start_transform);
        CommandResult::Success
    }

    fn label(&self) -> &str {
        "Move"
    }
}

/// 命令栈
///
/// 管理命令历史，支持撤销/重做。
#[derive(Debug)]
pub struct CommandStack {
    undo_stack: Vec<Box<dyn Command>>,
    redo_stack: Vec<Box<dyn Command>>,
    max_size: usize,
}

impl CommandStack {
    pub fn new() -> Self {
        Self {
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            max_size: 100,
        }
    }

    /// 执行命令
    pub fn execute(&mut self, mut command: Box<dyn Command>, scene: &mut SceneGraph) -> CommandResult {
        let result = command.execute(scene);
        if result == CommandResult::Success {
            self.undo_stack.push(command);
            self.redo_stack.clear();

            // 限制历史大小
            if self.undo_stack.len() > self.max_size {
                self.undo_stack.remove(0);
            }
        }
        result
    }

    /// 撤销
    pub fn undo(&mut self, scene: &mut SceneGraph) -> CommandResult {
        if let Some(mut command) = self.undo_stack.pop() {
            let result = command.undo(scene);
            if result == CommandResult::Success {
                self.redo_stack.push(command);
            }
            result
        } else {
            CommandResult::Failed("Nothing to undo".to_string())
        }
    }

    /// 重做
    pub fn redo(&mut self, scene: &mut SceneGraph) -> CommandResult {
        if let Some(mut command) = self.redo_stack.pop() {
            let result = command.execute(scene);
            if result == CommandResult::Success {
                self.undo_stack.push(command);
            }
            result
        } else {
            CommandResult::Failed("Nothing to redo".to_string())
        }
    }

    /// 检查是否可以撤销
    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }

    /// 检查是否可以重做
    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }

    /// 获取撤销栈大小
    pub fn undo_len(&self) -> usize {
        self.undo_stack.len()
    }

    /// 获取重做栈大小
    pub fn redo_len(&self) -> usize {
        self.redo_stack.len()
    }

    /// 清空历史
    pub fn clear(&mut self) {
        self.undo_stack.clear();
        self.redo_stack.clear();
    }
}

impl Default for CommandStack {
    fn default() -> Self {
        Self::new()
    }
}
