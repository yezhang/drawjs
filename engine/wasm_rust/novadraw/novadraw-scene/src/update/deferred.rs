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
use novadraw_render::NdCanvas;

use crate::scene::BlockId;
use crate::update::repair::{compute_damage_union, execute_repair_phase, merge_dirty_region};

/// Scene Update Manager
///
/// 场景图更新管理器，批量处理布局和重绘请求。
/// 参考 Eclipse Draw2D 的 DeferredUpdateManager 设计。
///
/// # 设计要点
///
/// - 脏区域使用 HashMap 合并，每个块最多一个脏区域
/// - 失效块使用 Vec 存储，支持重复添加（去重）
/// - 两阶段更新：先布局，再重绘
/// - 纯数据管理：具体的验证和渲染由 FigureGraph 通过 trait 方法执行
///
/// # 与 draw2d 的差异
///
/// draw2d 的 DeferredUpdateManager 直接持有 root Figure 引用并直接调用其方法。
/// 本实现将数据管理（UM）和业务逻辑（FigureGraph）分离，
/// 通过 `UpdateManagerSource` trait 定义回调接口，保持解耦。
#[derive(Default)]
pub struct SceneUpdateManager {
    /// 脏区域映射：block_id -> 脏区域
    pub(crate) dirty_regions: std::collections::HashMap<BlockId, Rectangle>,
    /// 失效块队列
    pub(crate) invalid_blocks: Vec<BlockId>,
    /// 是否有更新待处理
    pub(crate) update_queued: bool,
    pub(crate) updating: bool,
}

impl SceneUpdateManager {
    /// 创建新的场景更新管理器
    pub fn new() -> Self {
        Self {
            dirty_regions: std::collections::HashMap::new(),
            invalid_blocks: Vec::new(),
            update_queued: false,
            updating: false,
        }
    }

    /// 添加脏区域
    ///
    /// 对应 draw2d: UpdateManager.addDirtyRegion()
    ///
    /// # Arguments
    ///
    /// * `block_id` - 需要重绘的块 ID
    /// * `rect` - 脏区域（局部坐标）
    pub fn add_dirty_region(&mut self, block_id: BlockId, rect: Rectangle) {
        if merge_dirty_region(&mut self.dirty_regions, block_id, rect) {
            self.update_queued = true;
        }
    }

    /// 添加失效块
    ///
    /// 对应 draw2d: UpdateManager.addInvalidFigure()
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
    ///
    /// 对应 draw2d: updateQueued flag
    pub fn is_update_queued(&self) -> bool {
        self.update_queued
    }

    /// 计算合并后的脏区域
    ///
    /// 将所有脏区域合并为一个大的区域。
    pub fn compute_damage(&self) -> Rectangle {
        compute_damage_union(self.dirty_regions.values())
    }

    pub(crate) fn take_dirty_snapshot(&mut self) -> std::collections::HashMap<BlockId, Rectangle> {
        std::mem::take(&mut self.dirty_regions)
    }

    /// 清空所有待处理的更新
    pub fn clear(&mut self) {
        self.dirty_regions.clear();
        self.invalid_blocks.clear();
        self.update_queued = false;
        self.updating = false;
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

    /// 排空并返回所有待验证的块 ID
    ///
    /// 对应 draw2d: performValidation 中对 invalidFigures 的 drain。
    /// FigureGraph 使用此方法获取需要验证的块列表。
    pub fn drain_invalid_blocks(&mut self) -> Vec<BlockId> {
        self.invalid_blocks.drain(..).collect()
    }

    /// 清空脏区域和更新标记
    ///
    /// 对应 draw2d: performUpdate 完成后清空队列。
    /// 由 FigureGraph 在 repairDamage 完成后调用。
    pub fn clear_dirty_and_flag(&mut self) {
        self.update_queued = !self.invalid_blocks.is_empty() || !self.dirty_regions.is_empty();
    }
}

impl crate::update::UpdateManager for SceneUpdateManager {
    fn add_dirty_region(&mut self, block_id: BlockId, rect: Rectangle) {
        SceneUpdateManager::add_dirty_region(self, block_id, rect);
    }

    fn add_invalid_figure(&mut self, block_id: BlockId) {
        SceneUpdateManager::add_invalid_figure(self, block_id);
    }

