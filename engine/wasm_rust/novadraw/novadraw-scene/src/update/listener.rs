//! Update Listener - 更新监听器
//!
//! 监听更新事件，用于调试、性能分析等。

use novadraw_geometry::Rectangle;

/// Update Event - 更新事件
#[derive(Debug, Clone)]
pub enum UpdateEvent {
    /// 验证开始前
    Validating,
    /// 验证完成后
    Validated,
    /// 重绘开始
    Painting { damage: Rectangle },
    /// 重绘完成
    Painted { damage: Rectangle },
}

/// Update Listener trait
///
/// 监听场景图的更新事件。
///
/// # 使用场景
///
/// - 调试：观察更新时机和区域
/// - 性能分析：统计更新频率和耗时
/// - 动画：协调多个视图的更新
pub trait UpdateListener: Send + Sync {
    /// 通知更新事件
    fn notify(&self, event: UpdateEvent);

    /// 检查是否为验证监听器
    fn as_validating_listener(&self) -> Option<&dyn ValidatingListener> {
        None
    }
}

/// Validating Listener - 验证监听器
///
/// 专门监听验证（布局）阶段的监听器。
pub trait ValidatingListener: Send + Sync {
    /// 通知验证开始
    fn notify_validating(&self);

    /// 通知验证完成
    fn notify_validated(&self);
}

/// No-op 实现
impl UpdateListener for () {
    fn notify(&self, _event: UpdateEvent) {}
}

impl ValidatingListener for () {
    fn notify_validating(&self) {}
    fn notify_validated(&self) {}
}
