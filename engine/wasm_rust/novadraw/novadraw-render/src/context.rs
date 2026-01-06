//! 渲染上下文
//!
//! 提供 2D 渲染接口，维护渲染命令列表和变换栈。

use glam::DVec2;
use novadraw_core::Color;
use novadraw_math::{Transform, Vec2};

use crate::command::{RenderCommand, RenderCommandKind};

/// 渲染上下文
///
/// 用于记录渲染操作，包含：
/// * 渲染命令列表
/// * 变换栈（支持嵌套变换）
pub struct RenderContext {
    /// 渲染命令列表
    pub commands: Vec<RenderCommand>,
    /// 变换栈
    transform_stack: TransformStack,
}

struct TransformStack {
    stack: Vec<Transform>,
}

impl TransformStack {
    fn new() -> Self {
        Self {
            stack: vec![Transform::IDENTITY],
        }
    }

    fn push(&mut self, transform: Transform) {
        let current = *self.stack.last().unwrap();
        self.stack.push(transform * current);
    }

    fn pop(&mut self) {
        if self.stack.len() > 1 {
            self.stack.pop();
        }
    }

    fn current(&self) -> &Transform {
        self.stack.last().unwrap()
    }
}

impl RenderContext {
    /// 创建新的渲染上下文
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
            transform_stack: TransformStack::new(),
        }
    }

    /// 推入变换
    pub fn push_transform(&mut self, transform: Transform) {
        self.transform_stack.push(transform);
    }

    /// 弹出变换
    pub fn pop_transform(&mut self) {
        self.transform_stack.pop();
    }

    /// 设置填充样式
    pub fn set_fill_style(&mut self, color: Color) {
        self.commands.push(RenderCommand::new_fill_rect(
            [DVec2::new(0.0, 0.0), DVec2::new(0.0, 0.0)],
            Some(color),
        ));
    }

    /// 设置边框样式
    pub fn set_stroke_style(&mut self, color: Color, width: f64) {
        if let Some(last_cmd) = self.commands.last_mut() {
            *last_cmd = last_cmd.clone().with_stroke(color, width);
        }
    }

    /// 绘制填充矩形
    pub fn draw_rect(&mut self, x: f64, y: f64, width: f64, height: f64) {
        if let Some(last_cmd) = self.commands.last_mut() {
            let RenderCommandKind::FillRect { rect, .. } = &mut last_cmd.kind;
            rect[0] = DVec2::new(x, y);
            rect[1] = DVec2::new(x + width, y + height);
        }
    }

    /// 绘制边框矩形
    pub fn draw_stroke_rect(&mut self, x: f64, y: f64, width: f64, height: f64) {
        if let Some(last_cmd) = self.commands.last_mut() {
            let RenderCommandKind::FillRect { rect, .. } = &mut last_cmd.kind;
            rect[0] = DVec2::new(x, y);
            rect[1] = DVec2::new(x + width, y + height);
        }
    }

    /// 变换点坐标
    ///
    /// 将点从局部坐标变换到世界坐标
    pub fn transform_point(&self, point: DVec2) -> DVec2 {
        let vec2_point = Vec2::new(point.x, point.y);
        let transformed = self.transform_stack.current().transform_point(vec2_point);
        DVec2::new(transformed.x(), transformed.y())
    }
}
