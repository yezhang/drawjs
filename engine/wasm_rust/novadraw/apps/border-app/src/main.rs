//! Border App - Border 装饰器验证
//!
//! 验证 RectangleFigure 的 with_stroke 描边功能。

use novadraw_apps::run_demo_app;

const WINDOW_WIDTH: f64 = 800.0;
const WINDOW_HEIGHT: f64 = 600.0;

fn create_scene_0_rectangle_border() -> novadraw::SceneGraph {
    let mut scene = novadraw::SceneGraph::new();
    let container = novadraw::RectangleFigure::new(0.0, 0.0, WINDOW_WIDTH, WINDOW_HEIGHT);
    let container_id = scene.set_contents(Box::new(container));

    let rect1 = novadraw::RectangleFigure::new_with_color(50.0, 50.0, 200.0, 120.0, novadraw::Color::rgba(0.8, 0.9, 1.0, 1.0))
        .with_stroke(novadraw::Color::rgba(0.2, 0.2, 0.4, 1.0), 2.0);
    let rect2 = novadraw::RectangleFigure::new_with_color(300.0, 50.0, 200.0, 120.0, novadraw::Color::rgba(0.9, 0.8, 1.0, 1.0))
        .with_stroke(novadraw::Color::rgba(0.4, 0.2, 0.4, 1.0), 4.0);
    let rect3 = novadraw::RectangleFigure::new_with_color(550.0, 50.0, 200.0, 120.0, novadraw::Color::rgba(1.0, 0.9, 0.8, 1.0))
        .with_stroke(novadraw::Color::rgba(0.4, 0.4, 0.2, 1.0), 6.0);

    let _r1 = scene.add_child_to(container_id, Box::new(rect1));
    let _r2 = scene.add_child_to(container_id, Box::new(rect2));
    let _r3 = scene.add_child_to(container_id, Box::new(rect3));

    scene
}

fn create_scene_1_rounded_border() -> novadraw::SceneGraph {
    let mut scene = novadraw::SceneGraph::new();
    let container = novadraw::RectangleFigure::new(0.0, 0.0, WINDOW_WIDTH, WINDOW_HEIGHT);
    let container_id = scene.set_contents(Box::new(container));

    let rect1 = novadraw::RectangleFigure::new_with_color(100.0, 100.0, 150.0, 100.0, novadraw::Color::rgba(0.8, 0.9, 0.95, 1.0))
        .with_stroke(novadraw::Color::rgba(0.1, 0.3, 0.6, 1.0), 3.0);
    let rect2 = novadraw::RectangleFigure::new_with_color(350.0, 100.0, 200.0, 150.0, novadraw::Color::rgba(0.9, 0.95, 0.8, 1.0))
        .with_stroke(novadraw::Color::rgba(0.2, 0.5, 0.2, 1.0), 3.0);
    let rect3 = novadraw::RectangleFigure::new_with_color(550.0, 100.0, 150.0, 150.0, novadraw::Color::rgba(0.95, 0.9, 0.8, 1.0))
        .with_stroke(novadraw::Color::rgba(0.6, 0.4, 0.1, 1.0), 3.0);

    let _r1 = scene.add_child_to(container_id, Box::new(rect1));
    let _r2 = scene.add_child_to(container_id, Box::new(rect2));
    let _r3 = scene.add_child_to(container_id, Box::new(rect3));

    scene
}

fn create_scene_2_solid_line_border() -> novadraw::SceneGraph {
    let mut scene = novadraw::SceneGraph::new();
    let container = novadraw::RectangleFigure::new(0.0, 0.0, WINDOW_WIDTH, WINDOW_HEIGHT);
    let container_id = scene.set_contents(Box::new(container));

    let widths = [1.0, 2.0, 3.0, 4.0, 5.0];
    let colors = [
        novadraw::Color::rgba(0.2, 0.2, 0.2, 1.0),
        novadraw::Color::rgba(0.4, 0.4, 0.4, 1.0),
        novadraw::Color::rgba(0.6, 0.6, 0.6, 1.0),
        novadraw::Color::rgba(0.8, 0.8, 0.8, 1.0),
        novadraw::Color::rgba(0.0, 0.0, 0.0, 1.0),
    ];

    for (i, (width, color)) in widths.iter().zip(colors.iter()).enumerate() {
        let rect = novadraw::RectangleFigure::new_with_color(50.0 + i as f64 * 150.0, 50.0, 130.0, 80.0, novadraw::Color::rgba(0.95, 0.95, 0.95, 1.0))
            .with_stroke(*color, *width);
        let _rect = scene.add_child_to(container_id, Box::new(rect));
    }

    scene
}

