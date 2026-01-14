//! 工具系统
//!
//! 提供选择、拖拽、创建图形等交互工具。

use crate::editpart::EditPartId;
use crate::command::{CommandStack, CreateRectangleCommand};
use glam::DVec2;
use novadraw_geometry::Vec2;
use novadraw_scene::{SceneGraph, BlockId};
use novadraw_core::Color;

/// 工具状态
#[derive(Debug, Clone, PartialEq)]
pub enum ToolState {
    Inactive,
    Ready,
    Dragging,
    Creating,
}

/// 工具事件
#[derive(Debug, Clone)]
pub enum ToolEvent {
    MouseDown {
        position: DVec2,
        button: u32,
    },
    MouseMove {
        position: DVec2,
        button: u32,
    },
    MouseUp {
        position: DVec2,
        button: u32,
    },
    KeyDown {
        key_code: u32,
    },
    KeyUp {
        key_code: u32,
    },
    Wheel {
        delta: f64,
        position: DVec2,
    },
}

/// 工具 trait
///
/// 所有交互工具都要实现此 trait。
pub trait Tool: Send + Sync {
    /// 获取工具名称
    fn get_name(&self) -> &str;
    /// 获取工具提示
    fn get_tooltip(&self) -> &str;
    /// 处理工具事件
    fn handle_event(
        &mut self,
        event: &ToolEvent,
        scene: &mut SceneGraph,
        command_stack: &mut CommandStack,
    ) -> ToolResult;
    /// 激活工具
    fn activate(&mut self);
    /// 停用工具
    fn deactivate(&mut self);
    /// 设置当前选中的 EditPart
    fn set_active_edit_part(&mut self, part_id: Option<EditPartId>);
}

/// 工具执行结果
#[derive(Debug, Clone)]
pub struct ToolResult {
    pub handled: bool,
    pub needs_redraw: bool,
    pub cursor: Option<&'static str>,
}

impl ToolResult {
    pub fn none() -> Self {
        Self {
            handled: false,
            needs_redraw: false,
            cursor: None,
        }
    }

    pub fn handled() -> Self {
        Self {
            handled: true,
            needs_redraw: false,
            cursor: None,
        }
    }

    pub fn handled_with_redraw() -> Self {
        Self {
            handled: true,
            needs_redraw: true,
            cursor: None,
        }
    }

    pub fn with_cursor(cursor: &'static str) -> Self {
        Self {
            handled: false,
            needs_redraw: false,
            cursor: Some(cursor),
        }
    }
}

/// 选择工具
///
/// 用于选择和移动图形。
///
/// TODO: 待实现完整拖拽功能 - 需要 RuntimeBlock 支持 transform 字段
#[derive(Debug)]
pub struct SelectionTool {
    name: &'static str,
    state: ToolState,
    start_position: DVec2,
    current_position: DVec2,
    active_block_id: Option<BlockId>,
    // original_transform: Option<Transform>, // TODO: 启用此字段需要 RuntimeBlock 支持 transform
}

impl SelectionTool {
    pub fn new() -> Self {
        Self {
            name: "Selection Tool",
            state: ToolState::Ready,
            start_position: DVec2::ZERO,
            current_position: DVec2::ZERO,
            active_block_id: None,
            // original_transform: None, // TODO: 启用
        }
    }
}

impl Default for SelectionTool {
    fn default() -> Self {
        Self::new()
    }
}

impl Tool for SelectionTool {
    fn get_name(&self) -> &str {
        self.name
    }

    fn get_tooltip(&self) -> &str {
        "Select and move objects"
    }

    fn handle_event(
        &mut self,
        event: &ToolEvent,
        scene: &mut SceneGraph,
        _command_stack: &mut CommandStack,
    ) -> ToolResult {
        match event {
            ToolEvent::MouseDown { position, button: 0 } => {
                self.start_position = *position;
                self.current_position = *position;

                // 命中测试
                let hit_point = Vec2::new(position.x, position.y);
                if let Some(hit_id) = scene.hit_test(hit_point) {
                    self.active_block_id = Some(hit_id);
                    self.state = ToolState::Dragging;
                    scene.select_single(Some(hit_id));
                    return ToolResult::handled_with_redraw();
                } else {
                    scene.select_single(None);
                    return ToolResult::handled_with_redraw();
                }
            }

            ToolEvent::MouseMove { position, .. } => {
                self.current_position = *position;

                if self.state == ToolState::Dragging {
                    // TODO: 实现拖拽移动逻辑
                    // 需要 RuntimeBlock 支持 transform 字段
                    // if let Some(part_id) = self.active_block_id {
                    //     let dx = position.x - self.start_position.x;
                    //     let dy = position.y - self.start_position.y;
                    //     let translate = Transform::from_translation(dx, dy);
                    //     let new_transform = block.transform * translate;
                    //     scene.set_block_transform(part_id, new_transform);
                    // }
                    return ToolResult::handled_with_redraw();
                }

                // 悬停效果
                let hit_point = Vec2::new(position.x, position.y);
                if scene.hit_test(hit_point).is_some() {
                    ToolResult::with_cursor("pointer")
                } else {
                    ToolResult::with_cursor("default")
                }
            }

            ToolEvent::MouseUp { position, .. } => {
                if self.state == ToolState::Dragging {
                    let dx = position.x - self.start_position.x;
                    let dy = position.y - self.start_position.y;

                    // TODO: 启用命令记录需要 RuntimeBlock 支持 transform
                    // if let Some(part_id) = self.active_block_id {
                    //     if dx.abs() > 1.0 || dy.abs() > 1.0 {
                    //         let cmd = super::command::MoveCommand::new(
                    //             part_id, dx, dy, original_transform
                    //         );
                    //         command_stack.execute(Box::new(cmd), scene);
                    //     }
                    // }
                    let _ = (dx, dy); // 抑制未使用变量警告
                }

                self.state = ToolState::Ready;
                self.active_block_id = None;
                ToolResult::handled()
            }

            _ => ToolResult::none(),
        }
    }

