use novadraw::{Color, RectangleFigure, SceneGraph, BlockId};

pub struct SceneManager {
    pub scene: SceneGraph,
    /// 当前激活的场景类型
    pub current_scene: SceneType,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SceneType {
    Nested,      // 场景1：嵌套父子结构
    ZOrder,      // 场景2：Z-order 叠加
    Visibility,  // 场景3：不可见节点过滤
    Selection,   // 场景4：选中高亮测试
}

impl SceneManager {
    /// 创建默认场景（场景4：选中高亮测试）
    pub fn new() -> Self {
        Self::with_scene(SceneType::Selection)
    }

    /// 根据场景类型创建场景
    pub fn with_scene(scene_type: SceneType) -> Self {
        let mut scene = SceneGraph::new();

        match scene_type {
            SceneType::Selection => Self::create_selection_scene(&mut scene),
            SceneType::Visibility => Self::create_visibility_scene(&mut scene),
            SceneType::ZOrder => Self::create_zorder_scene(&mut scene),
            SceneType::Nested => Self::create_nested_scene(&mut scene),
        }

        Self { scene, current_scene: scene_type }
    }

    /// 场景 4：选中高亮测试
    ///
    /// 验证：`paint_highlight` 在 `PaintBorder` 之后执行
    fn create_selection_scene(scene: &mut SceneGraph) {
        // Normal - 未选中节点 (灰色)
        let rect_normal = RectangleFigure::new_with_color(
            50.0, 50.0, 150.0, 60.0,
            Color::rgba(0.5, 0.5, 0.5, 1.0),
        );
        let root_id = scene.set_contents(Box::new(rect_normal));

        // Selected - 选中节点 (紫色)
        let rect_selected = RectangleFigure::new_with_color(
            50.0, 150.0, 150.0, 60.0,
            Color::rgba(0.6, 0.3, 0.8, 1.0),
        );
        let id_selected = scene.add_child_to(root_id, Box::new(rect_selected));

        // 设置选中状态
        scene.blocks.get_mut(id_selected).unwrap().is_selected = true;

        // 保留：四个角矩形
        Self::add_corner_rects(scene, root_id);
    }

    /// 场景 3：不可见节点过滤测试
    ///
    /// 验证：`is_visible = false` 的节点不产生任何渲染命令
    fn create_visibility_scene(scene: &mut SceneGraph) {
        // Visible A - 红色 (contents)
        let rect_a = RectangleFigure::new_with_color(
            50.0, 50.0, 150.0, 60.0,
            Color::rgba(0.9, 0.2, 0.2, 1.0),
        );
        let root_id = scene.set_contents(Box::new(rect_a));

        // Hidden B - 蓝色 (不可见)
        let rect_b = RectangleFigure::new_with_color(
            50.0, 150.0, 150.0, 60.0,
            Color::rgba(0.2, 0.4, 0.9, 1.0),
        );
        let id_b = scene.add_child_to(root_id, Box::new(rect_b));

        // 设置不可见
        scene.blocks.get_mut(id_b).unwrap().is_visible = false;

        // Visible C - 绿色
        let rect_c = RectangleFigure::new_with_color(
            50.0, 250.0, 150.0, 60.0,
            Color::rgba(0.2, 0.8, 0.3, 1.0),
        );
        scene.add_child_to(root_id, Box::new(rect_c));

        // 保留：四个角矩形
        Self::add_corner_rects(scene, root_id);
    }

    /// 场景 2：Z-order 叠加测试
    ///
    /// 验证：后添加的节点视觉上在上层（遮挡先添加的）
    fn create_zorder_scene(scene: &mut SceneGraph) {
        // Z-Order Parent - 灰色容器
        let z_parent = RectangleFigure::new_with_color(
            50.0, 50.0, 200.0, 200.0,
            Color::rgba(0.3, 0.3, 0.3, 1.0),
        );
        let z_parent_id = scene.set_contents(Box::new(z_parent));

        // A - 先添加，红色
        let rect_a = RectangleFigure::new_with_color(
            0.0, 0.0, 100.0, 100.0,
            Color::rgba(0.9, 0.2, 0.2, 1.0),
        );
        scene.add_child_to(z_parent_id, Box::new(rect_a));

        // B - 后添加，蓝色，会覆盖 A 的右下角
        let rect_b = RectangleFigure::new_with_color(
            50.0, 50.0, 100.0, 100.0,
            Color::rgba(0.2, 0.4, 0.9, 1.0),
        );
        scene.add_child_to(z_parent_id, Box::new(rect_b));

        // 保留：四个角矩形
        Self::add_corner_rects(scene, z_parent_id);
    }

