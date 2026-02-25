//! Shape App - 形状展示验证
//!
//! 遵循 MECE 原则设计场景：
//! - 图形类型维度: Rectangle, Ellipse, Line
//! - 属性维度: Fill, Stroke, LineCap, LineJoin
//! - 组合维度: 混合图形, 父子嵌套, Z-order

use novadraw::command::LineJoin;
use novadraw_apps::{run_demo_app, run_demo_app_with_scene_screenshot, run_demo_app_with_screenshot};
use std::io::Write;

const WINDOW_WIDTH: f64 = 800.0;
const WINDOW_HEIGHT: f64 = 600.0;

// ============================================================================
// 维度1: 图形类型验证 (MECE - 按类型分类)
// ============================================================================

/// Scene 0: 矩形图形 - 验证 RectangleFigure
/// MECE: 单独验证矩形填充
fn create_scene_0_rectangle_fill() -> novadraw::SceneGraph {
    let mut scene = novadraw::SceneGraph::new();

    let container = novadraw::RectangleFigure::new(0.0, 0.0, WINDOW_WIDTH, WINDOW_HEIGHT);
    let container_id = scene.set_contents(Box::new(container));

    // 不同尺寸的填充矩形
    let rect_1 = novadraw::RectangleFigure::new_with_color(50.0, 50.0, 150.0, 100.0, novadraw::Color::rgba(0.8, 0.2, 0.2, 1.0));
    let rect_2 = novadraw::RectangleFigure::new_with_color(250.0, 50.0, 100.0, 100.0, novadraw::Color::rgba(0.2, 0.8, 0.2, 1.0));
    let rect_3 = novadraw::RectangleFigure::new_with_color(400.0, 50.0, 200.0, 80.0, novadraw::Color::rgba(0.2, 0.2, 0.8, 1.0));
    let rect_4 = novadraw::RectangleFigure::new_with_color(50.0, 200.0, 120.0, 150.0, novadraw::Color::rgba(0.9, 0.5, 0.1, 1.0));
    let rect_5 = novadraw::RectangleFigure::new_with_color(200.0, 200.0, 100.0, 100.0, novadraw::Color::rgba(0.5, 0.1, 0.9, 1.0));

    scene.add_child_to(container_id, Box::new(rect_1));
    scene.add_child_to(container_id, Box::new(rect_2));
    scene.add_child_to(container_id, Box::new(rect_3));
    scene.add_child_to(container_id, Box::new(rect_4));
    scene.add_child_to(container_id, Box::new(rect_5));

    scene
}

/// Scene 1: 椭圆图形 - 验证 EllipseFigure
/// MECE: 单独验证椭圆填充
fn create_scene_1_ellipse_fill() -> novadraw::SceneGraph {
    let mut scene = novadraw::SceneGraph::new();

    let container = novadraw::RectangleFigure::new(0.0, 0.0, WINDOW_WIDTH, WINDOW_HEIGHT);
    let container_id = scene.set_contents(Box::new(container));

    // 不同尺寸的椭圆
    let ellipse_1 = novadraw::EllipseFigure::new_with_color(150.0, 100.0, 200.0, 120.0, novadraw::Color::rgba(0.9, 0.5, 0.1, 1.0));
    let ellipse_2 = novadraw::EllipseFigure::new_with_color(450.0, 100.0, 150.0, 150.0, novadraw::Color::rgba(0.1, 0.5, 0.9, 1.0));
    let ellipse_3 = novadraw::EllipseFigure::new_with_color(150.0, 300.0, 100.0, 100.0, novadraw::Color::rgba(0.5, 0.9, 0.2, 1.0));
    let ellipse_4 = novadraw::EllipseFigure::new_with_color(350.0, 300.0, 180.0, 80.0, novadraw::Color::rgba(0.9, 0.2, 0.5, 1.0));
    let ellipse_5 = novadraw::EllipseFigure::new_with_color(600.0, 300.0, 120.0, 120.0, novadraw::Color::rgba(0.3, 0.3, 0.9, 1.0));

    scene.add_child_to(container_id, Box::new(ellipse_1));
    scene.add_child_to(container_id, Box::new(ellipse_2));
    scene.add_child_to(container_id, Box::new(ellipse_3));
    scene.add_child_to(container_id, Box::new(ellipse_4));
    scene.add_child_to(container_id, Box::new(ellipse_5));

    scene
}

