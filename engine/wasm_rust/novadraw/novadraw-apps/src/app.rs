//! 应用框架
//!
//! 提供通用的演示应用构建和运行功能。

use std::sync::Arc;

pub use novadraw::{RenderBackend, SceneGraph, WindowProxy};
pub use novadraw_render::backend::vello::{VelloRenderer, WinitWindowProxy};
pub use winit::dpi::{LogicalSize, PhysicalSize};
pub use winit::event::WindowEvent;
pub use winit::event_loop::{ActiveEventLoop, EventLoop};
pub use winit::keyboard::{KeyCode, PhysicalKey};
pub use winit::window::WindowAttributes;
pub use winit::{application::ApplicationHandler, window::WindowId};

/// 演示应用
///
/// 提供通用的演示应用框架，自动处理：
/// - 窗口创建
/// - 渲染器初始化
/// - 场景切换
/// - 事件处理
pub struct DemoApp {
    /// 场景名称列表
    scenes: Vec<(&'static str, Box<dyn FnMut() -> SceneGraph>)>,
    /// 当前场景索引
    current_scene_idx: usize,
    /// 场景图
    scene_graph: Option<SceneGraph>,
    /// 渲染器
    renderer: Option<VelloRenderer>,
    /// 窗口
    window: Option<Arc<winit::window::Window>>,
    /// 窗口标题
    title: String,
    /// 窗口宽度
    width: f64,
    /// 窗口高度
    height: f64,
    /// 渲染模式
    use_iterative_render: bool,
}

impl DemoApp {
    /// 创建一个新的演示应用
    pub fn new(
        title: &str,
        scenes: Vec<(&'static str, Box<dyn FnMut() -> SceneGraph>)>,
        width: f64,
        height: f64,
    ) -> Self {
        Self {
            scenes,
            current_scene_idx: 0,
            scene_graph: None,
            renderer: None,
            window: None,
            title: title.to_string(),
            width,
            height,
            use_iterative_render: false,
        }
    }

    /// 切换到指定场景
    pub fn switch_scene(&mut self, idx: usize) {
        if idx < self.scenes.len() {
            self.current_scene_idx = idx;
            let creator = &mut self.scenes[idx].1;
            self.scene_graph = Some(creator());
            log::info!("切换到场景: {}", self.scenes[idx].0);
        }
    }

    /// 获取当前场景名称
    pub fn current_scene_name(&self) -> Option<&'static str> {
        self.scenes.get(self.current_scene_idx).map(|(name, _)| *name)
    }

    /// 获取场景数量
    pub fn scene_count(&self) -> usize {
        self.scenes.len()
    }

    /// 渲染当前场景
    pub fn render(&mut self) {
        let Some(renderer) = &mut self.renderer else {
            return;
        };
        let Some(scene) = &self.scene_graph else {
            return;
        };

        let render_ctx = if self.use_iterative_render {
            scene.render_iterative()
        } else {
            scene.render()
        };
        renderer.render(render_ctx.commands());
    }
}

impl ApplicationHandler<()> for DemoApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.renderer.is_some() {
            return;
        }

        let window: Arc<winit::window::Window> = Arc::new(
            event_loop
                .create_window(
                    WindowAttributes::default()
                        .with_title(&self.title)
                        .with_inner_size(LogicalSize::new(self.width, self.height))
                        .with_resizable(true),
                )
                .unwrap(),
        );

        self.window = Some(window.clone());

        let window_proxy = Arc::new(WinitWindowProxy::new(window));
        let renderer = VelloRenderer::new(window_proxy, self.width, self.height);
        self.renderer = Some(renderer);

