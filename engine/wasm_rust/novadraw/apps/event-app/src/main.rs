//! Event App - 事件处理验证
//!
//! 验证鼠标、键盘事件处理的正确性。

use novadraw_apps::run_demo_app;

const WINDOW_WIDTH: f64 = 800.0;
const WINDOW_HEIGHT: f64 = 600.0;

fn create_scene_0_mouse_enter_exit() -> novadraw::SceneGraph {
    let mut scene = novadraw::SceneGraph::new();
    let container = novadraw::RectangleFigure::new(0.0, 0.0, WINDOW_WIDTH, WINDOW_HEIGHT);
    let container_id = scene.set_contents(Box::new(container));

    let hover_area = novadraw::RectangleFigure::new_with_color(300.0, 200.0, 200.0, 150.0, novadraw::Color::rgba(0.4, 0.6, 0.8, 1.0));
    let _area = scene.add_child_to(container_id, Box::new(hover_area));

    let info = novadraw::RectangleFigure::new_with_color(50.0, 50.0, 700.0, 40.0, novadraw::Color::rgba(0.9, 0.9, 0.8, 1.0));
    let _info = scene.add_child_to(container_id, Box::new(info));

    scene
}

fn create_scene_1_mouse_hover() -> novadraw::SceneGraph {
    let mut scene = novadraw::SceneGraph::new();
    let container = novadraw::RectangleFigure::new(0.0, 0.0, WINDOW_WIDTH, WINDOW_HEIGHT);
    let container_id = scene.set_contents(Box::new(container));

    let hover_area = novadraw::RectangleFigure::new_with_color(250.0, 150.0, 300.0, 200.0, novadraw::Color::rgba(0.3, 0.7, 0.5, 1.0));
    let _area = scene.add_child_to(container_id, Box::new(hover_area));

    let status = novadraw::RectangleFigure::new_with_color(50.0, 500.0, 700.0, 50.0, novadraw::Color::rgba(0.8, 0.8, 0.8, 1.0));
    let _status = scene.add_child_to(container_id, Box::new(status));

    scene
}

fn create_scene_2_mouse_down_up() -> novadraw::SceneGraph {
    let mut scene = novadraw::SceneGraph::new();
    let container = novadraw::RectangleFigure::new(0.0, 0.0, WINDOW_WIDTH, WINDOW_HEIGHT);
    let container_id = scene.set_contents(Box::new(container));

    let click_area = novadraw::RectangleFigure::new_with_color(300.0, 200.0, 200.0, 100.0, novadraw::Color::rgba(0.7, 0.4, 0.4, 1.0));
    let _area = scene.add_child_to(container_id, Box::new(click_area));

    let status = novadraw::RectangleFigure::new_with_color(50.0, 50.0, 700.0, 60.0, novadraw::Color::rgba(0.9, 0.9, 0.9, 1.0));
    let _status = scene.add_child_to(container_id, Box::new(status));

    scene
}

fn create_scene_3_mouse_move_basic() -> novadraw::SceneGraph {
    let mut scene = novadraw::SceneGraph::new();
    let container = novadraw::RectangleFigure::new(0.0, 0.0, WINDOW_WIDTH, WINDOW_HEIGHT);
    let container_id = scene.set_contents(Box::new(container));

    let draggable = novadraw::RectangleFigure::new_with_color(350.0, 250.0, 100.0, 60.0, novadraw::Color::rgba(0.5, 0.5, 0.8, 1.0));
    let _drag = scene.add_child_to(container_id, Box::new(draggable));

    scene
}

fn create_scene_4_mouse_drag() -> novadraw::SceneGraph {
    let mut scene = novadraw::SceneGraph::new();
    let container = novadraw::RectangleFigure::new(0.0, 0.0, WINDOW_WIDTH, WINDOW_HEIGHT);
    let container_id = scene.set_contents(Box::new(container));

    let colors = [
        novadraw::Color::rgba(0.9, 0.3, 0.3, 1.0),
        novadraw::Color::rgba(0.3, 0.9, 0.3, 1.0),
        novadraw::Color::rgba(0.3, 0.3, 0.9, 1.0),
        novadraw::Color::rgba(0.9, 0.9, 0.3, 1.0),
        novadraw::Color::rgba(0.9, 0.3, 0.9, 1.0),
    ];

    let positions = [(100.0, 100.0), (250.0, 100.0), (400.0, 100.0), (100.0, 250.0), (250.0, 250.0)];

    for ((x, y), color) in positions.iter().zip(colors.iter()) {
        let rect = novadraw::RectangleFigure::new_with_color(*x, *y, 100.0, 80.0, *color);
        let _rect = scene.add_child_to(container_id, Box::new(rect));
    }

    scene
}

fn create_scene_5_key_down_up() -> novadraw::SceneGraph {
    let mut scene = novadraw::SceneGraph::new();
    let container = novadraw::RectangleFigure::new(0.0, 0.0, WINDOW_WIDTH, WINDOW_HEIGHT);
    let container_id = scene.set_contents(Box::new(container));

    let key_display = novadraw::RectangleFigure::new_with_color(250.0, 200.0, 300.0, 150.0, novadraw::Color::rgba(0.4, 0.5, 0.6, 1.0));
    let _display = scene.add_child_to(container_id, Box::new(key_display));

    let hint = novadraw::RectangleFigure::new_with_color(50.0, 450.0, 700.0, 60.0, novadraw::Color::rgba(0.9, 0.9, 0.8, 1.0));
    let _hint = scene.add_child_to(container_id, Box::new(hint));

    scene
}

