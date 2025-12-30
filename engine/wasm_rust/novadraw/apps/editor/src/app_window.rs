use std::sync::Arc;

use crate::scene_manager::SceneManager;
use glam::DVec2;
use novadraw::{BlockId, Color, VelloRenderer};
use winit::event::MouseButton;
use winit::event::ElementState;
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
    renderer: Option<VelloRenderer>,
    scene_manager: Option<SceneManager>,
    cached_window: Option<Arc<Window>>,
    drag_state: Option<DragState>,
    last_mouse_pos: Option<DVec2>,
}

struct DragState {
    block_id: BlockId,
    start_pos: DVec2,
    initial_block_pos: DVec2,
}

impl GraphicsApp {
    fn new() -> Self {
        GraphicsApp {
            renderer: None,
            scene_manager: None,
            cached_window: None,
            drag_state: None,
            last_mouse_pos: None,
        }
    }

    fn redraw(&mut self) {
        let Some(renderer) = &mut self.renderer else {
            return;
        };

        let Some(scene_manager) = &self.scene_manager else {
            return;
        };

        let render_ctx = scene_manager.scene().render();
        renderer.render(&render_ctx.commands);
    }
}

impl ApplicationHandler<()> for GraphicsApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.renderer.is_some() {
            return;
        }

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

        let renderer = VelloRenderer::new(Arc::clone(&window), width, height);
        self.renderer = Some(renderer);

        if self.scene_manager.is_none() {
            let mut manager = SceneManager::new();

            manager.add_rectangle(100.0, 100.0, 200.0, 150.0, Color::hex("#3498db"));
            manager.add_rectangle(400.0, 200.0, 150.0, 150.0, Color::hex("#3498db"));
            manager.add_rectangle_at_center(400.0, 300.0, 200.0, 100.0, Color::hex("#e74c3c"));

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
                println!("收到窗口关闭请求，退出程序");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                self.redraw();
            }
            WindowEvent::Resized(new_size) => {
                println!("窗口大小调整为: {}x{}", new_size.width, new_size.height);
                if let Some(renderer) = &mut self.renderer {
                    renderer.resize(new_size.width, new_size.height);
                }
            }
            WindowEvent::KeyboardInput { event, .. } => {
                if let PhysicalKey::Code(KeyCode::Escape) = event.physical_key {
                    println!("按下ESC键，退出程序");
                    event_loop.exit();
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                let Some(renderer) = &self.renderer else {
                    return;
                };
                let scale_factor = renderer.window().scale_factor();
                let logical_x = position.x / scale_factor;
                let logical_y = position.y / scale_factor;
                let current_pos = DVec2::new(logical_x, logical_y);

                self.last_mouse_pos = Some(current_pos);

                if let Some(scene_manager) = &mut self.scene_manager {
                    if let Some(drag_state) = &mut self.drag_state {
                        let dx = current_pos.x - drag_state.start_pos.x;
                        let dy = current_pos.y - drag_state.start_pos.y;
                        scene_manager.scene_mut().translate(drag_state.block_id, dx, dy);
                        drag_state.start_pos = current_pos;
                    } else {
                        let hit_id = scene_manager.scene().hit_test(current_pos);
                        scene_manager.scene_mut().set_hovered(hit_id);
                    }
                    renderer.window().request_redraw();
                }
            }
            WindowEvent::MouseInput { button: MouseButton::Left, state, .. } => {
                let Some(renderer) = &self.renderer else {
                    return;
                };
                if state == ElementState::Pressed {
                    if let Some(mouse_pos) = self.last_mouse_pos {
                        if let Some(scene_manager) = &mut self.scene_manager {
                            let hover_id = scene_manager.scene().hit_test(mouse_pos);
                            if let Some(id) = hover_id {
                                scene_manager.scene_mut().set_selected(Some(id));
                                if let Some(block) = scene_manager.scene().get_block(id) {
                                    let bounds = block.figure.bounds();
                                    self.drag_state = Some(DragState {
                                        block_id: id,
                                        start_pos: mouse_pos,
                                        initial_block_pos: DVec2::new(bounds.x, bounds.y),
                                    });
                                }
                            }
                        }
                    }
                } else {
                    self.drag_state = None;
                }
                renderer.window().request_redraw();
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) -> () {
        if let Some(renderer) = &self.renderer {
            renderer.window().request_redraw();
        }
    }

    fn suspended(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(renderer) = self.renderer.take() {
            self.cached_window = Some(renderer.window().clone());
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
