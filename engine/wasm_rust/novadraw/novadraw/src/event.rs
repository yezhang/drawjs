use glam::DVec2;

#[derive(Clone, Copy, Debug)]
pub struct MouseEvent {
    pub x: f64,
    pub y: f64,
    pub event_type: MouseEventType,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MouseEventType {
    Move,
    Press,
    Release,
    Enter,
    Leave,
}

impl MouseEvent {
    pub fn new(x: f64, y: f64, event_type: MouseEventType) -> Self {
        Self { x, y, event_type }
    }

    pub fn position(&self) -> DVec2 {
        DVec2::new(self.x, self.y)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CursorStyle {
    Default,
    Pointer,
    ResizeNS,
    ResizeEW,
    Move,
    Text,
}

impl Default for CursorStyle {
    fn default() -> Self {
        Self::Default
    }
}