/// Scene 2: 直线图形 - 验证 LineFigure
/// MECE: 单独验证直线（水平、垂直、斜线）
fn create_scene_2_line_directions() -> novadraw::SceneGraph {
    let mut scene = novadraw::SceneGraph::new();

    let container = novadraw::RectangleFigure::new(0.0, 0.0, WINDOW_WIDTH, WINDOW_HEIGHT);
    let container_id = scene.set_contents(Box::new(container));

    // 水平线
    let h_line = novadraw::LineFigure::new_with_color(100.0, 100.0, 400.0, 100.0, novadraw::Color::rgba(0.9, 0.2, 0.2, 1.0))
        .with_width(3.0);

    // 垂直线
    let v_line = novadraw::LineFigure::new_with_color(500.0, 50.0, 500.0, 250.0, novadraw::Color::rgba(0.2, 0.2, 0.9, 1.0))
        .with_width(3.0);

    // 斜线 (45度)
    let diag_line = novadraw::LineFigure::new_with_color(100.0, 300.0, 300.0, 500.0, novadraw::Color::rgba(0.2, 0.9, 0.2, 1.0))
        .with_width(3.0);

    // 反向斜线
    let diag_line2 = novadraw::LineFigure::new_with_color(400.0, 300.0, 200.0, 400.0, novadraw::Color::rgba(0.9, 0.5, 0.1, 1.0))
        .with_width(3.0);

    // 短斜线
    let short_line = novadraw::LineFigure::new_with_color(600.0, 400.0, 650.0, 450.0, novadraw::Color::rgba(0.6, 0.3, 0.9, 1.0))
        .with_width(4.0);

    scene.add_child_to(container_id, Box::new(h_line));
    scene.add_child_to(container_id, Box::new(v_line));
    scene.add_child_to(container_id, Box::new(diag_line));
    scene.add_child_to(container_id, Box::new(diag_line2));
    scene.add_child_to(container_id, Box::new(short_line));

    scene
}

// ============================================================================
// 维度2: 属性验证 (MECE - 按属性分类)
// ============================================================================

