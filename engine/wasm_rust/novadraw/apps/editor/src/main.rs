use novadraw;
use log::{debug, error, log_enabled, info, Level};
use env_logger;

use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    event::WindowEvent,
    keyboard::{Key, KeyCode, PhysicalKey},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowAttributes, WindowId},
};
use winit::window::Window as WinitWindow;

struct GraphicsApp {
    window: Option<Window>,
}

impl GraphicsApp {
    fn new() -> Self {
        Self { window: None }
    }
}

impl ApplicationHandler<()> for GraphicsApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        // 创建窗口
        if self.window.is_none() {
            let window_attributes = WindowAttributes::default()
                .with_title("Vello 图形工具框架")
                .with_inner_size(winit::dpi::LogicalSize::new(800, 600))
                .with_resizable(true);

            match event_loop.create_window(window_attributes) {
                Ok(window) => {
                    self.window = Some(window);
                    println!("窗口创建成功");
                }
                Err(e) => {
                    eprintln!("创建窗口失败: {}", e);
                    event_loop.exit();
                }
            }
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _window_id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("收到窗口关闭请求，退出程序");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                // 后续在这里添加渲染逻辑
                if let Some(window) = self.window.as_ref() {
                    window.request_redraw(); // 持续请求重绘
                }
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
        if let Some(window) = self.window.as_ref() {
            window.request_redraw();
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化日志
    env_logger::init();
    
    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Wait);
    
    let mut app = GraphicsApp::new();
    
    println!("启动事件循环...");
    event_loop.run_app(&mut app)?;
    
    Ok(())
}

//fn main() {
//    let num = novadraw::add(1, 2);
//    println!("Hello, world! {num:?}");
//}
