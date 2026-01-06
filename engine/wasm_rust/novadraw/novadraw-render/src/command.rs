//! 渲染命令类型
//!
//! 定义了所有可用的渲染操作命令。

use novadraw_core::Color;

/// 渲染命令
///
/// 包含一个渲染操作类型。
#[derive(Clone, Debug)]
pub struct RenderCommand {
    /// 命令类型
    pub kind: RenderCommandKind,
}

/// 渲染命令类型
#[derive(Clone, Debug)]
pub enum RenderCommandKind {
    /// 填充矩形
    ///
    /// # 字段
    ///
    /// * `rect` - 矩形坐标 [左上角, 右下角]
    /// * `color` - 填充颜色
    /// * `stroke_color` - 边框颜色
    /// * `stroke_width` - 边框宽度
    FillRect {
        /// 矩形坐标 [左上角, 右下角]
        rect: [glam::DVec2; 2],
        /// 填充颜色
        color: Option<Color>,
        /// 边框颜色
        stroke_color: Option<Color>,
        /// 边框宽度
        stroke_width: f64,
    },
}

impl RenderCommand {
    /// 创建填充矩形命令
    ///
    /// # 参数
    ///
    /// * `rect` - 矩形坐标 [左上角, 右下角]
    /// * `color` - 填充颜色
    pub fn new_fill_rect(rect: [glam::DVec2; 2], color: Option<Color>) -> Self {
        Self {
            kind: RenderCommandKind::FillRect {
                rect,
                color,
                stroke_color: None,
                stroke_width: 0.0,
            },
        }
    }

    /// 添加边框样式
    ///
    /// # 参数
    ///
    /// * `stroke_color` - 边框颜色
    /// * `stroke_width` - 边框宽度
    pub fn with_stroke(mut self, stroke_color: Color, stroke_width: f64) -> Self {
        let RenderCommandKind::FillRect { stroke_color: sc, stroke_width: sw, .. } = &mut self.kind;
        *sc = Some(stroke_color);
        *sw = stroke_width;
        self
    }
}