        // 创建初始场景
        if !self.scenes.is_empty() {
            let creator = &mut self.scenes[0].1;
            self.scene_graph = Some(creator());
        }
        log::info!("应用启动: {}", self.title);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                self.render();
            }
            WindowEvent::Resized(new_size) => {
                if let Some(renderer) = &mut self.renderer {
                    let scale_factor = renderer.window().scale_factor();
                    let PhysicalSize { width, height } = new_size;
                    renderer.resize(width, height, scale_factor);
                    renderer.recreate_surface(width, height);
                    renderer.window().request_redraw();
                }
            }
            WindowEvent::KeyboardInput { event, .. } => {
                if event.state != winit::event::ElementState::Pressed {
                    return;
                }

                match event.physical_key {
                    PhysicalKey::Code(KeyCode::Escape) => {
                        event_loop.exit();
                    }
                    PhysicalKey::Code(KeyCode::KeyI) => {
                        // 切换渲染模式
                        self.use_iterative_render = !self.use_iterative_render;
                        log::info!(
                            "渲染模式: {}",
                            if self.use_iterative_render {
                                "迭代渲染"
                            } else {
                                "递归渲染"
                            }
                        );
                        self.window.as_ref().unwrap().request_redraw();
                    }
                    // 数字键 0-9 切换场景
                    _ => {
                        if let Some(digit) = get_digit_index(&event.physical_key) {
                            self.switch_scene(digit);
                            self.window.as_ref().unwrap().request_redraw();
                        }
                    }
                }
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(renderer) = &self.renderer {
            renderer.window().request_redraw();
        }
    }

    fn suspended(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(renderer) = self.renderer.take() {
            self.window = Some(renderer.window().clone_window());
        }
    }
}

/// 从按键获取数字索引（0-9）
fn get_digit_index(key: &winit::keyboard::PhysicalKey) -> Option<usize> {
    match key {
        winit::keyboard::PhysicalKey::Code(KeyCode::Digit0) => Some(0),
        winit::keyboard::PhysicalKey::Code(KeyCode::Digit1) => Some(1),
        winit::keyboard::PhysicalKey::Code(KeyCode::Digit2) => Some(2),
        winit::keyboard::PhysicalKey::Code(KeyCode::Digit3) => Some(3),
        winit::keyboard::PhysicalKey::Code(KeyCode::Digit4) => Some(4),
        winit::keyboard::PhysicalKey::Code(KeyCode::Digit5) => Some(5),
        winit::keyboard::PhysicalKey::Code(KeyCode::Digit6) => Some(6),
        winit::keyboard::PhysicalKey::Code(KeyCode::Digit7) => Some(7),
        winit::keyboard::PhysicalKey::Code(KeyCode::Digit8) => Some(8),
        winit::keyboard::PhysicalKey::Code(KeyCode::Digit9) => Some(9),
        _ => None,
    }
}

/// 应用构建器
///
/// 用于更灵活地配置应用
pub struct AppBuilder {
    title: String,
    scenes: Vec<(&'static str, Box<dyn FnMut() -> SceneGraph>)>,
    width: f64,
    height: f64,
}

impl AppBuilder {
    /// 创建一个新的构建器
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
            scenes: Vec::new(),
            width: 800.0,
            height: 600.0,
        }
    }

    /// 设置窗口尺寸
    pub fn with_size(mut self, width: f64, height: f64) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    /// 添加场景
    pub fn add_scene(mut self, name: &'static str, creator: impl FnMut() -> SceneGraph + 'static) -> Self {
        self.scenes.push((name, Box::new(creator)));
        self
    }

    /// 批量添加场景（已装箱）
    pub fn with_scenes_boxed(mut self, scenes: Vec<(&'static str, Box<dyn FnMut() -> SceneGraph>)>) -> Self {
        self.scenes = scenes;
        self
    }

    /// 构建并运行应用
    pub fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        let event_loop = EventLoop::new()?;
        let scenes: Vec<(_, Box<dyn FnMut() -> SceneGraph>)> = self.scenes.into_iter().map(|(n, c)| (n, c)).collect();
        let mut app = DemoApp::new(&self.title, scenes, self.width, self.height);
        event_loop.run_app(&mut app)?;
        Ok(())
    }
}

/// 运行演示应用的简便函数
///
/// # 示例
///
/// ```rust
/// use novadraw_apps::run_demo_app;
///
/// fn create_rect_scene() -> novadraw::SceneGraph {
///     let mut scene = novadraw::SceneGraph::new();
///     let rect = novadraw::RectangleFigure::new(100.0, 100.0, 200.0, 150.0);
///     scene.set_contents(Box::new(rect));
///     scene
/// }
///
/// fn main() {
///     run_demo_app("My App", vec![
///         ("Rectangle", Box::new(|| create_rect_scene())),
///     ]);
/// }
/// ```
pub fn run_demo_app(
    title: &str,
    scenes: Vec<(&'static str, Box<dyn FnMut() -> SceneGraph>)>,
) -> Result<(), Box<dyn std::error::Error>> {
    AppBuilder::new(title)
        .with_size(800.0, 600.0)
        .with_scenes_boxed(scenes)
        .run()
}