fn create_scene_3_border_with_background() -> novadraw::SceneGraph {
    let mut scene = novadraw::SceneGraph::new();
    let container = novadraw::RectangleFigure::new(0.0, 0.0, WINDOW_WIDTH, WINDOW_HEIGHT);
    let container_id = scene.set_contents(Box::new(container));

    let bg_colors = [
        novadraw::Color::rgba(0.9, 0.4, 0.4, 1.0),
        novadraw::Color::rgba(0.4, 0.9, 0.4, 1.0),
        novadraw::Color::rgba(0.4, 0.4, 0.9, 1.0),
        novadraw::Color::rgba(0.9, 0.9, 0.4, 1.0),
    ];

    for (i, bg_color) in bg_colors.iter().enumerate() {
        let rect = novadraw::RectangleFigure::new_with_color(50.0 + i as f64 * 190.0, 100.0, 170.0, 120.0, *bg_color)
            .with_stroke(novadraw::Color::rgba(0.1, 0.1, 0.1, 1.0), 3.0);
        let _rect = scene.add_child_to(container_id, Box::new(rect));
    }

    scene
}

fn create_scene_4_nested_border() -> novadraw::SceneGraph {
    let mut scene = novadraw::SceneGraph::new();
    let container = novadraw::RectangleFigure::new(0.0, 0.0, WINDOW_WIDTH, WINDOW_HEIGHT);
    let container_id = scene.set_contents(Box::new(container));

    let outer = novadraw::RectangleFigure::new_with_color(100.0, 80.0, 600.0, 400.0, novadraw::Color::rgba(0.9, 0.95, 0.9, 1.0))
        .with_stroke(novadraw::Color::rgba(0.2, 0.4, 0.2, 1.0), 4.0);
    let _outer = scene.add_child_to(container_id, Box::new(outer));

    let inner1 = novadraw::RectangleFigure::new_with_color(130.0, 110.0, 180.0, 120.0, novadraw::Color::rgba(0.8, 0.9, 1.0, 1.0))
        .with_stroke(novadraw::Color::rgba(0.1, 0.2, 0.4, 1.0), 2.0);
    let _inner1 = scene.add_child_to(container_id, Box::new(inner1));

    let inner2 = novadraw::RectangleFigure::new_with_color(350.0, 110.0, 300.0, 120.0, novadraw::Color::rgba(1.0, 0.95, 0.8, 1.0))
        .with_stroke(novadraw::Color::rgba(0.4, 0.3, 0.1, 1.0), 2.0);
    let _inner2 = scene.add_child_to(container_id, Box::new(inner2));

    let inner3 = novadraw::RectangleFigure::new_with_color(130.0, 280.0, 520.0, 150.0, novadraw::Color::rgba(0.95, 0.9, 0.95, 1.0))
        .with_stroke(novadraw::Color::rgba(0.3, 0.2, 0.3, 1.0), 2.0);
    let _inner3 = scene.add_child_to(container_id, Box::new(inner3));

    scene
}

fn create_scene_5_border_animation() -> novadraw::SceneGraph {
    let mut scene = novadraw::SceneGraph::new();
    let container = novadraw::RectangleFigure::new(0.0, 0.0, WINDOW_WIDTH, WINDOW_HEIGHT);
    let container_id = scene.set_contents(Box::new(container));

    let anim_rect = novadraw::RectangleFigure::new_with_color(250.0, 180.0, 300.0, 200.0, novadraw::Color::rgba(0.85, 0.9, 0.95, 1.0))
        .with_stroke(novadraw::Color::rgba(0.2, 0.4, 0.7, 1.0), 3.0);
    let _rect = scene.add_child_to(container_id, Box::new(anim_rect));

    scene
}

fn create_scene_6_dynamic_border() -> novadraw::SceneGraph {
    let mut scene = novadraw::SceneGraph::new();
    let container = novadraw::RectangleFigure::new(0.0, 0.0, WINDOW_WIDTH, WINDOW_HEIGHT);
    let container_id = scene.set_contents(Box::new(container));

    let dynamic = novadraw::RectangleFigure::new_with_color(250.0, 200.0, 300.0, 200.0, novadraw::Color::rgba(0.9, 0.92, 0.95, 1.0))
        .with_stroke(novadraw::Color::rgba(0.3, 0.3, 0.6, 1.0), 2.0);
    let _dynamic = scene.add_child_to(container_id, Box::new(dynamic));

    scene
}