/// Scene 3: 描边属性 - 验证 Stroke 属性
/// MECE: 单独验证 Rectangle/Line 的描边
fn create_scene_3_stroke_attributes() -> novadraw::SceneGraph {
    let mut scene = novadraw::SceneGraph::new();

    let container = novadraw::RectangleFigure::new(0.0, 0.0, WINDOW_WIDTH, WINDOW_HEIGHT);
    let container_id = scene.set_contents(Box::new(container));

    // 不同描边宽度的矩形
    let rect_1 = novadraw::RectangleFigure::new_with_color(50.0, 50.0, 150.0, 100.0, novadraw::Color::rgba(0.9, 0.5, 0.1, 1.0))
        .with_stroke(novadraw::Color::WHITE, 1.0);
    let rect_2 = novadraw::RectangleFigure::new_with_color(250.0, 50.0, 150.0, 100.0, novadraw::Color::rgba(0.1, 0.5, 0.9, 1.0))
        .with_stroke(novadraw::Color::WHITE, 2.0);
    let rect_3 = novadraw::RectangleFigure::new_with_color(450.0, 50.0, 150.0, 100.0, novadraw::Color::rgba(0.2, 0.8, 0.3, 1.0))
        .with_stroke(novadraw::Color::WHITE, 4.0);
    let rect_4 = novadraw::RectangleFigure::new_with_color(650.0, 50.0, 100.0, 100.0, novadraw::Color::rgba(0.9, 0.2, 0.2, 1.0))
        .with_stroke(novadraw::Color::WHITE, 8.0);

    // 不同描边颜色的矩形
    let rect_5 = novadraw::RectangleFigure::new_with_color(50.0, 200.0, 150.0, 100.0, novadraw::Color::rgba(0.3, 0.3, 0.3, 1.0))
        .with_stroke(novadraw::Color::RED, 3.0);
    let rect_6 = novadraw::RectangleFigure::new_with_color(250.0, 200.0, 150.0, 100.0, novadraw::Color::rgba(0.3, 0.3, 0.3, 1.0))
        .with_stroke(novadraw::Color::GREEN, 3.0);
    let rect_7 = novadraw::RectangleFigure::new_with_color(450.0, 200.0, 150.0, 100.0, novadraw::Color::rgba(0.3, 0.3, 0.3, 1.0))
        .with_stroke(novadraw::Color::rgba(0.0, 0.0, 1.0, 1.0), 3.0);
    let rect_8 = novadraw::RectangleFigure::new_with_color(650.0, 200.0, 100.0, 100.0, novadraw::Color::rgba(0.3, 0.3, 0.3, 1.0))
        .with_stroke(novadraw::Color::rgba(1.0, 1.0, 0.0, 1.0), 3.0);

    // 粗描边椭圆
    let ellipse_1 = novadraw::EllipseFigure::new_with_color(100.0, 350.0, 120.0, 80.0, novadraw::Color::rgba(0.9, 0.3, 0.6, 1.0))
        .with_stroke(novadraw::Color::WHITE, 2.0);
    let ellipse_2 = novadraw::EllipseFigure::new_with_color(300.0, 350.0, 120.0, 80.0, novadraw::Color::rgba(0.3, 0.9, 0.6, 1.0))
        .with_stroke(novadraw::Color::WHITE, 4.0);
    let ellipse_3 = novadraw::EllipseFigure::new_with_color(500.0, 350.0, 120.0, 80.0, novadraw::Color::rgba(0.6, 0.3, 0.9, 1.0))
        .with_stroke(novadraw::Color::WHITE, 6.0);

    scene.add_child_to(container_id, Box::new(rect_1));
    scene.add_child_to(container_id, Box::new(rect_2));
    scene.add_child_to(container_id, Box::new(rect_3));
    scene.add_child_to(container_id, Box::new(rect_4));
    scene.add_child_to(container_id, Box::new(rect_5));
    scene.add_child_to(container_id, Box::new(rect_6));
    scene.add_child_to(container_id, Box::new(rect_7));
    scene.add_child_to(container_id, Box::new(rect_8));
    scene.add_child_to(container_id, Box::new(ellipse_1));
    scene.add_child_to(container_id, Box::new(ellipse_2));
    scene.add_child_to(container_id, Box::new(ellipse_3));

    scene
}

/// Scene 4: 描边属性
/// MECE: 验证 RectangleFigure 的描边属性
fn create_scene_4_line_cap() -> novadraw::SceneGraph {
    let mut scene = novadraw::SceneGraph::new();

    // 灰色背景
    let bg = novadraw::RectangleFigure::new_with_color(0.0, 0.0, WINDOW_WIDTH, WINDOW_HEIGHT, novadraw::Color::rgba(0.2, 0.2, 0.2, 1.0));
    let bg_id = scene.set_contents(Box::new(bg));

    // 测试不同描边宽度
    let rect_1 = novadraw::RectangleFigure::new_with_color(50.0, 50.0, 150.0, 100.0, novadraw::Color::rgba(0.9, 0.3, 0.3, 1.0))
        .with_stroke(novadraw::Color::rgba(0.0, 1.0, 1.0, 1.0), 2.0);
    let rect_2 = novadraw::RectangleFigure::new_with_color(250.0, 50.0, 150.0, 100.0, novadraw::Color::rgba(0.3, 0.9, 0.3, 1.0))
        .with_stroke(novadraw::Color::rgba(0.0, 1.0, 1.0, 1.0), 4.0);
    let rect_3 = novadraw::RectangleFigure::new_with_color(450.0, 50.0, 150.0, 100.0, novadraw::Color::rgba(0.3, 0.3, 0.9, 1.0))
        .with_stroke(novadraw::Color::rgba(0.0, 1.0, 1.0, 1.0), 8.0);

    // 测试不同描边颜色
    let rect_4 = novadraw::RectangleFigure::new_with_color(50.0, 200.0, 150.0, 100.0, novadraw::Color::rgba(0.3, 0.3, 0.3, 1.0))
        .with_stroke(novadraw::Color::RED, 4.0);
    let rect_5 = novadraw::RectangleFigure::new_with_color(250.0, 200.0, 150.0, 100.0, novadraw::Color::rgba(0.3, 0.3, 0.3, 1.0))
        .with_stroke(novadraw::Color::GREEN, 4.0);
    let rect_6 = novadraw::RectangleFigure::new_with_color(450.0, 200.0, 150.0, 100.0, novadraw::Color::rgba(0.3, 0.3, 0.3, 1.0))
        .with_stroke(novadraw::Color::BLUE, 4.0);

    scene.add_child_to(bg_id, Box::new(rect_1));
    scene.add_child_to(bg_id, Box::new(rect_2));
    scene.add_child_to(bg_id, Box::new(rect_3));
    scene.add_child_to(bg_id, Box::new(rect_4));
    scene.add_child_to(bg_id, Box::new(rect_5));
    scene.add_child_to(bg_id, Box::new(rect_6));

    scene
}

