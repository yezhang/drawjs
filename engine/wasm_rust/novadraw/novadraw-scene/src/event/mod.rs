use crate::BlockId;
use tracing::info;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseEventKind {
    Pressed,
    Released,
    Moved,
    Entered,
    Exited,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MouseEvent {
    pub kind: MouseEventKind,
    pub x: f64,
    pub y: f64,
    pub button: MouseButton,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Event {
    Mouse(MouseEvent),
}

pub trait DispatchContext {
    fn find_mouse_event_target_at(&self, x: f64, y: f64) -> Option<BlockId>;
    fn mouse_target(&self) -> Option<BlockId>;
    fn set_mouse_target(&mut self, id: Option<BlockId>);
    fn focus_owner(&self) -> Option<BlockId>;
    fn set_focus_owner(&mut self, id: Option<BlockId>);
    fn captured(&self) -> Option<BlockId>;
    fn set_captured(&mut self, id: Option<BlockId>);
    fn wants_key_events(&self, _target_id: BlockId) -> bool {
        false
    }
    fn dispatch_to_target(&mut self, target_id: Option<BlockId>, event: &Event) -> bool;
}

pub trait EventDispatcher: Send + Sync {
    fn receive(&mut self, ctx: &mut dyn DispatchContext, x: f64, y: f64);
    fn dispatch_mouse_pressed(
        &mut self,
        ctx: &mut dyn DispatchContext,
        x: f64,
        y: f64,
        button: MouseButton,
    );
    fn dispatch_mouse_released(
        &mut self,
        ctx: &mut dyn DispatchContext,
        x: f64,
        y: f64,
        button: MouseButton,
    );
    fn dispatch_mouse_moved(&mut self, ctx: &mut dyn DispatchContext, x: f64, y: f64);
}

#[derive(Default)]
pub struct BasicEventDispatcher;

impl BasicEventDispatcher {
    fn refresh_mouse_target(&mut self, ctx: &mut dyn DispatchContext, x: f64, y: f64) {
        let hit_target = ctx.find_mouse_event_target_at(x, y);
        let captured = ctx.captured();
        let next_target = captured.or(hit_target);
        let previous_target = ctx.mouse_target();

        tracing::info!(
            "[EventDispatcher] refresh_mouse_target: captured={:?}, hit_target={:?}, next={:?}, prev={:?}, coords=({:.1}, {:.1})",
            captured, hit_target, next_target, previous_target, x, y
        );

        if previous_target == next_target {
            tracing::debug!("[EventDispatcher] mouse_target unchanged, no events dispatched");
            return;
        }

        info!(
            "mouse_target changed: previous={:?}, next={:?}, x={}, y={}",
            previous_target, next_target, x, y
        );

        if previous_target.is_some() {
            let exited = Event::Mouse(MouseEvent {
                kind: MouseEventKind::Exited,
                x,
                y,
                button: MouseButton::None,
            });
            let _ = ctx.dispatch_to_target(previous_target, &exited);
        }

        ctx.set_mouse_target(next_target);

        if next_target.is_some() {
            let entered = Event::Mouse(MouseEvent {
                kind: MouseEventKind::Entered,
                x,
                y,
                button: MouseButton::None,
            });
            let _ = ctx.dispatch_to_target(next_target, &entered);
        }
    }

    fn dispatch_mouse_event(
        &mut self,
        ctx: &mut dyn DispatchContext,
        kind: MouseEventKind,
        x: f64,
        y: f64,
        button: MouseButton,
    ) {
        self.refresh_mouse_target(ctx, x, y);
        let event = Event::Mouse(MouseEvent { kind, x, y, button });
        let _ = ctx.dispatch_to_target(ctx.mouse_target(), &event);
    }
}

impl EventDispatcher for BasicEventDispatcher {
    fn receive(&mut self, ctx: &mut dyn DispatchContext, x: f64, y: f64) {
        self.refresh_mouse_target(ctx, x, y);
    }

    fn dispatch_mouse_pressed(
        &mut self,
        ctx: &mut dyn DispatchContext,
        x: f64,
        y: f64,
        button: MouseButton,
    ) {
        self.refresh_mouse_target(ctx, x, y);
        let target = ctx.mouse_target();
        let event = Event::Mouse(MouseEvent {
            kind: MouseEventKind::Pressed,
            x,
            y,
            button,
        });
        let handled = ctx.dispatch_to_target(target, &event);
        if handled {
            ctx.set_captured(target);
        }
    }

    fn dispatch_mouse_released(
        &mut self,
        ctx: &mut dyn DispatchContext,
        x: f64,
        y: f64,
        button: MouseButton,
    ) {
        self.refresh_mouse_target(ctx, x, y);
        let target = ctx.mouse_target();
        let event = Event::Mouse(MouseEvent {
            kind: MouseEventKind::Released,
            x,
            y,
            button,
        });
        let _ = ctx.dispatch_to_target(target, &event);
        if ctx.captured().is_some() {
            ctx.set_captured(None);
            self.refresh_mouse_target(ctx, x, y);
        }
    }

    fn dispatch_mouse_moved(&mut self, ctx: &mut dyn DispatchContext, x: f64, y: f64) {
        self.dispatch_mouse_event(ctx, MouseEventKind::Moved, x, y, MouseButton::None);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{FigureGraph, RectangleFigure};

    struct MockDispatchContext {
        hit_target: Option<BlockId>,
        mouse_target: Option<BlockId>,
        focus_owner: Option<BlockId>,
        captured: Option<BlockId>,
        dispatched: Vec<(Option<BlockId>, Event)>,
        handled: bool,
    }

    impl MockDispatchContext {
        fn new(hit_target: Option<BlockId>) -> Self {
            Self {
                hit_target,
                mouse_target: None,
                focus_owner: None,
                captured: None,
                dispatched: Vec::new(),
                handled: false,
            }
        }
    }

    impl DispatchContext for MockDispatchContext {
        fn find_mouse_event_target_at(&self, _x: f64, _y: f64) -> Option<BlockId> {
            self.hit_target
        }

        fn mouse_target(&self) -> Option<BlockId> {
            self.mouse_target
        }

        fn set_mouse_target(&mut self, id: Option<BlockId>) {
            self.mouse_target = id;
        }

        fn focus_owner(&self) -> Option<BlockId> {
            self.focus_owner
        }

        fn set_focus_owner(&mut self, id: Option<BlockId>) {
            self.focus_owner = id;
        }

        fn captured(&self) -> Option<BlockId> {
            self.captured
        }

        fn set_captured(&mut self, id: Option<BlockId>) {
            self.captured = id;
        }

        fn dispatch_to_target(&mut self, target_id: Option<BlockId>, event: &Event) -> bool {
            self.dispatched.push((target_id, *event));
            self.handled
        }
    }

    #[test]
    fn test_receive_updates_mouse_target() {
        let mut dispatcher = BasicEventDispatcher;
        let mut ctx = MockDispatchContext::new(None);
        let mut scene = FigureGraph::new();
        let target = scene.set_contents(Box::new(RectangleFigure::new(0.0, 0.0, 10.0, 10.0)));
        ctx.hit_target = Some(target);

        dispatcher.receive(&mut ctx, 10.0, 20.0);

        assert_eq!(ctx.mouse_target(), Some(target));
        assert_eq!(ctx.dispatched.len(), 1);
        assert_eq!(ctx.dispatched[0].0, Some(target));
        assert_eq!(
            ctx.dispatched[0].1,
            Event::Mouse(MouseEvent {
                kind: MouseEventKind::Entered,
                x: 10.0,
                y: 20.0,
                button: MouseButton::None,
            })
        );
    }

    #[test]
    fn test_captured_target_overrides_hit_target() {
        let mut dispatcher = BasicEventDispatcher;
        let mut scene = FigureGraph::new();
        let hit_target = scene.set_contents(Box::new(RectangleFigure::new(0.0, 0.0, 10.0, 10.0)));
        let captured = scene.add_child_to(
            hit_target,
            Box::new(RectangleFigure::new(1.0, 1.0, 4.0, 4.0)),
        );
        let mut ctx = MockDispatchContext::new(Some(hit_target));
        ctx.set_captured(Some(captured));

        dispatcher.dispatch_mouse_moved(&mut ctx, 5.0, 6.0);

        assert_eq!(ctx.mouse_target(), Some(captured));
        assert_eq!(ctx.dispatched.last().unwrap().0, Some(captured));
    }

    #[test]
    fn test_press_sets_capture_when_handled() {
        let mut dispatcher = BasicEventDispatcher;
        let mut scene = FigureGraph::new();
        let target = scene.set_contents(Box::new(RectangleFigure::new(0.0, 0.0, 10.0, 10.0)));
        let mut ctx = MockDispatchContext::new(Some(target));
        ctx.handled = true;

        dispatcher.dispatch_mouse_pressed(&mut ctx, 4.0, 4.0, MouseButton::Left);

        assert_eq!(ctx.mouse_target(), Some(target));
        assert_eq!(ctx.captured(), Some(target));
        assert_eq!(ctx.dispatched.last().unwrap().0, Some(target));
    }

    #[test]
    fn test_release_uses_capture_and_then_clears_it() {
        let mut dispatcher = BasicEventDispatcher;
        let mut scene = FigureGraph::new();
        let target = scene.set_contents(Box::new(RectangleFigure::new(0.0, 0.0, 10.0, 10.0)));
        let mut ctx = MockDispatchContext::new(None);
        ctx.mouse_target = Some(target);
        ctx.captured = Some(target);
        ctx.handled = true;

        dispatcher.dispatch_mouse_released(&mut ctx, 40.0, 40.0, MouseButton::Left);

        assert_eq!(ctx.captured(), None);
        assert_eq!(ctx.mouse_target(), None);
        assert_eq!(ctx.dispatched[0].0, Some(target));
        assert_eq!(
            ctx.dispatched[0].1,
            Event::Mouse(MouseEvent {
                kind: MouseEventKind::Released,
                x: 40.0,
                y: 40.0,
                button: MouseButton::Left,
            })
        );
        assert_eq!(
            ctx.dispatched.last().unwrap().1,
            Event::Mouse(MouseEvent {
                kind: MouseEventKind::Exited,
                x: 40.0,
                y: 40.0,
                button: MouseButton::None,
            })
        );
    }
}
