//! Fill 布局器
//!
//! 参考 d2: FlowLayout 或 FillLayout
//! 第一个子元素填充容器，其他子元素保持原位。

use super::LayoutContext;
use super::LayoutManager;
use crate::scene::BlockId;
use novadraw_geometry::Rectangle;

/// Fill 布局器
///
/// 第一个子元素填充容器（减去 insets），其他子元素保持原位。
#[derive(Debug, Clone)]
pub struct FillLayout {
    /// 缓存的首选大小
    cached_preferred_size: Option<(f64, f64)>,
}

impl FillLayout {
    /// 创建新的 FillLayout
    pub fn new() -> Self {
        Self {
            cached_preferred_size: None,
        }
    }
}

impl Default for FillLayout {
    fn default() -> Self {
        Self::new()
    }
}

impl LayoutManager for FillLayout {
    fn get_constraint(&self, _child_id: BlockId) -> Option<Rectangle> {
        None
    }

    fn set_constraint(&mut self, _child_id: BlockId, _constraint: Rectangle) {
        self.invalidate();
    }

    fn remove_constraint(&mut self, _child_id: BlockId) {
        self.invalidate();
    }

    fn get_preferred_size(
        &self,
        _container: BlockId,
        _w_hint: f64,
        _h_hint: f64,
        _ctx: &dyn LayoutContext,
    ) -> (f64, f64) {
        if let Some(cached) = self.cached_preferred_size {
            return cached;
        }
        (0.0, 0.0)
    }

    fn get_minimum_size(
        &self,
        container: BlockId,
        w_hint: f64,
        h_hint: f64,
        ctx: &dyn LayoutContext,
    ) -> (f64, f64) {
        self.get_preferred_size(container, w_hint, h_hint, ctx)
    }

    fn layout(&self, container: BlockId, ctx: &mut dyn LayoutContext) {
        let children = ctx.get_children(container);
        if children.is_empty() {
            return;
        }

        // 获取第一个子元素
        if let Some((first_child_id, _)) = children.first() {
            // 获取容器的 bounds
            // FillLayout：第一个子元素填充容器
            // 需要获取容器的 bounds，这需要从场景图中获取
            // 简化实现：假设容器占据所有可用空间
            // 实际应该从 ctx 获取容器的 bounds
            let (width, height) = ctx.get_preferred_size(container);
            let bounds = Rectangle::new(0.0, 0.0, width, height);
            ctx.set_child_bounds(*first_child_id, bounds);
        }
    }

    fn invalidate(&mut self) {
        self.cached_preferred_size = None;
    }
}