/// Scene 5: LineJoin 连接样式
/// MECE: 验证 LineJoin (Miter, Round, Bevel)
fn create_scene_5_line_join() -> novadraw::SceneGraph {
    let mut scene = novadraw::SceneGraph::new();

    // 灰色背景
    let bg = novadraw::RectangleFigure::new_with_color(0.0, 0.0, WINDOW_WIDTH, WINDOW_HEIGHT, novadraw::Color::rgba(0.2, 0.2, 0.2, 1.0));
    let bg_id = scene.set_contents(Box::new(bg));

    // 测试不同 LineJoin - 粗白色描边
    // Miter (默认) - 尖角
    let rect_1 = novadraw::RectangleFigure::new_with_color(50.0, 50.0, 150.0, 100.0, novadraw::Color::rgba(0.9, 0.3, 0.3, 1.0))
        .with_stroke(novadraw::Color::WHITE, 8.0);

    // Round - 圆角
    let mut rect_2 = novadraw::RectangleFigure::new_with_color(250.0, 50.0, 150.0, 100.0, novadraw::Color::rgba(0.3, 0.9, 0.3, 1.0))
        .with_stroke(novadraw::Color::WHITE, 8.0);
    rect_2.line_join = LineJoin::Round;

    // Bevel - 斜切
    let mut rect_3 = novadraw::RectangleFigure::new_with_color(450.0, 50.0, 150.0, 100.0, novadraw::Color::rgba(0.3, 0.3, 0.9, 1.0))
        .with_stroke(novadraw::Color::WHITE, 8.0);
    rect_3.line_join = LineJoin::Bevel;

    scene.add_child_to(bg_id, Box::new(rect_1));
    scene.add_child_to(bg_id, Box::new(rect_2));
    scene.add_child_to(bg_id, Box::new(rect_3));

    scene
}

