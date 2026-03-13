//! Shape App - 形状展示验证
//!
//! 遵循 MECE 原则设计场景：
//! - 图形类型维度: Rectangle, Ellipse, Line
//! - 属性维度: Fill, Stroke, LineCap, LineJoin
//! - 组合维度: 混合图形, 父子嵌套, Z-order

use novadraw::command::LineJoin;
use novadraw_apps::{
    run_demo_app, run_demo_app_with_scene_screenshot, run_demo_app_with_screenshot,
};
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
    let rect_1 = novadraw::RectangleFigure::new_with_color(
        50.0,
        50.0,
        150.0,
        100.0,
        novadraw::Color::rgba(0.8, 0.2, 0.2, 1.0),
    );
    let rect_2 = novadraw::RectangleFigure::new_with_color(
        250.0,
        50.0,
        100.0,
        100.0,
        novadraw::Color::rgba(0.2, 0.8, 0.2, 1.0),
    );
    let rect_3 = novadraw::RectangleFigure::new_with_color(
        400.0,
        50.0,
        200.0,
        80.0,
        novadraw::Color::rgba(0.2, 0.2, 0.8, 1.0),
    );
    let rect_4 = novadraw::RectangleFigure::new_with_color(
        50.0,
        200.0,
        120.0,
        150.0,
        novadraw::Color::rgba(0.9, 0.5, 0.1, 1.0),
    );
    let rect_5 = novadraw::RectangleFigure::new_with_color(
        200.0,
        200.0,
        100.0,
        100.0,
        novadraw::Color::rgba(0.5, 0.1, 0.9, 1.0),
    );

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
    let ellipse_1 = novadraw::EllipseFigure::new_with_color(
        150.0,
        100.0,
        200.0,
        120.0,
        novadraw::Color::rgba(0.9, 0.5, 0.1, 1.0),
    );
    let ellipse_2 = novadraw::EllipseFigure::new_with_color(
        450.0,
        100.0,
        150.0,
        150.0,
        novadraw::Color::rgba(0.1, 0.5, 0.9, 1.0),
    );
    let ellipse_3 = novadraw::EllipseFigure::new_with_color(
        150.0,
        300.0,
        100.0,
        100.0,
        novadraw::Color::rgba(0.5, 0.9, 0.2, 1.0),
    );
    let ellipse_4 = novadraw::EllipseFigure::new_with_color(
        350.0,
        300.0,
        180.0,
        80.0,
        novadraw::Color::rgba(0.9, 0.2, 0.5, 1.0),
    );
    let ellipse_5 = novadraw::EllipseFigure::new_with_color(
        600.0,
        300.0,
        120.0,
        120.0,
        novadraw::Color::rgba(0.3, 0.3, 0.9, 1.0),
    );

    scene.add_child_to(container_id, Box::new(ellipse_1));
    scene.add_child_to(container_id, Box::new(ellipse_2));
    scene.add_child_to(container_id, Box::new(ellipse_3));
    scene.add_child_to(container_id, Box::new(ellipse_4));
    scene.add_child_to(container_id, Box::new(ellipse_5));

    scene
}

/// Scene 2: 圆角矩形
fn create_scene_2_rounded_rect() -> novadraw::SceneGraph {
    let mut scene = novadraw::SceneGraph::new();

    let container = novadraw::RectangleFigure::new(0.0, 0.0, WINDOW_WIDTH, WINDOW_HEIGHT);
    let container_id = scene.set_contents(Box::new(container));

    // 不同圆角半径的矩形
    let rect_0 = novadraw::RoundedRectangleFigure::new_with_color(
        50.0,
        50.0,
        150.0,
        80.0,
        0.0,
        novadraw::Color::rgba(0.9, 0.5, 0.1, 1.0),
    )
    .with_stroke(novadraw::Color::WHITE, 2.0);
    let rect_5 = novadraw::RoundedRectangleFigure::new_with_color(
        250.0,
        50.0,
        150.0,
        80.0,
        5.0,
        novadraw::Color::rgba(0.1, 0.5, 0.9, 1.0),
    )
    .with_stroke(novadraw::Color::WHITE, 2.0);
    let rect_15 = novadraw::RoundedRectangleFigure::new_with_color(
        450.0,
        50.0,
        150.0,
        80.0,
        15.0,
        novadraw::Color::rgba(0.2, 0.8, 0.3, 1.0),
    )
    .with_stroke(novadraw::Color::WHITE, 2.0);
    let rect_30 = novadraw::RoundedRectangleFigure::new_with_color(
        650.0,
        50.0,
        100.0,
        80.0,
        30.0,
        novadraw::Color::rgba(0.9, 0.2, 0.2, 1.0),
    )
    .with_stroke(novadraw::Color::WHITE, 2.0);

    // 纯填充（无描边）
    let rect_fill = novadraw::RoundedRectangleFigure::new_with_color(
        50.0,
        180.0,
        150.0,
        80.0,
        20.0,
        novadraw::Color::rgba(0.3, 0.3, 0.3, 1.0),
    );

    // 纯描边（无填充）
    let rect_stroke = novadraw::RoundedRectangleFigure::new(250.0, 180.0, 150.0, 80.0, 20.0)
        .with_stroke(novadraw::Color::RED, 3.0);

    // 填充 + 描边
    let rect_both = novadraw::RoundedRectangleFigure::new_with_color(
        450.0,
        180.0,
        150.0,
        80.0,
        20.0,
        novadraw::Color::rgba(0.3, 0.3, 0.3, 1.0),
    )
    .with_stroke(novadraw::Color::GREEN, 3.0);

    // 不同描边宽度
    let rect_sw_1 = novadraw::RoundedRectangleFigure::new_with_color(
        50.0,
        300.0,
        150.0,
        80.0,
        15.0,
        novadraw::Color::rgba(0.6, 0.3, 0.9, 1.0),
    )
    .with_stroke(novadraw::Color::WHITE, 1.0);
    let rect_sw_4 = novadraw::RoundedRectangleFigure::new_with_color(
        250.0,
        300.0,
        150.0,
        80.0,
        15.0,
        novadraw::Color::rgba(0.6, 0.3, 0.9, 1.0),
    )
    .with_stroke(novadraw::Color::WHITE, 4.0);
    let rect_sw_8 = novadraw::RoundedRectangleFigure::new_with_color(
        450.0,
        300.0,
        150.0,
        80.0,
        15.0,
        novadraw::Color::rgba(0.6, 0.3, 0.9, 1.0),
    )
    .with_stroke(novadraw::Color::WHITE, 8.0);

    // 大圆角（接近圆形）
    let rect_circle = novadraw::RoundedRectangleFigure::new_with_color(
        650.0,
        300.0,
        100.0,
        80.0,
        40.0,
        novadraw::Color::rgba(0.9, 0.6, 0.1, 1.0),
    )
    .with_stroke(novadraw::Color::WHITE, 2.0);

    scene.add_child_to(container_id, Box::new(rect_0));
    scene.add_child_to(container_id, Box::new(rect_5));
    scene.add_child_to(container_id, Box::new(rect_15));
    scene.add_child_to(container_id, Box::new(rect_30));
    scene.add_child_to(container_id, Box::new(rect_fill));
    scene.add_child_to(container_id, Box::new(rect_stroke));
    scene.add_child_to(container_id, Box::new(rect_both));
    scene.add_child_to(container_id, Box::new(rect_sw_1));
    scene.add_child_to(container_id, Box::new(rect_sw_4));
    scene.add_child_to(container_id, Box::new(rect_sw_8));
    scene.add_child_to(container_id, Box::new(rect_circle));

    scene
}

