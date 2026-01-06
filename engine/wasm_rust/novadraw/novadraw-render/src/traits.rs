//! 渲染器 traits
//!
//! 定义渲染器的抽象接口，支持不同的渲染后端实现。

use crate::command::RenderCommand;

/// 窗口代理 trait
///
/// 提供窗口的基本信息和方法。
pub trait WindowProxy: Send + Sync {
    /// 请求重绘
    fn request_redraw(&self);
    /// 获取缩放因子
    fn scale_factor(&self) -> f64;
    /// 获取窗口宽度
    fn width(&self) -> u32;
    /// 获取窗口高度
    fn height(&self) -> u32;
}

/// 渲染器 trait
///
/// 定义渲染器的通用接口。
pub trait Renderer {
    /// 关联的窗口代理类型
    type Window: WindowProxy;

    /// 获取关联的窗口代理
    fn window(&self) -> &Self::Window;

    /// 执行渲染
    fn render(&mut self, commands: &[RenderCommand]);

    /// 处理窗口大小变化
    fn resize(&mut self, width: u32, height: u32);
}
