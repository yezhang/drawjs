//! Border App - Border 装饰器验证
//!
//! 验证 Stroke (Shape 级别) 和 Border (装饰器级别) 的功能和区别。

use novadraw::border::{LineBorder, MarginBorder, RectangleBorder};
use novadraw_apps::{run_demo_app, run_demo_app_with_screenshot};

const WINDOW_WIDTH: f64 = 800.0;
const WINDOW_HEIGHT: f64 = 600.0;

// ============================================================================
// Stroke 级别场景 (使用 with_stroke)
// ============================================================================

/// 场景 0: Stroke 基础 - 不同线宽和颜色
fn create_scene_0_stroke_basic() -> novadraw::FigureGraph {
    let mut scene = novadraw::FigureGraph::new();
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
        let rect = novadraw::RectangleFigure::new_with_color(
            50.0 + i as f64 * 145.0,
            50.0,
            130.0,
            80.0,
            novadraw::Color::rgba(0.95, 0.95, 0.95, 1.0),
        )
        .with_stroke(*color, *width);
        let _rect = scene.add_child_to(container_id, Box::new(rect));
    }

    scene
}

/// 场景 1: Stroke 嵌套 - 多层边框
fn create_scene_1_stroke_nested() -> novadraw::FigureGraph {
    let mut scene = novadraw::FigureGraph::new();
    let container = novadraw::RectangleFigure::new(0.0, 0.0, WINDOW_WIDTH, WINDOW_HEIGHT);
    let container_id = scene.set_contents(Box::new(container));

    let outer = novadraw::RectangleFigure::new_with_color(
        100.0,
        80.0,
        600.0,
        400.0,
        novadraw::Color::rgba(0.9, 0.95, 0.9, 1.0),
    )
    .with_stroke(novadraw::Color::rgba(0.2, 0.4, 0.2, 1.0), 4.0);
    let _outer = scene.add_child_to(container_id, Box::new(outer));

    let inner1 = novadraw::RectangleFigure::new_with_color(
        130.0,
        110.0,
        180.0,
        120.0,
        novadraw::Color::rgba(0.8, 0.9, 1.0, 1.0),
    )
    .with_stroke(novadraw::Color::rgba(0.1, 0.2, 0.4, 1.0), 2.0);
    let _inner1 = scene.add_child_to(container_id, Box::new(inner1));

    let inner2 = novadraw::RectangleFigure::new_with_color(
        350.0,
        110.0,
        300.0,
        120.0,
        novadraw::Color::rgba(1.0, 0.95, 0.8, 1.0),
    )
    .with_stroke(novadraw::Color::rgba(0.4, 0.3, 0.1, 1.0), 2.0);
    let _inner2 = scene.add_child_to(container_id, Box::new(inner2));

    let inner3 = novadraw::RectangleFigure::new_with_color(
        130.0,
        280.0,
        520.0,
        150.0,
        novadraw::Color::rgba(0.95, 0.9, 0.95, 1.0),
    )
    .with_stroke(novadraw::Color::rgba(0.3, 0.2, 0.3, 1.0), 2.0);
    let _inner3 = scene.add_child_to(container_id, Box::new(inner3));

    scene
}

/// 场景 2: Stroke 同心矩形
fn create_scene_2_stroke_concentric() -> novadraw::FigureGraph {
    let mut scene = novadraw::FigureGraph::new();
    let container = novadraw::RectangleFigure::new(0.0, 0.0, WINDOW_WIDTH, WINDOW_HEIGHT);
    let container_id = scene.set_contents(Box::new(container));

    // 同心矩形：固定右下角，从外向内缩小
    // 右下角固定在 (600, 400)
    let start_x = 200.0;
    let start_y = 150.0;
    let start_width = 400.0;
    let start_height = 250.0;

    for i in 0..10 {
        let offset = i as f64 * 2.0;
        // x, y 向右向下偏移，width, height 缩小，保持右下角对齐
        let rect = novadraw::RectangleFigure::new_with_color(
            start_x + offset,
            start_y + offset,
            start_width - offset * 2.0,
            start_height - offset * 2.0,
            novadraw::Color::rgba(
                0.3 + i as f64 * 0.05,
                0.5 + i as f64 * 0.03,
                0.7 - i as f64 * 0.02,
                1.0,
            ),
        )
        .with_stroke(novadraw::Color::rgba(0.1, 0.2, 0.4, 1.0), 1.0);
        let _rect = scene.add_child_to(container_id, Box::new(rect));
    }

    scene
}

