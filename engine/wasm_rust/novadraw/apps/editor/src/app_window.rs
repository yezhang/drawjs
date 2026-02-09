use std::sync::Arc;

use crate::scene_manager::SceneManager;
use novadraw::traits::{RenderBackend, WindowProxy};
use winit::dpi::{self, PhysicalSize};
use winit::event::ElementState;
use winit::window::WindowAttributes;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowId},
};

use novadraw::backend::vello::{VelloRenderer, WinitWindowProxy};

pub struct GraphicsApp {
    renderer: Option<VelloRenderer>,
    scene_manager: Option<SceneManager>,
    cached_window: Option<Arc<Window>>,
    /// 是否使用迭代渲染模式
    use_iterative_render: bool,
}

impl GraphicsApp {
    fn new() -> Self {
        GraphicsApp {
            renderer: None,
            scene_manager: None,
            cached_window: None,
            use_iterative_render: false,
        }
    }

    fn redraw(&mut self) {
        let Some(renderer) = &mut self.renderer else {
            return;
        };

        let Some(scene_manager) = &self.scene_manager else {
            return;
        };

        let render_ctx = if self.use_iterative_render {
            scene_manager.scene().render_iterative()
        } else {
            scene_manager.scene().render()
        };
        renderer.render(render_ctx.commands());
    }
}

impl ApplicationHandler<()> for GraphicsApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.renderer.is_some() {
            return;
        }

        let window = self.cached_window.take().unwrap_or_else(|| {
            Arc::new(
                event_loop
                    .create_window(
                        WindowAttributes::default()
                            .with_title("Novadraw - 渲染验证 (按 0-5 切换场景, I 切换渲染模式)")
                            .with_inner_size(dpi::LogicalSize::new(800, 600))
                            .with_resizable(true),
                    )
                    .unwrap(),
            )
        });

        let logical_width = 800.0;
        let logical_height = 600.0;

        let window_proxy = WinitWindowProxy::new(window);
        let renderer = VelloRenderer::new(Arc::new(window_proxy), logical_width, logical_height);
        self.renderer = Some(renderer);

        if self.scene_manager.is_none() {
            let manager = SceneManager::new();
            self.scene_manager = Some(manager);
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                self.redraw();
            }
            WindowEvent::Resized(new_size) => {
                if let Some(renderer) = &mut self.renderer {
                    let scale_factor = renderer.window().scale_factor();
                    let PhysicalSize { width, height } = new_size;

                    renderer.resize(width, height, scale_factor);
                    // 重新创建 surface 以确保配置正确更新，避免抖动
                    renderer.recreate_surface(width, height);
                    renderer.window().request_redraw();
                }
            }
            WindowEvent::KeyboardInput { event, .. } => {
                if event.state != ElementState::Pressed {
                    return;
                }

                match event.physical_key {
                    PhysicalKey::Code(KeyCode::Digit0) => {
                        // 切换到场景 0：基础四个定位点
                        if let Some(manager) = &mut self.scene_manager {
                            use crate::scene_manager::SceneType;
                            manager.switch_scene(SceneType::BasicAnchors);
                            self.redraw();
                        }
                    }
                    PhysicalKey::Code(KeyCode::Escape) => {
                        event_loop.exit();
                    }
                    PhysicalKey::Code(KeyCode::Digit1) => {
                        // 切换到场景 1：嵌套父子结构
                        if let Some(manager) = &mut self.scene_manager {
                            use crate::scene_manager::SceneType;
                            manager.switch_scene(SceneType::Nested);
                            self.redraw();
                        }
                    }
                    PhysicalKey::Code(KeyCode::Digit2) => {
                        // 切换到场景 2：嵌套场景（含透明根节点）
                        if let Some(manager) = &mut self.scene_manager {
                            use crate::scene_manager::SceneType;
                            manager.switch_scene(SceneType::NestedWithRoot);
                            self.redraw();
                        }
                    }
                    PhysicalKey::Code(KeyCode::Digit3) => {
                        // 切换到场景 3：Z-order
                        if let Some(manager) = &mut self.scene_manager {
                            use crate::scene_manager::SceneType;
                            manager.switch_scene(SceneType::ZOrder);
                            self.redraw();
                        }
                    }
                    PhysicalKey::Code(KeyCode::Digit4) => {
                        // 切换到场景 4：不可见过滤
                        if let Some(manager) = &mut self.scene_manager {
                            use crate::scene_manager::SceneType;
                            manager.switch_scene(SceneType::Visibility);
                            self.redraw();
                        }
                    }
                    PhysicalKey::Code(KeyCode::Digit5) => {
                        // 切换到场景 5：BoundsTranslate 平移传播
                        if let Some(manager) = &mut self.scene_manager {
                            use crate::scene_manager::SceneType;
                            manager.switch_scene(SceneType::BoundsTranslate);
                            self.redraw();
                        }
                    }
                    PhysicalKey::Code(KeyCode::KeyT) => {
                        // 按 T 键：在 BoundsTranslate 场景中平移父节点
                        if let Some(manager) = &mut self.scene_manager {
                            use crate::scene_manager::SceneType;
                            if manager.current_scene == SceneType::BoundsTranslate {
                                if let Some(root_id) = manager.scene().get_contents() {
                                    manager.scene_mut().prim_translate(root_id, 10.0, 10.0);
                                    self.redraw();
                                }
                            }
                        }
                    }
                    PhysicalKey::Code(KeyCode::KeyI) => {
                        // 按 I 键：切换递归/迭代渲染模式
                        self.use_iterative_render = !self.use_iterative_render;
                        println!(
                            "渲染模式: {}",
                            if self.use_iterative_render {
                                "迭代渲染"
                            } else {
                                "递归渲染"
                            }
                        );
                        self.redraw();
                    }
                    _ => {}
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
            self.cached_window = Some(renderer.window().clone_window());
        }
    }
}

pub fn start_app() -> Result<(), Box<dyn std::error::Error>> {
    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Wait);

    let mut app = GraphicsApp::new();

    println!("启动事件循环...");
    println!("  按 0-5 切换场景：");
    println!("    0=基础定位点 1=嵌套父子 2=嵌套(含根) 3=Z-order 4=不可见 5=平移验证");
    println!("  按 T 键：在场景 5 中平移父节点");
    println!("  按 I 键：切换递归/迭代渲染模式");
    println!("  按 ESC 退出");
    event_loop.run_app(&mut app)?;

    Ok(())
}
