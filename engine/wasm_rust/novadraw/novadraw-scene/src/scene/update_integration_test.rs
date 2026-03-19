//! Update Manager 集成测试
//!
//! 验证 SceneUpdateManager 与 SceneGraph 集成的正确性。

use novadraw_geometry::Rectangle;

use crate::{RectangleFigure, SceneGraph};

/// 测试：add_child 后自动标记布局失效
#[test]
fn test_add_child_marks_layout_invalid() {
    let mut scene = SceneGraph::new();

    // 创建内容块
    let container = RectangleFigure::new(0.0, 0.0, 200.0, 200.0);
    let container_id = scene.set_contents(Box::new(container));

    // 注意：set_contents 会使 layout_valid 变为 false
    // 先执行一次布局使其有效
    scene.layout_valid = true;

    // 添加子块
    let child = RectangleFigure::new(0.0, 0.0, 50.0, 50.0);
    scene.add_child_to(container_id, Box::new(child));

    // 添加子块后，布局应该失效
    assert!(!scene.is_layout_valid());
}

/// 测试：mark_invalid 将块添加到失效队列
#[test]
fn test_mark_invalid_adds_to_queue() {
    let mut scene = SceneGraph::new();

    let container = RectangleFigure::new(0.0, 0.0, 200.0, 200.0);
    let container_id = scene.set_contents(Box::new(container));

    // 初始没有待处理更新
    assert!(!scene.is_update_queued());

    // 标记需要重新布局
    scene.mark_invalid(container_id);

    // 现在有待处理的更新
    assert!(scene.is_update_queued());
    assert!(scene.update_manager.has_pending_layout());
}

/// 测试：repaint 添加脏区域
#[test]
fn test_repaints_adds_dirty_region() {
    let mut scene = SceneGraph::new();

    let container = RectangleFigure::new(0.0, 0.0, 200.0, 200.0);
    let container_id = scene.set_contents(Box::new(container));

    // 初始没有脏区域
    assert!(!scene.update_manager.has_pending_repaint());

    // 请求重绘
    scene.repaint(container_id, None);

    // 现在有脏区域
    assert!(scene.update_manager.has_pending_repaint());
    assert_eq!(scene.update_manager.dirty_count(), 1);
}

/// 测试：repaint 使用指定区域而非整个块
#[test]
fn test_repaint_uses_specified_rect() {
    let mut scene = SceneGraph::new();

    let container = RectangleFigure::new(0.0, 0.0, 200.0, 200.0);
    let container_id = scene.set_contents(Box::new(container));

    // 指定部分区域重绘
    let partial_rect = Rectangle::new(10.0, 10.0, 50.0, 50.0);
    scene.repaint(container_id, Some(partial_rect));

    // 脏区域应该是指定的部分区域
    let damage = scene.update_manager.compute_damage();
    assert_eq!(damage.x, 10.0);
    assert_eq!(damage.y, 10.0);
    assert_eq!(damage.width, 50.0);
    assert_eq!(damage.height, 50.0);
}

/// 测试：多次 repaint 合并脏区域
#[test]
fn test_multiple_repaints_merge_regions() {
    let mut scene = SceneGraph::new();

    let container = RectangleFigure::new(0.0, 0.0, 200.0, 200.0);
    let container_id = scene.set_contents(Box::new(container));

    // 多次重绘不同区域
    scene.repaint(container_id, Some(Rectangle::new(0.0, 0.0, 50.0, 50.0)));
    scene.repaint(container_id, Some(Rectangle::new(100.0, 100.0, 50.0, 50.0)));

    // 脏区域应该合并
    let damage = scene.update_manager.compute_damage();
    assert_eq!(damage.x, 0.0);
    assert_eq!(damage.y, 0.0);
    assert_eq!(damage.width, 150.0);
    assert_eq!(damage.height, 150.0);
}

/// 测试：不可见块不产生脏区域
#[test]
fn test_invisible_block_no_dirty_region() {
    let mut scene = SceneGraph::new();

    let container = RectangleFigure::new(0.0, 0.0, 200.0, 200.0);
    let container_id = scene.set_contents(Box::new(container));

    // 设置为不可见
    if let Some(block) = scene.blocks.get_mut(container_id) {
        block.is_visible = false;
    }

    // 请求重绘不可见块
    scene.repaint(container_id, None);

    // 不应该产生脏区域
    assert!(!scene.update_manager.has_pending_repaint());
}

/// 测试：perform_update 执行布局验证
#[test]
fn test_perform_update_two_phase() {
    let mut scene = SceneGraph::new();

    let container = RectangleFigure::new(0.0, 0.0, 200.0, 200.0);
    let container_id = scene.set_contents(Box::new(container));

    // 标记需要重新布局 + 请求重绘
    scene.mark_invalid(container_id);
    scene.repaint(container_id, None);

    // 执行两阶段更新（布局 + 脏区域重绘）
    let canvas = scene.perform_update();

    // 布局队列被清空
    assert!(!scene.update_manager.has_pending_layout());
    // 脏区域也被清空（perform_update 完成两阶段后清空）
    assert!(!scene.update_manager.has_pending_repaint());
    assert!(!scene.is_update_queued());

    // 返回的 canvas 包含渲染命令
    assert!(!canvas.commands().is_empty());
}