/// 场景 3: Stroke 颜色对比
fn create_scene_3_stroke_colors() -> novadraw::FigureGraph {
    let mut scene = novadraw::FigureGraph::new();
    let container = novadraw::RectangleFigure::new(0.0, 0.0, WINDOW_WIDTH, WINDOW_HEIGHT);
    let container_id = scene.set_contents(Box::new(container));

    let bg_colors = [
        novadraw::Color::rgba(0.9, 0.4, 0.4, 1.0),
        novadraw::Color::rgba(0.4, 0.9, 0.4, 1.0),
        novadraw::Color::rgba(0.4, 0.4, 0.9, 1.0),
        novadraw::Color::rgba(0.9, 0.9, 0.4, 1.0),
    ];

    for (i, bg_color) in bg_colors.iter().enumerate() {
        let rect = novadraw::RectangleFigure::new_with_color(
            50.0 + i as f64 * 190.0,
            100.0,
            170.0,
            120.0,
            *bg_color,
        )
        .with_stroke(novadraw::Color::rgba(0.1, 0.1, 0.1, 1.0), 3.0);
        let _rect = scene.add_child_to(container_id, Box::new(rect));
    }

    scene
}

// ============================================================================
// Border 装饰器场景 (使用 with_border)
// ============================================================================

/// 场景 4: RectangleBorder 装饰器
fn create_scene_4_rectangle_border() -> novadraw::FigureGraph {
    let mut scene = novadraw::FigureGraph::new();
    let container = novadraw::RectangleFigure::new(0.0, 0.0, WINDOW_WIDTH, WINDOW_HEIGHT);
    let container_id = scene.set_contents(Box::new(container));

    let rect1 = novadraw::RectangleFigure::new_with_color(
        50.0,
        50.0,
        200.0,
        120.0,
        novadraw::Color::rgba(0.9, 0.95, 1.0, 1.0),
    )
    .with_border(RectangleBorder::new(
        novadraw::Color::rgba(0.2, 0.3, 0.5, 1.0),
        2.0,
    ));
    let rect2 = novadraw::RectangleFigure::new_with_color(
        300.0,
        50.0,
        200.0,
        120.0,
        novadraw::Color::rgba(0.95, 0.9, 1.0, 1.0),
    )
    .with_border(RectangleBorder::new(
        novadraw::Color::rgba(0.5, 0.2, 0.3, 1.0),
        4.0,
    ));
    let rect3 = novadraw::RectangleFigure::new_with_color(
        550.0,
        50.0,
        200.0,
        120.0,
        novadraw::Color::rgba(1.0, 0.95, 0.9, 1.0),
    )
    .with_border(RectangleBorder::new(
        novadraw::Color::rgba(0.3, 0.5, 0.2, 1.0),
        6.0,
    ));

    let _r1 = scene.add_child_to(container_id, Box::new(rect1));
    let _r2 = scene.add_child_to(container_id, Box::new(rect2));
    let _r3 = scene.add_child_to(container_id, Box::new(rect3));

    scene
}

