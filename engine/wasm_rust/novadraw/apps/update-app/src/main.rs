use std::sync::{Arc, Mutex};

use novadraw::{
    BlockId, Color, FigureEvent, FigureGraph, NotificationEffect, Rectangle, RectangleFigure,
    SceneUpdateManager, UpdateEvent, UpdateListener, XYLayout,
};
use novadraw_apps::run_demo_app;

const WINDOW_WIDTH: f64 = 800.0;
const WINDOW_HEIGHT: f64 = 600.0;

struct CaptureListener {
    effects: Arc<Mutex<Vec<NotificationEffect>>>,
}

impl UpdateListener for CaptureListener {
    fn on_update_event(&self, event: UpdateEvent) {
        self.effects
            .lock()
            .unwrap()
            .push(NotificationEffect::EmitUpdate(event));
    }

    fn on_figure_event(&self, event: FigureEvent) {
        self.effects
            .lock()
            .unwrap()
            .push(NotificationEffect::EmitFigure(event));
    }

    fn on_notify(&self, block_id: BlockId) {
        self.effects
            .lock()
            .unwrap()
            .push(NotificationEffect::Notify { block_id });
    }
}

fn gray_bg() -> RectangleFigure {
    RectangleFigure::new_with_color(0.0, 0.0, WINDOW_WIDTH, WINDOW_HEIGHT, Color::hex("#eeeeee"))
}

fn create_scene_0_static_baseline() -> FigureGraph {
    let mut scene = FigureGraph::new();
    let container_id = scene.set_contents(Box::new(gray_bg()));

    let rect1 = RectangleFigure::new_with_color(100.0, 200.0, 150.0, 100.0, Color::hex("#e74c3c"));
    let rect2 = RectangleFigure::new_with_color(325.0, 200.0, 150.0, 100.0, Color::hex("#2ecc71"));
    let rect3 = RectangleFigure::new_with_color(550.0, 200.0, 150.0, 100.0, Color::hex("#3498db"));

    scene.add_child_to(container_id, Box::new(rect1));
    scene.add_child_to(container_id, Box::new(rect2));
    scene.add_child_to(container_id, Box::new(rect3));

    scene
}

fn create_scene_1_prim_translate() -> FigureGraph {
    let mut scene = FigureGraph::new();
    let container_id = scene.set_contents(Box::new(gray_bg()));

    let rect1 = RectangleFigure::new_with_color(100.0, 200.0, 120.0, 80.0, Color::hex("#e74c3c"));
    let rect2 = RectangleFigure::new_with_color(340.0, 200.0, 120.0, 80.0, Color::hex("#2ecc71"));
    let rect3 = RectangleFigure::new_with_color(580.0, 200.0, 120.0, 80.0, Color::hex("#3498db"));

    let _r1 = scene.add_child_to(container_id, Box::new(rect1));
    let r2 = scene.add_child_to(container_id, Box::new(rect2));
    let _r3 = scene.add_child_to(container_id, Box::new(rect3));

    scene.prim_translate(r2, 50.0, 30.0);

    let effects = scene.drain_notification_effects();
    println!("[Scene 1] prim_translate effects:");
    for effect in &effects {
        println!("  {:?}", effect);
    }

    let has_figure_moved = effects.iter().any(|e| {
        matches!(
            e,
            NotificationEffect::EmitFigure(FigureEvent::FigureMoved { .. })
        )
    });
    println!("  FigureMoved recorded: {}", has_figure_moved);

    scene
}

fn create_scene_2_coordinate_root() -> FigureGraph {
    let mut scene = FigureGraph::new();
    let container_id = scene.set_contents(Box::new(gray_bg()));

    let parent = RectangleFigure::new_with_color(100.0, 100.0, 600.0, 400.0, Color::hex("#bdc3c7"))
        .with_local_coordinates(true);
    let parent_id = scene.add_child_to(container_id, Box::new(parent));

    let child1 = RectangleFigure::new_with_color(130.0, 180.0, 150.0, 100.0, Color::hex("#e74c3c"));
    let child2 = RectangleFigure::new_with_color(450.0, 180.0, 150.0, 100.0, Color::hex("#3498db"));
    let _c1 = scene.add_child_to(parent_id, Box::new(child1));
    let _c2 = scene.add_child_to(parent_id, Box::new(child2));

    scene.drain_notification_effects();

    scene.prim_translate(parent_id, 20.0, 20.0);

    let effects = scene.drain_notification_effects();
    println!("[Scene 2] Coordinate Root effects:");
    for effect in &effects {
        println!("  {:?}", effect);
    }

    let has_coord_changed = effects.iter().any(|e| {
        matches!(
            e,
            NotificationEffect::EmitFigure(FigureEvent::CoordinateSystemChanged { .. })
        )
    });
    let has_child_moved = effects.iter().any(|e| {
        matches!(
            e,
            NotificationEffect::EmitFigure(FigureEvent::FigureMoved { block_id, .. })
                if *block_id != parent_id
        )
    });
    println!("  CoordinateSystemChanged recorded: {}", has_coord_changed);
    println!(
        "  Children FigureMoved (should be false): {}",
        has_child_moved
    );

    scene
}

