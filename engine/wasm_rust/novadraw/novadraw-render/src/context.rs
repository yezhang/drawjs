//! 渲染上下文
//!
//! 参考 HTML5 Canvas API 设计，生成可重放的渲染命令。

use glam::DVec2;
use novadraw_core::Color;
use novadraw_geometry::Transform;

use crate::command::{Path, RenderCommand, RenderCommandKind};
use crate::submission::{DamageSet, RenderSubmission};

#[derive(Clone, Debug)]
struct GraphicsState {
    fill_color: Option<Color>,
    stroke_color: Option<Color>,
    stroke_width: f64,
    line_cap: crate::command::LineCap,
    line_join: crate::command::LineJoin,
    transform: Transform,
    clip_depth: usize,
}

impl Default for GraphicsState {
    fn default() -> Self {
        Self {
            fill_color: None,
            stroke_color: None,
            stroke_width: 1.0,
            line_cap: crate::command::LineCap::Butt,
            line_join: crate::command::LineJoin::Miter,
            transform: Transform::IDENTITY,
            clip_depth: 0,
        }
    }
}

pub struct NdCanvas {
    pub damage: DamageSet,
    commands: Vec<RenderCommand>,
    /// 当前正在构建的路径（用于 begin_path/fill/stroke 流程）
    current_path: Option<Path>,
    state: GraphicsState,
    state_stack: Vec<GraphicsState>,
}

impl Default for NdCanvas {
    fn default() -> Self {
        Self::new()
    }
}

impl NdCanvas {
    pub fn new() -> Self {
        Self {
            damage: DamageSet::default(),
            commands: Vec::new(),
            current_path: None,
            state: GraphicsState::default(),
            state_stack: Vec::new(),
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
        self.state_stack.push(self.state.clone());
        self.create_command(RenderCommandKind::PushState);
    }

    /// 恢复到最近一次 pushState 的状态（不弹出栈）
    ///
    /// 对应 Draw2D: Graphics.restoreState()
    /// 用于在 paintFigure 之后、paintChildren 之前恢复裁剪区
    pub fn restore_state(&mut self) {
        if let Some(saved) = self.state_stack.last() {
            self.state = saved.clone();
        }
        self.create_command(RenderCommandKind::RestoreState);
    }

    /// 弹出并恢复状态
    ///
    /// 对应 Draw2D: Graphics.popState()
    /// 用于在所有绘制完成后恢复 pushState 前的状态
    pub fn pop_state(&mut self) {
        if let Some(saved) = self.state_stack.pop() {
            self.state = saved;
        }
        self.create_command(RenderCommandKind::PopState);
    }

    /// 平移
    ///
    /// 生成 ConcatTransform 命令
    pub fn translate(&mut self, x: f64, y: f64) {
        let t = Transform::from_translation(x, y);
        self.state.transform = self.state.transform.then_transform(t);
        self.create_command(RenderCommandKind::ConcatTransform { matrix: t });
    }

    /// 旋转
    ///
    /// 生成 ConcatTransform 命令
    pub fn rotate(&mut self, angle: f64) {
        let t = Transform::from_rotation(angle);
        self.state.transform = self.state.transform.then_transform(t);
        self.create_command(RenderCommandKind::ConcatTransform { matrix: t });
    }

    /// 缩放
    ///
    /// 生成 ConcatTransform 命令
    pub fn scale(&mut self, x: f64, y: f64) {
        let t = Transform::from_scale(x, y);
        self.state.transform = self.state.transform.then_transform(t);
        self.create_command(RenderCommandKind::ConcatTransform { matrix: t });
    }

    pub fn transform(&mut self, a: f64, b: f64, c: f64, d: f64, e: f64, f: f64) {
        let t = Transform::new(a, b, c, d, e, f);
        self.state.transform = self.state.transform.then_transform(t);
        self.create_command(RenderCommandKind::ConcatTransform { matrix: t });
    }

    pub fn set_transform(&mut self, a: f64, b: f64, c: f64, d: f64, e: f64, f: f64) {
        let t = Transform::new(a, b, c, d, e, f);
        self.state.transform = t;
        self.create_command(RenderCommandKind::SetTransform { matrix: t });
    }

    pub fn reset_transform(&mut self) {
        self.state.transform = Transform::IDENTITY;
        self.create_command(RenderCommandKind::ResetTransform);
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
            if let Some(color) = self.state.fill_color {
                // 跳过完全透明的颜色
                if color.a > 0.0 {
                    self.create_command(RenderCommandKind::FillPath { path, color });
                }
            }
        }
    }

    /// 描边当前路径
    #[allow(clippy::collapsible_if)]
    pub fn stroke(&mut self) {
        if let Some(path) = self.current_path.take() {
            if let Some(color) = self.state.stroke_color {
                let width = self.state.stroke_width;
                let line_cap = self.state.line_cap;
                let line_join = self.state.line_join;
                self.create_command(RenderCommandKind::StrokePath {
                    path,
                    color,
                    width,
                    line_cap,
                    line_join,
                });
            }
        }
    }

    /// 填充并描边当前路径
    pub fn fill_and_stroke(&mut self) {
        if let Some(path) = self.current_path.take() {
            if let Some(color) = self.state.fill_color {
                self.create_command(RenderCommandKind::FillPath {
                    path: path.clone(),
                    color,
                });
            }
            if let Some(color) = self.state.stroke_color {
                let width = self.state.stroke_width;
                let line_cap = self.state.line_cap;
                let line_join = self.state.line_join;
                self.create_command(RenderCommandKind::StrokePath {
                    path,
                    color,
                    width,
                    line_cap,
                    line_join,
                });
            }
        }
    }