/// 场景 5: Border 装饰器 + insets
fn create_scene_5_border_with_insets() -> novadraw::FigureGraph {
    let mut scene = novadraw::FigureGraph::new();
    let container = novadraw::RectangleFigure::new(0.0, 0.0, WINDOW_WIDTH, WINDOW_HEIGHT);
    let container_id = scene.set_contents(Box::new(container));

    // 带 insets 的 RectangleBorder - insets 会影响子元素布局（需要布局系统支持）
    // 当前展示 insets 对边框位置的影响
    let rect1 = novadraw::RectangleFigure::new_with_color(
        50.0,
        50.0,
        200.0,
        150.0,
        novadraw::Color::rgba(0.9, 0.95, 1.0, 1.0),
    )
    .with_border(
        RectangleBorder::new(novadraw::Color::rgba(0.2, 0.3, 0.5, 1.0), 2.0)
            .with_insets(10.0, 10.0, 10.0, 10.0),
    );
    let rect2 = novadraw::RectangleFigure::new_with_color(
        300.0,
        50.0,
        200.0,
        150.0,
        novadraw::Color::rgba(0.95, 0.9, 1.0, 1.0),
    )
    .with_border(
        RectangleBorder::new(novadraw::Color::rgba(0.5, 0.2, 0.3, 1.0), 3.0)
            .with_insets(20.0, 20.0, 20.0, 20.0),
    );
    let rect3 = novadraw::RectangleFigure::new_with_color(
        550.0,
        50.0,
        200.0,
        150.0,
        novadraw::Color::rgba(1.0, 0.95, 0.9, 1.0),
    )
    .with_border(
        RectangleBorder::new(novadraw::Color::rgba(0.3, 0.5, 0.2, 1.0), 4.0)
            .with_insets(30.0, 30.0, 30.0, 30.0),
    );

    let _r1 = scene.add_child_to(container_id, Box::new(rect1));
    let _r2 = scene.add_child_to(container_id, Box::new(rect2));
    let _r3 = scene.add_child_to(container_id, Box::new(rect3));

    scene
}

/// 场景 6: LineBorder 装饰器
fn create_scene_6_line_border() -> novadraw::FigureGraph {
    let mut scene = novadraw::FigureGraph::new();
    let container = novadraw::RectangleFigure::new(0.0, 0.0, WINDOW_WIDTH, WINDOW_HEIGHT);
    let container_id = scene.set_contents(Box::new(container));

    let rect1 = novadraw::RectangleFigure::new_with_color(
        50.0,
        50.0,
        200.0,
        120.0,
        novadraw::Color::rgba(0.9, 0.95, 1.0, 1.0),
    )
    .with_border(LineBorder::new(
        novadraw::Color::rgba(0.2, 0.3, 0.5, 1.0),
        2.0,
    ));
    let rect2 = novadraw::RectangleFigure::new_with_color(
        300.0,
        50.0,
        200.0,
        120.0,
        novadraw::Color::rgba(0.95, 0.9, 1.0, 1.0),
    )
    .with_border(LineBorder::new(
        novadraw::Color::rgba(0.5, 0.2, 0.3, 1.0),
        3.0,
    ));
    let rect3 = novadraw::RectangleFigure::new_with_color(
        550.0,
        50.0,
        200.0,
        120.0,
        novadraw::Color::rgba(1.0, 0.95, 0.9, 1.0),
    )
    .with_border(LineBorder::new(
        novadraw::Color::rgba(0.3, 0.5, 0.2, 1.0),
        4.0,
    ));

    let _r1 = scene.add_child_to(container_id, Box::new(rect1));
    let _r2 = scene.add_child_to(container_id, Box::new(rect2));
    let _r3 = scene.add_child_to(container_id, Box::new(rect3));

    scene
}