fn create_scene_3_repaint() -> FigureGraph {
    let mut scene = FigureGraph::new();
    let container_id = scene.set_contents(Box::new(gray_bg()));

    let rect1 = RectangleFigure::new_with_color(100.0, 200.0, 120.0, 80.0, Color::hex("#e74c3c"));
    let rect2 = RectangleFigure::new_with_color(340.0, 200.0, 120.0, 80.0, Color::hex("#2ecc71"));
    let rect3 = RectangleFigure::new_with_color(580.0, 200.0, 120.0, 80.0, Color::hex("#3498db"));

    let _r1 = scene.add_child_to(container_id, Box::new(rect1));
    let r2 = scene.add_child_to(container_id, Box::new(rect2));
    let _r3 = scene.add_child_to(container_id, Box::new(rect3));

    let mut update_manager = SceneUpdateManager::new();
    scene.repaint(&mut update_manager, r2, None);

    println!("[Scene 3] repaint:");
    println!(
        "  has_pending_repaint: {}",
        update_manager.has_pending_repaint()
    );
    println!("  dirty_count: {}", update_manager.dirty_count());

    scene
}

fn create_scene_4_revalidate() -> FigureGraph {
    let mut scene = FigureGraph::new();
    let container_id = scene.set_contents(Box::new(gray_bg()));

    let inner = RectangleFigure::new_with_color(50.0, 50.0, 700.0, 500.0, Color::hex("#ecf0f1"));
    let inner_id = scene.add_child_to(container_id, Box::new(inner));

    let xy_layout = Arc::new(XYLayout::new());
    scene.set_block_layout_manager(inner_id, xy_layout);

    let child1 = RectangleFigure::new_with_color(0.0, 0.0, 150.0, 100.0, Color::hex("#e74c3c"));
    let child2 = RectangleFigure::new_with_color(0.0, 0.0, 150.0, 100.0, Color::hex("#3498db"));
    let c1 = scene.add_child_to(inner_id, Box::new(child1));
    let c2 = scene.add_child_to(inner_id, Box::new(child2));

    scene.set_constraint(c1, Rectangle::new(80.0, 100.0, 150.0, 100.0));
    scene.set_constraint(c2, Rectangle::new(350.0, 100.0, 150.0, 100.0));

    let mut update_manager = SceneUpdateManager::new();
    scene.mark_invalid(&mut update_manager, inner_id);

    println!("[Scene 4] revalidate:");
    println!(
        "  has_pending_layout: {}",
        update_manager.has_pending_layout()
    );
    println!("  is_update_queued: {}", update_manager.is_update_queued());

    scene.revalidate(inner_id);
    println!("  After revalidate: layout applied");

    scene
}

