//! winit 窗口代理实现
//!
//! 为 winit 窗口提供 WindowProxy 接口实现。

use std::sync::Arc;
use winit::window::Window;

use crate::traits::WindowProxy;

/// winit 窗口代理
///
/// 包装 winit 窗口以实现 WindowProxy trait。
pub struct WinitWindowProxy {
    window: Arc<Window>,
}

impl WinitWindowProxy {
    /// 创建新的窗口代理
    pub fn new(window: Arc<Window>) -> Self {
        Self { window }
    }

    /// 获取窗口引用
    pub fn window(&self) -> &Arc<Window> {
        &self.window
    }

    /// 克隆窗口
    pub fn clone_window(&self) -> Arc<Window> {
        Arc::clone(&self.window)
    }
}

/// 内部窗口代理类型别名
pub type WinitWindowProxyInner = WinitWindowProxy;

impl WindowProxy for WinitWindowProxy {
    fn request_redraw(&self) {
        self.window.request_redraw();
    }

    fn scale_factor(&self) -> f64 {
        self.window.scale_factor()
    }

    fn width(&self) -> u32 {
        self.window.inner_size().width
    }

    fn height(&self) -> u32 {
        self.window.inner_size().height
    }
}
