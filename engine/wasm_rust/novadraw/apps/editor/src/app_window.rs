use std::sync::Arc;

use crate::scene_manager::SceneManager;
use glam::DVec2;
use novadraw::Color;
use novadraw::traits::{Renderer, WindowProxy};
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

use novadraw::backend::vello::{VelloRenderer, WinitWindowProxy};

pub struct GraphicsApp {
    renderer: Option<VelloRenderer>,
    scene_manager: Option<SceneManager>,
    cached_window: Option<Arc<Window>>,
    viewport_drag_state: Option<ViewportDragState>,
    last_mouse_pos: Option<DVec2>,
}

struct ViewportDragState {
    start_pos: DVec2,
    start_origin: DVec2,
}

impl GraphicsApp {
    fn new() -> Self {
        GraphicsApp {
            renderer: None,
            scene_manager: None,
            cached_window: None,
            viewport_drag_state: None,
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

        let viewport = scene_manager.viewport();
        let viewport_transform = viewport.to_transform();

        let render_ctx = scene_manager.scene().render_with_viewport(viewport_transform);

        renderer.render(&render_ctx.commands);
    }
}

impl ApplicationHandler<()> for GraphicsApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.renderer.is_some() {
            return;
        }

        let window = self
            .cached_window
            .take()
            .unwrap_or_else(|| Arc::new(event_loop.create_window(
                WindowAttributes::default()
                    .with_title("Novadraw - Draw2D 等价验证")
                    .with_inner_size(dpi::LogicalSize::new(800, 600))
                    .with_resizable(true),
            ).unwrap()));

        let logical_width = 800.0;
        let logical_height = 600.0;

        let window_proxy = WinitWindowProxy::new(window);
        let renderer = VelloRenderer::new(Arc::new(window_proxy), logical_width, logical_height);
        self.renderer = Some(renderer);