    /// 场景 1：嵌套父子结构测试
    ///
    /// 验证：`parent.PaintBorder` 在所有子节点完成后执行
    fn create_nested_scene(scene: &mut SceneGraph) {
        // Parent - 深紫容器
        let parent = RectangleFigure::new_with_color(
            150.0, 50.0, 250.0, 200.0,
            Color::rgba(0.4, 0.2, 0.5, 1.0),
        );
        let parent_id = scene.set_contents(Box::new(parent));

        // Child - 橙色
        let child = RectangleFigure::new_with_color(
            30.0, 30.0, 150.0, 100.0,
            Color::rgba(0.9, 0.5, 0.1, 1.0),
        );
        let child_id = scene.add_child_to(parent_id, Box::new(child));

        // Grandchild - 青色（选中）
        let grandchild = RectangleFigure::new_with_color(
            20.0, 20.0, 80.0, 40.0,
            Color::rgba(0.1, 0.8, 0.8, 1.0),
        );
        let gc_id = scene.add_child_to(child_id, Box::new(grandchild));

        // 选中 Grandchild
        scene.blocks.get_mut(gc_id).unwrap().is_selected = true;

        // 保留：四个角矩形
        Self::add_corner_rects(scene, parent_id);
    }

    /// 添加四个角测试矩形（所有场景共用）
    fn add_corner_rects(scene: &mut SceneGraph, parent_id: BlockId) {
        // 左上角 - 红色
        let rect_tl = RectangleFigure::new_with_color(
            0.0, 0.0, 100.0, 50.0,
            Color::rgba(0.9, 0.2, 0.2, 1.0),
        );
        scene.add_child_to(parent_id, Box::new(rect_tl));

        // 右上角 - 绿色
        let rect_tr = RectangleFigure::new_with_color(
            700.0, 0.0, 100.0, 50.0,
            Color::rgba(0.2, 0.8, 0.3, 1.0),
        );
        scene.add_child_to(parent_id, Box::new(rect_tr));

        // 左下角 - 蓝色
        let rect_bl = RectangleFigure::new_with_color(
            0.0, 550.0, 100.0, 50.0,
            Color::rgba(0.2, 0.4, 0.9, 1.0),
        );
        scene.add_child_to(parent_id, Box::new(rect_bl));

        // 右下角 - 黄色
        let rect_br = RectangleFigure::new_with_color(
            700.0, 550.0, 100.0, 50.0,
            Color::rgba(0.9, 0.8, 0.2, 1.0),
        );
        scene.add_child_to(parent_id, Box::new(rect_br));
    }

    /// 切换场景
    pub fn switch_scene(&mut self, scene_type: SceneType) {
        self.scene = SceneGraph::new();
        match scene_type {
            SceneType::Selection => Self::create_selection_scene(&mut self.scene),
            SceneType::Visibility => Self::create_visibility_scene(&mut self.scene),
            SceneType::ZOrder => Self::create_zorder_scene(&mut self.scene),
            SceneType::Nested => Self::create_nested_scene(&mut self.scene),
        }
        self.current_scene = scene_type;
        println!("[SceneManager] 切换到场景 {:?}", scene_type);
    }

    /// 获取场景图
    pub fn scene(&self) -> &SceneGraph {
        &self.scene
    }

    /// 获取场景图可变引用
    pub fn scene_mut(&mut self) -> &mut SceneGraph {
        &mut self.scene
    }
}

impl Default for SceneManager {
    fn default() -> Self {
        Self::new()
    }
}