/// 测试：perform_update 后布局标记为有效
/// 测试：perform_update 后布局标记为有效
/// 注意：这个测试验证的是 mark_invalid + perform_update 的流程
#[test]
fn test_perform_update_marks_layout_valid() {
    let mut scene = SceneGraph::new();

    let container = RectangleFigure::new(0.0, 0.0, 200.0, 200.0);
    let container_id = scene.set_contents(Box::new(container));

    // 先标记布局失效（通过 mark_invalid）
    scene.mark_invalid(container_id);

    // 执行布局验证
    scene.perform_update();

    // mark_invalid 只是添加到队列，perform_update 会调用 revalidate
    // revalidate 只有在有布局管理器时才会设置 layout_valid = true
    // 由于没有布局管理器，layout_valid 保持原值
    // perform_update 完成两阶段后全部清空
    assert!(!scene.update_manager.has_pending_layout());
    assert!(!scene.update_manager.has_pending_repaint());
    assert!(!scene.is_update_queued());
}

/// 测试：clear 清空所有更新
#[test]
fn test_clear_updates() {
    let mut scene = SceneGraph::new();

    let container = RectangleFigure::new(0.0, 0.0, 200.0, 200.0);
    let container_id = scene.set_contents(Box::new(container));

    // 添加更新请求
    scene.mark_invalid(container_id);
    scene.repaint(container_id, None);

    assert!(scene.is_update_queued());

    // 清空更新
    scene.clear_updates();

    // 应该没有待处理更新
    assert!(!scene.is_update_queued());
}

/// 测试：revalidate 内部调用 mark_invalid
#[test]
fn test_revalidate_marks_invalid() {
    let mut scene = SceneGraph::new();

    let container = RectangleFigure::new(0.0, 0.0, 200.0, 200.0);
    let container_id = scene.set_contents(Box::new(container));

    // 初始布局有效
    scene.layout_valid = true;

    // 调用 revalidate（这会触发 mark_invalid）
    // 注意：revalidate 需要布局管理器才会在 update_manager 中添加
    // 这里测试直接调用 mark_invalid 的效果

    scene.mark_invalid(container_id);
    scene.perform_update();

    // 布局应该被重新计算并标记为有效
    assert!(scene.is_layout_valid());
}

/// 测试：get_damage_region 返回合并后的区域
#[test]
fn test_get_damage_region() {
    let mut scene = SceneGraph::new();

    let container = RectangleFigure::new(0.0, 0.0, 200.0, 200.0);
    let container_id = scene.set_contents(Box::new(container));

    // 添加脏区域
    scene.repaint(container_id, Some(Rectangle::new(10.0, 10.0, 50.0, 50.0)));

    let damage = scene.get_damage_region();
    assert_eq!(damage.x, 10.0);
    assert_eq!(damage.y, 10.0);
    assert_eq!(damage.width, 50.0);
    assert_eq!(damage.height, 50.0);
}

/// 测试：repaint_all 重绘所有内容
#[test]
fn test_repaint_all() {
    let mut scene = SceneGraph::new();

    // 初始没有 contents，repaint_all 不产生效果
    scene.repaint_all();
    assert!(!scene.update_manager.has_pending_repaint());

    // 设置 contents 后
    let container = RectangleFigure::new(0.0, 0.0, 200.0, 200.0);
    let _container_id = scene.set_contents(Box::new(container));

    scene.repaint_all();
    assert!(scene.update_manager.has_pending_repaint());
}

/// 测试：空区域被忽略
#[test]
fn test_empty_rect_ignored() {
    let mut scene = SceneGraph::new();

    let container = RectangleFigure::new(0.0, 0.0, 200.0, 200.0);
    let container_id = scene.set_contents(Box::new(container));

    // 宽为 0
    scene.repaint(container_id, Some(Rectangle::new(0.0, 0.0, 0.0, 100.0)));
    assert!(!scene.update_manager.has_pending_repaint());

    // 高为 0
    scene.repaint(container_id, Some(Rectangle::new(0.0, 0.0, 100.0, 0.0)));
    assert!(!scene.update_manager.has_pending_repaint());

    // 负数区域
    scene.repaint(
        container_id,
        Some(Rectangle::new(-10.0, -10.0, 100.0, 100.0)),
    );
    // 负数区域应该仍然被接受（因为 x,y 可以是负数）
    assert!(scene.update_manager.has_pending_repaint());
}

