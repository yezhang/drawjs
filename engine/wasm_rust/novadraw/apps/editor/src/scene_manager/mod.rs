use novadraw::{Color, RectangleFigure, SceneGraph};

pub struct SceneManager {
    pub scene: SceneGraph,
    /// 当前激活的场景类型
    pub current_scene: SceneType,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SceneType {
    BasicAnchors,      // 场景0：基础四个定位点
    Nested,            // 场景1：嵌套父子结构
    NestedWithRoot,    // 场景2：嵌套场景（含透明根节点）
    ZOrder,            // 场景3：Z-order 叠加
    Visibility,        // 场景4：不可见节点过滤
    BoundsTranslate,   // 场景5：prim_translate 平移传播
}

impl SceneManager {
    /// 创建默认场景（场景2：嵌套场景，含透明根节点和相对坐标）
    pub fn new() -> Self {
        Self::with_scene(SceneType::NestedWithRoot)
    }

    /// 根据场景类型创建场景
    pub fn with_scene(scene_type: SceneType) -> Self {
        let mut scene = SceneGraph::new();

        match scene_type {
            SceneType::BasicAnchors => Self::create_basic_anchors_scene(&mut scene),
            SceneType::Nested => Self::create_nested_scene(&mut scene),
            SceneType::NestedWithRoot => Self::create_nested_with_root_scene(&mut scene),
            SceneType::ZOrder => Self::create_zorder_scene(&mut scene),
            SceneType::Visibility => Self::create_visibility_scene(&mut scene),
            SceneType::BoundsTranslate => Self::create_bounds_translate_scene(&mut scene),
        }

        Self {
            scene,
            current_scene: scene_type,
        }
    }

    /// 场景 0：基础四个定位点（只有四个小正方形）
    ///
    /// 用于验证最基本的渲染逻辑
    fn create_basic_anchors_scene(scene: &mut SceneGraph) {
        // 创建一个基准矩形作为父容器（灰色边框）
        let root_fig = RectangleFigure::new_with_color(
            100.0,
            100.0,
            600.0,
            400.0,
            Color::rgba(0.3, 0.3, 0.3, 1.0),
        );
        let root_bounds = root_fig.bounds;
        let parent_id = scene.set_contents(Box::new(root_fig));

        // 角点手柄大小
        let handle_size = 20.0;

        // 左上角 - 红色小正方形
        let rect_tl = RectangleFigure::new_with_color(
            root_bounds.x,
            root_bounds.y,
            handle_size,
            handle_size,
            Color::rgba(0.9, 0.2, 0.2, 1.0),
        );
        scene.add_child_to(parent_id, Box::new(rect_tl));

        // 右上角 - 绿色小正方形
        let rect_tr = RectangleFigure::new_with_color(
            root_bounds.x + root_bounds.width - handle_size,
            root_bounds.y,
            handle_size,
            handle_size,
            Color::rgba(0.2, 0.8, 0.3, 1.0),
        );
        scene.add_child_to(parent_id, Box::new(rect_tr));

        // 左下角 - 蓝色小正方形
        let rect_bl = RectangleFigure::new_with_color(
            root_bounds.x,
            root_bounds.y + root_bounds.height - handle_size,
            handle_size,
            handle_size,
            Color::rgba(0.2, 0.4, 0.9, 1.0),
        );
        scene.add_child_to(parent_id, Box::new(rect_bl));

        // 右下角 - 黄色小正方形
        let rect_br = RectangleFigure::new_with_color(
            root_bounds.x + root_bounds.width - handle_size,
            root_bounds.y + root_bounds.height - handle_size,
            handle_size,
            handle_size,
            Color::rgba(0.9, 0.8, 0.2, 1.0),
        );
        scene.add_child_to(parent_id, Box::new(rect_br));
    }

    /// 场景 4：prim_translate 平移传播测试
    ///
    /// 验证：`prim_translate` 平移操作会传播到所有子节点
    fn create_bounds_translate_scene(scene: &mut SceneGraph) {
        // Parent - 深紫容器
        let parent = RectangleFigure::new_with_color(
            200.0,
            150.0,
            300.0,
            200.0,
            Color::rgba(0.4, 0.2, 0.5, 1.0),
        );
        let parent_id = scene.set_contents(Box::new(parent));

        // Child - 橙色（相对于父节点）
        let child = RectangleFigure::new_with_color(
            30.0,
            30.0,
            100.0,
            80.0,
            Color::rgba(0.9, 0.5, 0.1, 1.0),
        );
        let child_id = scene.add_child_to(parent_id, Box::new(child));

        // Grandchild - 青色（选中状态）
        let grandchild = RectangleFigure::new_with_color(
            20.0,
            20.0,
            40.0,
            30.0,
            Color::rgba(0.1, 0.8, 0.8, 1.0),
        );
        let gc_id = scene.add_child_to(child_id, Box::new(grandchild));

        // 选中 Grandchild
        scene.blocks.get_mut(gc_id).unwrap().is_selected = true;
    }

