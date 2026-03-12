//! XY 布局器
//!
//! 参考 d2: XYLayout
//! 使用约束（Rectangle）定位每个子元素。

use super::LayoutContext;
use super::LayoutManager;
use crate::scene::BlockId;
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
        Self {
            x,
            y,
            width,
            height,
        }
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
    fn get_constraint(&self, _child_id: BlockId) -> Option<Rectangle> {
        // XYLayout 不存储约束，由外部（如 SceneGraph）管理
        None
    }

    fn set_constraint(&mut self, _child_id: BlockId, _constraint: Rectangle) {
        // XYLayout 不存储约束，由外部管理
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
        // 简化实现：直接返回缓存或默认大小
        // 实际应该根据子元素的约束计算
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
        // 默认等于首选大小
        self.get_preferred_size(container, w_hint, h_hint, ctx)
    }

    fn layout(&self, container: BlockId, ctx: &mut dyn LayoutContext) {
        // 获取容器的 bounds
        let children = ctx.get_children(container);
        if children.is_empty() {
            return;
        }

        // 获取容器的 bounds（用于计算 client area）
        let container_bounds = ctx.get_container_bounds(container);

        // d2: getOrigin(parent) 返回 parent.getClientArea().getLocation()
        // 在 d2 中，useLocalCoordinates() 默认返回 false
        // client area = bounds - insets，默认 insets 为 0
        // 所以 origin = bounds.location()
        let offset_x = container_bounds.x;
        let offset_y = container_bounds.y;

        // XYLayout：将约束从"相对于 client area"转换为"相对于 bounds"
        // d2: bounds = bounds.getTranslated(offset)
        for (child_id, _) in children {
            // 获取约束（相对于 client area）
            if let Some(constraint) = ctx.get_constraint(child_id) {
                // 将约束平移 offset，得到相对于 bounds 的坐标
                let new_bounds = Rectangle::new(
                    constraint.x + offset_x,
                    constraint.y + offset_y,
                    constraint.width,
                    constraint.height,
                );
                // 应用约束作为新的 bounds
                ctx.set_child_bounds(child_id, new_bounds);
            }
        }
    }

    fn invalidate(&mut self) {
        self.cached_preferred_size = None;
    }
}