/// 场景 7: MarginBorder 装饰器
fn create_scene_7_margin_border() -> novadraw::FigureGraph {
    let mut scene = novadraw::FigureGraph::new();
    let container = novadraw::RectangleFigure::new(0.0, 0.0, WINDOW_WIDTH, WINDOW_HEIGHT);
    let container_id = scene.set_contents(Box::new(container));

    // MarginBorder 用于绘制四边边框（通过设置 margin）
    let rect1 = novadraw::RectangleFigure::new_with_color(
        50.0,
        50.0,
        200.0,
        120.0,
        novadraw::Color::rgba(0.9, 0.95, 1.0, 1.0),
    )
    .with_border(
        MarginBorder::new(novadraw::Color::rgba(0.2, 0.3, 0.5, 1.0), 2.0)
            .with_margins(5.0, 5.0, 5.0, 5.0),
    );
    let rect2 = novadraw::RectangleFigure::new_with_color(
        300.0,
        50.0,
        200.0,
        120.0,
        novadraw::Color::rgba(0.95, 0.9, 1.0, 1.0),
    )
    .with_border(
        MarginBorder::new(novadraw::Color::rgba(0.5, 0.2, 0.3, 1.0), 3.0)
            .with_margins(10.0, 10.0, 10.0, 10.0),
    );
    let rect3 = novadraw::RectangleFigure::new_with_color(
        550.0,
        50.0,
        200.0,
        120.0,
        novadraw::Color::rgba(1.0, 0.95, 0.9, 1.0),
    )
    .with_border(
        MarginBorder::new(novadraw::Color::rgba(0.3, 0.5, 0.2, 1.0), 4.0)
            .with_margins(15.0, 15.0, 15.0, 15.0),
    );

    let _r1 = scene.add_child_to(container_id, Box::new(rect1));
    let _r2 = scene.add_child_to(container_id, Box::new(rect2));
    let _r3 = scene.add_child_to(container_id, Box::new(rect3));

    scene
}

// ============================================================================
// 对比场景
// ============================================================================

/// 场景 8: Stroke vs Border 对比
fn create_scene_8_stroke_vs_border() -> novadraw::FigureGraph {
    let mut scene = novadraw::FigureGraph::new();
    let container = novadraw::RectangleFigure::new(0.0, 0.0, WINDOW_WIDTH, WINDOW_HEIGHT);
    let container_id = scene.set_contents(Box::new(container));

    // 第一行：with_stroke (Shape 级别描边)
    // 描边绘制在图形边界上
    let stroke1 = novadraw::RectangleFigure::new_with_color(
        50.0,
        50.0,
        150.0,
        80.0,
        novadraw::Color::rgba(0.9, 0.95, 1.0, 1.0),
    )
    .with_stroke(novadraw::Color::rgba(0.2, 0.3, 0.5, 1.0), 2.0);
    let stroke2 = novadraw::RectangleFigure::new_with_color(
        250.0,
        50.0,
        150.0,
        80.0,
        novadraw::Color::rgba(0.9, 0.95, 1.0, 1.0),
    )
    .with_stroke(novadraw::Color::rgba(0.2, 0.3, 0.5, 1.0), 4.0);
    let stroke3 = novadraw::RectangleFigure::new_with_color(
        450.0,
        50.0,
        150.0,
        80.0,
        novadraw::Color::rgba(0.9, 0.95, 1.0, 1.0),
    )
    .with_stroke(novadraw::Color::rgba(0.2, 0.3, 0.5, 1.0), 8.0);

    let _s1 = scene.add_child_to(container_id, Box::new(stroke1));
    let _s2 = scene.add_child_to(container_id, Box::new(stroke2));
    let _s3 = scene.add_child_to(container_id, Box::new(stroke3));

    // 第二行：with_border (Border 装饰器)
    // 边框绘制在 paintBorder 阶段，可以有 insets 等高级特性
    let border1 = novadraw::RectangleFigure::new_with_color(
        50.0,
        180.0,
        150.0,
        80.0,
        novadraw::Color::rgba(0.95, 0.9, 1.0, 1.0),
    )
    .with_border(RectangleBorder::new(
        novadraw::Color::rgba(0.5, 0.2, 0.3, 1.0),
        2.0,
    ));
    let border2 = novadraw::RectangleFigure::new_with_color(
        250.0,
        180.0,
        150.0,
        80.0,
        novadraw::Color::rgba(0.95, 0.9, 1.0, 1.0),
    )
    .with_border(RectangleBorder::new(
        novadraw::Color::rgba(0.5, 0.2, 0.3, 1.0),
        4.0,
    ));
    let border3 = novadraw::RectangleFigure::new_with_color(
        450.0,
        180.0,
        150.0,
        80.0,
        novadraw::Color::rgba(0.95, 0.9, 1.0, 1.0),
    )
    .with_border(RectangleBorder::new(
        novadraw::Color::rgba(0.5, 0.2, 0.3, 1.0),
        8.0,
    ));

    let _b1 = scene.add_child_to(container_id, Box::new(border1));
    let _b2 = scene.add_child_to(container_id, Box::new(border2));
    let _b3 = scene.add_child_to(container_id, Box::new(border3));

    // 第三行：同时有 border 和 outline（两者叠加）
    // 使用不同颜色：stroke=绿色（内），border=红色（外），insets=8 让两者分开
    let both1 = novadraw::RectangleFigure::new_with_color(
        50.0,
        320.0,
        150.0,
        80.0,
        novadraw::Color::rgba(0.9, 0.95, 0.9, 1.0),
    )
    .with_stroke(novadraw::Color::rgba(0.2, 0.6, 0.2, 1.0), 2.0)
    .with_border(
        RectangleBorder::new(novadraw::Color::rgba(0.8, 0.2, 0.2, 1.0), 2.0)
            .with_insets(8.0, 8.0, 8.0, 8.0),
    );
    let both2 = novadraw::RectangleFigure::new_with_color(
        250.0,
        320.0,
        150.0,
        80.0,
        novadraw::Color::rgba(0.9, 0.95, 0.9, 1.0),
    )
    .with_stroke(novadraw::Color::rgba(0.2, 0.6, 0.2, 1.0), 4.0)
    .with_border(
        RectangleBorder::new(novadraw::Color::rgba(0.8, 0.2, 0.2, 1.0), 4.0)
            .with_insets(8.0, 8.0, 8.0, 8.0),
    );
    let both3 = novadraw::RectangleFigure::new_with_color(
        450.0,
        320.0,
        150.0,
        80.0,
        novadraw::Color::rgba(0.9, 0.95, 0.9, 1.0),
    )
    .with_stroke(novadraw::Color::rgba(0.2, 0.6, 0.2, 1.0), 6.0)
    .with_border(
        RectangleBorder::new(novadraw::Color::rgba(0.8, 0.2, 0.2, 1.0), 6.0)
            .with_insets(8.0, 8.0, 8.0, 8.0),
    );

    let _both1 = scene.add_child_to(container_id, Box::new(both1));
    let _both2 = scene.add_child_to(container_id, Box::new(both2));
    let _both3 = scene.add_child_to(container_id, Box::new(both3));

    scene
}