        if self.scene_manager.is_none() {
            let mut manager = SceneManager::new();
            manager.add_rectangle(100.0, 100.0, 200.0, 150.0, Color::hex("#3498db"));
            manager.add_rectangle(400.0, 200.0, 150.0, 150.0, Color::hex("#e74c3c"));
            manager.add_rectangle_at_center(400.0, 400.0, 200.0, 100.0, Color::hex("#2ecc71"));
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
                println!("[Resize] pixel=({},{})", new_size.width, new_size.height);
                if let Some(renderer) = &mut self.renderer {
                    let scale_factor = renderer.window().scale_factor();
                    let logical_width = new_size.width as f64 / scale_factor;
                    let logical_height = new_size.height as f64 / scale_factor;
                    println!("[Resize] logical=({:.2},{:.2}), scale={:.2}", logical_width, logical_height, scale_factor);

                    if let Some(scene_manager) = &mut self.scene_manager {
                        let (before_w, before_h) = scene_manager.debug_background_size();
                        println!("[Resize] before bg: ({:.2},{:.2})", before_w, before_h);

                        scene_manager.set_window_size(logical_width, logical_height);

                        let (after_w, after_h) = scene_manager.debug_background_size();
                        println!("[Resize] after bg: ({:.2},{:.2})", after_w, after_h);
                    }

                    renderer.resize(logical_width as u32, logical_height as u32);
                    println!("[Resize] done");
                }
            }
            WindowEvent::KeyboardInput { event, .. } => {
                if event.state != ElementState::Pressed {
                    return;
                }

                let Some(renderer) = &self.renderer else {
                    return;
                };
                let Some(scene_manager) = &mut self.scene_manager else {
                    return;
                };

                match event.physical_key {
                    PhysicalKey::Code(KeyCode::Escape) => {
                        event_loop.exit();
                    }
                    PhysicalKey::Code(KeyCode::Equal) | PhysicalKey::Code(KeyCode::NumpadAdd) => {
                        scene_manager.viewport_mut().zoom = (scene_manager.viewport().zoom * 1.1).max(0.1);
                        renderer.window().request_redraw();
                    }
                    PhysicalKey::Code(KeyCode::Minus) | PhysicalKey::Code(KeyCode::NumpadSubtract) => {
                        scene_manager.viewport_mut().zoom = (scene_manager.viewport().zoom * 0.9).max(0.1);
                        renderer.window().request_redraw();
                    }
                    PhysicalKey::Code(KeyCode::KeyW) | PhysicalKey::Code(KeyCode::ArrowUp) => {
                        scene_manager.viewport_mut().origin.y -= 50.0 / scene_manager.viewport().zoom;
                        renderer.window().request_redraw();
                    }
                    PhysicalKey::Code(KeyCode::KeyS) | PhysicalKey::Code(KeyCode::ArrowDown) => {
                        scene_manager.viewport_mut().origin.y += 50.0 / scene_manager.viewport().zoom;
                        renderer.window().request_redraw();
                    }
                    PhysicalKey::Code(KeyCode::KeyA) | PhysicalKey::Code(KeyCode::ArrowLeft) => {
                        scene_manager.viewport_mut().origin.x -= 50.0 / scene_manager.viewport().zoom;
                        renderer.window().request_redraw();
                    }
                    PhysicalKey::Code(KeyCode::KeyD) | PhysicalKey::Code(KeyCode::ArrowRight) => {
                        scene_manager.viewport_mut().origin.x += 50.0 / scene_manager.viewport().zoom;
                        renderer.window().request_redraw();
                    }
                    PhysicalKey::Code(KeyCode::KeyR) => {
                        scene_manager.viewport_mut().origin = DVec2::ZERO;
                        scene_manager.viewport_mut().zoom = 1.0;
                        renderer.window().request_redraw();
                    }
                    _ => {}
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
                    if let Some(drag_state) = &self.viewport_drag_state {
                        let dx = current_pos.x - drag_state.start_pos.x;
                        let dy = current_pos.y - drag_state.start_pos.y;
                        let zoom = scene_manager.viewport().zoom;
                        scene_manager.viewport_mut().set_origin(
                            drag_state.start_origin.x - dx / zoom,
                            drag_state.start_origin.y - dy / zoom,
                        );
                    }
                    renderer.window().request_redraw();
                }
            }
            WindowEvent::MouseInput { button: MouseButton::Left, state, .. } => {
                let Some(renderer) = &self.renderer else {
                    return;
                };
                if let Some(scene_manager) = &mut self.scene_manager {
                    if state == ElementState::Pressed {
                        if let Some(mouse_pos) = self.last_mouse_pos {
                            let hover_id = scene_manager.scene().hit_test(
                                scene_manager.viewport().screen_to_world(mouse_pos)
                            );
                            scene_manager.scene_mut().set_selected(hover_id);
                        }
                    }
                    renderer.window().request_redraw();
                }
            }
            WindowEvent::MouseInput { button: MouseButton::Middle, state, .. } => {
                let Some(renderer) = &self.renderer else {
                    return;
                };
                if let Some(scene_manager) = &mut self.scene_manager {
                    if let Some(mouse_pos) = self.last_mouse_pos {
                        if state == ElementState::Pressed {
                            self.viewport_drag_state = Some(ViewportDragState {
                                start_pos: mouse_pos,
                                start_origin: scene_manager.viewport().origin,
                            });
                        } else {
                            self.viewport_drag_state = None;
                        }
                    }
                    renderer.window().request_redraw();
                }
            }
            WindowEvent::MouseWheel { delta, .. } => {
                let Some(renderer) = &self.renderer else {
                    return;
                };
                let Some(mouse_pos) = self.last_mouse_pos else {
                    return;
                };
                if let Some(scene_manager) = &mut self.scene_manager {
                    let zoom_factor = match delta {
                        winit::event::MouseScrollDelta::LineDelta(_, y) => {
                            if y > 0.0 { 1.1 } else if y < 0.0 { 0.9 } else { 1.0 }
                        }
                        winit::event::MouseScrollDelta::PixelDelta(p) => {
                            let factor = 1.0 + p.y * 0.001;
                            if factor > 0.0 { factor } else { 1.0 }
                        }
                    };
                    if zoom_factor != 1.0 {
                        scene_manager.viewport_mut().zoom_at(zoom_factor, mouse_pos);
                        renderer.window().request_redraw();
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
            self.cached_window = Some(renderer.window().clone_window());
        }
    }
}

pub fn start_app() -> Result<(), Box<dyn std::error::Error>> {
    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Wait);

    let mut app = GraphicsApp::new();

    println!("启动事件循环... (按 ESC 退出)");
    println!("操作: ");
    println!("  - 中键拖拽 或 WASD/方向键 = 平移视图");
    println!("  - 滚轮 或 +/- 键 = 缩放");
    println!("  - 点击 = 选择图形");
    println!("  - R 键 = 重置视图");
    event_loop.run_app(&mut app)?;

    Ok(())
}