    fn perform_update(
        &mut self,
        graph: &mut crate::scene::FigureGraph,
        canvas: &mut NdCanvas,
    ) {
        if self.updating {
            return;
        }

        self.updating = true;
        self.perform_validation(graph);
        self.update_queued = false;
        let dirty_snapshot = self.take_dirty_snapshot();

        execute_repair_phase(graph, canvas, dirty_snapshot.iter());

        self.clear_dirty_and_flag();
        self.updating = false;
    }

    fn perform_validation(&mut self, graph: &mut crate::scene::FigureGraph) {
        while !self.invalid_blocks.is_empty() {
            let block_ids = self.drain_invalid_blocks();
            for block_id in block_ids {
                if let Some(block) = graph.blocks.get(block_id) {
                    if block.is_visible && block.is_enabled {
                        graph.revalidate(block_id);
                    }
                }
            }
        }
    }

    fn is_updating(&self) -> bool {
        self.updating
    }

    fn is_update_queued(&self) -> bool {
        self.update_queued
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{FigureGraph, RectangleFigure, scene::BlockId, update::UpdateManager};
    use novadraw_core::Color;
    use slotmap::KeyData;

    fn create_test_key(data: u64) -> BlockId {
        BlockId::from(KeyData::from_ffi(data))
    }

    #[test]
    fn test_dirty_region_tracking() {
        let mut manager = SceneUpdateManager::new();

        let rect = Rectangle::new(0.0, 0.0, 100.0, 100.0);
        manager.add_dirty_region(create_test_key(1), rect);

        assert!(manager.is_update_queued());
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

        assert!(!manager.is_update_queued());
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

    #[test]
    fn test_drain_invalid_blocks() {
        let mut manager = SceneUpdateManager::new();

        manager.add_invalid_figure(create_test_key(1));
        manager.add_invalid_figure(create_test_key(2));

        let drained = manager.drain_invalid_blocks();
        assert_eq!(drained.len(), 2);
        assert!(!manager.has_pending_layout());
    }

    #[test]
    fn test_take_dirty_snapshot_freezes_current_cycle() {
        let mut manager = SceneUpdateManager::new();
        let key1 = create_test_key(1);
        let key2 = create_test_key(2);
        manager.add_dirty_region(key1, Rectangle::new(0.0, 0.0, 10.0, 10.0));

        let snapshot = manager.take_dirty_snapshot();
        manager.add_dirty_region(key2, Rectangle::new(20.0, 20.0, 5.0, 5.0));

        assert_eq!(snapshot.len(), 1);
        assert_eq!(
            snapshot.get(&key1),
            Some(&Rectangle::new(0.0, 0.0, 10.0, 10.0))
        );
        assert!(!snapshot.contains_key(&key2));
        assert_eq!(manager.dirty_count(), 1);
        assert_eq!(
            manager.dirty_regions.get(&key2),
            Some(&Rectangle::new(20.0, 20.0, 5.0, 5.0))
        );
    }

    #[test]
    fn test_clear_dirty_and_flag_preserves_next_cycle_work() {
        let mut manager = SceneUpdateManager::new();
        manager.add_dirty_region(create_test_key(1), Rectangle::new(0.0, 0.0, 10.0, 10.0));
        let _snapshot = manager.take_dirty_snapshot();
        manager.add_dirty_region(create_test_key(2), Rectangle::new(5.0, 5.0, 5.0, 5.0));

        manager.clear_dirty_and_flag();

        assert!(manager.is_update_queued());
        assert!(manager.has_pending_repaint());
    }

    #[test]
    fn test_perform_update_writes_damage_set_to_canvas() {
        let mut manager = SceneUpdateManager::new();
        let mut graph = FigureGraph::new();
        let root_id = graph.set_contents(Box::new(RectangleFigure::new_with_color(
            0.0,
            0.0,
            400.0,
            300.0,
            Color::rgba(0.1, 0.1, 0.1, 1.0),
        )));
        manager.add_dirty_region(root_id, Rectangle::new(10.0, 20.0, 30.0, 40.0));

        let mut canvas = NdCanvas::new();
        manager.perform_update(&mut graph, &mut canvas);

        assert_eq!(
            canvas.damage().union,
            Some(Rectangle::new(10.0, 20.0, 30.0, 40.0))
        );
        assert_eq!(canvas.damage().regions, vec![Rectangle::new(10.0, 20.0, 30.0, 40.0)]);
    }
}
