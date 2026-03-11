//! Fill 布局器
//!
//! 参考 d2: FlowLayout 或 FillLayout
//! 第一个子元素填充容器，其他子元素保持原位。

use super::LayoutManager;
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
    fn get_constraint(&self, _child_id: usize) -> Option<Rectangle> {
        None
    }

    fn set_constraint(&mut self, _child_id: usize, _constraint: Rectangle) {
        self.invalidate();
    }

    fn remove_constraint(&mut self, _child_id: usize) {
        self.invalidate();
    }

    fn get_preferred_size(
        &self,
        container: Rectangle,
        _w_hint: f64,
        _h_hint: f64,
    ) -> (f64, f64) {
        if let Some(cached) = self.cached_preferred_size {
            return cached;
        }
        (container.width, container.height)
    }

    fn get_minimum_size(
        &self,
        container: Rectangle,
        w_hint: f64,
        h_hint: f64,
    ) -> (f64, f64) {
        self.get_preferred_size(container, w_hint, h_hint)
    }

    fn layout(&mut self, container: Rectangle, children: &mut [(usize, Rectangle)]) {
        if children.is_empty() {
            return;
        }

        // 第一个子元素填充容器
        let (_, first_rect) = children.first_mut().unwrap();
        first_rect.x = container.x;
        first_rect.y = container.y;
        first_rect.width = container.width;
        first_rect.height = container.height;

        // 其他子元素保持原位（在外部坐标系中）
    }

    fn invalidate(&mut self) {
        self.cached_preferred_size = None;
    }
}