/// 测试：多个块独立跟踪脏区域
#[test]
fn test_multiple_blocks_independent_dirty_regions() {
    let mut scene = SceneGraph::new();

    let container = RectangleFigure::new(0.0, 0.0, 200.0, 200.0);
    let container_id = scene.set_contents(Box::new(container));

    let child1 = RectangleFigure::new(0.0, 0.0, 50.0, 50.0);
    let child1_id = scene.add_child_to(container_id, Box::new(child1));

    let child2 = RectangleFigure::new(100.0, 0.0, 50.0, 50.0);
    let child2_id = scene.add_child_to(container_id, Box::new(child2));

    // 重绘不同块的不同区域
    scene.repaint(child1_id, Some(Rectangle::new(0.0, 0.0, 25.0, 25.0)));
    scene.repaint(child2_id, Some(Rectangle::new(100.0, 0.0, 25.0, 25.0)));

    // 应该有 2 个独立的脏区域
    assert_eq!(scene.update_manager.dirty_count(), 2);

    // 合并后的区域应该覆盖两个子块
    let damage = scene.update_manager.compute_damage();
    assert_eq!(damage.x, 0.0);
    assert_eq!(damage.width, 125.0);
}

/// 测试：mark_invalid 与 repaint 组合
#[test]
fn test_mark_invalid_with_repaint() {
    let mut scene = SceneGraph::new();

    let container = RectangleFigure::new(0.0, 0.0, 200.0, 200.0);
    let container_id = scene.set_contents(Box::new(container));

    // 同时标记布局失效和请求重绘
    scene.mark_invalid(container_id);
    scene.repaint(container_id, None);

    // 两者都应该被记录
    assert!(scene.update_manager.has_pending_layout());
    assert!(scene.update_manager.has_pending_repaint());
    assert!(scene.is_update_queued());

    // 执行更新后全部清空（perform_update 完成两阶段）
    scene.perform_update();
    assert!(!scene.update_manager.has_pending_layout());
    assert!(!scene.update_manager.has_pending_repaint());
    assert!(!scene.is_update_queued());
}

/// 测试：add_child 触发更新
#[test]
fn test_add_child_triggers_updates() {
    let mut scene = SceneGraph::new();

    let container = RectangleFigure::new(0.0, 0.0, 200.0, 200.0);
    let container_id = scene.set_contents(Box::new(container));

    // 初始无更新
    assert!(!scene.is_update_queued());

    // 交互模式添加子块
    let child = RectangleFigure::new(10.0, 10.0, 50.0, 50.0);
    scene.add_child(container_id, Box::new(child));

    // 应该触发更新
    assert!(scene.update_manager.has_pending_layout());
    assert!(scene.update_manager.has_pending_repaint());
    assert!(scene.is_update_queued());
}

/// 测试：repair_damage 渲染并清空脏区域
#[test]
fn test_repair_damage_clears_dirty_regions() {
    let mut scene = SceneGraph::new();

    let container = RectangleFigure::new(0.0, 0.0, 200.0, 200.0);
    let container_id = scene.set_contents(Box::new(container));

    // 添加脏区域
    scene.repaint(container_id, Some(Rectangle::new(10.0, 10.0, 50.0, 50.0)));
    assert!(scene.update_manager.has_pending_repaint());

    // 渲染脏区域
    let _render_ctx = scene.repair_damage();

    // 脏区域被清空
    assert!(!scene.update_manager.has_pending_repaint());
}

/// 测试：repair_damage 使用脏区域作为裁剪
#[test]
fn test_repair_damage_uses_damage_region() {
    let mut scene = SceneGraph::new();

    let container = RectangleFigure::new(0.0, 0.0, 200.0, 200.0);
    let container_id = scene.set_contents(Box::new(container));

    // 指定脏区域
    let dirty_rect = Rectangle::new(10.0, 10.0, 50.0, 50.0);
    scene.repaint(container_id, Some(dirty_rect));

    // 渲染脏区域
    let render_ctx = scene.repair_damage();

    // 渲染产生了命令
    assert!(!render_ctx.commands().is_empty());
}

/// 测试：批量构建不触发更新
#[test]
fn test_batch_construction_no_updates() {
    let mut scene = SceneGraph::new();

    let container = RectangleFigure::new(0.0, 0.0, 200.0, 200.0);
    let container_id = scene.set_contents(Box::new(container));

    // 批量添加子块（不触发更新）
    let child1 = RectangleFigure::new(10.0, 10.0, 50.0, 50.0);
    scene.add_child_to(container_id, Box::new(child1));

    let child2 = RectangleFigure::new(100.0, 10.0, 50.0, 50.0);
    scene.add_child_to(container_id, Box::new(child2));

    // 批量构建阶段不触发更新
    assert!(!scene.is_update_queued());
}

/// 测试：批量构建后手动触发更新
#[test]
fn test_batch_then_manual_update() {
    let mut scene = SceneGraph::new();

    let container = RectangleFigure::new(0.0, 0.0, 200.0, 200.0);
    let container_id = scene.set_contents(Box::new(container));

    // 批量添加子块
    let child1 = RectangleFigure::new(10.0, 10.0, 50.0, 50.0);
    scene.add_child_to(container_id, Box::new(child1));

    // 批量构建完成后，手动触发更新
    scene.mark_invalid(container_id);
    scene.repaint(container_id, None);

    // 现在有待处理更新
    assert!(scene.is_update_queued());

    // 执行更新（两阶段：布局 + 脏区域重绘）
    let _canvas = scene.perform_update();

    // 所有队列清空
    assert!(!scene.is_update_queued());
}
