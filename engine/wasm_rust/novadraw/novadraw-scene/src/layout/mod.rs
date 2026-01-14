//! 布局管理
//!
//! 提供 LayoutManager 接口和常用布局器实现。

mod fill_layout;
mod xy_layout;
pub use fill_layout::FillLayout;
pub use xy_layout::XYLayout;

use crate::BlockId;
use novadraw_geometry::Rect;

/// 布局管理器 trait
///
/// 所有布局器都需要实现此接口，用于计算和设置子元素的位置。
pub trait LayoutManager: Send + Sync {
    /// 计算容器所需的最小大小
    fn compute_size(
        &self,
        container_bounds: Rect,
        children_bounds: &[Rect],
    ) -> (f64, f64);

    /// 执行布局
    ///
    /// 根据容器的边界和子元素的当前位置，计算并返回新的子元素边界。
    fn layout(
        &self,
        container_bounds: Rect,
        children_bounds: &mut [(BlockId, Rect)],
    );
}