/// Scene 3: 折线图形 - 验证 PolylineFigure
/// MECE: 按折线属性分类（点数、线宽、线帽、连接样式、闭合）
fn create_scene_3_polyline() -> novadraw::SceneGraph {
    let mut scene = novadraw::SceneGraph::new();

    let container = novadraw::RectangleFigure::new_with_color(
        0.0,
        0.0,
        WINDOW_WIDTH as f64,
        WINDOW_HEIGHT as f64,
        novadraw::Color::rgba(0.15, 0.15, 0.15, 1.0),
    );
    let container_id = scene.set_contents(Box::new(container));

    // ============================================================
    // 测试1: 不同点数的折线
    // ============================================================
    // 2点折线（直线）
    let line_2pt =
        novadraw::PolylineFigure::new_with_color(50.0, 40.0, 200.0, 40.0, novadraw::Color::WHITE)
            .with_width(3.0);
    scene.add_child_to(container_id, Box::new(line_2pt));

    // 3点折线（折线）
    let line_3pt = novadraw::PolylineFigure::from_points(vec![
        novadraw_geometry::Vec2::new(50.0, 80.0),
        novadraw_geometry::Vec2::new(125.0, 40.0),
        novadraw_geometry::Vec2::new(200.0, 80.0),
    ])
    .with_width(3.0);
    scene.add_child_to(container_id, Box::new(line_3pt));

    // 5点折线（多段折线）
    let line_5pt = novadraw::PolylineFigure::from_points(vec![
        novadraw_geometry::Vec2::new(50.0, 120.0),
        novadraw_geometry::Vec2::new(100.0, 80.0),
        novadraw_geometry::Vec2::new(150.0, 160.0),
        novadraw_geometry::Vec2::new(200.0, 120.0),
        novadraw_geometry::Vec2::new(250.0, 160.0),
    ])
    .with_width(3.0);
    scene.add_child_to(container_id, Box::new(line_5pt));

    // ============================================================
    // 测试2: 不同线宽
    // ============================================================
    let widths = [1.0, 2.0, 4.0, 6.0, 8.0];
    for (i, &w) in widths.iter().enumerate() {
        let line = novadraw::PolylineFigure::new_with_color(
            300.0 + i as f64 * 80.0,
            40.0,
            350.0 + i as f64 * 80.0,
            80.0,
            novadraw::Color::WHITE,
        )
        .with_width(w);
        scene.add_child_to(container_id, Box::new(line));
    }

    // ============================================================
    // 测试3: 不同线帽样式 (LineCap)
    // ============================================================
    let cap_butt = novadraw::PolylineFigure::new_with_color(
        50.0,
        200.0,
        150.0,
        200.0,
        novadraw::Color::rgba(1.0, 0.3, 0.3, 1.0),
    )
    .with_width(8.0)
    .with_cap(novadraw::render::command::LineCap::Butt);
    scene.add_child_to(container_id, Box::new(cap_butt));

    let cap_round = novadraw::PolylineFigure::new_with_color(
        200.0,
        200.0,
        300.0,
        200.0,
        novadraw::Color::rgba(0.3, 1.0, 0.3, 1.0),
    )
    .with_width(8.0)
    .with_cap(novadraw::render::command::LineCap::Round);
    scene.add_child_to(container_id, Box::new(cap_round));

    let cap_square = novadraw::PolylineFigure::new_with_color(
        350.0,
        200.0,
        450.0,
        200.0,
        novadraw::Color::rgba(0.3, 0.3, 1.0, 1.0),
    )
    .with_width(8.0)
    .with_cap(novadraw::render::command::LineCap::Square);
    scene.add_child_to(container_id, Box::new(cap_square));

    // ============================================================
    // 测试4: 不同连接样式 (LineJoin)
    // ============================================================
    // 尖角连接
    let join_miter = novadraw::PolylineFigure::from_points(vec![
        novadraw_geometry::Vec2::new(50.0, 260.0),
        novadraw_geometry::Vec2::new(100.0, 220.0),
        novadraw_geometry::Vec2::new(150.0, 300.0),
    ])
    .with_width(8.0)
    .with_join(novadraw::render::command::LineJoin::Miter)
    .with_color(novadraw::Color::rgba(1.0, 0.5, 0.0, 1.0));
    scene.add_child_to(container_id, Box::new(join_miter));

    // 圆角连接
    let join_round = novadraw::PolylineFigure::from_points(vec![
        novadraw_geometry::Vec2::new(200.0, 260.0),
        novadraw_geometry::Vec2::new(250.0, 220.0),
        novadraw_geometry::Vec2::new(300.0, 300.0),
    ])
    .with_width(8.0)
    .with_join(novadraw::render::command::LineJoin::Round)
    .with_color(novadraw::Color::rgba(0.0, 1.0, 1.0, 1.0));
    scene.add_child_to(container_id, Box::new(join_round));

    // 斜切连接
    let join_bevel = novadraw::PolylineFigure::from_points(vec![
        novadraw_geometry::Vec2::new(350.0, 260.0),
        novadraw_geometry::Vec2::new(400.0, 220.0),
        novadraw_geometry::Vec2::new(450.0, 300.0),
    ])
    .with_width(8.0)
    .with_join(novadraw::render::command::LineJoin::Bevel)
    .with_color(novadraw::Color::rgba(1.0, 0.0, 1.0, 1.0));
    scene.add_child_to(container_id, Box::new(join_bevel));

    // ============================================================
    // 测试5: 水平/垂直/对角线
    // ============================================================
    let h_line = novadraw::PolylineFigure::new_with_color(
        500.0,
        40.0,
        750.0,
        40.0,
        novadraw::Color::rgba(0.9, 0.2, 0.2, 1.0),
    )
    .with_width(3.0);
    scene.add_child_to(container_id, Box::new(h_line));

    let v_line = novadraw::PolylineFigure::new_with_color(
        700.0,
        50.0,
        700.0,
        150.0,
        novadraw::Color::rgba(0.2, 0.2, 0.9, 1.0),
    )
    .with_width(3.0);
    scene.add_child_to(container_id, Box::new(v_line));

    let diag_45 = novadraw::PolylineFigure::new_with_color(
        500.0,
        200.0,
        650.0,
        350.0,
        novadraw::Color::rgba(0.2, 0.9, 0.2, 1.0),
    )
    .with_width(3.0);
    scene.add_child_to(container_id, Box::new(diag_45));

    let diag_135 = novadraw::PolylineFigure::new_with_color(
        650.0,
        200.0,
        500.0,
        350.0,
        novadraw::Color::rgba(0.9, 0.5, 0.1, 1.0),
    )
    .with_width(3.0);
    scene.add_child_to(container_id, Box::new(diag_135));

    // ============================================================
    // 测试6: 自相交折线
    // ============================================================
    let self_intersect = novadraw::PolylineFigure::from_points(vec![
        novadraw_geometry::Vec2::new(50.0, 380.0),
        novadraw_geometry::Vec2::new(150.0, 480.0),
        novadraw_geometry::Vec2::new(150.0, 380.0),
        novadraw_geometry::Vec2::new(50.0, 480.0),
    ])
    .with_width(2.0)
    .with_color(novadraw::Color::rgba(1.0, 1.0, 0.0, 1.0));
    scene.add_child_to(container_id, Box::new(self_intersect));

    // ============================================================
    // 测试7: 密集多段折线（波浪形）
    // ============================================================
    let mut points = Vec::new();
    for i in 0..20 {
        let x = 250.0 + i as f64 * 25.0;
        let y = 400.0 + (i as f64 * 25.0).sin() * 50.0;
        points.push(novadraw_geometry::Vec2::new(x, y));
    }
    let wave = novadraw::PolylineFigure::from_points(points)
        .with_width(2.0)
        .with_color(novadraw::Color::rgba(0.0, 0.8, 1.0, 1.0));
    scene.add_child_to(container_id, Box::new(wave));

    // ============================================================
    // 测试8: 短折线（端点测试）
    // ============================================================
    let short_1 = novadraw::PolylineFigure::new_with_color(
        600.0,
        380.0,
        610.0,
        390.0,
        novadraw::Color::WHITE,
    )
    .with_width(3.0);
    scene.add_child_to(container_id, Box::new(short_1));

    let short_2 = novadraw::PolylineFigure::new_with_color(
        640.0,
        380.0,
        650.0,
        380.0,
        novadraw::Color::WHITE,
    )
    .with_width(3.0);
    scene.add_child_to(container_id, Box::new(short_2));

    let short_3 = novadraw::PolylineFigure::new_with_color(
        680.0,
        380.0,
        680.0,
        390.0,
        novadraw::Color::WHITE,
    )
    .with_width(3.0);
    scene.add_child_to(container_id, Box::new(short_3));

    // ============================================================
    // 测试9: 坐标边界（靠近边界）
    // ============================================================
    let edge_1 = novadraw::PolylineFigure::new_with_color(
        0.0,
        500.0,
        100.0,
        500.0,
        novadraw::Color::rgba(1.0, 0.3, 0.7, 1.0),
    )
    .with_width(4.0);
    scene.add_child_to(container_id, Box::new(edge_1));

    let edge_2 = novadraw::PolylineFigure::new_with_color(
        700.0,
        500.0,
        800.0,
        500.0,
        novadraw::Color::rgba(1.0, 0.3, 0.7, 1.0),
    )
    .with_width(4.0);
    scene.add_child_to(container_id, Box::new(edge_2));

    scene
}

