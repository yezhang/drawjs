use std::sync::Arc;

use crate::scene_manager::SceneManager;
use novadraw::{Color, VelloRenderer, VelloSurface};
use winit::window::WindowAttributes;
use winit::{
    application::ApplicationHandler,
    dpi,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowId},
};

pub struct GraphicsApp {
    renderer: VelloRenderer,
    surface: Option<VelloSurface>,
    scene_manager: Option<SceneManager>,
    cached_window: Option<Arc<Window>>,
}

impl GraphicsApp {
    fn new() -> Self {
        GraphicsApp {
            renderer: VelloRenderer::new(),
            surface: None,
            scene_manager: None,
            cached_window: None,
        }
    }

    fn redraw(&mut self) {
        let Some(surface) = &mut self.surface else {
            return;
        };

        let Some(scene_manager) = &self.scene_manager else {
            return;
        };

        let render_ctx = scene_manager.scene().render();
        self.renderer.render(surface, &render_ctx.commands);
    }
}

impl ApplicationHandler<()> for GraphicsApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let Some(_) = self.surface else {
            let window_attributes = WindowAttributes::default()
                .with_title("Novadraw Editor - 场景图引擎演示")
                .with_inner_size(dpi::LogicalSize::new(800, 600))
                .with_resizable(true)
                .with_transparent(true);

            let window = self
                .cached_window
                .take()
                .unwrap_or_else(|| Arc::new(event_loop.create_window(window_attributes).unwrap()));

            let width = 800;
            let height = 600;

            let surface = self
                .renderer
                .create_surface(Arc::clone(&window), width, height);
            self.surface = Some(surface);

            if self.scene_manager.is_none() {
                let mut manager = SceneManager::new();

                manager.add_rectangle(100.0, 100.0, 200.0, 150.0, Color::hex("#3498db"));
                manager.add_rectangle(400.0, 200.0, 150.0, 150.0, Color::hex("#3498db"));
                manager.add_rectangle_at_center(400.0, 300.0, 200.0, 100.0, Color::hex("#e74c3c"));

                self.scene_manager = Some(manager);
            }
            return;
        };
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        let Some(_surface) = &mut self.surface else {
            return;
        };
        match event {
            WindowEvent::CloseRequested => {
                println!("收到窗口关闭请求，退出程序");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                self.redraw();
            }
            WindowEvent::Resized(new_size) => {
                println!("窗口大小调整为: {}x{}", new_size.width, new_size.height);
            }
            WindowEvent::KeyboardInput { event, .. } => {
                if let PhysicalKey::Code(KeyCode::Escape) = event.physical_key {
                    println!("按下ESC键，退出程序");
                    event_loop.exit();
                }
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) -> () {
        if let Some(surface) = &mut self.surface {
            surface.window.request_redraw();
        }
    }

    fn suspended(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(surface) = self.surface.take() {
            self.cached_window = Some(surface.window);
        }
    }
}

pub fn start_app() -> Result<(), Box<dyn std::error::Error>> {
    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Wait);

    let mut app = GraphicsApp::new();

    println!("启动事件循环... (按 ESC 退出)");
    event_loop.run_app(&mut app)?;

    Ok(())
}
