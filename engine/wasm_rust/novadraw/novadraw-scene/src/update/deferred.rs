//! Scene Update Manager - 场景更新管理器
//!
//! 实现延迟批量更新机制，参考 Eclipse Draw2D 的 DeferredUpdateManager。
//!
//! # 核心功能
//!
//! 1. **脏区域（Dirty Region）跟踪**
//!    - 收集 repaint() 请求
//!    - 合并重叠区域，减少重绘次数
//!
//! 2. **失效块（Invalid Block）队列**
//!    - 收集 revalidate() 请求
//!    - 在重绘前先执行布局
//!
//! 3. **两阶段更新**
//!    - Phase 1: 布局失效的块
//!    - Phase 2: 合并并重绘脏区域

use novadraw_geometry::Rectangle;

use crate::scene::BlockId;

/// Scene Update Manager
///
/// 场景图更新管理器，批量处理布局和重绘请求。
/// 此结构体作为 SceneGraph 的内部状态使用。
///
/// # 设计要点
///
/// - 脏区域使用 HashMap 合并，每个块最多一个脏区域
/// - 失效块使用 Vec 存储，支持重复添加（去重）
/// - 两阶段更新：先布局，再重绘
#[derive(Default)]
pub struct SceneUpdateManager {
    /// 脏区域映射：block_id -> 脏区域
    pub(crate) dirty_regions: std::collections::HashMap<BlockId, Rectangle>,
    /// 失效块队列
    pub(crate) invalid_blocks: Vec<BlockId>,
    /// 是否有更新待处理
    pub(crate) update_queued: bool,
    /// 合并脏区域时的扩展边距
    #[allow(dead_code)]
    expand_margin: f64,
}

impl SceneUpdateManager {
    /// 创建新的场景更新管理器
    pub fn new() -> Self {
        Self {
            dirty_regions: std::collections::HashMap::new(),
            invalid_blocks: Vec::new(),
            update_queued: false,
            expand_margin: 1.0,
        }
    }

    /// 添加脏区域
    ///
    /// 对应 d2: UpdateManager.addDirtyRegion()
    ///
    /// # Arguments
    ///
    /// * `block_id` - 需要重绘的块 ID
    /// * `rect` - 脏区域（局部坐标）
    pub fn add_dirty_region(&mut self, block_id: BlockId, rect: Rectangle) {
        // 检查区域有效性
        if rect.width <= 0.0 || rect.height <= 0.0 {
            return;
        }

        // 合并脏区域
        if let Some(existing) = self.dirty_regions.get_mut(&block_id) {
            // 扩展区域以包含新区域
            let min_x = existing.x.min(rect.x);
            let min_y = existing.y.min(rect.y);
            let max_x = (existing.x + existing.width).max(rect.x + rect.width);
            let max_y = (existing.y + existing.height).max(rect.y + rect.height);
            existing.x = min_x;
            existing.y = min_y;
            existing.width = max_x - min_x;
            existing.height = max_y - min_y;
        } else {
            self.dirty_regions.insert(block_id, rect);
        }

        self.update_queued = true;
    }

    /// 添加失效块
    ///
    /// 对应 d2: UpdateManager.addInvalidFigure()
    ///
    /// 失效的块将在下一帧进行布局计算。
    ///
    /// # Arguments
    ///
    /// * `block_id` - 需要重新布局的块 ID
    pub fn add_invalid_figure(&mut self, block_id: BlockId) {
        // 检查是否已在队列中
        if self.invalid_blocks.contains(&block_id) {
            return;
        }

        self.invalid_blocks.push(block_id);
        self.update_queued = true;
    }

    /// 检查是否有待处理的布局
    pub fn has_pending_layout(&self) -> bool {
        !self.invalid_blocks.is_empty()
    }

    /// 检查是否有待处理的重绘
    pub fn has_pending_repaint(&self) -> bool {
        !self.dirty_regions.is_empty()
    }

    /// 检查是否有待处理的更新
    pub fn has_pending_updates(&self) -> bool {
        self.update_queued
    }