    /// 场景 3：不可见节点过滤测试
    ///
    /// 验证：`is_visible = false` 的节点不产生任何渲染命令
    fn create_visibility_scene(scene: &mut SceneGraph) {
        // Visible A - 红色 (contents)
        let rect_a = RectangleFigure::new_with_color(
            50.0,
            50.0,
            150.0,
            60.0,
            Color::rgba(0.9, 0.2, 0.2, 1.0),
        );
        let root_id = scene.set_contents(Box::new(rect_a));

        // Hidden B - 蓝色 (不可见)
        let rect_b = RectangleFigure::new_with_color(
            50.0,
            150.0,
            150.0,
            60.0,
            Color::rgba(0.2, 0.4, 0.9, 1.0),
        );
        let id_b = scene.add_child_to(root_id, Box::new(rect_b));

        // 设置不可见
        scene.blocks.get_mut(id_b).unwrap().is_visible = false;

        // Visible C - 绿色
        let rect_c = RectangleFigure::new_with_color(
            50.0,
            250.0,
            150.0,
            60.0,
            Color::rgba(0.2, 0.8, 0.3, 1.0),
        );
        scene.add_child_to(root_id, Box::new(rect_c));
    }

    /// 场景 2：Z-order 叠加测试
    ///
    /// 验证：后添加的节点视觉上在上层（遮挡先添加的）
    fn create_zorder_scene(scene: &mut SceneGraph) {
        // Z-Order Parent - 灰色容器
        let z_parent = RectangleFigure::new_with_color(
            50.0,
            50.0,
            200.0,
            200.0,
            Color::rgba(0.3, 0.3, 0.3, 1.0),
        );
        let z_parent_id = scene.set_contents(Box::new(z_parent));

        // A - 先添加，红色
        let rect_a = RectangleFigure::new_with_color(
            0.0,
            0.0,
            100.0,
            100.0,
            Color::rgba(0.9, 0.2, 0.2, 1.0),
        );
        scene.add_child_to(z_parent_id, Box::new(rect_a));

        // B - 后添加，蓝色，会覆盖 A 的右下角
        let rect_b = RectangleFigure::new_with_color(
            50.0,
            50.0,
            100.0,
            100.0,
            Color::rgba(0.2, 0.4, 0.9, 1.0),
        );
        scene.add_child_to(z_parent_id, Box::new(rect_b));
    }

    /// 场景 1：嵌套父子结构测试
    ///
    /// 验证：`parent.PaintBorder` 在所有子节点完成后执行
    fn create_nested_scene(scene: &mut SceneGraph) {
        // Parent - 深紫容器
        let parent = RectangleFigure::new_with_color(
            150.0,
            50.0,
            250.0,
            200.0,
            Color::rgba(0.4, 0.2, 0.5, 1.0),
        );
        let parent_id = scene.set_contents(Box::new(parent));

        // Child - 橙色
        let child = RectangleFigure::new_with_color(
            30.0,
            30.0,
            150.0,
            100.0,
            Color::rgba(0.9, 0.5, 0.1, 1.0),
        );
        let child_id = scene.add_child_to(parent_id, Box::new(child));

        // Grandchild - 青色（选中）
        let grandchild = RectangleFigure::new_with_color(
            20.0,
            20.0,
            80.0,
            40.0,
            Color::rgba(0.1, 0.8, 0.8, 1.0),
        );
        let gc_id = scene.add_child_to(child_id, Box::new(grandchild));

        // 选中 Grandchild
        scene.blocks.get_mut(gc_id).unwrap().is_selected = true;
    }

    /// 场景 2：嵌套场景（含透明根节点）
    ///
    /// contents 根节点下包含一个相对坐标模式的子树，
    /// 验证局部坐标模式下子元素坐标的累积效果。
    /// 注意：与场景 1 保持相同的矩形尺寸，方便比较。
    fn create_nested_with_root_scene(scene: &mut SceneGraph) {
        // 创建透明背景作为根容器
        let root = RectangleFigure::new_with_color(
            0.0, 0.0, 800.0, 600.0,
            Color::rgba(0.0, 0.0, 0.0, 0.0),
        );
        let root_id = scene.set_contents(Box::new(root));

        // Parent - 深紫色（与场景 1 相同尺寸：250x200）
        let parent = RectangleFigure::new_with_color(
            350.0, 50.0, 250.0, 200.0,
            Color::rgba(0.4, 0.2, 0.5, 1.0),
        )
        .with_local_coordinates(true);
        let parent_id = scene.add_child_to(root_id, Box::new(parent));

        // Child - 橙色（与场景 1 相同尺寸：150x100）
        let child = RectangleFigure::new_with_color(
            30.0, 30.0, 150.0, 100.0,
            Color::rgba(0.9, 0.5, 0.1, 1.0),
        )
        .with_local_coordinates(true);
        let child_id = scene.add_child_to(parent_id, Box::new(child));

        // Grandchild - 青色（选中状态，与场景 1 相同尺寸：80x40）
        // 实际位置 = parent(350,50) + child(30,30) + gc(20,20) = (400, 100)
        let gc = RectangleFigure::new_with_color(
            20.0, 20.0, 80.0, 40.0,
            Color::rgba(0.1, 0.8, 0.8, 1.0),
        )
        .with_local_coordinates(true);
        let gc_id = scene.add_child_to(child_id, Box::new(gc));
        scene.blocks.get_mut(gc_id).unwrap().is_selected = true;
    }

    /// 切换场景
    pub fn switch_scene(&mut self, scene_type: SceneType) {
        self.scene = SceneGraph::new();
        match scene_type {
            SceneType::BasicAnchors => Self::create_basic_anchors_scene(&mut self.scene),
            SceneType::Nested => Self::create_nested_scene(&mut self.scene),
            SceneType::NestedWithRoot => Self::create_nested_with_root_scene(&mut self.scene),
            SceneType::ZOrder => Self::create_zorder_scene(&mut self.scene),
            SceneType::Visibility => Self::create_visibility_scene(&mut self.scene),
            SceneType::BoundsTranslate => Self::create_bounds_translate_scene(&mut self.scene),
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