/// Scene 6: 透明度验证
/// MECE: 单独验证 Alpha 透明度
fn create_scene_6_alpha() -> novadraw::SceneGraph {
    let mut scene = novadraw::SceneGraph::new();

    let container = novadraw::RectangleFigure::new(0.0, 0.0, WINDOW_WIDTH, WINDOW_HEIGHT);
    let container_id = scene.set_contents(Box::new(container));

    // 不同透明度层级 (堆叠验证 alpha 混合)
    let alpha_levels = [1.0, 0.8, 0.6, 0.4, 0.2];

    for (i, &alpha) in alpha_levels.iter().enumerate() {
        let x = 50.0 + (i as f64) * 120.0;
        let rect = novadraw::RectangleFigure::new_with_color(x, 50.0, 100.0, 100.0, novadraw::Color::rgba(0.9, 0.2, 0.2, alpha));
        scene.add_child_to(container_id, Box::new(rect));
    }

    // 椭圆透明度
    for (i, &alpha) in alpha_levels.iter().enumerate() {
        let x = 50.0 + (i as f64) * 120.0;
        let ellipse = novadraw::EllipseFigure::new_with_color(x + 10.0, 200.0, 80.0, 80.0, novadraw::Color::rgba(0.2, 0.5, 0.9, alpha));
        scene.add_child_to(container_id, Box::new(ellipse));
    }

    // 直线透明度
    for (i, &alpha) in alpha_levels.iter().enumerate() {
        let y = 350.0 + (i as f64) * 40.0;
        let line = novadraw::LineFigure::new_with_color(50.0, y, 700.0, y, novadraw::Color::rgba(0.2, 0.8, 0.2, alpha))
            .with_width(6.0);
        scene.add_child_to(container_id, Box::new(line));
    }

    // 半透明叠加验证
    let red_overlay = novadraw::RectangleFigure::new_with_color(550.0, 50.0, 100.0, 100.0, novadraw::Color::rgba(1.0, 0.0, 0.0, 0.5));
    let blue_overlay = novadraw::RectangleFigure::new_with_color(580.0, 80.0, 100.0, 100.0, novadraw::Color::rgba(0.0, 0.0, 1.0, 0.5));
    let green_overlay = novadraw::RectangleFigure::new_with_color(610.0, 110.0, 100.0, 100.0, novadraw::Color::rgba(0.0, 1.0, 0.0, 0.5));

    scene.add_child_to(container_id, Box::new(red_overlay));
    scene.add_child_to(container_id, Box::new(blue_overlay));
    scene.add_child_to(container_id, Box::new(green_overlay));

    scene
}

// ============================================================================
// 维度3: 组合验证 (MECE - 按组合方式分类)
// ============================================================================

