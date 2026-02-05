//! Fill 布局器
//!
//! 填充布局，第一个子元素填充容器，其他子元素保持原位。

use super::LayoutManager;
use crate::BlockId;
use novadraw_geometry::Rectangle;

#[derive(Debug, Clone)]
pub struct FillLayout;

impl FillLayout {
    pub fn new() -> Self {
        Self
    }
}


impl LayoutManager for FillLayout {
    fn compute_size(
        &self,
        container_bounds: Rectangle,
        _children_bounds: &[Rectangle],
    ) -> (f64, f64) {
        (container_bounds.width, container_bounds.height)
    }

    fn layout(
        &self,
        container_bounds: Rectangle,
        children_bounds: &mut [(BlockId, Rectangle)],
    ) {
        if children_bounds.is_empty() {
            return;
        }

        let (_, first_rect) = children_bounds.first_mut().unwrap();
        first_rect.x = container_bounds.x;
        first_rect.y = container_bounds.y;
        first_rect.width = container_bounds.width;
        first_rect.height = container_bounds.height;

        for (_, _rect) in children_bounds.iter_mut().skip(1) {
            // 保持其他子节点的原始 bounds（不做修改）
        }
    }
}
