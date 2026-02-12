//! Layout App - 布局管理器验证
//!
//! 验证各种布局管理器的正确性。

use novadraw_apps::run_demo_app;

const WINDOW_WIDTH: f64 = 800.0;
const WINDOW_HEIGHT: f64 = 600.0;

fn create_scene_0_flow_horizontal() -> novadraw::SceneGraph {
    let mut scene = novadraw::SceneGraph::new();
    let container = novadraw::RectangleFigure::new(0.0, 0.0, WINDOW_WIDTH, WINDOW_HEIGHT);
    let container_id = scene.set_contents(Box::new(container));

    let rect1 = novadraw::RectangleFigure::new_with_color(50.0, 50.0, 100.0, 60.0, novadraw::Color::rgba(0.9, 0.3, 0.3, 1.0));
    let rect2 = novadraw::RectangleFigure::new_with_color(160.0, 50.0, 120.0, 60.0, novadraw::Color::rgba(0.3, 0.9, 0.3, 1.0));
    let rect3 = novadraw::RectangleFigure::new_with_color(290.0, 50.0, 80.0, 60.0, novadraw::Color::rgba(0.3, 0.3, 0.9, 1.0));
    let rect4 = novadraw::RectangleFigure::new_with_color(380.0, 50.0, 100.0, 60.0, novadraw::Color::rgba(0.9, 0.9, 0.3, 1.0));

    let _r1 = scene.add_child_to(container_id, Box::new(rect1));
    let _r2 = scene.add_child_to(container_id, Box::new(rect2));
    let _r3 = scene.add_child_to(container_id, Box::new(rect3));
    let _r4 = scene.add_child_to(container_id, Box::new(rect4));

    scene
}

fn create_scene_1_flow_vertical() -> novadraw::SceneGraph {
    let mut scene = novadraw::SceneGraph::new();
    let container = novadraw::RectangleFigure::new(0.0, 0.0, WINDOW_WIDTH, WINDOW_HEIGHT);
    let container_id = scene.set_contents(Box::new(container));

    let rect1 = novadraw::RectangleFigure::new_with_color(300.0, 50.0, 200.0, 50.0, novadraw::Color::rgba(0.9, 0.5, 0.1, 1.0));
    let rect2 = novadraw::RectangleFigure::new_with_color(300.0, 110.0, 200.0, 70.0, novadraw::Color::rgba(0.1, 0.5, 0.9, 1.0));
    let rect3 = novadraw::RectangleFigure::new_with_color(300.0, 190.0, 200.0, 60.0, novadraw::Color::rgba(0.5, 0.1, 0.9, 1.0));
    let rect4 = novadraw::RectangleFigure::new_with_color(300.0, 260.0, 200.0, 80.0, novadraw::Color::rgba(0.6, 0.8, 0.2, 1.0));

    let _r1 = scene.add_child_to(container_id, Box::new(rect1));
    let _r2 = scene.add_child_to(container_id, Box::new(rect2));
    let _r3 = scene.add_child_to(container_id, Box::new(rect3));
    let _r4 = scene.add_child_to(container_id, Box::new(rect4));

    scene
}

fn create_scene_2_flow_wrap() -> novadraw::SceneGraph {
    let mut scene = novadraw::SceneGraph::new();
    let container = novadraw::RectangleFigure::new(0.0, 0.0, WINDOW_WIDTH, WINDOW_HEIGHT);
    let container_id = scene.set_contents(Box::new(container));

    for i in 0..8 {
        let rect = novadraw::RectangleFigure::new_with_color(
            50.0 + (i % 4) as f64 * 170.0,
            50.0 + (i / 4) as f64 * 100.0,
            100.0 + i as f64 * 5.0,
            50.0 + i as f64 * 3.0,
            novadraw::Color::rgba((i as f64 * 0.15) % 1.0, 0.5, 0.7, 1.0),
        );
        let _rect = scene.add_child_to(container_id, Box::new(rect));
    }

    scene
}

fn create_scene_3_border_layout() -> novadraw::SceneGraph {
    let mut scene = novadraw::SceneGraph::new();
    let container = novadraw::RectangleFigure::new(0.0, 0.0, WINDOW_WIDTH, WINDOW_HEIGHT);
    let container_id = scene.set_contents(Box::new(container));

    let north = novadraw::RectangleFigure::new_with_color(150.0, 20.0, 500.0, 80.0, novadraw::Color::rgba(0.9, 0.3, 0.3, 1.0));
    let _north = scene.add_child_to(container_id, Box::new(north));

    let south = novadraw::RectangleFigure::new_with_color(150.0, 500.0, 500.0, 80.0, novadraw::Color::rgba(0.3, 0.9, 0.3, 1.0));
    let _south = scene.add_child_to(container_id, Box::new(south));

    let west = novadraw::RectangleFigure::new_with_color(20.0, 110.0, 120.0, 380.0, novadraw::Color::rgba(0.3, 0.3, 0.9, 1.0));
    let _west = scene.add_child_to(container_id, Box::new(west));

    let east = novadraw::RectangleFigure::new_with_color(660.0, 110.0, 120.0, 380.0, novadraw::Color::rgba(0.9, 0.9, 0.3, 1.0));
    let _east = scene.add_child_to(container_id, Box::new(east));

    let center = novadraw::RectangleFigure::new_with_color(150.0, 110.0, 500.0, 380.0, novadraw::Color::rgba(0.6, 0.4, 0.7, 1.0));
    let _center = scene.add_child_to(container_id, Box::new(center));

    scene
}

