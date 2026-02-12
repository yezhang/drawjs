//! Transform App - 坐标变换验证
//!
//! 验证平移、缩放、旋转变换的正确性。

use novadraw_apps::run_demo_app;

const WINDOW_WIDTH: f64 = 800.0;
const WINDOW_HEIGHT: f64 = 600.0;

fn create_scene_0_translate() -> novadraw::SceneGraph {
    let mut scene = novadraw::SceneGraph::new();
    let container = novadraw::RectangleFigure::new(0.0, 0.0, WINDOW_WIDTH, WINDOW_HEIGHT);
    let container_id = scene.set_contents(Box::new(container));

    let rect = novadraw::RectangleFigure::new_with_color(300.0, 250.0, 200.0, 100.0, novadraw::Color::rgba(0.2, 0.6, 0.9, 1.0));
    let _rect_id = scene.add_child_to(container_id, Box::new(rect));

    let h_line = novadraw::RectangleFigure::new_with_color(0.0, 300.0, WINDOW_WIDTH, 2.0, novadraw::Color::rgba(0.5, 0.5, 0.5, 1.0));
    let v_line = novadraw::RectangleFigure::new_with_color(400.0, 0.0, 2.0, WINDOW_HEIGHT, novadraw::Color::rgba(0.5, 0.5, 0.5, 1.0));
    let _h = scene.add_child_to(container_id, Box::new(h_line));
    let _v = scene.add_child_to(container_id, Box::new(v_line));

    scene
}

fn create_scene_1_scale_center() -> novadraw::SceneGraph {
    let mut scene = novadraw::SceneGraph::new();
    let container = novadraw::RectangleFigure::new(0.0, 0.0, WINDOW_WIDTH, WINDOW_HEIGHT);
    let container_id = scene.set_contents(Box::new(container));

    let rect = novadraw::RectangleFigure::new_with_color(300.0, 200.0, 200.0, 200.0, novadraw::Color::rgba(0.9, 0.3, 0.3, 1.0));
    let _rect_id = scene.add_child_to(container_id, Box::new(rect));

    let center = novadraw::EllipseFigure::new_with_color(390.0, 290.0, 20.0, 20.0, novadraw::Color::rgba(1.0, 1.0, 0.0, 1.0));
    let _center = scene.add_child_to(container_id, Box::new(center));

    scene
}

fn create_scene_2_scale_anchor() -> novadraw::SceneGraph {
    let mut scene = novadraw::SceneGraph::new();
    let container = novadraw::RectangleFigure::new(0.0, 0.0, WINDOW_WIDTH, WINDOW_HEIGHT);
    let container_id = scene.set_contents(Box::new(container));

    let rect = novadraw::RectangleFigure::new_with_color(100.0, 100.0, 150.0, 100.0, novadraw::Color::rgba(0.3, 0.9, 0.3, 1.0));
    let _rect_id = scene.add_child_to(container_id, Box::new(rect));

    let anchor = novadraw::EllipseFigure::new_with_color(90.0, 90.0, 20.0, 20.0, novadraw::Color::rgba(1.0, 0.0, 1.0, 1.0));
    let _anchor = scene.add_child_to(container_id, Box::new(anchor));

    scene
}

fn create_scene_3_rotate() -> novadraw::SceneGraph {
    let mut scene = novadraw::SceneGraph::new();
    let container = novadraw::RectangleFigure::new(0.0, 0.0, WINDOW_WIDTH, WINDOW_HEIGHT);
    let container_id = scene.set_contents(Box::new(container));

    let center = novadraw::EllipseFigure::new_with_color(390.0, 290.0, 20.0, 20.0, novadraw::Color::rgba(1.0, 1.0, 0.0, 1.0));
    let _center = scene.add_child_to(container_id, Box::new(center));

    let rect = novadraw::RectangleFigure::new_with_color(300.0, 150.0, 200.0, 100.0, novadraw::Color::rgba(0.3, 0.3, 0.9, 1.0));
    let _rect_id = scene.add_child_to(container_id, Box::new(rect));

    scene
}

fn create_scene_4_transform_propagation() -> novadraw::SceneGraph {
    let mut scene = novadraw::SceneGraph::new();
    let container = novadraw::RectangleFigure::new(0.0, 0.0, WINDOW_WIDTH, WINDOW_HEIGHT);
    let container_id = scene.set_contents(Box::new(container));

    let parent = novadraw::RectangleFigure::new_with_color(200.0, 150.0, 400.0, 300.0, novadraw::Color::rgba(0.9, 0.5, 0.1, 1.0));
    let parent_id = scene.add_child_to(container_id, Box::new(parent));

    let child = novadraw::RectangleFigure::new_with_color(250.0, 200.0, 100.0, 80.0, novadraw::Color::rgba(0.2, 0.8, 0.4, 1.0));
    let _child_id = scene.add_child_to(parent_id, Box::new(child));

    let grandchild = novadraw::RectangleFigure::new_with_color(280.0, 230.0, 50.0, 40.0, novadraw::Color::rgba(0.8, 0.2, 0.6, 1.0));
    let _grandchild_id = scene.add_child_to(parent_id, Box::new(grandchild));

    scene
}

