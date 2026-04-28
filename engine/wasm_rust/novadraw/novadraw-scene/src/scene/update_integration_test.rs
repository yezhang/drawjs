use novadraw_geometry::Rectangle;

use crate::{FigureGraph, PendingMutation, RectangleFigure, SceneUpdateManager};

fn new_scene() -> (FigureGraph, SceneUpdateManager) {
    (FigureGraph::new(), SceneUpdateManager::new())
}

#[test]
fn test_add_child_marks_layout_invalid() {
    let (mut scene, _) = new_scene();
    let container_id = scene.set_contents(Box::new(RectangleFigure::new(0.0, 0.0, 200.0, 200.0)));
    scene.validate();
    scene.add_child_to(container_id, Box::new(RectangleFigure::new(0.0, 0.0, 50.0, 50.0)));
    assert!(!scene.is_layout_valid());
}

#[test]
fn test_mark_invalid_adds_to_queue() {
    let (mut scene, mut update_manager) = new_scene();
    let container_id = scene.set_contents(Box::new(RectangleFigure::new(0.0, 0.0, 200.0, 200.0)));
    assert!(!update_manager.is_update_queued());
    scene.mark_invalid(&mut update_manager, container_id);
    assert!(update_manager.is_update_queued());
    assert!(update_manager.has_pending_layout());
}

#[test]
fn test_repaint_adds_dirty_region() {
    let (mut scene, mut update_manager) = new_scene();
    let container_id = scene.set_contents(Box::new(RectangleFigure::new(0.0, 0.0, 200.0, 200.0)));
    assert!(!update_manager.has_pending_repaint());
    scene.repaint(&mut update_manager, container_id, None);
    assert!(update_manager.has_pending_repaint());
    assert_eq!(update_manager.dirty_count(), 1);
}

#[test]
fn test_repaint_uses_specified_rect() {
    let (mut scene, mut update_manager) = new_scene();
    let container_id = scene.set_contents(Box::new(RectangleFigure::new(0.0, 0.0, 200.0, 200.0)));
    let partial_rect = Rectangle::new(10.0, 10.0, 50.0, 50.0);
    scene.repaint(&mut update_manager, container_id, Some(partial_rect));
    let damage = update_manager.compute_damage();
    assert_eq!(damage, partial_rect);
}

#[test]
fn test_multiple_repaints_merge_regions() {
    let (mut scene, mut update_manager) = new_scene();
    let container_id = scene.set_contents(Box::new(RectangleFigure::new(0.0, 0.0, 200.0, 200.0)));
    scene.repaint(
        &mut update_manager,
        container_id,
        Some(Rectangle::new(0.0, 0.0, 50.0, 50.0)),
    );
    scene.repaint(
        &mut update_manager,
        container_id,
        Some(Rectangle::new(100.0, 100.0, 50.0, 50.0)),
    );
    let damage = update_manager.compute_damage();
    assert_eq!(damage.x, 0.0);
    assert_eq!(damage.y, 0.0);
    assert_eq!(damage.width, 150.0);
    assert_eq!(damage.height, 150.0);
}

#[test]
fn test_invisible_block_no_dirty_region() {
    let (mut scene, mut update_manager) = new_scene();
    let container_id = scene.set_contents(Box::new(RectangleFigure::new(0.0, 0.0, 200.0, 200.0)));
    scene.blocks.get_mut(container_id).unwrap().is_visible = false;
    scene.repaint(&mut update_manager, container_id, None);
    assert!(!update_manager.has_pending_repaint());
}

#[test]
fn test_perform_update_two_phase() {
    let (mut scene, mut update_manager) = new_scene();
    let container_id = scene.set_contents(Box::new(RectangleFigure::new(0.0, 0.0, 200.0, 200.0)));
    scene.mark_invalid(&mut update_manager, container_id);
    scene.repaint(&mut update_manager, container_id, None);
    let canvas = scene.perform_update(&mut update_manager);
    assert!(!update_manager.has_pending_layout());
    assert!(!update_manager.has_pending_repaint());
    assert!(!update_manager.is_update_queued());
    assert!(!canvas.commands().is_empty());
}