/// Scene 7: 混合图形组合
/// MECE: 验证多种图形在同一场景中正确渲染
fn create_scene_7_mixed_shapes() -> novadraw::SceneGraph {
    let mut scene = novadraw::SceneGraph::new();

    let container = novadraw::RectangleFigure::new(0.0, 0.0, WINDOW_WIDTH, WINDOW_HEIGHT);
    let container_id = scene.set_contents(Box::new(container));

    // 组合1: 矩形 + 椭圆
    let rect_combo = novadraw::RectangleFigure::new_with_color(50.0, 50.0, 150.0, 100.0, novadraw::Color::rgba(0.9, 0.5, 0.1, 1.0))
        .with_stroke(novadraw::Color::BLACK, 2.0);
    let ellipse_combo = novadraw::EllipseFigure::new_with_color(125.0, 100.0, 80.0, 50.0, novadraw::Color::rgba(0.1, 0.5, 0.9, 1.0));

    // 组合2: 椭圆 + 直线
    let ellipse_combo2 = novadraw::EllipseFigure::new_with_color(300.0, 80.0, 100.0, 60.0, novadraw::Color::rgba(0.9, 0.2, 0.2, 1.0));
    let line_through = novadraw::LineFigure::new_with_color(250.0, 110.0, 400.0, 110.0, novadraw::Color::BLACK)
        .with_width(2.0);

    // 组合3: 矩形框住多个图形
    let container_rect = novadraw::RectangleFigure::new_with_color(450.0, 50.0, 200.0, 150.0, novadraw::Color::rgba(0.95, 0.95, 0.95, 1.0))
        .with_stroke(novadraw::Color::rgba(0.2, 0.2, 0.2, 1.0), 1.0);
    let inner_rect = novadraw::RectangleFigure::new_with_color(470.0, 70.0, 60.0, 40.0, novadraw::Color::rgba(0.9, 0.3, 0.3, 1.0));
    let inner_ellipse = novadraw::EllipseFigure::new_with_color(560.0, 90.0, 50.0, 30.0, novadraw::Color::rgba(0.3, 0.9, 0.3, 1.0));
    let inner_line = novadraw::LineFigure::new_with_color(480.0, 150.0, 620.0, 150.0, novadraw::Color::rgba(0.3, 0.3, 0.9, 1.0))
        .with_width(2.0);

    // 组合4: 复杂图形组合
    let complex_rect = novadraw::RectangleFigure::new_with_color(50.0, 250.0, 120.0, 80.0, novadraw::Color::rgba(0.8, 0.2, 0.5, 1.0))
        .with_stroke(novadraw::Color::WHITE, 3.0);
    let complex_ellipse = novadraw::EllipseFigure::new_with_color(50.0, 350.0, 120.0, 80.0, novadraw::Color::rgba(0.2, 0.5, 0.8, 1.0))
        .with_stroke(novadraw::Color::WHITE, 3.0);
    let complex_line1 = novadraw::LineFigure::new_with_color(200.0, 250.0, 300.0, 280.0, novadraw::Color::rgba(0.9, 0.6, 0.1, 1.0))
        .with_width(3.0);
    let complex_line2 = novadraw::LineFigure::new_with_color(200.0, 280.0, 300.0, 250.0, novadraw::Color::rgba(0.1, 0.6, 0.9, 1.0))
        .with_width(3.0);
    let complex_line3 = novadraw::LineFigure::new_with_color(250.0, 350.0, 250.0, 420.0, novadraw::Color::rgba(0.3, 0.8, 0.3, 1.0))
        .with_width(4.0);

    // 组合5: 细线装饰
    let decor_line1 = novadraw::LineFigure::new_with_color(400.0, 260.0, 550.0, 260.0, novadraw::Color::rgba(0.3, 0.3, 0.3, 1.0))
        .with_width(1.0);
    let decor_line2 = novadraw::LineFigure::new_with_color(400.0, 300.0, 550.0, 300.0, novadraw::Color::rgba(0.3, 0.3, 0.3, 1.0))
        .with_width(2.0);
    let decor_line3 = novadraw::LineFigure::new_with_color(400.0, 350.0, 550.0, 350.0, novadraw::Color::rgba(0.3, 0.3, 0.3, 1.0))
        .with_width(4.0);

    // 边框装饰
    let border_outer = novadraw::RectangleFigure::new_with_color(600.0, 250.0, 150.0, 150.0, novadraw::Color::rgba(0.9, 0.9, 0.9, 1.0))
        .with_stroke(novadraw::Color::BLACK, 1.0);
    let border_inner = novadraw::RectangleFigure::new_with_color(610.0, 260.0, 130.0, 130.0, novadraw::Color::WHITE)
        .with_stroke(novadraw::Color::rgba(0.5, 0.5, 0.5, 1.0), 1.0);

    scene.add_child_to(container_id, Box::new(rect_combo));
    scene.add_child_to(container_id, Box::new(ellipse_combo));
    scene.add_child_to(container_id, Box::new(ellipse_combo2));
    scene.add_child_to(container_id, Box::new(line_through));
    scene.add_child_to(container_id, Box::new(container_rect));
    scene.add_child_to(container_id, Box::new(inner_rect));
    scene.add_child_to(container_id, Box::new(inner_ellipse));
    scene.add_child_to(container_id, Box::new(inner_line));
    scene.add_child_to(container_id, Box::new(complex_rect));
    scene.add_child_to(container_id, Box::new(complex_ellipse));
    scene.add_child_to(container_id, Box::new(complex_line1));
    scene.add_child_to(container_id, Box::new(complex_line2));
    scene.add_child_to(container_id, Box::new(complex_line3));
    scene.add_child_to(container_id, Box::new(decor_line1));
    scene.add_child_to(container_id, Box::new(decor_line2));
    scene.add_child_to(container_id, Box::new(decor_line3));
    scene.add_child_to(container_id, Box::new(border_outer));
    scene.add_child_to(container_id, Box::new(border_inner));

    scene
}

