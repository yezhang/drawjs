use std::sync::Arc;

use crate::scene_manager::{SceneManager, scene_host::WinitSceneHost};
use novadraw::SceneHost;
use novadraw::backend::vello::{VelloRenderer, WinitWindowProxy};
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

pub struct GraphicsApp {
    renderer: Option<VelloRenderer>,
    scene_manager: Option<SceneManager>,
    scene_host: Option<WinitSceneHost>,
    cached_window: Option<Arc<Window>>,
}

impl GraphicsApp {
    fn new() -> Self {
        GraphicsApp {
            renderer: None,
            scene_manager: None,
            scene_host: None,
            cached_window: None,
        }
    }

    /// 执行一轮完整的两阶段更新（布局验证 + 脏区域重绘）
    ///
    /// 对应 draw2d: DeferredUpdateManager.performUpdate() → repairDamage()。
    /// SceneHost 负责调用顺序和渲染触发。
    fn execute_update(&mut self) {
        let Some(scene_host) = &self.scene_host else {
            return;
        };
        let Some(renderer) = &mut self.renderer else {
            return;
        };
        let Some(scene_manager) = &mut self.scene_manager else {
            return;
        };

        scene_host.execute_update(&mut scene_manager.scene, &mut *renderer);
    }

    /// 请求在下一次渲染帧执行更新
    fn request_update(&self) {
        if let Some(h) = &self.scene_host {
            h.request_update();
        }
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
                            .with_title("Novadraw - 渲染验证 (按 0-8 切换场景, I 切换渲染模式)")
                            .with_inner_size(dpi::LogicalSize::new(800, 600))
                            .with_resizable(true),
                    )
                    .unwrap(),
            )
        });

        let logical_width = 800.0;
        let logical_height = 600.0;

        let window_proxy = Arc::new(WinitWindowProxy::new(window));
        self.scene_host = Some(WinitSceneHost::new(Arc::clone(&window_proxy)));

        let renderer = VelloRenderer::new(Arc::clone(&window_proxy), logical_width, logical_height);
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
                self.execute_update();
            }
            WindowEvent::Resized(new_size) => {
                if let Some(renderer) = &mut self.renderer {
                    let scale_factor = renderer.window().scale_factor();
                    let PhysicalSize { width, height } = new_size;

                    renderer.resize(width, height, scale_factor);
                    renderer.recreate_surface(width, height);
                    self.request_update();
                }
            }
            WindowEvent::KeyboardInput { event, .. } => {
                if event.state != ElementState::Pressed {
                    return;
                }

                match event.physical_key {
                    PhysicalKey::Code(KeyCode::Digit0) => {
                        if let Some(manager) = &mut self.scene_manager {
                            manager.switch_scene(crate::scene_manager::SceneType::BasicAnchors);
                            self.request_update();
                        }
                    }
                    PhysicalKey::Code(KeyCode::Escape) => {
                        event_loop.exit();
                    }
                    PhysicalKey::Code(KeyCode::Digit1) => {
                        if let Some(manager) = &mut self.scene_manager {
                            manager.switch_scene(crate::scene_manager::SceneType::Nested);
                            self.request_update();
                        }
                    }
                    PhysicalKey::Code(KeyCode::Digit2) => {
                        if let Some(manager) = &mut self.scene_manager {
                            manager.switch_scene(crate::scene_manager::SceneType::NestedWithRoot);
                            self.request_update();
                        }
                    }
                    PhysicalKey::Code(KeyCode::Digit3) => {
                        if let Some(manager) = &mut self.scene_manager {
                            manager.switch_scene(crate::scene_manager::SceneType::ZOrder);
                            self.request_update();
                        }
                    }
                    PhysicalKey::Code(KeyCode::Digit4) => {
                        if let Some(manager) = &mut self.scene_manager {
                            manager.switch_scene(crate::scene_manager::SceneType::Visibility);
                            self.request_update();
                        }
                    }
                    PhysicalKey::Code(KeyCode::Digit5) => {
                        if let Some(manager) = &mut self.scene_manager {
                            manager.switch_scene(crate::scene_manager::SceneType::BoundsTranslate);
                            self.request_update();
                        }
                    }
                    PhysicalKey::Code(KeyCode::Digit6) => {
                        if let Some(manager) = &mut self.scene_manager {
                            manager.switch_scene(crate::scene_manager::SceneType::ClipTest);
                            self.request_update();
                        }
                    }
                    PhysicalKey::Code(KeyCode::Digit7) => {
                        if let Some(manager) = &mut self.scene_manager {
                            manager.switch_scene(crate::scene_manager::SceneType::EllipseTest);
                            self.request_update();
                        }
                    }
                    PhysicalKey::Code(KeyCode::Digit8) => {
                        if let Some(manager) = &mut self.scene_manager {
                            manager.switch_scene(crate::scene_manager::SceneType::LineTest);
                            self.request_update();
                        }
                    }
                    PhysicalKey::Code(KeyCode::KeyT) => {
                        if let Some(manager) = &mut self.scene_manager {
                            if manager.current_scene
                                == crate::scene_manager::SceneType::BoundsTranslate
                            {
                                if let Some(root_id) = manager.scene().get_contents() {
                                    manager.scene_mut().prim_translate(root_id, 10.0, 10.0);
                                    self.request_update();
                                }
                            }
                        }
                    }
                    PhysicalKey::Code(KeyCode::KeyI) => {
                        if let Some(host) = &mut self.scene_host {
                            let new_mode = !host.use_iterative_render();
                            host.set_use_iterative_render(new_mode);
                            println!(
                                "渲染模式: {}",
                                if new_mode {
                                    "迭代渲染"
                                } else {
                                    "递归渲染"
                                }
                            );
                            self.request_update();
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        self.request_update();
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
    println!("  按 0-8 切换场景：");
    println!(
        "    0=基础定位点 1=嵌套父子 2=嵌套(含根) 3=Z-order 4=不可见 5=平移验证 6=裁剪测试 7=椭圆测试 8=直线测试"
    );
    println!("  按 T 键：在场景 5 中平移父节点");
    println!("  按 I 键：切换递归/迭代渲染模式");
    println!("  按 ESC 退出");
    event_loop.run_app(&mut app)?;

    Ok(())
}