#[test]
fn test_perform_update_marks_layout_valid() {
    let (mut scene, mut update_manager) = new_scene();
    let container_id = scene.set_contents(Box::new(RectangleFigure::new(0.0, 0.0, 200.0, 200.0)));
    scene.mark_invalid(&mut update_manager, container_id);
    scene.perform_update(&mut update_manager);
    assert!(!update_manager.has_pending_layout());
    assert!(scene.is_layout_valid());
}

#[test]
fn test_clear_updates() {
    let (mut scene, mut update_manager) = new_scene();
    let container_id = scene.set_contents(Box::new(RectangleFigure::new(0.0, 0.0, 200.0, 200.0)));
    scene.mark_invalid(&mut update_manager, container_id);
    scene.repaint(&mut update_manager, container_id, None);
    assert!(update_manager.is_update_queued());
    update_manager.clear();
    assert!(!update_manager.is_update_queued());
}

#[test]
fn test_revalidate_flow() {
    let (mut scene, mut update_manager) = new_scene();
    let container_id = scene.set_contents(Box::new(RectangleFigure::new(0.0, 0.0, 200.0, 200.0)));
    scene.validate();
    scene.mark_invalid(&mut update_manager, container_id);
    scene.perform_update(&mut update_manager);
    assert!(scene.is_layout_valid());
}

#[test]
fn test_repaint_all() {
    let (mut scene, mut update_manager) = new_scene();
    scene.repaint_all(&mut update_manager);
    assert!(!update_manager.has_pending_repaint());
    scene.set_contents(Box::new(RectangleFigure::new(0.0, 0.0, 200.0, 200.0)));
    scene.repaint_all(&mut update_manager);
    assert!(update_manager.has_pending_repaint());
}

#[test]
fn test_empty_rect_ignored() {
    let (mut scene, mut update_manager) = new_scene();
    let container_id = scene.set_contents(Box::new(RectangleFigure::new(0.0, 0.0, 200.0, 200.0)));
    scene.repaint(
        &mut update_manager,
        container_id,
        Some(Rectangle::new(0.0, 0.0, 0.0, 100.0)),
    );
    assert!(!update_manager.has_pending_repaint());
    scene.repaint(
        &mut update_manager,
        container_id,
        Some(Rectangle::new(0.0, 0.0, 100.0, 0.0)),
    );
    assert!(!update_manager.has_pending_repaint());
    scene.repaint(
        &mut update_manager,
        container_id,
        Some(Rectangle::new(-10.0, -10.0, 100.0, 100.0)),
    );
    assert!(update_manager.has_pending_repaint());
}

#[test]
fn test_multiple_blocks_independent_dirty_regions() {
    let (mut scene, mut update_manager) = new_scene();
    let container_id = scene.set_contents(Box::new(RectangleFigure::new(0.0, 0.0, 200.0, 200.0)));
    let child1_id = scene.add_child_to(container_id, Box::new(RectangleFigure::new(0.0, 0.0, 50.0, 50.0)));
    let child2_id = scene.add_child_to(
        container_id,
        Box::new(RectangleFigure::new(100.0, 0.0, 50.0, 50.0)),
    );
    scene.repaint(
        &mut update_manager,
        child1_id,
        Some(Rectangle::new(0.0, 0.0, 25.0, 25.0)),
    );
    scene.repaint(
        &mut update_manager,
        child2_id,
        Some(Rectangle::new(100.0, 0.0, 25.0, 25.0)),
    );
    assert_eq!(update_manager.dirty_count(), 2);
    let damage = update_manager.compute_damage();
    assert_eq!(damage.x, 0.0);
    assert_eq!(damage.width, 125.0);
}