/// Scene 8: Z-order 遮挡关系
/// MECE: 验证后添加的图形遮挡先添加的图形
fn create_scene_8_zorder() -> novadraw::SceneGraph {
    let mut scene = novadraw::SceneGraph::new();

    let container = novadraw::RectangleFigure::new(0.0, 0.0, WINDOW_WIDTH, WINDOW_HEIGHT);
    let container_id = scene.set_contents(Box::new(container));

    // 底层图形（先添加）
    let bottom_rect = novadraw::RectangleFigure::new_with_color(100.0, 100.0, 300.0, 200.0, novadraw::Color::BLUE);
    let bottom_ellipse = novadraw::EllipseFigure::new_with_color(250.0, 150.0, 200.0, 100.0, novadraw::Color::GREEN);
    let bottom_line = novadraw::LineFigure::new_with_color(150.0, 200.0, 400.0, 250.0, novadraw::Color::rgba(1.0, 1.0, 0.0, 1.0))
        .with_width(8.0);

    // 中间层（遮挡部分底层）
    let mid_rect = novadraw::RectangleFigure::new_with_color(200.0, 150.0, 200.0, 120.0, novadraw::Color::RED);
    let mid_ellipse = novadraw::EllipseFigure::new_with_color(350.0, 200.0, 120.0, 80.0, novadraw::Color::rgba(1.0, 0.65, 0.0, 1.0));

    // 顶层（最后添加，遮挡所有底层）
    let top_rect = novadraw::RectangleFigure::new_with_color(300.0, 180.0, 150.0, 100.0, novadraw::Color::rgba(0.5, 0.0, 0.5, 1.0));

    // 验证 Z-order 的辅助标记
    let marker_1 = novadraw::RectangleFigure::new_with_color(50.0, 50.0, 30.0, 30.0, novadraw::Color::rgba(0.2, 0.2, 0.8, 1.0));
    let marker_2 = novadraw::RectangleFigure::new_with_color(80.0, 50.0, 30.0, 30.0, novadraw::Color::rgba(0.2, 0.8, 0.2, 1.0));
    let marker_3 = novadraw::RectangleFigure::new_with_color(110.0, 50.0, 30.0, 30.0, novadraw::Color::rgba(0.8, 0.2, 0.2, 1.0));

    scene.add_child_to(container_id, Box::new(bottom_rect));
    scene.add_child_to(container_id, Box::new(bottom_ellipse));
    scene.add_child_to(container_id, Box::new(bottom_line));
    scene.add_child_to(container_id, Box::new(mid_rect));
    scene.add_child_to(container_id, Box::new(mid_ellipse));
    scene.add_child_to(container_id, Box::new(top_rect));
    scene.add_child_to(container_id, Box::new(marker_1));
    scene.add_child_to(container_id, Box::new(marker_2));
    scene.add_child_to(container_id, Box::new(marker_3));

    scene
}

/// Scene 9: 父子嵌套结构
/// MECE: 验证 Figure 嵌套渲染
fn create_scene_9_parent_child() -> novadraw::SceneGraph {
    let mut scene = novadraw::SceneGraph::new();

    let container = novadraw::RectangleFigure::new(0.0, 0.0, WINDOW_WIDTH, WINDOW_HEIGHT);
    let container_id = scene.set_contents(Box::new(container));

    // 父矩形
    let parent_rect = novadraw::RectangleFigure::new_with_color(50.0, 50.0, 350.0, 250.0, novadraw::Color::rgba(0.9, 0.5, 0.1, 0.3))
        .with_stroke(novadraw::Color::rgba(1.0, 0.65, 0.0, 1.0), 2.0);
    let parent_id = scene.add_child_to(container_id, Box::new(parent_rect));

    // 子图形 - 直接添加到父矩形中
    let child_rect = novadraw::RectangleFigure::new_with_color(80.0, 80.0, 100.0, 60.0, novadraw::Color::rgba(0.2, 0.5, 0.9, 1.0));
    let child_ellipse = novadraw::EllipseFigure::new_with_color(200.0, 100.0, 80.0, 50.0, novadraw::Color::rgba(0.9, 0.2, 0.2, 1.0));
    let child_line = novadraw::LineFigure::new_with_color(100.0, 200.0, 300.0, 200.0, novadraw::Color::rgba(0.2, 0.8, 0.2, 1.0))
        .with_width(3.0);

    scene.add_child_to(parent_id, Box::new(child_rect));
    scene.add_child_to(parent_id, Box::new(child_ellipse));
    scene.add_child_to(parent_id, Box::new(child_line));

    // 另一个父容器
    let parent_rect2 = novadraw::RectangleFigure::new_with_color(450.0, 50.0, 300.0, 200.0, novadraw::Color::rgba(0.5, 0.2, 0.8, 0.3))
        .with_stroke(novadraw::Color::rgba(0.5, 0.0, 0.5, 1.0), 2.0);
    let parent_id2 = scene.add_child_to(container_id, Box::new(parent_rect2));

    // 子图形 - 更深层次
    let grandchild_rect = novadraw::RectangleFigure::new_with_color(480.0, 80.0, 80.0, 60.0, novadraw::Color::rgba(0.3, 0.8, 0.5, 1.0));
    let grandchild_ellipse = novadraw::EllipseFigure::new_with_color(580.0, 100.0, 60.0, 40.0, novadraw::Color::rgba(0.8, 0.5, 0.2, 1.0));

    scene.add_child_to(parent_id2, Box::new(grandchild_rect));
    scene.add_child_to(parent_id2, Box::new(grandchild_ellipse));

    // 独立图形（不嵌套）
    let standalone = novadraw::RectangleFigure::new_with_color(50.0, 350.0, 150.0, 100.0, novadraw::Color::rgba(0.1, 0.7, 0.7, 1.0))
        .with_stroke(novadraw::Color::rgba(0.0, 1.0, 1.0, 1.0), 2.0);
    scene.add_child_to(container_id, Box::new(standalone));

    scene
}

