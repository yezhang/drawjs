//! SceneHost - 场景图主机环境
//!
//! 定义场景图与平台环境交互的接口。平台特定的实现（如 WinitSceneHost）
//! 负责管理更新调度（帧合并）和渲染触发。
//!
//! 对应 draw2d: LightweightSystem 中持有 root + UpdateManager + paint() 的职责。
//!
//! # 设计理念
//!
//! draw2d 的 LightweightSystem 通过 SWT 的 `Display.asyncExec` 实现延迟批处理。
//! 本实现将调度策略抽象为 trait，不同平台提供不同实现：
//!
//! - **WinitSceneHost**: `request_update()` → `window.request_redraw()`，
//!   利用 winit 的 `about_to_wait` + `request_redraw` 实现帧合并
//! - **WebSceneHost** (未来): `requestAnimationFrame` 调度
//!
//! # 更新流程
//!
//! ```text
//! FigureGraph.mark_invalid() / repaint()
//!         │
//!         ▼ (内部调用 notify_update_requested)
//! SceneHost.notify_update_requested()
//!         │
//!         ▼ (平台调度)
//! WinitSceneHost:  window.request_redraw()
//!         │
//!         ▼ (RedrawRequested 事件)
//! SceneHost.execute_update()
//!         │
//!         ├─► scene.perform_update()   (Phase 1: 布局验证)
//!         └─► scene.repair_damage()   (Phase 2: 脏区域重绘)
//!                 │
//!                 ▼
//!         renderer.render(commands)
//! ```
//!
//! # 职责边界
//!
//! - **SceneHost**: 平台调度 + update 编排 + 渲染触发。不直接持有 FigureGraph。
//! - **FigureGraph**: 块树管理 + 布局计算 + 渲染命令生成。平台无关。
//! - **SceneUpdateManager**: 脏区域和失效块的纯数据管理。

use novadraw_render::{NdCanvas, RenderBackend};

/// 场景图主机环境 trait
///
/// 定义场景图与平台环境交互的接口。
///
/// # 实现说明
///
/// 典型实现（WinitSceneHost）：
/// 1. `notify_update_requested()` 设置 `update_queued = true`
/// 2. 平台事件循环收到 `RedrawRequested` → 调用 `execute_update()`
/// 3. `execute_update()` 执行两阶段更新后设置 `update_queued = false`
///
/// 不同平台可提供不同的调度策略（如 requestAnimationFrame、节流等）。
pub trait SceneHost: Send + Sync {
    /// 请求在下一次渲染帧执行更新
    ///
    /// 多次调用应合并为一次（由具体实现保证，如 winit 的 request_redraw 已是幂等）。
    fn request_update(&self);

    /// 检查是否有待执行的更新
    ///
    /// 对应 draw2d: DeferredUpdateManager.updateQueued 标志。
    fn is_update_queued(&self) -> bool;

    /// 执行一轮完整的两阶段更新（布局验证 + 脏区域重绘）
    ///
    /// 对应 draw2d: DeferredUpdateManager.performUpdate()。
    ///
    /// 调用顺序：先 `perform_update()`，再 `repair_damage()`。
    /// 调用方负责在 `execute_update()` 返回后将渲染命令交给渲染器。
    ///
    /// # Arguments
    ///
    /// * `scene` - 场景图
    /// * `renderer` - 渲染器后端
    fn execute_update(
        &self,
        scene: &mut impl SceneUpdateTarget,
        renderer: &mut impl RenderBackend,
    ) -> NdCanvas;

    /// 获取视口（窗口）尺寸
    ///
    /// # Returns
    ///
    /// `(width, height)` 单位为逻辑像素
    fn viewport_size(&self) -> (f64, f64);
}

/// SceneUpdateTarget - FigureGraph 的更新相关操作子集
///
/// 用于 `execute_update` 的参数化，避免传递完整 FigureGraph。
/// 对应 draw2d 中 UpdateManager 直接持有 root Figure 并调用其方法。
pub trait SceneUpdateTarget {
    /// 执行布局验证
    ///
    /// 对应 draw2d: performValidation() → fig.validate()
    fn perform_update(&mut self) -> NdCanvas;

    /// 渲染脏区域
    ///
    /// 对应 draw2d: repairDamage() → root.paint(graphics)
    fn repair_damage(&mut self) -> NdCanvas;
}
