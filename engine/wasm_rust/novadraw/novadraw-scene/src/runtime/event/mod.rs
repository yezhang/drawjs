use novadraw_geometry::Point;

use crate::BlockId;

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
    /// 鼠标点在当前 target/source Figure 坐标域中的 x 值。
    pub x: f64,
    /// 鼠标点在当前 target/source Figure 坐标域中的 y 值。
    pub y: f64,
    pub button: MouseButton,
    entry_point: Point,
}

impl MouseEvent {
    /// 创建一个入口域鼠标事件。
    ///
    /// 此时 `x/y` 与 `entry_point()` 相同；引擎在投递给 target 前会调用
    /// `with_target_point()` 生成 target/source Figure 坐标域中的事件点。
    pub fn new(kind: MouseEventKind, x: f64, y: f64, button: MouseButton) -> Self {
        Self {
            kind,
            x,
            y,
            button,
            entry_point: Point::new(x, y),
        }
    }

    /// 返回平台输入归一化后的入口节点坐标域点。
    ///
    /// 该点只读保留，用于调试、录制回放或跨 target 手势分析；Figure 的常规业务逻辑
    /// 应优先使用 `x/y`，它们已在引擎层转换到当前 target/source Figure 坐标域。
    pub fn entry_point(&self) -> Point {
        self.entry_point
    }

    /// 返回一个保留 entry point、但使用 target/source 坐标域点的新事件。
    pub fn with_target_point(self, x: f64, y: f64) -> Self {
        Self { x, y, ..self }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Event {
    Mouse(MouseEvent),
}

pub trait DispatchContext {
    fn find_mouse_event_target_at(&self, x: f64, y: f64) -> Option<BlockId>;
    fn mouse_target(&self) -> Option<BlockId>;
    fn set_mouse_target(&mut self, id: Option<BlockId>);
    fn set_hovered(&mut self, id: BlockId, hovered: bool);
    fn set_pressed(&mut self, id: BlockId, pressed: bool);
    fn focus_owner(&self) -> Option<BlockId>;
    fn set_focus_owner(&mut self, id: Option<BlockId>);
    fn captured(&self) -> Option<BlockId>;
    fn set_captured(&mut self, id: Option<BlockId>);
    fn wants_key_events(&self, _target_id: BlockId) -> bool {
        false
    }
    /// 将事件投递给 target。
    ///
    /// 传入的 `Event` 使用入口节点坐标域；具体实现负责在投递前把鼠标点转换到
    /// target Figure 的坐标域，以对齐 draw2d 的 `source.translateToRelative()` 语义。
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

        if previous_target == next_target {
            return;
        }

        if previous_target.is_some() {
            if let Some(previous_target) = previous_target {
                ctx.set_hovered(previous_target, false);
            }
            let exited = Event::Mouse(MouseEvent::new(
                MouseEventKind::Exited,
                x,
                y,
                MouseButton::None,
            ));
            let _ = ctx.dispatch_to_target(previous_target, &exited);
        }

        ctx.set_mouse_target(next_target);

        if next_target.is_some() {
            if let Some(next_target) = next_target {
                ctx.set_hovered(next_target, true);
            }
            let entered = Event::Mouse(MouseEvent::new(
                MouseEventKind::Entered,
                x,
                y,
                MouseButton::None,
            ));
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
        let event = Event::Mouse(MouseEvent::new(kind, x, y, button));
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
        let event = Event::Mouse(MouseEvent::new(MouseEventKind::Pressed, x, y, button));
        let handled = ctx.dispatch_to_target(target, &event);
        if handled {
            ctx.set_captured(target);
            if let Some(target) = target {
                ctx.set_pressed(target, true);
            }
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
        let event = Event::Mouse(MouseEvent::new(MouseEventKind::Released, x, y, button));
        let _ = ctx.dispatch_to_target(target, &event);
        if let Some(captured) = ctx.captured() {
            ctx.set_pressed(captured, false);
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

        fn set_hovered(&mut self, _id: BlockId, _hovered: bool) {}

        fn set_pressed(&mut self, _id: BlockId, _pressed: bool) {}

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
            Event::Mouse(MouseEvent::new(
                MouseEventKind::Entered,
                10.0,
                20.0,
                MouseButton::None,
            ))
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
            Event::Mouse(MouseEvent::new(
                MouseEventKind::Released,
                40.0,
                40.0,
                MouseButton::Left,
            ))
        );
        assert_eq!(
            ctx.dispatched.last().unwrap().1,
            Event::Mouse(MouseEvent::new(
                MouseEventKind::Exited,
                40.0,
                40.0,
                MouseButton::None,
            ))
        );
    }
}