fn create_scene_4_border_nested() -> novadraw::SceneGraph {
    let mut scene = novadraw::SceneGraph::new();
    let container = novadraw::RectangleFigure::new(0.0, 0.0, WINDOW_WIDTH, WINDOW_HEIGHT);
    let container_id = scene.set_contents(Box::new(container));

    let north = novadraw::RectangleFigure::new_with_color(100.0, 10.0, 600.0, 60.0, novadraw::Color::rgba(0.8, 0.4, 0.4, 1.0));
    let _north = scene.add_child_to(container_id, Box::new(north));

    let south = novadraw::RectangleFigure::new_with_color(100.0, 490.0, 600.0, 60.0, novadraw::Color::rgba(0.8, 0.8, 0.4, 1.0));
    let _south = scene.add_child_to(container_id, Box::new(south));

    let west = novadraw::RectangleFigure::new_with_color(100.0, 80.0, 100.0, 400.0, novadraw::Color::rgba(0.4, 0.8, 0.4, 1.0));
    let _west = scene.add_child_to(container_id, Box::new(west));

    let east = novadraw::RectangleFigure::new_with_color(600.0, 80.0, 100.0, 400.0, novadraw::Color::rgba(0.4, 0.4, 0.8, 1.0));
    let _east = scene.add_child_to(container_id, Box::new(east));

    let center = novadraw::RectangleFigure::new_with_color(210.0, 80.0, 380.0, 400.0, novadraw::Color::rgba(0.6, 0.6, 0.6, 1.0));
    let center_id = scene.add_child_to(container_id, Box::new(center));

    for i in 0..4 {
        let rect = novadraw::RectangleFigure::new_with_color(
            220.0 + i as f64 * 90.0,
            90.0,
            80.0,
            40.0,
            novadraw::Color::rgba((i as f64 * 0.2) % 1.0, 0.6, 0.8, 1.0),
        );
        let _rect = scene.add_child_to(center_id, Box::new(rect));
    }

    scene
}

fn create_scene_5_stack_layout() -> novadraw::SceneGraph {
    let mut scene = novadraw::SceneGraph::new();
    let container = novadraw::RectangleFigure::new(0.0, 0.0, WINDOW_WIDTH, WINDOW_HEIGHT);
    let container_id = scene.set_contents(Box::new(container));

    let card1 = novadraw::RectangleFigure::new_with_color(250.0, 150.0, 300.0, 250.0, novadraw::Color::rgba(0.9, 0.3, 0.3, 1.0));
    let card2 = novadraw::RectangleFigure::new_with_color(270.0, 170.0, 300.0, 250.0, novadraw::Color::rgba(0.3, 0.9, 0.3, 1.0));
    let card3 = novadraw::RectangleFigure::new_with_color(290.0, 190.0, 300.0, 250.0, novadraw::Color::rgba(0.3, 0.3, 0.9, 1.0));
    let card4 = novadraw::RectangleFigure::new_with_color(310.0, 210.0, 300.0, 250.0, novadraw::Color::rgba(0.9, 0.9, 0.3, 1.0));

    let _c1 = scene.add_child_to(container_id, Box::new(card1));
    let _c2 = scene.add_child_to(container_id, Box::new(card2));
    let _c3 = scene.add_child_to(container_id, Box::new(card3));
    let _c4 = scene.add_child_to(container_id, Box::new(card4));

    scene
}

fn create_scene_6_xy_layout() -> novadraw::SceneGraph {
    let mut scene = novadraw::SceneGraph::new();
    let container = novadraw::RectangleFigure::new(0.0, 0.0, WINDOW_WIDTH, WINDOW_HEIGHT);
    let container_id = scene.set_contents(Box::new(container));

    let positions = [
        (50.0, 50.0, 150.0, 100.0, novadraw::Color::rgba(0.9, 0.3, 0.3, 1.0)),
        (250.0, 100.0, 200.0, 80.0, novadraw::Color::rgba(0.3, 0.9, 0.3, 1.0)),
        (500.0, 50.0, 120.0, 120.0, novadraw::Color::rgba(0.3, 0.3, 0.9, 1.0)),
        (100.0, 300.0, 180.0, 150.0, novadraw::Color::rgba(0.9, 0.9, 0.3, 1.0)),
        (350.0, 250.0, 250.0, 200.0, novadraw::Color::rgba(0.9, 0.5, 0.1, 1.0)),
        (150.0, 480.0, 200.0, 70.0, novadraw::Color::rgba(0.5, 0.1, 0.9, 1.0)),
    ];

    for (x, y, w, h, color) in positions {
        let rect = novadraw::RectangleFigure::new_with_color(x, y, w, h, color);
        let _rect = scene.add_child_to(container_id, Box::new(rect));
    }

    scene
}

