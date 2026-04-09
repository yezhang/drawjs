use core_graphics::event::{CGEvent, CGEventTapLocation, CGEventType, CGMouseButton};
use core_graphics::event_source::{CGEventSource, CGEventSourceStateID};
use core_graphics::base::CGFloat;
use core_graphics::geometry::CGPoint;
use tracing::info;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
}

impl MouseButton {
    fn to_cg(&self) -> CGMouseButton {
        match self {
            MouseButton::Left => CGMouseButton::Left,
            MouseButton::Right => CGMouseButton::Right,
            MouseButton::Middle => CGMouseButton::Center,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ScreenPosition {
    pub x: f64,
    pub y: f64,
}

impl ScreenPosition {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    fn to_cgpoint(&self) -> CGPoint {
        CGPoint::new(self.x as CGFloat, self.y as CGFloat)
    }
}

pub trait MouseSimulator {
    fn move_to(&mut self, pos: ScreenPosition);
    fn press(&mut self, pos: ScreenPosition, button: MouseButton);
    fn release(&mut self, pos: ScreenPosition, button: MouseButton);
    fn click(&mut self, pos: ScreenPosition, button: MouseButton);
    fn double_click(&mut self, pos: ScreenPosition, button: MouseButton);
    fn drag(&mut self, start: ScreenPosition, end: ScreenPosition, button: MouseButton);
    fn scroll(&mut self, pos: ScreenPosition, delta_x: f64, delta_y: f64);
    fn hover(&mut self, pos: ScreenPosition, duration_ms: u64);
}

pub struct CGEventMouseSimulator {
    source: CGEventSource,
}

unsafe impl Send for CGEventMouseSimulator {}
unsafe impl Sync for CGEventMouseSimulator {}

impl CGEventMouseSimulator {
    pub fn new() -> Option<Self> {
        let source = CGEventSource::new(CGEventSourceStateID::CombinedSessionState).ok()?;
        Some(Self { source })
    }

    pub fn is_available() -> bool {
        CGEventSource::new(CGEventSourceStateID::CombinedSessionState).is_ok()
    }

    fn create_mouse_event(
        &self,
        event_type: CGEventType,
        pos: ScreenPosition,
        button: MouseButton,
    ) -> Option<CGEvent> {
        CGEvent::new_mouse_event(
            self.source.clone(),
            event_type,
            pos.to_cgpoint(),
            button.to_cg(),
        ).ok()
    }

    fn post(&self, event: &CGEvent) {
        event.post(CGEventTapLocation::HID);
    }
}

impl MouseSimulator for CGEventMouseSimulator {
    fn move_to(&mut self, pos: ScreenPosition) {
        if let Some(event) = self.create_mouse_event(CGEventType::MouseMoved, pos, MouseButton::Left) {
            self.post(&event);
            info!("CGEvent: MouseMoved to ({:.1}, {:.1})", pos.x, pos.y);
        }
    }

    fn press(&mut self, pos: ScreenPosition, button: MouseButton) {
        let event_type = match button {
            MouseButton::Left => CGEventType::LeftMouseDown,
            MouseButton::Right => CGEventType::RightMouseDown,
            MouseButton::Middle => CGEventType::OtherMouseDown,
        };

        if let Some(event) = self.create_mouse_event(event_type, pos, button) {
            self.post(&event);
            info!("CGEvent: {:?}Down at ({:.1}, {:.1})", button, pos.x, pos.y);
        }
    }

    fn release(&mut self, pos: ScreenPosition, button: MouseButton) {
        let event_type = match button {
            MouseButton::Left => CGEventType::LeftMouseUp,
            MouseButton::Right => CGEventType::RightMouseUp,
            MouseButton::Middle => CGEventType::OtherMouseUp,
        };

        if let Some(event) = self.create_mouse_event(event_type, pos, button) {
            self.post(&event);
            info!("CGEvent: {:?}Up at ({:.1}, {:.1})", button, pos.x, pos.y);
        }
    }

    fn click(&mut self, pos: ScreenPosition, button: MouseButton) {
        self.move_to(pos);
        std::thread::sleep(std::time::Duration::from_millis(10));
        self.press(pos, button);
        std::thread::sleep(std::time::Duration::from_millis(10));
        self.release(pos, button);
        info!("CGEvent: Click at ({:.1}, {:.1})", pos.x, pos.y);
    }

    fn double_click(&mut self, pos: ScreenPosition, button: MouseButton) {
        self.click(pos, button);
        std::thread::sleep(std::time::Duration::from_millis(50));

        let event_type = match button {
            MouseButton::Left => CGEventType::LeftMouseDown,
            MouseButton::Right => CGEventType::RightMouseDown,
            MouseButton::Middle => CGEventType::OtherMouseDown,
        };

        if let Some(event) = self.create_mouse_event(event_type, pos, button) {
            self.post(&event);
        }

        std::thread::sleep(std::time::Duration::from_millis(10));

        let event_type = match button {
            MouseButton::Left => CGEventType::LeftMouseUp,
            MouseButton::Right => CGEventType::RightMouseUp,
            MouseButton::Middle => CGEventType::OtherMouseUp,
        };

        if let Some(event) = self.create_mouse_event(event_type, pos, button) {
            self.post(&event);
        }

        info!("CGEvent: DoubleClick at ({:.1}, {:.1})", pos.x, pos.y);
    }

    fn drag(&mut self, start: ScreenPosition, end: ScreenPosition, button: MouseButton) {
        self.move_to(start);
        std::thread::sleep(std::time::Duration::from_millis(50));
        self.press(start, button);

        let steps = 20;
        for i in 1..=steps {
            let t = i as f64 / steps as f64;
            let x = start.x + (end.x - start.x) * t;
            let y = start.y + (end.y - start.y) * t;
            let pos = ScreenPosition::new(x, y);
            self.move_to(pos);
            std::thread::sleep(std::time::Duration::from_millis(5));
        }

        self.release(end, button);
        info!("CGEvent: Drag from ({:.1}, {:.1}) to ({:.1}, {:.1})", start.x, start.y, end.x, end.y);
    }

    fn scroll(&mut self, pos: ScreenPosition, delta_x: f64, delta_y: f64) {
        let _ = pos;
        let _ = delta_x;
        let _ = delta_y;
        info!("CGEvent: Scroll simulation is not implemented on current backend");
    }

    fn hover(&mut self, pos: ScreenPosition, duration_ms: u64) {
        self.move_to(pos);
        info!("CGEvent: Hover at ({:.1}, {:.1}) for {} ms", pos.x, pos.y, duration_ms);
        std::thread::sleep(std::time::Duration::from_millis(duration_ms));
    }
}

pub struct ScreenPositionConverter {
    scale_factor: f64,
    window_x: f64,
    window_y: f64,
    window_width: f64,
    window_height: f64,
}

impl ScreenPositionConverter {
    pub fn new(scale_factor: f64, window_x: f64, window_y: f64, window_width: f64, window_height: f64) -> Self {
        Self {
            scale_factor,
            window_x,
            window_y,
            window_width,
            window_height,
        }
    }

    pub fn logical_to_screen(&self, logical_x: f64, logical_y: f64) -> ScreenPosition {
        let screen_x = self.window_x + logical_x * self.scale_factor;
        let screen_y = self.window_y + logical_y * self.scale_factor;
        ScreenPosition::new(screen_x, screen_y)
    }

    pub fn screen_to_logical(&self, screen: ScreenPosition) -> (f64, f64) {
        let logical_x = (screen.x - self.window_x) / self.scale_factor;
        let logical_y = (screen.y - self.window_y) / self.scale_factor;
        (logical_x, logical_y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cgevent_availability() {
        let available = CGEventMouseSimulator::is_available();
        if available {
            println!("✓ CGEventMouseSimulator 可用 - Accessibility 权限已授予");
        } else {
            println!("✗ CGEventMouseSimulator 不可用 - 需要授予 Accessibility 权限");
            println!("  打开: System Settings → Privacy & Security → Accessibility");
            println!("  添加并启用: target/debug/editor 或 cargo");
        }
        assert!(available, "CGEventMouseSimulator requires Accessibility permissions");
    }

    #[test]
    fn test_mouse_simulator_lifecycle() {
        let simulator = CGEventMouseSimulator::new();
        assert!(simulator.is_some(), "Should create simulator when permissions granted");

        if let Some(mut sim) = simulator {
            let pos = ScreenPosition::new(100.0, 100.0);
            sim.move_to(pos);
            println!("✓ 鼠标移动事件发送成功");
        }
    }
}
