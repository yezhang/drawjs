//! Winit 平台 SceneHost 实现
//!
//! 对应 draw2d: LightweightSystem 的平台 paint 调度入口职责。
//!
//! # 调度策略
//!
//! 利用 winit 的 `request_redraw` 实现 request-driven 帧合并：
//! - `request_update()` → `window.request_redraw()`（幂等，系统自动去重）
//! - `RedrawRequested` 事件 → `execute_update()` 执行两阶段更新

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use novadraw::{
    FigureGraph, NdCanvas, RenderBackend, SceneHost, UpdateManager,
    backend::vello::WinitWindowProxy, traits::WindowProxy,
};

/// Winit 平台的 SceneHost 实现
///
/// 持有 winit 窗口引用和 redraw 挂起标记，协调平台 redraw 入口。
///
/// 不持有 FigureGraph / UpdateManager；核心对象由组合根在调用 `execute_update()` 时传入。
pub struct WinitSceneHost {
    window: Arc<WinitWindowProxy>,
    /// 是否有待执行的更新
    update_queued: AtomicBool,
}

impl WinitSceneHost {
    /// 创建新的 WinitSceneHost
    pub fn new(window: Arc<WinitWindowProxy>) -> Self {
        Self {
            window,
            update_queued: AtomicBool::new(false),
        }
    }
}

impl SceneHost for WinitSceneHost {
    fn request_update(&self) {
        if !self.update_queued.swap(true, Ordering::Relaxed) {
            self.window.request_redraw();
        }
    }

    fn is_update_queued(&self) -> bool {
        self.update_queued.load(Ordering::Relaxed)
    }

    fn execute_update(
        &self,
        scene: &mut FigureGraph,
        update_manager: &mut dyn UpdateManager,
        renderer: &mut impl RenderBackend,
    ) -> NdCanvas {
        if !self.update_queued.load(Ordering::Relaxed) && !update_manager.is_update_queued() {
            return NdCanvas::new();
        }

        let canvas = scene.perform_update(update_manager);
        let submission = canvas.to_submission();
        renderer.render(&submission);

        let still_queued = update_manager.is_update_queued();
        self.update_queued.store(still_queued, Ordering::Relaxed);
        if still_queued {
            self.window.request_redraw();
        }
        canvas
    }

    fn viewport_size(&self) -> (f64, f64) {
        (self.window.width() as f64, self.window.height() as f64)
    }
}