fn create_scene_6_focus() -> novadraw::SceneGraph {
    let mut scene = novadraw::SceneGraph::new();
    let container = novadraw::RectangleFigure::new(0.0, 0.0, WINDOW_WIDTH, WINDOW_HEIGHT);
    let container_id = scene.set_contents(Box::new(container));

    let focusable1 = novadraw::RectangleFigure::new_with_color(150.0, 100.0, 200.0, 80.0, novadraw::Color::rgba(0.6, 0.4, 0.7, 1.0));
    let focusable2 = novadraw::RectangleFigure::new_with_color(450.0, 100.0, 200.0, 80.0, novadraw::Color::rgba(0.4, 0.6, 0.7, 1.0));
    let focusable3 = novadraw::RectangleFigure::new_with_color(150.0, 250.0, 200.0, 80.0, novadraw::Color::rgba(0.7, 0.6, 0.4, 1.0));
    let focusable4 = novadraw::RectangleFigure::new_with_color(450.0, 250.0, 200.0, 80.0, novadraw::Color::rgba(0.5, 0.7, 0.5, 1.0));

    let _f1 = scene.add_child_to(container_id, Box::new(focusable1));
    let _f2 = scene.add_child_to(container_id, Box::new(focusable2));
    let _f3 = scene.add_child_to(container_id, Box::new(focusable3));
    let _f4 = scene.add_child_to(container_id, Box::new(focusable4));

    scene
}

fn create_scene_7_combo_keys() -> novadraw::SceneGraph {
    let mut scene = novadraw::SceneGraph::new();
    let container = novadraw::RectangleFigure::new(0.0, 0.0, WINDOW_WIDTH, WINDOW_HEIGHT);
    let container_id = scene.set_contents(Box::new(container));

    let combo_area = novadraw::RectangleFigure::new_with_color(200.0, 150.0, 400.0, 250.0, novadraw::Color::rgba(0.5, 0.5, 0.7, 1.0));
    let _area = scene.add_child_to(container_id, Box::new(combo_area));

    let status = novadraw::RectangleFigure::new_with_color(50.0, 450.0, 700.0, 60.0, novadraw::Color::rgba(0.9, 0.9, 0.9, 1.0));
    let _status = scene.add_child_to(container_id, Box::new(status));

    scene
}

fn create_scene_8_event_propagation() -> novadraw::SceneGraph {
    let mut scene = novadraw::SceneGraph::new();
    let container = novadraw::RectangleFigure::new(0.0, 0.0, WINDOW_WIDTH, WINDOW_HEIGHT);
    let container_id = scene.set_contents(Box::new(container));

    let outer = novadraw::RectangleFigure::new_with_color(100.0, 80.0, 600.0, 400.0, novadraw::Color::rgba(0.8, 0.5, 0.5, 1.0));
    let outer_id = scene.add_child_to(container_id, Box::new(outer));

    let middle = novadraw::RectangleFigure::new_with_color(150.0, 130.0, 400.0, 250.0, novadraw::Color::rgba(0.5, 0.8, 0.5, 1.0));
    let middle_id = scene.add_child_to(outer_id, Box::new(middle));

    let inner = novadraw::RectangleFigure::new_with_color(250.0, 200.0, 200.0, 100.0, novadraw::Color::rgba(0.5, 0.5, 0.8, 1.0));
    let _inner = scene.add_child_to(middle_id, Box::new(inner));

    scene
}

fn create_scene_9_custom_event() -> novadraw::SceneGraph {
    let mut scene = novadraw::SceneGraph::new();
    let container = novadraw::RectangleFigure::new(0.0, 0.0, WINDOW_WIDTH, WINDOW_HEIGHT);
    let container_id = scene.set_contents(Box::new(container));

    let custom_area = novadraw::RectangleFigure::new_with_color(250.0, 200.0, 300.0, 150.0, novadraw::Color::rgba(0.6, 0.6, 0.4, 1.0));
    let _area = scene.add_child_to(container_id, Box::new(custom_area));

    let log_area = novadraw::RectangleFigure::new_with_color(50.0, 400.0, 700.0, 150.0, novadraw::Color::rgba(0.95, 0.95, 0.95, 1.0));
    let _log = scene.add_child_to(container_id, Box::new(log_area));

    scene
}

fn main() {
    run_demo_app(
        "Event App - 事件处理验证 (按数字键 0-9 切换场景)",
        vec![
            ("0: MouseEnter/Exit", Box::new(|| create_scene_0_mouse_enter_exit())),
            ("1: MouseHover", Box::new(|| create_scene_1_mouse_hover())),
            ("2: MouseDown/Up", Box::new(|| create_scene_2_mouse_down_up())),
            ("3: MouseMove 基础", Box::new(|| create_scene_3_mouse_move_basic())),
            ("4: MouseMove 拖拽", Box::new(|| create_scene_4_mouse_drag())),
            ("5: KeyDown/Up", Box::new(|| create_scene_5_key_down_up())),
            ("6: Focus 获得/失去", Box::new(|| create_scene_6_focus())),
            ("7: 组合键", Box::new(|| create_scene_7_combo_keys())),
            ("8: 事件传播", Box::new(|| create_scene_8_event_propagation())),
            ("9: 自定义事件", Box::new(|| create_scene_9_custom_event())),
        ],
    ).expect("Failed to run app");
}