#[test]
fn test_mark_invalid_with_repaint() {
    let (mut scene, mut update_manager) = new_scene();
    let container_id = scene.set_contents(Box::new(RectangleFigure::new(0.0, 0.0, 200.0, 200.0)));
    scene.mark_invalid(&mut update_manager, container_id);
    scene.repaint(&mut update_manager, container_id, None);
    assert!(update_manager.has_pending_layout());
    assert!(update_manager.has_pending_repaint());
    assert!(update_manager.is_update_queued());
    scene.perform_update(&mut update_manager);
    assert!(!update_manager.has_pending_layout());
    assert!(!update_manager.has_pending_repaint());
    assert!(!update_manager.is_update_queued());
}

#[test]
fn test_add_child_triggers_updates() {
    let (mut scene, mut update_manager) = new_scene();
    let container_id = scene.set_contents(Box::new(RectangleFigure::new(0.0, 0.0, 200.0, 200.0)));
    assert!(!update_manager.is_update_queued());
    scene.add_child(
        &mut update_manager,
        container_id,
        Box::new(RectangleFigure::new(10.0, 10.0, 50.0, 50.0)),
    );
    assert!(update_manager.has_pending_layout());
    assert!(update_manager.has_pending_repaint());
    assert!(update_manager.is_update_queued());
}

#[test]
fn test_perform_update_clears_dirty_regions() {
    let (mut scene, mut update_manager) = new_scene();
    let container_id = scene.set_contents(Box::new(RectangleFigure::new(0.0, 0.0, 200.0, 200.0)));
    scene.repaint(
        &mut update_manager,
        container_id,
        Some(Rectangle::new(10.0, 10.0, 50.0, 50.0)),
    );
    assert!(update_manager.has_pending_repaint());
    scene.perform_update(&mut update_manager);
    assert!(!update_manager.has_pending_repaint());
}

#[test]
fn test_perform_update_uses_damage_region() {
    let (mut scene, mut update_manager) = new_scene();
    let container_id = scene.set_contents(Box::new(RectangleFigure::new(0.0, 0.0, 200.0, 200.0)));
    scene.repaint(
        &mut update_manager,
        container_id,
        Some(Rectangle::new(10.0, 10.0, 50.0, 50.0)),
    );
    let render_ctx = scene.perform_update(&mut update_manager);
    assert!(!render_ctx.commands().is_empty());
}

#[test]
fn test_batch_construction_no_updates() {
    let (mut scene, update_manager) = new_scene();
    let container_id = scene.set_contents(Box::new(RectangleFigure::new(0.0, 0.0, 200.0, 200.0)));
    scene.add_child_to(container_id, Box::new(RectangleFigure::new(10.0, 10.0, 50.0, 50.0)));
    scene.add_child_to(container_id, Box::new(RectangleFigure::new(100.0, 10.0, 50.0, 50.0)));
    assert!(!update_manager.is_update_queued());
}

#[test]
fn test_batch_then_manual_update() {
    let (mut scene, mut update_manager) = new_scene();
    let container_id = scene.set_contents(Box::new(RectangleFigure::new(0.0, 0.0, 200.0, 200.0)));
    scene.add_child_to(container_id, Box::new(RectangleFigure::new(10.0, 10.0, 50.0, 50.0)));
    scene.mark_invalid(&mut update_manager, container_id);
    scene.repaint(&mut update_manager, container_id, None);
    assert!(update_manager.is_update_queued());
    scene.perform_update(&mut update_manager);
    assert!(!update_manager.is_update_queued());
}

#[test]
fn test_mark_invalid_updates_block_validity() {
    let (mut scene, mut update_manager) = new_scene();
    let container_id = scene.set_contents(Box::new(RectangleFigure::new(0.0, 0.0, 200.0, 200.0)));
    scene.validate();
    scene.mark_invalid(&mut update_manager, container_id);
    assert!(!scene.get_block(container_id).unwrap().is_valid);
    assert!(!scene.is_layout_valid());
}

#[test]
fn test_hidden_block_skips_validation_but_drains_queue() {
    let (mut scene, mut update_manager) = new_scene();
    let container_id = scene.set_contents(Box::new(RectangleFigure::new(0.0, 0.0, 200.0, 200.0)));
    scene.validate();
    scene.blocks.get_mut(container_id).unwrap().is_visible = false;

    scene.mark_invalid(&mut update_manager, container_id);
    scene.perform_update(&mut update_manager);

    assert!(!update_manager.has_pending_layout());
    assert!(!update_manager.is_update_queued());
    assert!(!scene.get_block(container_id).unwrap().is_valid);
}