// ============================================================================
// 场景映射
// ============================================================================

fn main() {
    // 解析命令行参数
    let args: Vec<String> = std::env::args().collect();

    // 场景列表
    let scenes: Vec<(&'static str, Box<dyn FnMut() -> novadraw::SceneGraph>)> = vec![
        // 维度1: 图形类型验证
        ("0:Rectangle Fill", Box::new(|| create_scene_0_rectangle_fill())),
        ("1:Ellipse Fill", Box::new(|| create_scene_1_ellipse_fill())),
        ("2:Line Directions", Box::new(|| create_scene_2_line_directions())),
        // 维度2: 属性验证
        ("3:Stroke Attr", Box::new(|| create_scene_3_stroke_attributes())),
        ("4:LineCap", Box::new(|| create_scene_4_line_cap())),
        ("5:LineJoin", Box::new(|| create_scene_5_line_join())),
        ("6:Alpha", Box::new(|| create_scene_6_alpha())),
        // 维度3: 组合验证
        ("7:Mixed Shapes", Box::new(|| create_scene_7_mixed_shapes())),
        ("8:Z-Order", Box::new(|| create_scene_8_zorder())),
        ("9:Parent-Child", Box::new(|| create_scene_9_parent_child())),
    ];

    // 检查截图模式
    let title = "Shape App - 形状验证 (按数字键 0-9 切换场景)\nMECE: 图形类型 | 属性 | 组合";
    let app_name = "shape-app";

    if args.len() > 1 {
        match args[1].as_str() {
            "--screenshot-all" => {
                // 截图所有场景
                println!("截图所有场景...");
                std::io::stdout().flush().ok();
                run_demo_app_with_screenshot(title, app_name, scenes, true)
                    .expect("Failed to run app with screenshot");
            }
            arg if arg.starts_with("--screenshot=") => {
                // 截图指定场景，例如 --screenshot=0 或 --screenshot=5
                let scene_idx = arg.strip_prefix("--screenshot=")
                    .and_then(|s| s.parse::<usize>().ok());
                match scene_idx {
                    Some(idx) => {
                        run_demo_app_with_scene_screenshot(title, app_name, scenes, idx)
                            .expect("Failed to run app with scene screenshot");
                    }
                    None => {
                        eprintln!("无效的场景索引: {}", &arg[12..]);
                        eprintln!("用法: cargo run -- --screenshot=<0-9> 或 --screenshot-all");
                        std::process::exit(1);
                    }
                }
            }
            "--help" | "-h" => {
                println!("用法: cargo run -- [选项]");
                println!("选项:");
                println!("  --screenshot-all    截图所有场景");
                println!("  --screenshot=<N>   截图指定场景 (0-9)");
                println!("  --help, -h         显示帮助");
                std::process::exit(0);
            }
            _ => {
                eprintln!("未知参数: {}", args[1]);
                eprintln!("用法: cargo run -- --screenshot=<0-9> 或 --screenshot-all");
                std::process::exit(1);
            }
        }
    } else {
        // 正常启动
        run_demo_app(title, app_name, scenes).expect("Failed to run app");
    }
}