fn create_scene_5_local_coords() -> novadraw::SceneGraph {
    let mut scene = novadraw::SceneGraph::new();
    let container = novadraw::RectangleFigure::new(0.0, 0.0, WINDOW_WIDTH, WINDOW_HEIGHT);
    let container_id = scene.set_contents(Box::new(container));

    let parent = novadraw::RectangleFigure::new_with_color(100.0, 100.0, 300.0, 200.0, novadraw::Color::rgba(0.6, 0.3, 0.8, 1.0))
        .with_local_coordinates(true);
    let parent_id = scene.add_child_to(container_id, Box::new(parent));

    let child = novadraw::RectangleFigure::new_with_color(20.0, 20.0, 100.0, 60.0, novadraw::Color::rgba(0.3, 0.8, 0.7, 1.0));
    let _child_id = scene.add_child_to(parent_id, Box::new(child));

    scene
}

fn create_scene_6_transform_matrix() -> novadraw::SceneGraph {
    let mut scene = novadraw::SceneGraph::new();
    let container = novadraw::RectangleFigure::new(0.0, 0.0, WINDOW_WIDTH, WINDOW_HEIGHT);
    let container_id = scene.set_contents(Box::new(container));

    let rect = novadraw::RectangleFigure::new_with_color(250.0, 200.0, 300.0, 150.0, novadraw::Color::rgba(0.2, 0.5, 0.9, 1.0));
    let _rect_id = scene.add_child_to(container_id, Box::new(rect));

    scene
}

fn create_scene_7_scale_animation() -> novadraw::SceneGraph {
    let mut scene = novadraw::SceneGraph::new();
    let container = novadraw::RectangleFigure::new(0.0, 0.0, WINDOW_WIDTH, WINDOW_HEIGHT);
    let container_id = scene.set_contents(Box::new(container));

    let rect = novadraw::RectangleFigure::new_with_color(300.0, 200.0, 200.0, 200.0, novadraw::Color::rgba(0.9, 0.2, 0.4, 1.0));
    let _rect_id = scene.add_child_to(container_id, Box::new(rect));

    scene
}

fn create_scene_8_rotate_animation() -> novadraw::SceneGraph {
    let mut scene = novadraw::SceneGraph::new();
    let container = novadraw::RectangleFigure::new(0.0, 0.0, WINDOW_WIDTH, WINDOW_HEIGHT);
    let container_id = scene.set_contents(Box::new(container));

    let rect = novadraw::RectangleFigure::new_with_color(300.0, 200.0, 200.0, 100.0, novadraw::Color::rgba(0.4, 0.9, 0.2, 1.0));
    let _rect_id = scene.add_child_to(container_id, Box::new(rect));

    scene
}

fn create_scene_9_screen_to_world() -> novadraw::SceneGraph {
    let mut scene = novadraw::SceneGraph::new();
    let container = novadraw::RectangleFigure::new(0.0, 0.0, WINDOW_WIDTH, WINDOW_HEIGHT);
    let container_id = scene.set_contents(Box::new(container));

    let rect = novadraw::RectangleFigure::new_with_color(200.0, 150.0, 400.0, 300.0, novadraw::Color::rgba(0.8, 0.6, 0.2, 1.0));
    let _rect_id = scene.add_child_to(container_id, Box::new(rect));

    scene
}

fn main() {
    run_demo_app(
        "Transform App - 坐标变换验证 (按数字键 0-9 切换场景)",
        vec![
            ("0: 平移验证", Box::new(|| create_scene_0_translate())),
            ("1: 缩放(中心)", Box::new(|| create_scene_1_scale_center())),
            ("2: 缩放(基准点)", Box::new(|| create_scene_2_scale_anchor())),
            ("3: 旋转变换", Box::new(|| create_scene_3_rotate())),
            ("4: 变换传播", Box::new(|| create_scene_4_transform_propagation())),
            ("5: 局部坐标", Box::new(|| create_scene_5_local_coords())),
            ("6: 变换组合", Box::new(|| create_scene_6_transform_matrix())),
            ("7: 缩放动画", Box::new(|| create_scene_7_scale_animation())),
            ("8: 旋转动画", Box::new(|| create_scene_8_rotate_animation())),
            ("9: 屏幕到世界", Box::new(|| create_scene_9_screen_to_world())),
        ],
    ).expect("Failed to run app");
}
