use novadraw::{Color, RectangleFigure, SceneGraph};

pub struct SceneManager {
    pub scene: SceneGraph,
}

impl SceneManager {
    pub fn new() -> Self {
        let mut scene = SceneGraph::new();

        // ============================================
        // 验证 Trampoline 任务调度：四个角矩形
        // ============================================
        //
        // ┌────────────────────────────────────────┐
        // │  □ 左上           □ 右上              │
        // │  (0,0)           (700,0)              │
        // │                                        │
        // │  □ 左下           □ 右下              │
        // │  (0,550)         (700,550)            │
        // └────────────────────────────────────────┘
        //
        // 验证点：
        // 1. 渲染任务正确生成
        // 2. 四个角位置准确

        // 左上角 - 红色
        let rect_tl = RectangleFigure::new_with_color(
            0.0, 0.0, 100.0, 50.0,
            Color::rgba(0.9, 0.2, 0.2, 1.0),
        );
        let root_id = scene.set_contents(Box::new(rect_tl));
        println!("[SceneManager] 左上角 (0,0) 100×50 红色");

        // 右上角 - 绿色
        let rect_tr = RectangleFigure::new_with_color(
            700.0, 0.0, 100.0, 50.0,
            Color::rgba(0.2, 0.8, 0.3, 1.0),
        );
        let _id_tr = scene.add_child_to(root_id, Box::new(rect_tr));
        println!("[SceneManager] 右上角 (700,0) 100×50 绿色");

        // 左下角 - 蓝色
        let rect_bl = RectangleFigure::new_with_color(
            0.0, 550.0, 100.0, 50.0,
            Color::rgba(0.2, 0.4, 0.9, 1.0),
        );
        let _id_bl = scene.add_child_to(root_id, Box::new(rect_bl));
        println!("[SceneManager] 左下角 (0,550) 100×50 蓝色");

        // 右下角 - 黄色
        let rect_br = RectangleFigure::new_with_color(
            700.0, 550.0, 100.0, 50.0,
            Color::rgba(0.9, 0.8, 0.2, 1.0),
        );
        let _id_br = scene.add_child_to(root_id, Box::new(rect_br));
        println!("[SceneManager] 右下角 (700,550) 100×50 黄色");

        Self { scene }
    }

    pub fn scene(&self) -> &SceneGraph {
        &self.scene
    }
}

impl Default for SceneManager {
    fn default() -> Self {
        Self::new()
    }
}
