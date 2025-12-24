use std::{env::Args, sync::Arc};

use novadraw::{self};
use vello::{
    AaConfig,
    peniko::color::palette,
    util::{RenderContext, RenderSurface},
    wgpu,
};
use winit::{
    application::ApplicationHandler,
    dpi,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowAttributes, WindowId},
};

struct RenderState {
    surface: RenderSurface<'static>,
    window: Arc<Window>,
}

pub struct GraphicsApp {
    context: RenderContext,
    scene_graph: Option<novadraw::SceneGraph>,
    state: Option<RenderState>,
    cached_window: Option<Arc<Window>>,
}

impl GraphicsApp {
    fn new(render_context: RenderContext, render_state: Option<RenderState>) -> Self {
        let scene_graph = novadraw::SceneGraph::new();

        Self {
            context: render_context,
            scene_graph: Some(scene_graph),
            state: render_state,
            cached_window: None,
        }
    }
    fn redraw(&self) {
        if let Some(scene_graph) = &self.scene_graph {
            let gc = scene_graph.render();
            let Some(RenderState { surface, window }) = &self.state else {
                return;
            };
            let width = surface.config.width;
            let height = surface.config.height;
            let device_handle = &self.context.devices[surface.dev_id];

            let base_color = palette::css::BLACK;
            let antialiasing_method = AaConfig::Area; // 可以修改为其他配置
            let render_params = vello::RenderParams {
                base_color,
                width,
                height,
                antialiasing_method,
            };
            // 输出到渲染后端
            let vello_renderer = novadraw::Renderer::new();

            vello_renderer.submit_commands(&gc.commands);
        }
    }
}

impl ApplicationHandler<()> for GraphicsApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let None = self.state else {
            return;
        };

        let window_attributes = WindowAttributes::default()
            .with_title("Vello 图形工具框架")
            .with_inner_size(dpi::LogicalSize::new(800, 600))
            .with_resizable(true);
        let window = self
            .cached_window
            .take()
            .unwrap_or_else(|| Arc::new(event_loop.create_window(window_attributes).unwrap()));
        let size = window.inner_size();
        let present_mode = wgpu::PresentMode::AutoVsync;
        let surface_future =
            self.context
                .create_surface(Arc::clone(&window), size.width, size.height, present_mode);
        // 阻塞在这里，防止 Suspended 事件发生
        let surface = pollster::block_on(surface_future).expect("error: 创建 surface");
        self.state = {
            let render_state = RenderState { window, surface };
            Some(render_state)
        };

        if self.scene_graph.is_none() {
            self.scene_graph = Some(novadraw::SceneGraph::new());
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        let Some(render_state) = &mut self.state else {
            return;
        };
        match event {
            WindowEvent::CloseRequested => {
                println!("收到窗口关闭请求，退出程序");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                // 后续在这里添加渲染逻辑
                // 创建 novadraw 场景图，创建渲染后端，执行渲染；
                // 如果是导出其他格式，那么使用不同的渲染后端，执行渲染。

                render_state.window.request_redraw();

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

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        // 请求重绘以保持动画循环
        if let Some(render_state) = &mut self.state {
            render_state.window.request_redraw();
        }
    }

    fn suspended(&mut self, event_loop: &ActiveEventLoop) {
        if let Some(render_state) = self.state.take() {
            self.cached_window = Some(render_state.window);
        }
    }
}

pub fn start_app() -> Result<(), Box<dyn std::error::Error>> {
    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Wait);

    let render_cx = RenderContext::new();
    let mut app = GraphicsApp::new(render_cx, None);

    println!("启动事件循环...");
    event_loop.run_app(&mut app)?;
    Ok(())
}