fn create_scene_5_notification_effects() -> FigureGraph {
    let mut scene = FigureGraph::new();
    let container_id = scene.set_contents(Box::new(gray_bg()));

    let coord_root =
        RectangleFigure::new_with_color(50.0, 80.0, 700.0, 440.0, Color::hex("#dfe6e9"))
            .with_local_coordinates(true);
    let coord_root_id = scene.add_child_to(container_id, Box::new(coord_root));

    let rect1 = RectangleFigure::new_with_color(100.0, 200.0, 120.0, 80.0, Color::hex("#e74c3c"));
    let rect2 = RectangleFigure::new_with_color(400.0, 200.0, 120.0, 80.0, Color::hex("#2ecc71"));
    let _r1 = scene.add_child_to(coord_root_id, Box::new(rect1));
    let r2 = scene.add_child_to(coord_root_id, Box::new(rect2));

    scene.drain_notification_effects();

    let mut update_manager = SceneUpdateManager::new();
    let captured: Arc<Mutex<Vec<NotificationEffect>>> = Arc::new(Mutex::new(Vec::new()));
    update_manager.add_listener(Box::new(CaptureListener {
        effects: captured.clone(),
    }));

    scene.prim_translate(r2, 50.0, 30.0);
    scene.repaint(&mut update_manager, r2, None);

    let _canvas = scene.perform_update(&mut update_manager);

    let effects = captured.lock().unwrap();
    println!("[Scene 5] Notification Effects after perform_update:");
    for effect in effects.iter() {
        println!("  {:?}", effect);
    }

    let has_validating = effects
        .iter()
        .any(|e| matches!(e, NotificationEffect::EmitUpdate(UpdateEvent::Validating)));
    let has_validated = effects
        .iter()
        .any(|e| matches!(e, NotificationEffect::EmitUpdate(UpdateEvent::Validated)));
    let has_painting = effects.iter().any(|e| {
        matches!(
            e,
            NotificationEffect::EmitUpdate(UpdateEvent::Painting { .. })
        )
    });
    let has_painted = effects.iter().any(|e| {
        matches!(
            e,
            NotificationEffect::EmitUpdate(UpdateEvent::Painted { .. })
        )
    });
    let has_figure_moved = effects.iter().any(|e| {
        matches!(
            e,
            NotificationEffect::EmitFigure(FigureEvent::FigureMoved { .. })
        )
    });
    let has_notify = effects
        .iter()
        .any(|e| matches!(e, NotificationEffect::Notify { .. }));

    println!("  Validating: {}", has_validating);
    println!("  Validated: {}", has_validated);
    println!("  Painting: {}", has_painting);
    println!("  Painted: {}", has_painted);
    println!("  FigureMoved: {}", has_figure_moved);
    println!("  Notify: {}", has_notify);

    scene
}

fn create_scene_6_damage_repair() -> FigureGraph {
    let mut scene = FigureGraph::new();
    let container_id = scene.set_contents(Box::new(gray_bg()));

    let rect1 = RectangleFigure::new_with_color(200.0, 200.0, 200.0, 150.0, Color::hex("#e74c3c"));
    let rect2 = RectangleFigure::new_with_color(300.0, 250.0, 200.0, 150.0, Color::hex("#3498db"));
    let _r1 = scene.add_child_to(container_id, Box::new(rect1));
    let r2 = scene.add_child_to(container_id, Box::new(rect2));

    scene.drain_notification_effects();

    let mut update_manager = SceneUpdateManager::new();
    let captured: Arc<Mutex<Vec<NotificationEffect>>> = Arc::new(Mutex::new(Vec::new()));
    update_manager.add_listener(Box::new(CaptureListener {
        effects: captured.clone(),
    }));

    let old_bounds = scene.figure_bounds(r2).unwrap();

    scene.prim_translate(r2, 80.0, 60.0);
    scene.repaint(&mut update_manager, r2, None);

    let damage_before = update_manager.compute_damage();
    println!("[Scene 6] Damage Repair:");
    println!("  rect2 old_bounds: {:?}", old_bounds);
    println!("  rect2 new_bounds: {:?}", scene.figure_bounds(r2).unwrap());
    println!("  Damage before perform_update: {:?}", damage_before);

    let _canvas = scene.perform_update(&mut update_manager);

    let effects = captured.lock().unwrap();
    for effect in effects.iter() {
        if let NotificationEffect::EmitUpdate(UpdateEvent::Painting { damage }) = effect {
            println!("  Painting damage: {:?}", damage);
        }
        if let NotificationEffect::EmitUpdate(UpdateEvent::Painted { damage }) = effect {
            println!("  Painted damage: {:?}", damage);
        }
    }

    scene
}

fn main() {
    run_demo_app(
        "Update App - UpdateManager 生命周期验证 (←→ 切换场景)",
        "update-app",
        vec![
            (
                "Static Baseline",
                Box::new(|| create_scene_0_static_baseline()),
            ),
            (
                "prim_translate",
                Box::new(|| create_scene_1_prim_translate()),
            ),
            (
                "Coordinate Root",
                Box::new(|| create_scene_2_coordinate_root()),
            ),
            ("repaint", Box::new(|| create_scene_3_repaint())),
            ("revalidate", Box::new(|| create_scene_4_revalidate())),
            (
                "Notification Effects",
                Box::new(|| create_scene_5_notification_effects()),
            ),
            ("Damage Repair", Box::new(|| create_scene_6_damage_repair())),
        ],
    )
    .expect("Failed to run app");
}