fn main() {
    // 创建场景列表
    let scenes: Vec<(&'static str, Box<dyn FnMut() -> novadraw::FigureGraph>)> = vec![
        ("Stroke基础", Box::new(|| create_scene_0_stroke_basic())),
        ("Stroke嵌套", Box::new(|| create_scene_1_stroke_nested())),
        (
            "Stroke同心",
            Box::new(|| create_scene_2_stroke_concentric()),
        ),
        ("Stroke颜色", Box::new(|| create_scene_3_stroke_colors())),
        (
            "RectangleBorder",
            Box::new(|| create_scene_4_rectangle_border()),
        ),
        (
            "Border+insets",
            Box::new(|| create_scene_5_border_with_insets()),
        ),
        ("LineBorder", Box::new(|| create_scene_6_line_border())),
        ("MarginBorder", Box::new(|| create_scene_7_margin_border())),
        (
            "Stroke vs Border",
            Box::new(|| create_scene_8_stroke_vs_border()),
        ),
    ];

    // 检查命令行参数
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 && args[1] == "--screenshot" {
        // 截图模式：截图所有场景
        run_demo_app_with_screenshot(
            "Border App - Stroke vs Border 验证",
            "border-app",
            scenes,
            true,
        )
        .expect("Failed to run app in screenshot mode");
    } else {
        // 正常模式
        run_demo_app(
            "Border App - Stroke vs Border 验证 (按方向键/鼠标滚轮切换)",
            "border-app",
            scenes,
        )
        .expect("Failed to run app");
    }
}