    /// 计算合并后的脏区域
    ///
    /// 将所有脏区域合并为一个大的区域。
    pub fn compute_damage(&self) -> Rectangle {
        if self.dirty_regions.is_empty() {
            return Rectangle::new(0.0, 0.0, 0.0, 0.0);
        }

        let mut min_x = f64::MAX;
        let mut min_y = f64::MAX;
        let mut max_x = f64::MIN;
        let mut max_y = f64::MIN;

        for rect in self.dirty_regions.values() {
            min_x = min_x.min(rect.x);
            min_y = min_y.min(rect.y);
            max_x = max_x.max(rect.x + rect.width);
            max_y = max_y.max(rect.y + rect.height);
        }

        Rectangle::new(min_x, min_y, max_x - min_x, max_y - min_y)
    }

    /// 清空所有待处理的更新
    pub fn clear(&mut self) {
        self.dirty_regions.clear();
        self.invalid_blocks.clear();
        self.update_queued = false;
    }

    /// 获取失效块数量
    #[allow(dead_code)]
    pub fn invalid_count(&self) -> usize {
        self.invalid_blocks.len()
    }

    /// 获取脏区域数量
    #[allow(dead_code)]
    pub fn dirty_count(&self) -> usize {
        self.dirty_regions.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scene::BlockId;
    use slotmap::KeyData;

    fn create_test_key(data: u64) -> BlockId {
        BlockId::from(KeyData::from_ffi(data))
    }

    #[test]
    fn test_dirty_region_tracking() {
        let mut manager = SceneUpdateManager::new();

        let rect = Rectangle::new(0.0, 0.0, 100.0, 100.0);
        manager.add_dirty_region(create_test_key(1), rect);

        assert!(manager.has_pending_updates());
        assert!(manager.has_pending_repaint());
        assert_eq!(manager.dirty_count(), 1);
    }

    #[test]
    fn test_dirty_region_merge() {
        let mut manager = SceneUpdateManager::new();

        let rect1 = Rectangle::new(0.0, 0.0, 100.0, 100.0);
        let rect2 = Rectangle::new(50.0, 50.0, 100.0, 100.0);

        let key = create_test_key(1);
        manager.add_dirty_region(key, rect1);
        manager.add_dirty_region(key, rect2);

        // 应该合并为一个区域
        assert_eq!(manager.dirty_count(), 1);

        let damage = manager.compute_damage();
        assert_eq!(damage.x, 0.0);
        assert_eq!(damage.y, 0.0);
        assert_eq!(damage.width, 150.0);
        assert_eq!(damage.height, 150.0);
    }

    #[test]
    fn test_invalid_block_queue() {
        let mut manager = SceneUpdateManager::new();

        let key = create_test_key(1);
        manager.add_invalid_figure(key);

        assert!(manager.has_pending_layout());
        assert_eq!(manager.invalid_count(), 1);
    }

    #[test]
    fn test_invalid_block_dedup() {
        let mut manager = SceneUpdateManager::new();

        let key = create_test_key(1);
        manager.add_invalid_figure(key);
        manager.add_invalid_figure(key); // 重复添加

        // 应该去重
        assert_eq!(manager.invalid_count(), 1);
    }

    #[test]
    fn test_clear() {
        let mut manager = SceneUpdateManager::new();

        let key = create_test_key(1);
        manager.add_dirty_region(key, Rectangle::new(0.0, 0.0, 100.0, 100.0));
        manager.add_invalid_figure(key);

        manager.clear();

        assert!(!manager.has_pending_updates());
        assert!(!manager.has_pending_layout());
        assert!(!manager.has_pending_repaint());
    }

    #[test]
    fn test_invalid_region() {
        let mut manager = SceneUpdateManager::new();

        // 无效区域应该被忽略
        let rect = Rectangle::new(0.0, 0.0, 0.0, 100.0);
        manager.add_dirty_region(create_test_key(1), rect);

        assert!(!manager.has_pending_repaint());
    }
}
