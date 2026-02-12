//! Shape App - 形状展示验证
//!
//! 验证各种 Figure 图形的渲染能力。

use novadraw_apps::run_demo_app;

const WINDOW_WIDTH: f64 = 800.0;
const WINDOW_HEIGHT: f64 = 600.0;

fn create_scene_0_rectangle() -> novadraw::SceneGraph {
    let mut scene = novadraw::SceneGraph::new();

    let container = novadraw::RectangleFigure::new(0.0, 0.0, WINDOW_WIDTH, WINDOW_HEIGHT);
    let container_id = scene.set_contents(Box::new(container));

    let rect_1 = novadraw::RectangleFigure::new_with_color(100.0, 100.0, 150.0, 100.0, novadraw::Color::rgba(0.8, 0.2, 0.2, 1.0));
    let rect_2 = novadraw::RectangleFigure::new_with_color(300.0, 200.0, 200.0, 150.0, novadraw::Color::rgba(0.2, 0.8, 0.2, 1.0));
    let rect_3 = novadraw::RectangleFigure::new_with_color(500.0, 100.0, 100.0, 100.0, novadraw::Color::rgba(0.2, 0.2, 0.8, 1.0));

    let _id_1 = scene.add_child_to(container_id, Box::new(rect_1));
    let _id_2 = scene.add_child_to(container_id, Box::new(rect_2));
    let _id_3 = scene.add_child_to(container_id, Box::new(rect_3));

    scene
}

fn create_scene_1_ellipse() -> novadraw::SceneGraph {
    let mut scene = novadraw::SceneGraph::new();

    let container = novadraw::RectangleFigure::new(0.0, 0.0, WINDOW_WIDTH, WINDOW_HEIGHT);
    let container_id = scene.set_contents(Box::new(container));

    let ellipse_1 = novadraw::EllipseFigure::new_with_color(200.0, 150.0, 150.0, 100.0, novadraw::Color::rgba(0.9, 0.5, 0.1, 1.0));
    let ellipse_2 = novadraw::EllipseFigure::new_with_color(450.0, 300.0, 200.0, 150.0, novadraw::Color::rgba(0.1, 0.5, 0.9, 1.0));
    let ellipse_3 = novadraw::EllipseFigure::new_with_color(600.0, 150.0, 100.0, 100.0, novadraw::Color::rgba(0.5, 0.1, 0.9, 1.0));

    let _id_1 = scene.add_child_to(container_id, Box::new(ellipse_1));
    let _id_2 = scene.add_child_to(container_id, Box::new(ellipse_2));
    let _id_3 = scene.add_child_to(container_id, Box::new(ellipse_3));

    scene
}

fn create_scene_2_lines() -> novadraw::SceneGraph {
    let mut scene = novadraw::SceneGraph::new();

    let container = novadraw::RectangleFigure::new(0.0, 0.0, WINDOW_WIDTH, WINDOW_HEIGHT);
    let container_id = scene.set_contents(Box::new(container));

    // 水平线 - 红色
    let line_1 = novadraw::LineFigure::new_with_color(100.0, 100.0, 400.0, 100.0, novadraw::Color::rgba(0.9, 0.2, 0.2, 1.0))
        .with_width(3.0);
    // 垂直线 - 蓝色
    let line_2 = novadraw::LineFigure::new_with_color(450.0, 50.0, 450.0, 250.0, novadraw::Color::rgba(0.2, 0.2, 0.9, 1.0))
        .with_width(3.0);
    // 斜线 - 绿色
    let line_3 = novadraw::LineFigure::new_with_color(100.0, 350.0, 300.0, 500.0, novadraw::Color::rgba(0.2, 0.9, 0.2, 1.0))
        .with_width(3.0);
    // 粗线 - 紫色
    let line_4 = novadraw::LineFigure::new_with_color(400.0, 400.0, 700.0, 450.0, novadraw::Color::rgba(0.6, 0.3, 0.9, 1.0))
        .with_width(6.0);

    let _id_1 = scene.add_child_to(container_id, Box::new(line_1));
    let _id_2 = scene.add_child_to(container_id, Box::new(line_2));
    let _id_3 = scene.add_child_to(container_id, Box::new(line_3));
    let _id_4 = scene.add_child_to(container_id, Box::new(line_4));

    scene
}

fn create_scene_3_rect_stroke() -> novadraw::SceneGraph {
    let mut scene = novadraw::SceneGraph::new();

    let container = novadraw::RectangleFigure::new(0.0, 0.0, WINDOW_WIDTH, WINDOW_HEIGHT);
    let container_id = scene.set_contents(Box::new(container));

    // 填充 + 描边矩形
    let rect_1 = novadraw::RectangleFigure::new_with_color(100.0, 100.0, 200.0, 150.0, novadraw::Color::rgba(0.9, 0.5, 0.1, 1.0))
        .with_stroke(novadraw::Color::WHITE, 3.0);

    let rect_2 = novadraw::RectangleFigure::new_with_color(400.0, 100.0, 200.0, 150.0, novadraw::Color::rgba(0.1, 0.5, 0.9, 1.0))
        .with_stroke(novadraw::Color::rgba(1.0, 1.0, 0.0, 1.0), 5.0);

    let rect_3 = novadraw::RectangleFigure::new_with_color(100.0, 350.0, 200.0, 150.0, novadraw::Color::rgba(0.2, 0.8, 0.3, 1.0))
        .with_stroke(novadraw::Color::RED, 2.0);

    let rect_4 = novadraw::RectangleFigure::new_with_color(400.0, 350.0, 200.0, 150.0, novadraw::Color::rgba(0.9, 0.2, 0.2, 1.0))
        .with_stroke(novadraw::Color::WHITE, 1.0);

    let _id_1 = scene.add_child_to(container_id, Box::new(rect_1));
    let _id_2 = scene.add_child_to(container_id, Box::new(rect_2));
    let _id_3 = scene.add_child_to(container_id, Box::new(rect_3));
    let _id_4 = scene.add_child_to(container_id, Box::new(rect_4));

    scene
}

fn main() {
    run_demo_app(
        "Shape App - 形状展示验证 (按数字键 0-9 切换场景)",
        vec![
            ("0: Rectangle 填充", Box::new(|| create_scene_0_rectangle())),
            ("1: Ellipse 椭圆", Box::new(|| create_scene_1_ellipse())),
            ("2: Line 直线", Box::new(|| create_scene_2_lines())),
            ("3: Rectangle 描边", Box::new(|| create_scene_3_rect_stroke())),
        ],
    ).expect("Failed to run app");
}