fn create_scene_7_composite_border() -> novadraw::SceneGraph {
    let mut scene = novadraw::SceneGraph::new();
    let container = novadraw::RectangleFigure::new(0.0, 0.0, WINDOW_WIDTH, WINDOW_HEIGHT);
    let container_id = scene.set_contents(Box::new(container));

    let outer = novadraw::RectangleFigure::new_with_color(200.0, 100.0, 400.0, 350.0, novadraw::Color::rgba(0.95, 0.95, 0.98, 1.0))
        .with_stroke(novadraw::Color::rgba(0.6, 0.6, 0.7, 1.0), 5.0);
    let _outer = scene.add_child_to(container_id, Box::new(outer));

    let inner = novadraw::RectangleFigure::new_with_color(230.0, 130.0, 340.0, 290.0, novadraw::Color::rgba(0.9, 0.92, 0.95, 1.0))
        .with_stroke(novadraw::Color::rgba(0.2, 0.3, 0.5, 1.0), 2.0);
    let _inner = scene.add_child_to(container_id, Box::new(inner));

    scene
}

fn create_scene_8_insets() -> novadraw::SceneGraph {
    let mut scene = novadraw::SceneGraph::new();
    let container = novadraw::RectangleFigure::new(0.0, 0.0, WINDOW_WIDTH, WINDOW_HEIGHT);
    let container_id = scene.set_contents(Box::new(container));

    let rect1 = novadraw::RectangleFigure::new_with_color(50.0, 50.0, 200.0, 150.0, novadraw::Color::rgba(0.9, 0.9, 0.95, 1.0))
        .with_stroke(novadraw::Color::rgba(0.3, 0.3, 0.4, 1.0), 2.0);
    let rect2 = novadraw::RectangleFigure::new_with_color(300.0, 50.0, 200.0, 150.0, novadraw::Color::rgba(0.9, 0.9, 0.95, 1.0))
        .with_stroke(novadraw::Color::rgba(0.3, 0.3, 0.4, 1.0), 2.0);
    let rect3 = novadraw::RectangleFigure::new_with_color(550.0, 50.0, 200.0, 150.0, novadraw::Color::rgba(0.9, 0.9, 0.95, 1.0))
        .with_stroke(novadraw::Color::rgba(0.3, 0.3, 0.4, 1.0), 2.0);

    let _r1 = scene.add_child_to(container_id, Box::new(rect1));
    let _r2 = scene.add_child_to(container_id, Box::new(rect2));
    let _r3 = scene.add_child_to(container_id, Box::new(rect3));

    scene
}

fn create_scene_9_special_border() -> novadraw::SceneGraph {
    let mut scene = novadraw::SceneGraph::new();
    let container = novadraw::RectangleFigure::new(0.0, 0.0, WINDOW_WIDTH, WINDOW_HEIGHT);
    let container_id = scene.set_contents(Box::new(container));

    for i in 0..10 {
        let offset = i as f64 * 2.0;
        let size_offset = i as f64 * 2.0;
        let rect = novadraw::RectangleFigure::new_with_color(
            200.0 + offset,
            150.0 + offset,
            400.0 - size_offset,
            250.0 - size_offset,
            novadraw::Color::rgba(0.3 + i as f64 * 0.05, 0.5 + i as f64 * 0.03, 0.7 - i as f64 * 0.02, 1.0),
        ).with_stroke(novadraw::Color::rgba(0.1, 0.2, 0.4, 1.0), 1.0);
        let _rect = scene.add_child_to(container_id, Box::new(rect));
    }

    scene
}

fn main() {
    run_demo_app(
        "Border App - Border 装饰器验证 (按数字键 0-9 切换场景)",
        vec![
            ("0: RectangleBorder 基本", Box::new(|| create_scene_0_rectangle_border())),
            ("1: 圆角边框", Box::new(|| create_scene_1_rounded_border())),
            ("2: 实线边框", Box::new(|| create_scene_2_solid_line_border())),
            ("3: 边框+背景", Box::new(|| create_scene_3_border_with_background())),
            ("4: 嵌套 Border", Box::new(|| create_scene_4_nested_border())),
            ("5: 边框动画", Box::new(|| create_scene_5_border_animation())),
            ("6: 动态边框", Box::new(|| create_scene_6_dynamic_border())),
            ("7: 组合 Border", Box::new(|| create_scene_7_composite_border())),
            ("8: 内边距", Box::new(|| create_scene_8_insets())),
            ("9: 特殊边框", Box::new(|| create_scene_9_special_border())),
        ],
    ).expect("Failed to run app");
}
