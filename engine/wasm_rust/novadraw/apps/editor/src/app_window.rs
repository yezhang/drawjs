use std::sync::Arc;

use crate::scene_manager::SceneManager;
use glam::DVec2;
use novadraw::{BlockId, Color, Transform};
use novadraw::renderer::{VelloRenderer, WinitWindowProxy};
use novadraw::{Renderer, WindowProxy};
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
    selection_state: Option<SelectionState>,
    draw_state: Option<DrawState>,
    viewport_drag_state: Option<ViewportDragState>,
    space_drag_state: Option<SpaceDragState>,
    last_mouse_pos: Option<DVec2>,
    active_tool: crate::scene_manager::Tool,
    space_pressed: bool,
}

struct ViewportDragState {
    start_pos: DVec2,
    start_origin: DVec2,
}

struct SpaceDragState {
    start_pos: DVec2,
    start_origin: DVec2,
}

struct DragState {
    block_id: BlockId,
    start_pos: DVec2,
    start_transform: Transform,
}

struct SelectionState {
    #[allow(dead_code)]
    start_pos: DVec2,
}

struct DrawState {
    start_pos: DVec2,
    temp_rect_id: BlockId,
}

impl GraphicsApp {
    fn new() -> Self {
        GraphicsApp {
            renderer: None,
            scene_manager: None,
            cached_window: None,
            drag_state: None,
            selection_state: None,
            draw_state: None,
            viewport_drag_state: None,
            space_drag_state: None,
            last_mouse_pos: None,
            active_tool: crate::scene_manager::Tool::Select,
            space_pressed: false,
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

    fn set_tool(&mut self, tool: crate::scene_manager::Tool) {
        self.active_tool = tool;
        if let Some(scene_manager) = &mut self.scene_manager {
            scene_manager.set_tool(tool);
        }
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
                    .with_title("Novadraw Editor")
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
            manager.add_rectangle(400.0, 200.0, 150.0, 150.0, Color::hex("#3498db"));
            manager.add_rectangle_at_center(400.0, 300.0, 200.0, 100.0, Color::hex("#e74c3c"));
            manager.set_tool(self.active_tool);
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
                    let logical_width = new_size.width as f64 / scale_factor;
                    let logical_height = new_size.height as f64 / scale_factor;

                    if let Some(scene_manager) = &mut self.scene_manager {
                        scene_manager.set_window_size(logical_width, logical_height);
                    }

                    renderer.resize(new_size.width, new_size.height);
                }
            }
            WindowEvent::KeyboardInput { event, .. } => {
                if let PhysicalKey::Code(KeyCode::Escape) = event.physical_key {
                    event_loop.exit();
                }
                if event.state == ElementState::Pressed {
                    match event.physical_key {
                        PhysicalKey::Code(KeyCode::KeyV) => self.set_tool(crate::scene_manager::Tool::Select),
                        PhysicalKey::Code(KeyCode::KeyR) => self.set_tool(crate::scene_manager::Tool::Rectangle),
                        PhysicalKey::Code(KeyCode::KeyC) => self.set_tool(crate::scene_manager::Tool::Circle),
                        PhysicalKey::Code(KeyCode::Space) => self.space_pressed = true,
                        _ => {}
                    }
                } else if event.state == ElementState::Released {
                    if let PhysicalKey::Code(KeyCode::Space) = event.physical_key {
                        self.space_pressed = false;
                        self.space_drag_state = None;
                    }
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
                    let tool = scene_manager.active_tool();

                    if let Some(drag_state) = &self.viewport_drag_state {
                        let dx = current_pos.x - drag_state.start_pos.x;
                        let dy = current_pos.y - drag_state.start_pos.y;
                        let zoom = scene_manager.viewport().zoom;
                        scene_manager.viewport_mut().set_origin(
                            drag_state.start_origin.x - dx / zoom,
                            drag_state.start_origin.y - dy / zoom,
                        );
                    } else if self.space_pressed {
                        if let Some(drag_state) = &self.space_drag_state {
                            let dx = current_pos.x - drag_state.start_pos.x;
                            let dy = current_pos.y - drag_state.start_pos.y;
                            let zoom = scene_manager.viewport().zoom;
                            scene_manager.viewport_mut().set_origin(
                                drag_state.start_origin.x - dx / zoom,
                                drag_state.start_origin.y - dy / zoom,
                            );
                        } else {
                            self.space_drag_state = Some(SpaceDragState {
                                start_pos: current_pos,
                                start_origin: scene_manager.viewport().origin,
                            });
                        }
                    } else if tool == crate::scene_manager::Tool::Select {
                        if let Some(drag_state) = &self.drag_state {
                            let dx = current_pos.x - drag_state.start_pos.x;
                            let dy = current_pos.y - drag_state.start_pos.y;
                            let translate = Transform::from_translation_2d(dx, dy);
                            let new_transform = drag_state.start_transform * translate;
                            scene_manager.scene_mut().set_block_transform(drag_state.block_id, new_transform);
                        } else if self.selection_state.is_some() {
                            scene_manager.update_selection_box(current_pos);
                        } else {
                            let world_pos = scene_manager.viewport().screen_to_world(current_pos);
                            let hit_id = scene_manager.scene().hit_test(world_pos);
                            scene_manager.set_hovered(hit_id);
                        }
                    } else if tool == crate::scene_manager::Tool::Rectangle {
                        if let Some(draw_state) = &mut self.draw_state {
                            let world_start = draw_state.start_pos;
                            let world_end = scene_manager.viewport().screen_to_world(current_pos);
                            scene_manager.update_temp_rectangle(
                                draw_state.temp_rect_id,
                                world_start,
                                world_end,
                            );
                        }
                    }
                    renderer.window().request_redraw();
                }
            }
            WindowEvent::MouseInput { button: MouseButton::Left, state, .. } => {
                let Some(renderer) = &self.renderer else {
                    return;
                };
                if let Some(scene_manager) = &mut self.scene_manager {
                    let tool = scene_manager.active_tool();

                    if state == ElementState::Pressed {
                        if let Some(mouse_pos) = self.last_mouse_pos {
                            if tool == crate::scene_manager::Tool::Select {
                                let hover_id = scene_manager.scene().hit_test(
                                    scene_manager.viewport().screen_to_world(mouse_pos)
                                );
                                if let Some(id) = hover_id {
                                    scene_manager.scene_mut().set_selected(Some(id));
                                    let start_transform = scene_manager.scene()
                                        .get_block(id)
                                        .map(|b| b.transform)
                                        .unwrap_or_else(Transform::identity);
                                    self.drag_state = Some(DragState {
                                        block_id: id,
                                        start_pos: mouse_pos,
                                        start_transform,
                                    });
                                    self.selection_state = None;
                                } else {
                                    scene_manager.scene_mut().set_selected(None);
                                    self.selection_state = Some(SelectionState {
                                        start_pos: mouse_pos,
                                    });
                                    scene_manager.start_selection_box(mouse_pos);
                                }
                            } else if tool == crate::scene_manager::Tool::Rectangle {
                                let world_pos = scene_manager.viewport().screen_to_world(mouse_pos);
                                let temp_id = scene_manager.create_temp_rectangle(world_pos);
                                self.draw_state = Some(DrawState {
                                    start_pos: world_pos,
                                    temp_rect_id: temp_id,
                                });
                            }
                        }
                    } else {
                        if tool == crate::scene_manager::Tool::Select {
                            if let Some(mouse_pos) = self.last_mouse_pos {
                                if self.selection_state.is_some() {
                                    scene_manager.end_selection_box(mouse_pos);
                                }
                            }
                            self.drag_state = None;
                            self.selection_state = None;
                        } else if tool == crate::scene_manager::Tool::Rectangle {
                            if let Some(draw_state) = self.draw_state.take() {
                                scene_manager.finalize_temp_rectangle(draw_state.temp_rect_id);
                            }
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
    println!("快捷键: V=选择, R=矩形, C=圆形, L=线条");
    event_loop.run_app(&mut app)?;

    Ok(())
}
