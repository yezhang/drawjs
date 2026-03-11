//! 渲染上下文
//!
//! 参考 HTML5 Canvas API 设计，直接生成命令（无状态栈）。
//! 状态管理由 VelloRenderer 执行器负责。

use glam::DVec2;
use novadraw_core::Color;
use novadraw_geometry::Transform;

use crate::command::{Path, RenderCommand, RenderCommandKind};

pub struct NdCanvas {
    commands: Vec<RenderCommand>,
    /// 当前正在构建的路径（用于 begin_path/fill/stroke 流程）
    current_path: Option<Path>,
    /// 当前填充颜色
    fill_color: Option<Color>,
    /// 当前描边颜色
    stroke_color: Option<Color>,
    /// 当前描边宽度
    stroke_width: f64,
    /// 当前线帽样式
    line_cap: crate::command::LineCap,
    /// 当前连接样式
    line_join: crate::command::LineJoin,
}

impl Default for NdCanvas {
    fn default() -> Self {
        Self::new()
    }
}

impl NdCanvas {
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
            current_path: None,
            fill_color: None,
            stroke_color: None,
            stroke_width: 1.0,
            line_cap: crate::command::LineCap::Butt,
            line_join: crate::command::LineJoin::Miter,
        }
    }

    fn create_command(&mut self, kind: RenderCommandKind) {
        let command = RenderCommand { kind };
        self.commands.push(command);
    }

    /// 保存当前状态（压栈）
    ///
    /// 对应 Draw2D: Graphics.pushState()
    /// 将当前状态复制并压入状态栈
    pub fn push_state(&mut self) {
        self.create_command(RenderCommandKind::PushState);
    }

    /// 恢复到最近一次 pushState 的状态（不弹出栈）
    ///
    /// 对应 Draw2D: Graphics.restoreState()
    /// 用于在 paintFigure 之后、paintChildren 之前恢复裁剪区
    pub fn restore_state(&mut self) {
        self.create_command(RenderCommandKind::RestoreState);
    }

    /// 弹出并恢复状态
    ///
    /// 对应 Draw2D: Graphics.popState()
    /// 用于在所有绘制完成后恢复 pushState 前的状态
    pub fn pop_state(&mut self) {
        self.create_command(RenderCommandKind::PopState);
    }

    /// 平移
    ///
    /// 生成 ConcatTransform 命令
    pub fn translate(&mut self, x: f64, y: f64) {
        let t = Transform::from_translation(x, y);
        self.create_command(RenderCommandKind::ConcatTransform { matrix: t });
    }

    /// 旋转
    ///
    /// 生成 ConcatTransform 命令
    pub fn rotate(&mut self, angle: f64) {
        let t = Transform::from_rotation(angle);
        self.create_command(RenderCommandKind::ConcatTransform { matrix: t });
    }

    /// 缩放
    ///
    /// 生成 ConcatTransform 命令
    pub fn scale(&mut self, x: f64, y: f64) {
        let t = Transform::from_scale(x, y);
        self.create_command(RenderCommandKind::ConcatTransform { matrix: t });
    }

    pub fn transform(&mut self, a: f64, b: f64, c: f64, d: f64, e: f64, f: f64) {
        let t = Transform::new(a, b, c, d, e, f);
        self.create_command(RenderCommandKind::ConcatTransform { matrix: t });
    }

    pub fn set_transform(&mut self, a: f64, b: f64, c: f64, d: f64, e: f64, f: f64) {
        let t = Transform::new(a, b, c, d, e, f);
        self.create_command(RenderCommandKind::ConcatTransform { matrix: t });
    }

    pub fn reset_transform(&mut self) {
        let t = Transform::IDENTITY;
        self.create_command(RenderCommandKind::ConcatTransform { matrix: t });
    }

    pub fn clear_rect(&mut self, x: f64, y: f64, width: f64, height: f64, color: Color) {
        let rect = [DVec2::new(x, y), DVec2::new(x + width, y + height)];
        self.create_command(RenderCommandKind::ClearRect { rect, color });
    }

    pub fn fill_rect(&mut self, x: f64, y: f64, width: f64, height: f64, color: Color) {
        let rect = [DVec2::new(x, y), DVec2::new(x + width, y + height)];
        self.create_command(RenderCommandKind::FillRect { rect, color });
    }

    #[allow(clippy::too_many_arguments)]
    pub fn stroke_rect(
        &mut self,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        color: Color,
        stroke_width: f64,
        cap: crate::command::LineCap,
        join: crate::command::LineJoin,
    ) {
        let rect = [DVec2::new(x, y), DVec2::new(x + width, y + height)];
        self.create_command(RenderCommandKind::StrokeRect {
            rect,
            color,
            width: stroke_width,
            cap,
            join,
        });
    }

    /// 绘制椭圆
    ///
    /// 椭圆中心为 (cx, cy)，x 轴半径 rx，y 轴半径 ry
    #[allow(clippy::too_many_arguments)]
    pub fn ellipse(
        &mut self,
        cx: f64,
        cy: f64,
        rx: f64,
        ry: f64,
        fill_color: Option<Color>,
        stroke_color: Option<Color>,
        stroke_width: f64,
        cap: crate::command::LineCap,
        join: crate::command::LineJoin,
    ) {
        self.create_command(RenderCommandKind::Ellipse {
            cx,
            cy,
            rx,
            ry,
            fill_color,
            stroke_color,
            stroke_width,
            cap,
            join,
        });
    }

    /// 绘制直线
    ///
    /// 从 p1 到 p2 的直线
    pub fn line(
        &mut self,
        p1: DVec2,
        p2: DVec2,
        color: Color,
        width: f64,
        cap: crate::command::LineCap,
        join: crate::command::LineJoin,
    ) {
        self.create_command(RenderCommandKind::Line {
            p1,
            p2,
            color,
            width,
            cap,
            join,
        });
    }

    /// 绘制折线
    ///
    /// 从 points[0] 到 points[1] ... 到 points[n] 的折线
    pub fn polyline(
        &mut self,
        points: &[DVec2],
        color: Color,
        width: f64,
        cap: crate::command::LineCap,
        join: crate::command::LineJoin,
    ) {
        if points.len() < 2 {
            return;
        }
        self.create_command(RenderCommandKind::Polyline {
            points: points.to_vec(),
            color,
            width,
            cap,
            join,
        });
    }

    /// 开始构建路径
    pub fn begin_path(&mut self) {
        self.current_path = Some(Path::new());
    }

    /// 闭合路径
    pub fn close_path(&mut self) {
        if let Some(ref mut path) = self.current_path {
            path.close();
        }
    }

    /// 移动到指定点（路径起点）
    pub fn move_to(&mut self, x: f64, y: f64) {
        if let Some(ref mut path) = self.current_path {
            path.move_to(x, y);
        }
    }

    /// 直线连接到指定点
    pub fn line_to(&mut self, x: f64, y: f64) {
        if let Some(ref mut path) = self.current_path {
            path.line_to(x, y);
        }
    }

    /// 添加矩形路径
    pub fn rect_path(&mut self, x: f64, y: f64, width: f64, height: f64) {
        if let Some(ref mut path) = self.current_path {
            path.rect(x, y, width, height);
        }
    }

    /// 添加弧线
    #[allow(unused_variables)]
    pub fn arc(
        &mut self,
        x: f64,
        y: f64,
        radius: f64,
        start_angle: f64,
        end_angle: f64,
        anticlockwise: bool,
    ) {
        if let Some(ref mut path) = self.current_path {
            // 将角度转换为弧度
            let start = start_angle * std::f64::consts::PI / 180.0;
            let end = end_angle * std::f64::consts::PI / 180.0;
            // 简化的 arc 实现：使用贝塞尔曲线近似
            let steps = 8;
            for i in 0..=steps {
                let angle = start + (end - start) * (i as f64 / steps as f64);
                let px = x + radius * angle.cos();
                let py = y + radius * angle.sin();
                if i == 0 {
                    path.move_to(px, py);
                } else {
                    path.line_to(px, py);
                }
            }
        }
    }

    /// 二次贝塞尔曲线
    pub fn quadratic_curve_to(&mut self, cpx: f64, cpy: f64, x: f64, y: f64) {
        if let Some(ref mut path) = self.current_path {
            path.quad_to(cpx, cpy, x, y);
        }
    }

    /// 三次贝塞尔曲线
    pub fn bezier_curve_to(&mut self, cp1x: f64, cp1y: f64, cp2x: f64, cp2y: f64, x: f64, y: f64) {
        if let Some(ref mut path) = self.current_path {
            path.cubic_to(cp1x, cp1y, cp2x, cp2y, x, y);
        }
    }

    /// 填充当前路径
    #[allow(clippy::collapsible_if)]
    pub fn fill(&mut self) {
        if let Some(path) = self.current_path.take() {
            if let Some(color) = self.fill_color {
                self.create_command(RenderCommandKind::FillPath(path));
                // 保存颜色供后续使用
                self.fill_color = Some(color);
            }
        }
    }

    /// 描边当前路径
    #[allow(clippy::collapsible_if)]
    pub fn stroke(&mut self) {
        if let Some(path) = self.current_path.take() {
            if self.stroke_color.is_some() {
                self.create_command(RenderCommandKind::StrokePath(path));
            }
        }
    }

    /// 填充并描边当前路径
    pub fn fill_and_stroke(&mut self) {
        if let Some(path) = self.current_path.take() {
            if let Some(_fill_color) = self.fill_color {
                self.create_command(RenderCommandKind::FillPath(path.clone()));
            }
            if self.stroke_color.is_some() {
                self.create_command(RenderCommandKind::StrokePath(path));
            }
        }
    }

    pub fn clip_rect(&mut self, x: f64, y: f64, width: f64, height: f64) {
        let rect = [DVec2::new(x, y), DVec2::new(x + width, y + height)];
        self.create_command(RenderCommandKind::Clip { rect });
    }

    pub fn reset_clip(&mut self) {}

    pub fn clear_commands(&mut self) {
        self.commands.clear();
    }

    pub fn commands(&self) -> &Vec<RenderCommand> {
        &self.commands
    }

    pub fn commands_mut(&mut self) -> &mut Vec<RenderCommand> {
        &mut self.commands
    }

    pub fn fill_style(&mut self, color: Color) {
        self.fill_color = Some(color);
    }

    pub fn stroke_style(&mut self, color: Color) {
        self.stroke_color = Some(color);
    }

    pub fn line_width(&mut self, width: f64) {
        self.stroke_width = width;
    }

    pub fn line_cap(&mut self, cap: crate::command::LineCap) {
        self.line_cap = cap;
    }

    pub fn line_join(&mut self, join: crate::command::LineJoin) {
        self.line_join = join;
    }

    pub fn line_dash_offset(&mut self, _offset: f64) {}

    pub fn set_line_dash(&mut self, _dash: &[f64]) {}

    pub fn miter_limit(&mut self, _limit: f64) {}

    pub fn font(&mut self, _font: &str) {}

    pub fn text_align(&mut self, _align: &str) {}

    pub fn text_baseline(&mut self, _baseline: &str) {}

    pub fn fill_text(&mut self, _text: &str, _x: f64, _y: f64) {}

    pub fn stroke_text(&mut self, _text: &str, _x: f64, _y: f64) {}

    pub fn measure_text(&mut self, _text: &str) -> f64 {
        0.0
    }

    pub fn draw_image(&mut self, _image: &crate::command::ImageData, _x: f64, _y: f64) {}

    pub fn draw_image_with_size(
        &mut self,
        _image: &crate::command::ImageData,
        _x: f64,
        _y: f64,
        _width: f64,
        _height: f64,
    ) {
    }

    pub fn global_alpha(&mut self, _alpha: f64) {}

    pub fn global_composite_operation(&mut self, _op: &str) {}

    pub fn shadow_color(&mut self, _color: Color) {}

    pub fn shadow_blur(&mut self, _blur: f64) {}

    pub fn shadow_offset_x(&mut self, _offset: f64) {}

    pub fn shadow_offset_y(&mut self, _offset: f64) {}

    pub fn is_point_in_path(&mut self, _x: f64, _y: f64) -> bool {
        false
    }

    pub fn is_point_in_stroke(&mut self, _x: f64, _y: f64) -> bool {
        false
    }

    pub fn clip_depth(&self) -> usize {
        0
    }
}