    pub fn clip_rect(&mut self, x: f64, y: f64, width: f64, height: f64) {
        let rect = [DVec2::new(x, y), DVec2::new(x + width, y + height)];
        self.state.clip_depth += 1;
        self.create_command(RenderCommandKind::Clip { rect });
    }

    pub fn reset_clip(&mut self) {
        self.state.clip_depth = 0;
        self.create_command(RenderCommandKind::ResetClip);
    }

    pub fn clear_commands(&mut self) {
        self.commands.clear();
    }

    pub fn damage(&self) -> &DamageSet {
        &self.damage
    }

    pub fn damage_mut(&mut self) -> &mut DamageSet {
        &mut self.damage
    }

    pub fn commands(&self) -> &Vec<RenderCommand> {
        &self.commands
    }

    pub fn to_submission(&self) -> RenderSubmission {
        RenderSubmission {
            commands: self.commands.clone(),
            damage: self.damage.clone(),
        }
    }

    pub fn commands_mut(&mut self) -> &mut Vec<RenderCommand> {
        &mut self.commands
    }

    pub fn fill_style(&mut self, color: Color) {
        self.state.fill_color = Some(color);
    }

    pub fn stroke_style(&mut self, color: Color) {
        self.state.stroke_color = Some(color);
    }

    pub fn line_width(&mut self, width: f64) {
        self.state.stroke_width = width;
    }

    pub fn line_cap(&mut self, cap: crate::command::LineCap) {
        self.state.line_cap = cap;
    }

    pub fn line_join(&mut self, join: crate::command::LineJoin) {
        self.state.line_join = join;
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
        self.state.clip_depth
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::command::LineJoin;

    fn assert_transform_eq(actual: Transform, expected: Transform) {
        let actual = actual.coeffs();
        let expected = expected.coeffs();
        assert_eq!(actual, expected);
    }

    #[test]
    fn graphics_state_stack_restores_nested_clip_transform_and_stroke_state() {
        let mut canvas = NdCanvas::new();
        canvas.translate(10.0, 0.0);
        canvas.clip_rect(0.0, 0.0, 100.0, 100.0);
        canvas.line_width(2.0);

        canvas.push_state();
        canvas.scale(2.0, 2.0);
        canvas.clip_rect(10.0, 10.0, 20.0, 20.0);
        canvas.line_width(7.0);

        assert_eq!(canvas.clip_depth(), 2);

        canvas.restore_state();
        assert_eq!(canvas.clip_depth(), 1);

        canvas.stroke_style(Color::rgba(1.0, 0.0, 0.0, 1.0));
        canvas.line_join(LineJoin::Bevel);
        canvas.begin_path();
        canvas.move_to(0.0, 0.0);
        canvas.line_to(10.0, 10.0);
        canvas.stroke();

        let stroke = canvas.commands().last().expect("stroke command");
        let RenderCommandKind::StrokePath {
            width, line_join, ..
        } = stroke.kind
        else {
            panic!("expected StrokePath after restored state");
        };
        assert_eq!(width, 2.0);
        assert_eq!(line_join, LineJoin::Bevel);

        canvas.pop_state();
        assert_eq!(canvas.clip_depth(), 1);
    }

    #[test]
    fn set_transform_and_reset_transform_emit_snapshot_commands() {
        let mut canvas = NdCanvas::new();

        canvas.translate(10.0, 20.0);
        canvas.set_transform(2.0, 0.0, 0.0, 2.0, 5.0, 6.0);
        canvas.reset_transform();

        let commands = canvas.commands();
        assert_eq!(commands.len(), 3);

        match commands[0].kind {
            RenderCommandKind::ConcatTransform { matrix } => {
                assert_transform_eq(matrix, Transform::from_translation(10.0, 20.0));
            }
            _ => panic!("expected translate as ConcatTransform"),
        }

        match commands[1].kind {
            RenderCommandKind::SetTransform { matrix } => {
                assert_transform_eq(matrix, Transform::new(2.0, 0.0, 0.0, 2.0, 5.0, 6.0));
            }
            _ => panic!("expected SetTransform"),
        }

        assert!(matches!(
            commands[2].kind,
            RenderCommandKind::ResetTransform
        ));
    }

    #[test]
    fn clip_reset_and_restore_are_visible_in_command_snapshot() {
        let mut canvas = NdCanvas::new();

        canvas.push_state();
        canvas.clip_rect(0.0, 0.0, 100.0, 100.0);
        canvas.translate(5.0, 6.0);
        canvas.clip_rect(10.0, 10.0, 30.0, 40.0);
        canvas.restore_state();
        canvas.reset_clip();
        canvas.pop_state();

        assert_eq!(canvas.clip_depth(), 0);
        let kinds = canvas
            .commands()
            .iter()
            .map(|command| &command.kind)
            .collect::<Vec<_>>();

        assert!(matches!(kinds[0], RenderCommandKind::PushState));
        assert!(matches!(kinds[1], RenderCommandKind::Clip { .. }));
        assert!(matches!(
            kinds[2],
            RenderCommandKind::ConcatTransform { .. }
        ));
        assert!(matches!(kinds[3], RenderCommandKind::Clip { .. }));
        assert!(matches!(kinds[4], RenderCommandKind::RestoreState));
        assert!(matches!(kinds[5], RenderCommandKind::ResetClip));
        assert!(matches!(kinds[6], RenderCommandKind::PopState));
    }
}