// ============================================================================
// 维度2: 属性验证 (MECE - 按属性分类)
// ============================================================================

/// Scene 3: 描边属性 - 验证 Stroke 属性
/// MECE: 单独验证 Rectangle/Line 的描边
fn create_scene_4_stroke_width() -> novadraw::SceneGraph {
    let mut scene = novadraw::SceneGraph::new();

    let container = novadraw::RectangleFigure::new(0.0, 0.0, WINDOW_WIDTH, WINDOW_HEIGHT);
    let container_id = scene.set_contents(Box::new(container));

    // 不同描边宽度的矩形
    let rect_1 = novadraw::RectangleFigure::new_with_color(
        50.0,
        50.0,
        150.0,
        100.0,
        novadraw::Color::rgba(0.9, 0.5, 0.1, 1.0),
    )
    .with_stroke(novadraw::Color::WHITE, 1.0);
    let rect_2 = novadraw::RectangleFigure::new_with_color(
        250.0,
        50.0,
        150.0,
        100.0,
        novadraw::Color::rgba(0.1, 0.5, 0.9, 1.0),
    )
    .with_stroke(novadraw::Color::WHITE, 2.0);
    let rect_3 = novadraw::RectangleFigure::new_with_color(
        450.0,
        50.0,
        150.0,
        100.0,
        novadraw::Color::rgba(0.2, 0.8, 0.3, 1.0),
    )
    .with_stroke(novadraw::Color::WHITE, 4.0);
    let rect_4 = novadraw::RectangleFigure::new_with_color(
        650.0,
        50.0,
        100.0,
        100.0,
        novadraw::Color::rgba(0.9, 0.2, 0.2, 1.0),
    )
    .with_stroke(novadraw::Color::WHITE, 8.0);

    // 不同描边颜色的矩形
    let rect_5 = novadraw::RectangleFigure::new_with_color(
        50.0,
        200.0,
        150.0,
        100.0,
        novadraw::Color::rgba(0.3, 0.3, 0.3, 1.0),
    )
    .with_stroke(novadraw::Color::RED, 3.0);
    let rect_6 = novadraw::RectangleFigure::new_with_color(
        250.0,
        200.0,
        150.0,
        100.0,
        novadraw::Color::rgba(0.3, 0.3, 0.3, 1.0),
    )
    .with_stroke(novadraw::Color::GREEN, 3.0);
    let rect_7 = novadraw::RectangleFigure::new_with_color(
        450.0,
        200.0,
        150.0,
        100.0,
        novadraw::Color::rgba(0.3, 0.3, 0.3, 1.0),
    )
    .with_stroke(novadraw::Color::rgba(0.0, 0.0, 1.0, 1.0), 3.0);
    let rect_8 = novadraw::RectangleFigure::new_with_color(
        650.0,
        200.0,
        100.0,
        100.0,
        novadraw::Color::rgba(0.3, 0.3, 0.3, 1.0),
    )
    .with_stroke(novadraw::Color::rgba(1.0, 1.0, 0.0, 1.0), 3.0);

    // 粗描边椭圆
    let ellipse_1 = novadraw::EllipseFigure::new_with_color(
        100.0,
        350.0,
        120.0,
        80.0,
        novadraw::Color::rgba(0.9, 0.3, 0.6, 1.0),
    )
    .with_stroke(novadraw::Color::WHITE, 2.0);
    let ellipse_2 = novadraw::EllipseFigure::new_with_color(
        300.0,
        350.0,
        120.0,
        80.0,
        novadraw::Color::rgba(0.3, 0.9, 0.6, 1.0),
    )
    .with_stroke(novadraw::Color::WHITE, 4.0);
    let ellipse_3 = novadraw::EllipseFigure::new_with_color(
        500.0,
        350.0,
        120.0,
        80.0,
        novadraw::Color::rgba(0.6, 0.3, 0.9, 1.0),
    )
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
/// Scene 5: 描边属性
/// MECE: 验证描边宽度和颜色
fn create_scene_5_stroke_color() -> novadraw::SceneGraph {
    let mut scene = novadraw::SceneGraph::new();

    // 灰色背景
    let bg = novadraw::RectangleFigure::new_with_color(
        0.0,
        0.0,
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
        novadraw::Color::rgba(0.2, 0.2, 0.2, 1.0),
    );
    let bg_id = scene.set_contents(Box::new(bg));

    // 测试不同描边宽度
    let rect_1 = novadraw::RectangleFigure::new_with_color(
        50.0,
        50.0,
        150.0,
        100.0,
        novadraw::Color::rgba(0.9, 0.3, 0.3, 1.0),
    )
    .with_stroke(novadraw::Color::rgba(0.0, 1.0, 1.0, 1.0), 2.0);
    let rect_2 = novadraw::RectangleFigure::new_with_color(
        250.0,
        50.0,
        150.0,
        100.0,
        novadraw::Color::rgba(0.3, 0.9, 0.3, 1.0),
    )
    .with_stroke(novadraw::Color::rgba(0.0, 1.0, 1.0, 1.0), 4.0);
    let rect_3 = novadraw::RectangleFigure::new_with_color(
        450.0,
        50.0,
        150.0,
        100.0,
        novadraw::Color::rgba(0.3, 0.3, 0.9, 1.0),
    )
    .with_stroke(novadraw::Color::rgba(0.0, 1.0, 1.0, 1.0), 8.0);

    // 测试不同描边颜色
    let rect_4 = novadraw::RectangleFigure::new_with_color(
        50.0,
        200.0,
        150.0,
        100.0,
        novadraw::Color::rgba(0.3, 0.3, 0.3, 1.0),
    )
    .with_stroke(novadraw::Color::RED, 4.0);
    let rect_5 = novadraw::RectangleFigure::new_with_color(
        250.0,
        200.0,
        150.0,
        100.0,
        novadraw::Color::rgba(0.3, 0.3, 0.3, 1.0),
    )
    .with_stroke(novadraw::Color::GREEN, 4.0);
    let rect_6 = novadraw::RectangleFigure::new_with_color(
        450.0,
        200.0,
        150.0,
        100.0,
        novadraw::Color::rgba(0.3, 0.3, 0.3, 1.0),
    )
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
fn create_scene_6_line_join() -> novadraw::SceneGraph {
    let mut scene = novadraw::SceneGraph::new();

    // 灰色背景
    let bg = novadraw::RectangleFigure::new_with_color(
        0.0,
        0.0,
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
        novadraw::Color::rgba(0.2, 0.2, 0.2, 1.0),
    );
    let bg_id = scene.set_contents(Box::new(bg));

    // 测试不同 LineJoin - 更粗的描边使差异更明显
    // Miter (默认) - 尖角
    let rect_1 = novadraw::RectangleFigure::new_with_color(
        50.0,
        50.0,
        150.0,
        100.0,
        novadraw::Color::rgba(0.9, 0.3, 0.3, 1.0),
    )
    .with_stroke(novadraw::Color::WHITE, 24.0);

    // Round - 圆角
    let mut rect_2 = novadraw::RectangleFigure::new_with_color(
        250.0,
        50.0,
        150.0,
        100.0,
        novadraw::Color::rgba(0.3, 0.9, 0.3, 1.0),
    )
    .with_stroke(novadraw::Color::WHITE, 24.0);
    rect_2.line_join = LineJoin::Round;

    // Bevel - 斜切
    let mut rect_3 = novadraw::RectangleFigure::new_with_color(
        450.0,
        50.0,
        150.0,
        100.0,
        novadraw::Color::rgba(0.3, 0.3, 0.9, 1.0),
    )
    .with_stroke(novadraw::Color::WHITE, 24.0);
    rect_3.line_join = LineJoin::Bevel;

    scene.add_child_to(bg_id, Box::new(rect_1));
    scene.add_child_to(bg_id, Box::new(rect_2));
    scene.add_child_to(bg_id, Box::new(rect_3));

    scene
}

/// Scene 6: 透明度验证
/// MECE: 单独验证 Alpha 透明度
fn create_scene_7_alpha() -> novadraw::SceneGraph {
    let mut scene = novadraw::SceneGraph::new();

    let container = novadraw::RectangleFigure::new(0.0, 0.0, WINDOW_WIDTH, WINDOW_HEIGHT);
    let container_id = scene.set_contents(Box::new(container));

    // 不同透明度层级 (堆叠验证 alpha 混合)
    let alpha_levels = [1.0, 0.8, 0.6, 0.4, 0.2];

    for (i, &alpha) in alpha_levels.iter().enumerate() {
        let x = 50.0 + (i as f64) * 120.0;
        let rect = novadraw::RectangleFigure::new_with_color(
            x,
            50.0,
            100.0,
            100.0,
            novadraw::Color::rgba(0.9, 0.2, 0.2, alpha),
        );
        scene.add_child_to(container_id, Box::new(rect));
    }

    // 椭圆透明度
    for (i, &alpha) in alpha_levels.iter().enumerate() {
        let x = 50.0 + (i as f64) * 120.0;
        let ellipse = novadraw::EllipseFigure::new_with_color(
            x + 10.0,
            200.0,
            80.0,
            80.0,
            novadraw::Color::rgba(0.2, 0.5, 0.9, alpha),
        );
        scene.add_child_to(container_id, Box::new(ellipse));
    }

    // 直线透明度
    for (i, &alpha) in alpha_levels.iter().enumerate() {
        let y = 350.0 + (i as f64) * 40.0;
        let line = novadraw::PolylineFigure::new_with_color(
            50.0,
            y,
            700.0,
            y,
            novadraw::Color::rgba(0.2, 0.8, 0.2, alpha),
        )
        .with_width(6.0);
        scene.add_child_to(container_id, Box::new(line));
    }

    // 半透明叠加验证
    let red_overlay = novadraw::RectangleFigure::new_with_color(
        550.0,
        50.0,
        100.0,
        100.0,
        novadraw::Color::rgba(1.0, 0.0, 0.0, 0.5),
    );
    let blue_overlay = novadraw::RectangleFigure::new_with_color(
        580.0,
        80.0,
        100.0,
        100.0,
        novadraw::Color::rgba(0.0, 0.0, 1.0, 0.5),
    );
    let green_overlay = novadraw::RectangleFigure::new_with_color(
        610.0,
        110.0,
        100.0,
        100.0,
        novadraw::Color::rgba(0.0, 1.0, 0.0, 0.5),
    );

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
fn create_scene_8_mixed_shapes() -> novadraw::SceneGraph {
    let mut scene = novadraw::SceneGraph::new();

    let container = novadraw::RectangleFigure::new(0.0, 0.0, WINDOW_WIDTH, WINDOW_HEIGHT);
    let container_id = scene.set_contents(Box::new(container));

    // 组合1: 矩形 + 椭圆
    let rect_combo = novadraw::RectangleFigure::new_with_color(
        50.0,
        50.0,
        150.0,
        100.0,
        novadraw::Color::rgba(0.9, 0.5, 0.1, 1.0),
    )
    .with_stroke(novadraw::Color::BLACK, 2.0);
    let ellipse_combo = novadraw::EllipseFigure::new_with_color(
        125.0,
        100.0,
        80.0,
        50.0,
        novadraw::Color::rgba(0.1, 0.5, 0.9, 1.0),
    );

    // 组合2: 椭圆 + 直线
    let ellipse_combo2 = novadraw::EllipseFigure::new_with_color(
        300.0,
        80.0,
        100.0,
        60.0,
        novadraw::Color::rgba(0.9, 0.2, 0.2, 1.0),
    );
    let line_through = novadraw::PolylineFigure::new_with_color(
        250.0,
        110.0,
        400.0,
        110.0,
        novadraw::Color::BLACK,
    )
    .with_width(2.0);

    // 组合3: 矩形框住多个图形
    let container_rect = novadraw::RectangleFigure::new_with_color(
        450.0,
        50.0,
        200.0,
        150.0,
        novadraw::Color::rgba(0.95, 0.95, 0.95, 1.0),
    )
    .with_stroke(novadraw::Color::rgba(0.2, 0.2, 0.2, 1.0), 1.0);
    let inner_rect = novadraw::RectangleFigure::new_with_color(
        470.0,
        70.0,
        60.0,
        40.0,
        novadraw::Color::rgba(0.9, 0.3, 0.3, 1.0),
    );
    let inner_ellipse = novadraw::EllipseFigure::new_with_color(
        560.0,
        90.0,
        50.0,
        30.0,
        novadraw::Color::rgba(0.3, 0.9, 0.3, 1.0),
    );
    let inner_line = novadraw::PolylineFigure::new_with_color(
        480.0,
        150.0,
        620.0,
        150.0,
        novadraw::Color::rgba(0.3, 0.3, 0.9, 1.0),
    )
    .with_width(2.0);

    // 组合4: 复杂图形组合
    let complex_rect = novadraw::RectangleFigure::new_with_color(
        50.0,
        250.0,
        120.0,
        80.0,
        novadraw::Color::rgba(0.8, 0.2, 0.5, 1.0),
    )
    .with_stroke(novadraw::Color::WHITE, 3.0);
    let complex_ellipse = novadraw::EllipseFigure::new_with_color(
        50.0,
        350.0,
        120.0,
        80.0,
        novadraw::Color::rgba(0.2, 0.5, 0.8, 1.0),
    )
    .with_stroke(novadraw::Color::WHITE, 3.0);
    let complex_line1 = novadraw::PolylineFigure::new_with_color(
        200.0,
        250.0,
        300.0,
        280.0,
        novadraw::Color::rgba(0.9, 0.6, 0.1, 1.0),
    )
    .with_width(3.0);
    let complex_line2 = novadraw::PolylineFigure::new_with_color(
        200.0,
        280.0,
        300.0,
        250.0,
        novadraw::Color::rgba(0.1, 0.6, 0.9, 1.0),
    )
    .with_width(3.0);
    let complex_line3 = novadraw::PolylineFigure::new_with_color(
        250.0,
        350.0,
        250.0,
        420.0,
        novadraw::Color::rgba(0.3, 0.8, 0.3, 1.0),
    )
    .with_width(4.0);

    // 组合5: 细线装饰
    let decor_line1 = novadraw::PolylineFigure::new_with_color(
        400.0,
        260.0,
        550.0,
        260.0,
        novadraw::Color::rgba(0.3, 0.3, 0.3, 1.0),
    )
    .with_width(1.0);
    let decor_line2 = novadraw::PolylineFigure::new_with_color(
        400.0,
        300.0,
        550.0,
        300.0,
        novadraw::Color::rgba(0.3, 0.3, 0.3, 1.0),
    )
    .with_width(2.0);
    let decor_line3 = novadraw::PolylineFigure::new_with_color(
        400.0,
        350.0,
        550.0,
        350.0,
        novadraw::Color::rgba(0.3, 0.3, 0.3, 1.0),
    )
    .with_width(4.0);

    // 边框装饰
    let border_outer = novadraw::RectangleFigure::new_with_color(
        600.0,
        250.0,
        150.0,
        150.0,
        novadraw::Color::rgba(0.9, 0.9, 0.9, 1.0),
    )
    .with_stroke(novadraw::Color::BLACK, 1.0);
    let border_inner = novadraw::RectangleFigure::new_with_color(
        610.0,
        260.0,
        130.0,
        130.0,
        novadraw::Color::WHITE,
    )
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
fn create_scene_9_zorder() -> novadraw::SceneGraph {
    let mut scene = novadraw::SceneGraph::new();

    let container = novadraw::RectangleFigure::new(0.0, 0.0, WINDOW_WIDTH, WINDOW_HEIGHT);
    let container_id = scene.set_contents(Box::new(container));

    // 底层图形（先添加）
    let bottom_rect = novadraw::RectangleFigure::new_with_color(
        100.0,
        100.0,
        300.0,
        200.0,
        novadraw::Color::BLUE,
    );
    let bottom_ellipse =
        novadraw::EllipseFigure::new_with_color(250.0, 150.0, 200.0, 100.0, novadraw::Color::GREEN);
    let bottom_line = novadraw::PolylineFigure::new_with_color(
        150.0,
        200.0,
        400.0,
        250.0,
        novadraw::Color::rgba(1.0, 1.0, 0.0, 1.0),
    )
    .with_width(8.0);

    // 中间层（遮挡部分底层）
    let mid_rect =
        novadraw::RectangleFigure::new_with_color(200.0, 150.0, 200.0, 120.0, novadraw::Color::RED);
    let mid_ellipse = novadraw::EllipseFigure::new_with_color(
        350.0,
        200.0,
        120.0,
        80.0,
        novadraw::Color::rgba(1.0, 0.65, 0.0, 1.0),
    );

    // 顶层（最后添加，遮挡所有底层）
    let top_rect = novadraw::RectangleFigure::new_with_color(
        300.0,
        180.0,
        150.0,
        100.0,
        novadraw::Color::rgba(0.5, 0.0, 0.5, 1.0),
    );

    // 验证 Z-order 的辅助标记
    let marker_1 = novadraw::RectangleFigure::new_with_color(
        50.0,
        50.0,
        30.0,
        30.0,
        novadraw::Color::rgba(0.2, 0.2, 0.8, 1.0),
    );
    let marker_2 = novadraw::RectangleFigure::new_with_color(
        80.0,
        50.0,
        30.0,
        30.0,
        novadraw::Color::rgba(0.2, 0.8, 0.2, 1.0),
    );
    let marker_3 = novadraw::RectangleFigure::new_with_color(
        110.0,
        50.0,
        30.0,
        30.0,
        novadraw::Color::rgba(0.8, 0.2, 0.2, 1.0),
    );

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

/// Scene 10: 三角形图形 - 验证 TriangleFigure
/// MECE: 验证等边三角形、直角三角形、描边宽度、纯填充/纯描边
fn create_scene_10_triangle() -> novadraw::SceneGraph {
    let mut scene = novadraw::SceneGraph::new();

    let container = novadraw::RectangleFigure::new_with_color(
        0.0,
        0.0,
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
        novadraw::Color::hex("#eeeeee"),
    );
    let container_id = scene.set_contents(Box::new(container));

    // 不同朝向的三角形（参考 d2 Triangle，direction 属性）
    // NORTH - 顶点朝上
    let tri_north = novadraw::TriangleFigure::new_with_direction(50.0, 50.0, 80.0, 80.0, novadraw::Direction::North)
        .with_fill_color(novadraw::Color::hex("#e74c3c"))
        .with_stroke_color(novadraw::Color::BLACK)
        .with_stroke_width(3.0);

    // SOUTH - 顶点朝下
    let tri_south = novadraw::TriangleFigure::new_with_direction(180.0, 50.0, 80.0, 80.0, novadraw::Direction::South)
        .with_fill_color(novadraw::Color::hex("#2ecc71"))
        .with_stroke_color(novadraw::Color::BLACK)
        .with_stroke_width(3.0);

    // EAST - 顶点朝右
    let tri_east = novadraw::TriangleFigure::new_with_direction(310.0, 50.0, 80.0, 80.0, novadraw::Direction::East)
        .with_fill_color(novadraw::Color::hex("#3498db"))
        .with_stroke_color(novadraw::Color::BLACK)
        .with_stroke_width(3.0);

    // WEST - 顶点朝左
    let tri_west = novadraw::TriangleFigure::new_with_direction(440.0, 50.0, 80.0, 80.0, novadraw::Direction::West)
        .with_fill_color(novadraw::Color::hex("#9b59b6"))
        .with_stroke_color(novadraw::Color::BLACK)
        .with_stroke_width(3.0);

    // 不同尺寸
    let tri_small = novadraw::TriangleFigure::new(50.0, 180.0, 40.0, 40.0)
        .with_fill_color(novadraw::Color::hex("#f39c12"))
        .with_stroke_color(novadraw::Color::BLACK)
        .with_stroke_width(2.0);

    let tri_medium = novadraw::TriangleFigure::new(120.0, 180.0, 80.0, 80.0)
        .with_fill_color(novadraw::Color::hex("#f39c12"))
        .with_stroke_color(novadraw::Color::BLACK)
        .with_stroke_width(2.0);

    let tri_large = novadraw::TriangleFigure::new(240.0, 180.0, 120.0, 120.0)
        .with_fill_color(novadraw::Color::hex("#f39c12"))
        .with_stroke_color(novadraw::Color::BLACK)
        .with_stroke_width(2.0);

    // 纯填充和纯描边
    let tri_fill = novadraw::TriangleFigure::new_with_direction(400.0, 180.0, 60.0, 60.0, novadraw::Direction::North)
        .with_fill_color(novadraw::Color::hex("#e91e63"))
        .with_stroke_color(novadraw::Color::TRANSPARENT)
        .with_stroke_width(0.0);

    let tri_stroke = novadraw::TriangleFigure::new_with_direction(500.0, 180.0, 60.0, 60.0, novadraw::Direction::North)
        .with_fill_color(novadraw::Color::TRANSPARENT)
        .with_stroke_color(novadraw::Color::hex("#e91e63"))
        .with_stroke_width(3.0);

    scene.add_child_to(container_id, Box::new(tri_north));
    scene.add_child_to(container_id, Box::new(tri_south));
    scene.add_child_to(container_id, Box::new(tri_east));
    scene.add_child_to(container_id, Box::new(tri_west));
    scene.add_child_to(container_id, Box::new(tri_small));
    scene.add_child_to(container_id, Box::new(tri_medium));
    scene.add_child_to(container_id, Box::new(tri_large));
    scene.add_child_to(container_id, Box::new(tri_fill));
    scene.add_child_to(container_id, Box::new(tri_stroke));

    scene
}

/// Scene 9: 父子嵌套结构
/// MECE: 验证 Figure 嵌套渲染
fn create_scene_11_parent_child() -> novadraw::SceneGraph {
    let mut scene = novadraw::SceneGraph::new();

    let container = novadraw::RectangleFigure::new(0.0, 0.0, WINDOW_WIDTH, WINDOW_HEIGHT);
    let container_id = scene.set_contents(Box::new(container));

    // 父矩形
    let parent_rect = novadraw::RectangleFigure::new_with_color(
        50.0,
        50.0,
        350.0,
        250.0,
        novadraw::Color::rgba(0.9, 0.5, 0.1, 0.3),
    )
    .with_stroke(novadraw::Color::rgba(1.0, 0.65, 0.0, 1.0), 2.0);
    let parent_id = scene.add_child_to(container_id, Box::new(parent_rect));

    // 子图形 - 直接添加到父矩形中
    let child_rect = novadraw::RectangleFigure::new_with_color(
        80.0,
        80.0,
        100.0,
        60.0,
        novadraw::Color::rgba(0.2, 0.5, 0.9, 1.0),
    );
    let child_ellipse = novadraw::EllipseFigure::new_with_color(
        200.0,
        100.0,
        80.0,
        50.0,
        novadraw::Color::rgba(0.9, 0.2, 0.2, 1.0),
    );
    let child_line = novadraw::PolylineFigure::new_with_color(
        100.0,
        200.0,
        300.0,
        200.0,
        novadraw::Color::rgba(0.2, 0.8, 0.2, 1.0),
    )
    .with_width(3.0);

    scene.add_child_to(parent_id, Box::new(child_rect));
    scene.add_child_to(parent_id, Box::new(child_ellipse));
    scene.add_child_to(parent_id, Box::new(child_line));

    // 另一个父容器
    let parent_rect2 = novadraw::RectangleFigure::new_with_color(
        450.0,
        50.0,
        300.0,
        200.0,
        novadraw::Color::rgba(0.5, 0.2, 0.8, 0.3),
    )
    .with_stroke(novadraw::Color::rgba(0.5, 0.0, 0.5, 1.0), 2.0);
    let parent_id2 = scene.add_child_to(container_id, Box::new(parent_rect2));

    // 子图形 - 更深层次
    let grandchild_rect = novadraw::RectangleFigure::new_with_color(
        480.0,
        80.0,
        80.0,
        60.0,
        novadraw::Color::rgba(0.3, 0.8, 0.5, 1.0),
    );
    let grandchild_ellipse = novadraw::EllipseFigure::new_with_color(
        580.0,
        100.0,
        60.0,
        40.0,
        novadraw::Color::rgba(0.8, 0.5, 0.2, 1.0),
    );

    scene.add_child_to(parent_id2, Box::new(grandchild_rect));
    scene.add_child_to(parent_id2, Box::new(grandchild_ellipse));

    // 独立图形（不嵌套）
    let standalone = novadraw::RectangleFigure::new_with_color(
        50.0,
        350.0,
        150.0,
        100.0,
        novadraw::Color::rgba(0.1, 0.7, 0.7, 1.0),
    )
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
        (
            "0:Rectangle Fill",
            Box::new(|| create_scene_0_rectangle_fill()),
        ),
        ("1:Ellipse Fill", Box::new(|| create_scene_1_ellipse_fill())),
        ("2:Rounded Rect", Box::new(|| create_scene_2_rounded_rect())),
        ("3:Polyline", Box::new(|| create_scene_3_polyline())),
        // 维度2: 属性验证
        ("4:Stroke Width", Box::new(|| create_scene_4_stroke_width())),
        ("5:Stroke Color", Box::new(|| create_scene_5_stroke_color())),
        ("6:LineJoin", Box::new(|| create_scene_6_line_join())),
        ("7:Alpha", Box::new(|| create_scene_7_alpha())),
        // 维度3: 组合验证
        ("8:Mixed Shapes", Box::new(|| create_scene_8_mixed_shapes())),
        ("9:Z-Order", Box::new(|| create_scene_9_zorder())),
        ("10:Triangle", Box::new(|| create_scene_10_triangle())),
        (
            "11:Parent-Child",
            Box::new(|| create_scene_11_parent_child()),
        ),
    ];

    // 检查截图模式
    let title = "Shape App";
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
                let scene_idx = arg
                    .strip_prefix("--screenshot=")
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
