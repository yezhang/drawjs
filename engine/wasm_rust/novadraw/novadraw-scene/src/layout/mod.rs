//! 布局管理
//!
//! 提供 LayoutManager 接口和常用布局器实现，参考 Eclipse Draw2D 设计。

mod border_layout;
mod fill_layout;
mod flow_layout;
mod xy_layout;

pub use border_layout::{BorderLayout, BorderRegion};
pub use fill_layout::FillLayout;
pub use flow_layout::{FlowDirection, FlowLayout};
pub use xy_layout::XYLayout;

use crate::scene::BlockId;
use novadraw_geometry::Rectangle;

/// 布局上下文 trait
///
/// 提供布局器所需的场景图查询接口。
pub trait LayoutContext: Send + Sync {
    /// 获取子元素列表
    ///
    /// 返回 (child_id, current_bounds) 列表
    fn get_children(&self, parent_id: BlockId) -> Vec<(BlockId, Rectangle)>;

    /// 获取子元素的布局约束
    fn get_constraint(&self, child_id: BlockId) -> Option<Rectangle>;

    /// 获取块的首选尺寸
    fn get_preferred_size(&self, block_id: BlockId) -> (f64, f64);

    /// 设置子元素的边界
    fn set_child_bounds(&mut self, child_id: BlockId, bounds: Rectangle);

    /// 获取容器的 bounds（用于计算 client area）
    fn get_container_bounds(&self, container_id: BlockId) -> Rectangle;
}

/// 布局管理器 trait
///
/// 参考 draw2d: LayoutManager
/// 用于计算和设置子元素的位置。
pub trait LayoutManager: Send + Sync {
    /// 获取布局约束
    ///
    /// 对应 draw2d: getConstraint(IFigure)
    fn get_constraint(&self, child_id: BlockId) -> Option<Rectangle>;

    /// 设置布局约束
    ///
    /// 对应 draw2d: setConstraint(IFigure, Object)
    fn set_constraint(&mut self, child_id: BlockId, constraint: Rectangle);

    /// 移除布局约束
    ///
    /// 对应 draw2d: remove(IFigure)
    fn remove_constraint(&mut self, child_id: BlockId);

    /// 获取首选大小
    ///
    /// 对应 draw2d: getPreferredSize(IFigure, int, int)
    /// wHint, hHint 为建议的宽高，-1 表示无限制
    fn get_preferred_size(
        &self,
        container: BlockId,
        w_hint: f64,
        h_hint: f64,
        ctx: &dyn LayoutContext,
    ) -> (f64, f64);

    /// 获取最小大小
    ///
    /// 对应 draw2d: getMinimumSize(IFigure, int, int)
    fn get_minimum_size(
        &self,
        container: BlockId,
        w_hint: f64,
        h_hint: f64,
        ctx: &dyn LayoutContext,
    ) -> (f64, f64);

    /// 执行布局
    ///
    /// 对应 draw2d: layout(IFigure)
    fn layout(&self, container: BlockId, ctx: &mut dyn LayoutContext);

    /// 使缓存失效
    ///
    /// 对应 draw2d: invalidate()
    fn invalidate(&mut self);
}
