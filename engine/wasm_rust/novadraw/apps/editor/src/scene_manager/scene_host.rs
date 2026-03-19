//! Winit 平台 SceneHost 实现
//!
//! 对应 draw2d: LightweightSystem 中持有 root + UpdateManager + paint() 的职责。
//!
//! # 调度策略
//!
//! 利用 winit 的 `about_to_wait` + `request_redraw` 实现帧合并：
//! - `request_update()` → `window.request_redraw()`（幂等，系统自动去重）
//! - `about_to_wait` → `window.request_redraw()` 保证每帧执行一次
//! - `RedrawRequested` 事件 → `execute_update()` 执行两阶段更新

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use novadraw::{
    NdCanvas, RenderBackend, SceneHost, SceneUpdateTarget, backend::vello::WinitWindowProxy,
    traits::WindowProxy,
};

/// Winit 平台的 SceneHost 实现
///
/// 持有 winit 窗口引用和更新标记，协调 FigureGraph 和渲染器之间的更新流程。
///
/// 对应 draw2d: DeferredUpdateManager 编排 update + LightweightSystem 持有 root。
pub struct WinitSceneHost {
    window: Arc<WinitWindowProxy>,
    /// 是否有待执行的更新
    update_queued: AtomicBool,
    /// 是否使用迭代渲染模式（true = 脏区域裁剪，false = 全量重绘）
    use_iterative_render: bool,
}

impl WinitSceneHost {
    /// 创建新的 WinitSceneHost
    pub fn new(window: Arc<WinitWindowProxy>) -> Self {
        Self {
            window,
            update_queued: AtomicBool::new(false),
            use_iterative_render: false,
        }
    }

    /// 设置是否使用迭代渲染模式
    pub fn set_use_iterative_render(&mut self, value: bool) {
        self.use_iterative_render = value;
    }

    /// 是否使用迭代渲染模式
    pub fn use_iterative_render(&self) -> bool {
        self.use_iterative_render
    }
}

impl SceneHost for WinitSceneHost {
    fn request_update(&self) {
        self.update_queued.store(true, Ordering::Relaxed);
        self.window.request_redraw();
    }

    fn is_update_queued(&self) -> bool {
        self.update_queued.load(Ordering::Relaxed)
    }

    fn execute_update(
        &self,
        scene: &mut impl SceneUpdateTarget,
        renderer: &mut impl RenderBackend,
    ) -> NdCanvas {
        if !self.update_queued.load(Ordering::Relaxed) {
            return NdCanvas::new();
        }

        // Phase 1: 布局验证
        let _ = scene.perform_update();

        // Phase 2: 渲染
        let canvas = scene.repair_damage();
        renderer.render(canvas.commands());

        self.update_queued.store(false, Ordering::Relaxed);
        NdCanvas::new()
    }

    fn viewport_size(&self) -> (f64, f64) {
        (self.window.width() as f64, self.window.height() as f64)
    }
}
