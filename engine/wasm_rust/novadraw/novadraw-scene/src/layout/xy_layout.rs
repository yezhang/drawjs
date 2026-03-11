//! XY 布局器
//!
//! 参考 d2: XYLayout
//! 使用约束（Rectangle）定位每个子元素。

use super::LayoutManager;
use novadraw_geometry::Rectangle;

/// XY 布局约束
///
/// 对应 d2 中的 Rectangle 约束
#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct XYConstraint {
    /// 位置 x
    pub x: f64,
    /// 位置 y
    pub y: f64,
    /// 宽度，-1 表示使用首选宽度
    pub width: f64,
    /// 高度，-1 表示使用首选高度
    pub height: f64,
}

#[allow(dead_code)]
impl XYConstraint {
    /// 从 Rectangle 创建约束
    pub fn from_rect(rect: Rectangle) -> Self {
        Self {
            x: rect.x,
            y: rect.y,
            width: rect.width,
            height: rect.height,
        }
    }

    /// 创建指定位置的约束
    pub fn at(x: f64, y: f64) -> Self {
        Self {
            x,
            y,
            width: -1.0,
            height: -1.0,
        }
    }

    /// 创建指定位置的约束
    pub fn at_size(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self { x, y, width, height }
    }
}

impl Default for XYConstraint {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            width: -1.0,
            height: -1.0,
        }
    }
}

/// XY 布局器
///
/// 参考 d2: XYLayout
/// 根据每个子元素的约束 Rectangle 来定位和设置大小。
#[derive(Debug, Clone)]
pub struct XYLayout {
    /// 缓存的首选大小
    cached_preferred_size: Option<(f64, f64)>,
}

impl XYLayout {
    /// 创建新的 XYLayout
    pub fn new() -> Self {
        Self {
            cached_preferred_size: None,
        }
    }
}

impl Default for XYLayout {
    fn default() -> Self {
        Self::new()
    }
}

impl LayoutManager for XYLayout {
    fn get_constraint(&self, _child_id: usize) -> Option<Rectangle> {
        // XYLayout 不存储约束，由外部（如 SceneGraph）管理
        None
    }

    fn set_constraint(&mut self, _child_id: usize, _constraint: Rectangle) {
        // XYLayout 不存储约束，由外部管理
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
        // 简化实现：直接返回容器的首选大小
        // 实际应该根据子元素的约束计算
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
        // 默认等于首选大小
        self.get_preferred_size(container, w_hint, h_hint)
    }

    fn layout(&mut self, container: Rectangle, children: &mut [(usize, Rectangle)]) {
        // 计算偏移量（client area 的起始位置）
        let offset_x = container.x;
        let offset_y = container.y;

        for (_, child_bounds) in children.iter_mut() {
            // XYLayout：子元素的 bounds 直接就是约束
            // 将子元素移到相对于容器的位置
            child_bounds.x += offset_x;
            child_bounds.y += offset_y;
        }
    }

    fn invalidate(&mut self) {
        self.cached_preferred_size = None;
    }
}