    fn activate(&mut self) {
        self.state = ToolState::Ready;
    }

    fn deactivate(&mut self) {
        self.state = ToolState::Inactive;
        self.active_block_id = None;
    }

    fn set_active_edit_part(&mut self, _part_id: Option<EditPartId>) {}
}

/// 矩形创建工具
///
/// 用于创建矩形图形。
#[derive(Debug)]
pub struct RectangleCreationTool {
    name: &'static str,
    state: ToolState,
    start_position: DVec2,
    end_position: DVec2,
    color: Color,
}

impl RectangleCreationTool {
    pub fn new() -> Self {
        Self {
            name: "Rectangle Tool",
            state: ToolState::Ready,
            start_position: DVec2::ZERO,
            end_position: DVec2::ZERO,
            color: Color::hex("#3498db"),
        }
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }
}

impl Default for RectangleCreationTool {
    fn default() -> Self {
        Self::new()
    }
}

impl Tool for RectangleCreationTool {
    fn get_name(&self) -> &str {
        self.name
    }

    fn get_tooltip(&self) -> &str {
        "Create rectangles"
    }

    fn handle_event(
        &mut self,
        event: &ToolEvent,
        scene: &mut SceneGraph,
        command_stack: &mut CommandStack,
    ) -> ToolResult {
        match event {
            ToolEvent::MouseDown { position, button: 0 } => {
                self.start_position = *position;
                self.end_position = *position;
                self.state = ToolState::Creating;
                ToolResult::handled()
            }

            ToolEvent::MouseMove { position, .. } => {
                self.end_position = *position;

                if self.state == ToolState::Creating {
                    // 可以在这里绘制预览
                }
                ToolResult::handled()
            }

            ToolEvent::MouseUp { position, .. } => {
                if self.state == ToolState::Creating {
                    let x = self.start_position.x.min(position.x);
                    let y = self.start_position.y.min(position.y);
                    let width = (position.x - self.start_position.x).abs();
                    let height = (position.y - self.start_position.y).abs();

                    if width > 5.0 && height > 5.0 {
                        let cmd = CreateRectangleCommand::new(x, y, width, height, self.color);
                        command_stack.execute(Box::new(cmd), scene);
                    }
                }
                self.state = ToolState::Ready;
                ToolResult::handled_with_redraw()
            }

            _ => ToolResult::none(),
        }
    }

    fn activate(&mut self) {
        self.state = ToolState::Ready;
    }

    fn deactivate(&mut self) {
        self.state = ToolState::Inactive;
    }

    fn set_active_edit_part(&mut self, _part_id: Option<EditPartId>) {}
}

/// 框选工具
///
/// 用于矩形选择多个图形。
#[derive(Debug)]
pub struct MarqueeTool {
    name: &'static str,
    state: ToolState,
    start_position: DVec2,
    end_position: DVec2,
}

impl MarqueeTool {
    pub fn new() -> Self {
        Self {
            name: "Marquee Tool",
            state: ToolState::Ready,
            start_position: DVec2::ZERO,
            end_position: DVec2::ZERO,
        }
    }
}

impl Default for MarqueeTool {
    fn default() -> Self {
        Self::new()
    }
}

impl Tool for MarqueeTool {
    fn get_name(&self) -> &str {
        self.name
    }

    fn get_tooltip(&self) -> &str {
        "Marquee selection"
    }

    fn handle_event(
        &mut self,
        event: &ToolEvent,
        scene: &mut SceneGraph,
        _command_stack: &mut CommandStack,
    ) -> ToolResult {
        match event {
            ToolEvent::MouseDown { position, .. } => {
                self.start_position = *position;
                self.end_position = *position;
                self.state = ToolState::Dragging;
                scene.select_single(None);
                ToolResult::handled()
            }

            ToolEvent::MouseMove { position, .. } => {
                self.end_position = *position;
                ToolResult::handled()
            }

            ToolEvent::MouseUp { position, .. } => {
                let x = self.start_position.x.min(position.x);
                let y = self.start_position.y.min(position.y);
                let width = (position.x - self.start_position.x).abs();
                let height = (position.y - self.start_position.y).abs();

                if width > 5.0 && height > 5.0 {
                    let rect = novadraw_scene::Rect::new(x, y, width, height);
                    scene.select_by_rect(rect);
                }

                self.state = ToolState::Ready;
                ToolResult::handled_with_redraw()
            }

            _ => ToolResult::none(),
        }
    }

    fn activate(&mut self) {
        self.state = ToolState::Ready;
    }

    fn deactivate(&mut self) {
        self.state = ToolState::Inactive;
    }

    fn set_active_edit_part(&mut self, _part_id: Option<EditPartId>) {}
}
