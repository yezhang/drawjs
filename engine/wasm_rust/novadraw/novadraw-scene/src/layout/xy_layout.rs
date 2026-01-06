//! XY 布局器
//!
//! 简单的 XY 布局，按顺序排列子元素，每行一个子元素。

use super::LayoutManager;
use crate::{BlockId, Rect};

/// XY 布局器
///
/// 按顺序排列子元素，每个子元素占一行，宽度填充容器。
#[derive(Debug, Clone)]
pub struct XYLayout {
    /// 行间距
    pub spacing: f64,
    /// 边距
    pub margin: f64,
}

impl XYLayout {
    /// 创建新的 XYLayout
    pub fn new() -> Self {
        Self {
            spacing: 10.0,
            margin: 10.0,
        }
    }

    /// 设置边距
    pub fn with_margin(mut self, margin: f64) -> Self {
        self.margin = margin;
        self
    }

    /// 设置间距
    pub fn with_spacing(mut self, spacing: f64) -> Self {
        self.spacing = spacing;
        self
    }
}

impl Default for XYLayout {
    fn default() -> Self {
        Self::new()
    }
}

impl LayoutManager for XYLayout {
    fn compute_size(
        &self,
        container_bounds: Rect,
        children_bounds: &[Rect],
    ) -> (f64, f64) {
        let mut height = self.margin * 2.0;
        let mut max_width: f64 = 0.0;

        for child_bounds in children_bounds {
            max_width = max_width.max(child_bounds.width);
            height += child_bounds.height + self.spacing;
        }

        if !children_bounds.is_empty() {
            height -= self.spacing;
        }

        let width = container_bounds.width.max(max_width + self.margin * 2.0);
        (width, height)
    }

    fn layout(
        &self,
        container_bounds: Rect,
        children_bounds: &mut [(BlockId, Rect)],
    ) {
        let mut y = self.margin;

        for (_, rect) in children_bounds.iter_mut() {
            rect.x = self.margin;
            rect.y = y;
            rect.width = container_bounds.width - self.margin * 2.0;
            rect.height = rect.height; // 保持原始高度

            y += rect.height + self.spacing;
        }
    }
}