fn create_scene_7_grid_layout() -> novadraw::SceneGraph {
    let mut scene = novadraw::SceneGraph::new();
    let container = novadraw::RectangleFigure::new(0.0, 0.0, WINDOW_WIDTH, WINDOW_HEIGHT);
    let container_id = scene.set_contents(Box::new(container));

    for row in 0..3 {
        for col in 0..3 {
            let rect = novadraw::RectangleFigure::new_with_color(
                150.0 + col as f64 * 170.0,
                100.0 + row as f64 * 130.0,
                150.0,
                110.0,
                novadraw::Color::rgba((col as f64 * 0.3) % 1.0, (row as f64 * 0.3) % 1.0, 0.5, 1.0),
            );
            let _rect = scene.add_child_to(container_id, Box::new(rect));
        }
    }

    scene
}

fn create_scene_8_layout_nesting() -> novadraw::SceneGraph {
    let mut scene = novadraw::SceneGraph::new();
    let container = novadraw::RectangleFigure::new(0.0, 0.0, WINDOW_WIDTH, WINDOW_HEIGHT);
    let container_id = scene.set_contents(Box::new(container));

    let outer = novadraw::RectangleFigure::new_with_color(100.0, 50.0, 600.0, 500.0, novadraw::Color::rgba(0.7, 0.7, 0.7, 1.0));
    let outer_id = scene.add_child_to(container_id, Box::new(outer));

    for i in 0..4 {
        let rect = novadraw::RectangleFigure::new_with_color(
            120.0 + i as f64 * 140.0,
            70.0,
            120.0,
            80.0,
            novadraw::Color::rgba((i as f64 * 0.25) % 1.0, 0.6, 0.8, 1.0),
        );
        let _rect = scene.add_child_to(outer_id, Box::new(rect));
    }

    scene
}

fn create_scene_9_layout_constraints() -> novadraw::SceneGraph {
    let mut scene = novadraw::SceneGraph::new();
    let container = novadraw::RectangleFigure::new(0.0, 0.0, WINDOW_WIDTH, WINDOW_HEIGHT);
    let container_id = scene.set_contents(Box::new(container));

    let fixed = novadraw::RectangleFigure::new_with_color(50.0, 50.0, 150.0, 100.0, novadraw::Color::rgba(0.9, 0.3, 0.3, 1.0));
    let flexible = novadraw::RectangleFigure::new_with_color(250.0, 50.0, 300.0, 100.0, novadraw::Color::rgba(0.3, 0.9, 0.3, 1.0));
    let min_size = novadraw::RectangleFigure::new_with_color(600.0, 50.0, 100.0, 60.0, novadraw::Color::rgba(0.3, 0.3, 0.9, 1.0));
    let max_size = novadraw::RectangleFigure::new_with_color(50.0, 200.0, 200.0, 150.0, novadraw::Color::rgba(0.9, 0.9, 0.3, 1.0));

    let _fixed = scene.add_child_to(container_id, Box::new(fixed));
    let _flex = scene.add_child_to(container_id, Box::new(flexible));
    let _min = scene.add_child_to(container_id, Box::new(min_size));
    let _max = scene.add_child_to(container_id, Box::new(max_size));

    scene
}

fn main() {
    run_demo_app(
        "Layout App - 布局管理器验证 (按数字键 0-9 切换场景)",
        vec![
            ("0: FlowLayout 水平", Box::new(|| create_scene_0_flow_horizontal())),
            ("1: FlowLayout 垂直", Box::new(|| create_scene_1_flow_vertical())),
            ("2: FlowLayout 换行", Box::new(|| create_scene_2_flow_wrap())),
            ("3: BorderLayout", Box::new(|| create_scene_3_border_layout())),
            ("4: BorderLayout 嵌套", Box::new(|| create_scene_4_border_nested())),
            ("5: StackLayout", Box::new(|| create_scene_5_stack_layout())),
            ("6: XYLayout 绝对", Box::new(|| create_scene_6_xy_layout())),
            ("7: GridLayout", Box::new(|| create_scene_7_grid_layout())),
            ("8: 布局嵌套", Box::new(|| create_scene_8_layout_nesting())),
            ("9: 布局约束", Box::new(|| create_scene_9_layout_constraints())),
        ],
    ).expect("Failed to run app");
}
