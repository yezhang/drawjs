//! 布局管理
//!
//! 提供 LayoutManager 接口和常用布局器实现，参考 Eclipse Draw2D 设计。

mod fill_layout;
mod xy_layout;

pub use fill_layout::FillLayout;
pub use xy_layout::XYLayout;

use novadraw_geometry::Rectangle;

/// 布局管理器 trait
///
/// 参考 d2: LayoutManager
/// 用于计算和设置子元素的位置。
pub trait LayoutManager: Send + Sync {
    /// 获取布局约束
    ///
    /// 对应 d2: getConstraint(IFigure)
    fn get_constraint(&self, child_id: usize) -> Option<Rectangle>;

    /// 设置布局约束
    ///
    /// 对应 d2: setConstraint(IFigure, Object)
    fn set_constraint(&mut self, child_id: usize, constraint: Rectangle);

    /// 移除布局约束
    ///
    /// 对应 d2: remove(IFigure)
    fn remove_constraint(&mut self, child_id: usize);

    /// 获取首选大小
    ///
    /// 对应 d2: getPreferredSize(IFigure, int, int)
    /// wHint, hHint 为建议的宽高，-1 表示无限制
    fn get_preferred_size(&self, container: Rectangle, w_hint: f64, h_hint: f64) -> (f64, f64);

    /// 获取最小大小
    ///
    /// 对应 d2: getMinimumSize(IFigure, int, int)
    fn get_minimum_size(&self, container: Rectangle, w_hint: f64, h_hint: f64) -> (f64, f64);

    /// 执行布局
    ///
    /// 对应 d2: layout(IFigure)
    fn layout(&mut self, container: Rectangle, children: &mut [(usize, Rectangle)]);

    /// 使缓存失效
    ///
    /// 对应 d2: invalidate()
    fn invalidate(&mut self);
}