#[test]
fn test_interaction_state_accessors() {
    let mut scene = FigureGraph::new();
    let container_id = scene.set_contents(Box::new(RectangleFigure::new(0.0, 0.0, 200.0, 200.0)));
    assert_eq!(scene.mouse_target(), None);
    assert_eq!(scene.focus_owner(), None);
    assert_eq!(scene.captured(), None);
    scene.set_mouse_target(Some(container_id));
    scene.set_focus_owner(Some(container_id));
    scene.set_captured(Some(container_id));
    assert_eq!(scene.mouse_target(), Some(container_id));
    assert_eq!(scene.focus_owner(), Some(container_id));
    assert_eq!(scene.captured(), Some(container_id));
}

#[test]
fn test_apply_pending_add_child_attaches_detached_block() {
    let (mut scene, mut update_manager) = new_scene();
    let parent_id = scene.set_contents(Box::new(RectangleFigure::new(0.0, 0.0, 200.0, 200.0)));
    let child_id = scene.allocate_block(Box::new(RectangleFigure::new(10.0, 10.0, 50.0, 50.0)));

    assert_eq!(scene.get_block(child_id).unwrap().parent, None);

    assert!(scene.apply_pending_mutations(
        &mut update_manager,
        vec![PendingMutation::AddChild {
            parent: parent_id,
            child: child_id,
        }],
    ));

    assert_eq!(scene.get_block(child_id).unwrap().parent, Some(parent_id));
    assert!(scene.get_block(parent_id).unwrap().children.contains(&child_id));
    assert!(update_manager.has_pending_layout());
    assert!(update_manager.has_pending_repaint());
}

#[test]
fn test_apply_pending_remove_child_clears_interaction_state() {
    let (mut scene, mut update_manager) = new_scene();
    let parent_id = scene.set_contents(Box::new(RectangleFigure::new(0.0, 0.0, 200.0, 200.0)));
    let child_id = scene.add_child_to(parent_id, Box::new(RectangleFigure::new(10.0, 10.0, 50.0, 50.0)));
    scene.set_mouse_target(Some(child_id));
    scene.set_focus_owner(Some(child_id));
    scene.set_captured(Some(child_id));

    assert!(scene.apply_pending_mutations(
        &mut update_manager,
        vec![PendingMutation::RemoveChild {
            parent: parent_id,
            child: child_id,
        }],
    ));

    assert_eq!(scene.get_block(child_id).unwrap().parent, None);
    assert!(!scene.get_block(parent_id).unwrap().children.contains(&child_id));
    assert_eq!(scene.mouse_target(), None);
    assert_eq!(scene.focus_owner(), None);
    assert_eq!(scene.captured(), None);
}

#[test]
fn test_apply_pending_reparent_moves_child_between_containers() {
    let (mut scene, mut update_manager) = new_scene();
    let root_id = scene.set_contents(Box::new(RectangleFigure::new(0.0, 0.0, 400.0, 400.0)));
    let left_id = scene.add_child_to(root_id, Box::new(RectangleFigure::new(0.0, 0.0, 100.0, 100.0)));
    let right_id =
        scene.add_child_to(root_id, Box::new(RectangleFigure::new(200.0, 0.0, 100.0, 100.0)));
    let child_id = scene.add_child_to(left_id, Box::new(RectangleFigure::new(10.0, 10.0, 20.0, 20.0)));

    assert!(scene.apply_pending_mutations(
        &mut update_manager,
        vec![PendingMutation::Reparent {
            child: child_id,
            new_parent: right_id,
        }],
    ));

    assert_eq!(scene.get_block(child_id).unwrap().parent, Some(right_id));
    assert!(!scene.get_block(left_id).unwrap().children.contains(&child_id));
    assert!(scene.get_block(right_id).unwrap().children.contains(&child_id));
}
