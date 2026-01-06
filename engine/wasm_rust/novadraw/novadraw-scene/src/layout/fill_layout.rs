//! Fill 布局器
//!
//! 填充布局，第一个子元素填充容器，其他子元素保持原位。

use super::LayoutManager;
use crate::{BlockId, Rect};

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
        container_bounds: Rect,
        _children_bounds: &[Rect],
    ) -> (f64, f64) {
        (container_bounds.width, container_bounds.height)
    }

    fn layout(
        &self,
        container_bounds: Rect,
        children_bounds: &mut [(BlockId, Rect)],
    ) {
        if children_bounds.is_empty() {
            return;
        }

        let (_, first_rect) = children_bounds.first_mut().unwrap();
        first_rect.x = container_bounds.x;
        first_rect.y = container_bounds.y;
        first_rect.width = container_bounds.width;
        first_rect.height = container_bounds.height;

        for (_, rect) in children_bounds.iter_mut().skip(1) {
            rect.x = rect.x;
            rect.y = rect.y;
            rect.width = rect.width;
            rect.height = rect.height;
        }
    }
}
