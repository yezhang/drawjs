use std::{sync::Arc, time::Duration};

use novadraw::{
    backend::vello::WinitWindowProxy, BasicEventDispatcher, BlockId, DispatchContext, Event,
    EventDispatcher, MouseButton, MouseEventKind, NdCanvas, NovadrawContext, NovadrawSystem,
    PendingMutations, Rectangle, RenderBackend, SceneHost, SceneUpdateManager, UpdateManager,
};

use crate::scene_manager::{
    mouse_simulator::{CGEventMouseSimulator, MouseButton as SimMouseButton, MouseSimulator, ScreenPositionConverter},
    SceneManager,
    scene_host::WinitSceneHost,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RawPointerInput {
    pub physical_x: f64,
    pub physical_y: f64,
    pub scale_factor: f64,
}

impl RawPointerInput {
    pub fn new(physical_x: f64, physical_y: f64, scale_factor: f64) -> Self {
        Self {
            physical_x,
            physical_y,
            scale_factor,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LogicalPointerPosition {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct InteractionTrace {
    pub phase: &'static str,
    pub raw: Option<RawPointerInput>,
    pub logical: LogicalPointerPosition,
    pub button: Option<MouseButton>,
    pub hit_target_before: Option<BlockId>,
    pub mouse_target_before: Option<BlockId>,
    pub mouse_target_after: Option<BlockId>,
    pub focus_owner_after: Option<BlockId>,
    pub captured_after: Option<BlockId>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InteractionStep {
    Move(RawPointerInput),
    Press {
        input: RawPointerInput,
        button: MouseButton,
    },
    Release {
        input: RawPointerInput,
        button: MouseButton,
    },
    Hover {
        input: RawPointerInput,
        duration_ms: u64,
    },
    Click {
        input: RawPointerInput,
        button: MouseButton,
    },
    Wait {
        duration_ms: u64,
    },
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct InteractionReport {
    pub traces: Vec<InteractionTrace>,
}

pub struct EditorInteractionCore {
    scene_manager: SceneManager,
    update_manager: SceneUpdateManager,
    dispatcher: BasicEventDispatcher,
    pending_mutations: PendingMutations,
}

impl Default for EditorInteractionCore {
    fn default() -> Self {
        Self::new()
    }
}

impl EditorInteractionCore {
    pub fn new() -> Self {
        Self {
            scene_manager: SceneManager::new(),
            update_manager: SceneUpdateManager::new(),
            dispatcher: BasicEventDispatcher,
            pending_mutations: PendingMutations::new(),
        }
    }

    pub fn scene_manager(&self) -> &SceneManager {
        &self.scene_manager
    }

    pub fn scene_manager_mut(&mut self) -> &mut SceneManager {
        &mut self.scene_manager
    }

    pub fn logical_from_raw(input: RawPointerInput) -> LogicalPointerPosition {
        let scale_factor = if input.scale_factor > 0.0 {
            input.scale_factor
        } else {
            1.0
        };
        LogicalPointerPosition {
            x: input.physical_x / scale_factor,
            y: input.physical_y / scale_factor,
        }
    }

    fn build_trace(
        &self,
        phase: &'static str,
        raw: Option<RawPointerInput>,
        logical: LogicalPointerPosition,
        button: Option<MouseButton>,
    ) -> InteractionTrace {
        InteractionTrace {
            phase,
            raw,
            logical,
            button,
            hit_target_before: self
                .scene_manager
                .scene
                .find_mouse_event_target_at(logical.x, logical.y),
            mouse_target_before: self.scene_manager.scene.mouse_target(),
            mouse_target_after: None,
            focus_owner_after: None,
            captured_after: None,
        }
    }

    fn finish_trace(&self, trace: &mut InteractionTrace) {
        trace.mouse_target_after = self.scene_manager.scene.mouse_target();
        trace.focus_owner_after = self.scene_manager.scene.focus_owner();
        trace.captured_after = self.scene_manager.scene.captured();
    }

    pub fn dispatch_mouse_moved(&mut self, x: f64, y: f64) {
        tracing::info!("[MouseEvent] Moved - scene_coords=({:.1}, {:.1})", x, y);
        let mut ctx = EditorDispatchContext::new(&mut self.scene_manager.scene, &mut self.update_manager);
        self.dispatcher.dispatch_mouse_moved(&mut ctx, x, y);
        self.apply_pending_mutations();
    }

    pub fn dispatch_mouse_pressed(&mut self, x: f64, y: f64, button: MouseButton) {
        tracing::info!(
            "[MouseEvent] Pressed - scene_coords=({:.1}, {:.1}), button={:?}",
            x,
            y,
            button
        );
        let mut ctx = EditorDispatchContext::new(&mut self.scene_manager.scene, &mut self.update_manager);
        self.dispatcher
            .dispatch_mouse_pressed(&mut ctx, x, y, button);
        self.apply_pending_mutations();
    }

    pub fn dispatch_mouse_released(&mut self, x: f64, y: f64, button: MouseButton) {
        tracing::info!(
            "[MouseEvent] Released - scene_coords=({:.1}, {:.1}), button={:?}",
            x,
            y,
            button
        );
        let mut ctx = EditorDispatchContext::new(&mut self.scene_manager.scene, &mut self.update_manager);
        self.dispatcher
            .dispatch_mouse_released(&mut ctx, x, y, button);
        self.apply_pending_mutations();
    }

    pub fn dispatch_hover(&mut self, x: f64, y: f64, duration_ms: u64) {
        tracing::info!(
            "[Internal Hover] dispatching hover at ({:.1}, {:.1}) for {} ms",
            x,
            y,
            duration_ms
        );
        self.dispatch_mouse_moved(x, y);
        std::thread::sleep(Duration::from_millis(duration_ms));
        tracing::info!("[Internal Hover] completed");
    }

    pub fn dispatch_click(&mut self, x: f64, y: f64) {
        tracing::info!("[Internal Click] dispatching click at ({:.1}, {:.1})", x, y);
        self.dispatch_mouse_pressed(x, y, MouseButton::Left);
        self.dispatch_mouse_released(x, y, MouseButton::Left);
        tracing::info!("[Internal Click] completed");
    }

    pub fn dispatch_raw_mouse_moved(&mut self, input: RawPointerInput) -> InteractionTrace {
        let logical = Self::logical_from_raw(input);
        tracing::info!(
            "[RawPointer] move physical=({:.1}, {:.1}) scale_factor={:.2} logical=({:.1}, {:.1})",
            input.physical_x,
            input.physical_y,
            input.scale_factor,
            logical.x,
            logical.y
        );
        let mut trace = self.build_trace("move", Some(input), logical, None);
        self.dispatch_mouse_moved(logical.x, logical.y);
        self.finish_trace(&mut trace);
        trace
    }

    pub fn dispatch_raw_mouse_pressed(
        &mut self,
        input: RawPointerInput,
        button: MouseButton,
    ) -> InteractionTrace {
        let logical = Self::logical_from_raw(input);
        tracing::info!(
            "[RawPointer] press physical=({:.1}, {:.1}) scale_factor={:.2} logical=({:.1}, {:.1}) button={:?}",
            input.physical_x,
            input.physical_y,
            input.scale_factor,
            logical.x,
            logical.y,
            button
        );
        let mut trace = self.build_trace("press", Some(input), logical, Some(button));
        self.dispatch_mouse_pressed(logical.x, logical.y, button);
        self.finish_trace(&mut trace);
        trace
    }

    pub fn dispatch_raw_mouse_released(
        &mut self,
        input: RawPointerInput,
        button: MouseButton,
    ) -> InteractionTrace {
        let logical = Self::logical_from_raw(input);
        tracing::info!(
            "[RawPointer] release physical=({:.1}, {:.1}) scale_factor={:.2} logical=({:.1}, {:.1}) button={:?}",
            input.physical_x,
            input.physical_y,
            input.scale_factor,
            logical.x,
            logical.y,
            button
        );
        let mut trace = self.build_trace("release", Some(input), logical, Some(button));
        self.dispatch_mouse_released(logical.x, logical.y, button);
        self.finish_trace(&mut trace);
        trace
    }

    pub fn run_interaction_script(&mut self, steps: &[InteractionStep]) -> InteractionReport {
        let mut report = InteractionReport::default();
        for step in steps {
            match *step {
                InteractionStep::Move(input) => {
                    report.traces.push(self.dispatch_raw_mouse_moved(input));
                }
                InteractionStep::Press { input, button } => {
                    report
                        .traces
                        .push(self.dispatch_raw_mouse_pressed(input, button));
                }
                InteractionStep::Release { input, button } => {
                    report
                        .traces
                        .push(self.dispatch_raw_mouse_released(input, button));
                }
                InteractionStep::Hover { input, duration_ms } => {
                    report.traces.push(self.dispatch_raw_mouse_moved(input));
                    std::thread::sleep(Duration::from_millis(duration_ms));
                }
                InteractionStep::Click { input, button } => {
                    report
                        .traces
                        .push(self.dispatch_raw_mouse_pressed(input, button));
                    report
                        .traces
                        .push(self.dispatch_raw_mouse_released(input, button));
                }
                InteractionStep::Wait { duration_ms } => {
                    std::thread::sleep(Duration::from_millis(duration_ms));
                }
            }
        }
        report
    }

    fn apply_pending_mutations(&mut self) -> bool {
        let mutations = self.pending_mutations.drain();
        self.scene_manager
            .scene
            .apply_pending_mutations(&mut self.update_manager, mutations)
    }
}

pub struct WinitNovadrawSystem {
    core: EditorInteractionCore,
    scene_host: WinitSceneHost,
    mouse_simulator: Option<CGEventMouseSimulator>,
    position_converter: Option<ScreenPositionConverter>,
}

impl WinitNovadrawSystem {
    pub fn new(window_proxy: Arc<WinitWindowProxy>) -> Self {
        Self {
            core: EditorInteractionCore::new(),
            scene_host: WinitSceneHost::new(window_proxy),
            mouse_simulator: CGEventMouseSimulator::new(),
            position_converter: None,
        }
    }

    pub fn scene_manager(&self) -> &SceneManager {
        self.core.scene_manager()
    }

    pub fn scene_manager_mut(&mut self) -> &mut SceneManager {
        self.core.scene_manager_mut()
    }

    pub fn set_position_converter(&mut self, converter: ScreenPositionConverter) {
        self.position_converter = Some(converter);
    }

    pub fn sim_click(&mut self, logical_x: f64, logical_y: f64) {
        if let (Some(sim), Some(converter)) = (&mut self.mouse_simulator, &self.position_converter) {
            let screen_pos = converter.logical_to_screen(logical_x, logical_y);
            let btn = SimMouseButton::Left;
            sim.click(screen_pos, btn);
        } else {
            tracing::warn!("MouseSimulator not available - run with Accessibility permissions");
        }
    }

    pub fn sim_double_click(&mut self, logical_x: f64, logical_y: f64) {
        if let (Some(sim), Some(converter)) = (&mut self.mouse_simulator, &self.position_converter) {
            let screen_pos = converter.logical_to_screen(logical_x, logical_y);
            let btn = SimMouseButton::Left;
            sim.double_click(screen_pos, btn);
        } else {
            tracing::warn!("MouseSimulator not available - run with Accessibility permissions");
        }
    }

    pub fn sim_drag(&mut self, logical_x1: f64, logical_y1: f64, logical_x2: f64, logical_y2: f64) {
        if let (Some(sim), Some(converter)) = (&mut self.mouse_simulator, &self.position_converter) {
            let start = converter.logical_to_screen(logical_x1, logical_y1);
            let end = converter.logical_to_screen(logical_x2, logical_y2);
            let btn = SimMouseButton::Left;
            sim.drag(start, end, btn);
        } else {
            tracing::warn!("MouseSimulator not available - run with Accessibility permissions");
        }
    }

    pub fn sim_move_to(&mut self, logical_x: f64, logical_y: f64) {
        if let (Some(sim), Some(converter)) = (&mut self.mouse_simulator, &self.position_converter) {
            let screen_pos = converter.logical_to_screen(logical_x, logical_y);
            sim.move_to(screen_pos);
        } else {
            tracing::warn!("MouseSimulator not available - run with Accessibility permissions");
        }
    }

    pub fn sim_hover(&mut self, logical_x: f64, logical_y: f64, duration_ms: u64) {
        if let (Some(sim), Some(converter)) = (&mut self.mouse_simulator, &self.position_converter) {
            let screen_pos = converter.logical_to_screen(logical_x, logical_y);
            sim.hover(screen_pos, duration_ms);
        } else {
            tracing::warn!("MouseSimulator not available - run with Accessibility permissions");
        }
    }

    fn schedule_update_if_transitioned(&self, was_queued: bool) {
        if !was_queued && self.core.update_manager.is_update_queued() {
            self.scene_host.request_update();
        }
    }

    pub fn dispatch_raw_mouse_moved(&mut self, input: RawPointerInput) -> InteractionTrace {
        let was_queued = self.core.update_manager.is_update_queued();
        let trace = self.core.dispatch_raw_mouse_moved(input);
        self.schedule_update_if_transitioned(was_queued);
        trace
    }

    pub fn dispatch_raw_mouse_pressed(
        &mut self,
        input: RawPointerInput,
        button: MouseButton,
    ) -> InteractionTrace {
        let was_queued = self.core.update_manager.is_update_queued();
        let trace = self.core.dispatch_raw_mouse_pressed(input, button);
        self.schedule_update_if_transitioned(was_queued);
        trace
    }

    pub fn dispatch_raw_mouse_released(
        &mut self,
        input: RawPointerInput,
        button: MouseButton,
    ) -> InteractionTrace {
        let was_queued = self.core.update_manager.is_update_queued();
        let trace = self.core.dispatch_raw_mouse_released(input, button);
        self.schedule_update_if_transitioned(was_queued);
        trace
    }

    pub fn run_interaction_script(&mut self, steps: &[InteractionStep]) -> InteractionReport {
        let was_queued = self.core.update_manager.is_update_queued();
        let report = self.core.run_interaction_script(steps);
        self.schedule_update_if_transitioned(was_queued);
        report
    }

    pub fn dispatch_hover(&mut self, x: f64, y: f64, duration_ms: u64) {
        let was_queued = self.core.update_manager.is_update_queued();
        self.core.dispatch_hover(x, y, duration_ms);
        self.schedule_update_if_transitioned(was_queued);
    }

    pub fn dispatch_click(&mut self, x: f64, y: f64) {
        let was_queued = self.core.update_manager.is_update_queued();
        self.core.dispatch_click(x, y);
        self.schedule_update_if_transitioned(was_queued);
    }

    pub fn dispatch_mouse_moved(&mut self, x: f64, y: f64) {
        let was_queued = self.core.update_manager.is_update_queued();
        self.core.dispatch_mouse_moved(x, y);
        self.schedule_update_if_transitioned(was_queued);
    }

    pub fn dispatch_mouse_pressed(&mut self, x: f64, y: f64, button: MouseButton) {
        let was_queued = self.core.update_manager.is_update_queued();
        self.core.dispatch_mouse_pressed(x, y, button);
        self.schedule_update_if_transitioned(was_queued);
    }

    pub fn dispatch_mouse_released(&mut self, x: f64, y: f64, button: MouseButton) {
        let was_queued = self.core.update_manager.is_update_queued();
        self.core.dispatch_mouse_released(x, y, button);
        self.schedule_update_if_transitioned(was_queued);
    }

    pub fn request_update(&self) {
        self.scene_host.request_update();
    }

    pub fn set_use_iterative_render(&mut self, value: bool) {
        self.scene_host.set_use_iterative_render(value);
    }

    pub fn use_iterative_render(&self) -> bool {
        self.scene_host.use_iterative_render()
    }
}

impl NovadrawSystem for WinitNovadrawSystem {
    fn scene(&mut self) -> &mut novadraw::FigureGraph {
        &mut self.core.scene_manager.scene
    }

    fn update_manager(&mut self) -> &mut dyn UpdateManager {
        &mut self.core.update_manager
    }

    fn dispatcher(&mut self) -> &mut dyn EventDispatcher {
        &mut self.core.dispatcher
    }

    fn render(&mut self, renderer: &mut impl RenderBackend) -> NdCanvas {
        self.scene_host
            .execute_update(
                &mut self.core.scene_manager.scene,
                &mut self.core.update_manager,
                renderer,
            )
    }

    fn viewport_size(&self) -> (f64, f64) {
        self.scene_host.viewport_size()
    }

    fn request_update(&self) {
        self.scene_host.request_update();
    }
}

struct EditorDispatchContext<'a> {
    scene: &'a mut novadraw::FigureGraph,
    update_manager: &'a mut dyn UpdateManager,
}

impl<'a> EditorDispatchContext<'a> {
    fn new(
        scene: &'a mut novadraw::FigureGraph,
        update_manager: &'a mut dyn UpdateManager,
    ) -> Self {
        Self {
            scene,
            update_manager,
        }
    }
}

impl DispatchContext for EditorDispatchContext<'_> {
    fn find_mouse_event_target_at(&self, x: f64, y: f64) -> Option<novadraw::BlockId> {
        self.scene.find_mouse_event_target_at(x, y)
    }

    fn mouse_target(&self) -> Option<novadraw::BlockId> {
        self.scene.mouse_target()
    }

    fn set_mouse_target(&mut self, id: Option<novadraw::BlockId>) {
        self.scene.set_mouse_target(id);
    }

    fn focus_owner(&self) -> Option<novadraw::BlockId> {
        self.scene.focus_owner()
    }

    fn set_focus_owner(&mut self, id: Option<novadraw::BlockId>) {
        self.scene.set_focus_owner(id);
    }

    fn captured(&self) -> Option<novadraw::BlockId> {
        self.scene.captured()
    }

    fn set_captured(&mut self, id: Option<novadraw::BlockId>) {
        self.scene.set_captured(id);
    }

    fn dispatch_to_target(&mut self, target_id: Option<novadraw::BlockId>, event: &Event) -> bool {
        let Some(target_id) = target_id else {
            return false;
        };
        let scene = &mut self.scene;
        let update_manager = &mut self.update_manager;
        let Some(block) = scene.blocks.get(target_id) else {
            return false;
        };
        let bounds = block.figure_bounds();
        let mut ctx = EditorNovadrawContext::new(target_id, bounds, *update_manager);

        match event {
            Event::Mouse(mouse_event) => match mouse_event.kind {
                MouseEventKind::Pressed => block.figure.on_mouse_pressed(mouse_event, &mut ctx),
                MouseEventKind::Released => block.figure.on_mouse_released(mouse_event, &mut ctx),
                MouseEventKind::Moved => block.figure.on_mouse_moved(mouse_event, &mut ctx),
                MouseEventKind::Entered => block.figure.on_mouse_entered(mouse_event, &mut ctx),
                MouseEventKind::Exited => block.figure.on_mouse_exited(mouse_event, &mut ctx),
            },
        }
    }
}

struct EditorNovadrawContext<'a> {
    target_id: novadraw::BlockId,
    bounds: Rectangle,
    update_manager: &'a mut dyn UpdateManager,
}

impl<'a> EditorNovadrawContext<'a> {
    fn new(
        target_id: novadraw::BlockId,
        bounds: Rectangle,
        update_manager: &'a mut dyn UpdateManager,
    ) -> Self {
        Self {
            target_id,
            bounds,
            update_manager,
        }
    }
}

impl NovadrawContext for EditorNovadrawContext<'_> {
    fn target_id(&self) -> novadraw::BlockId {
        self.target_id
    }

    fn repaint(&mut self, rect: Option<Rectangle>) {
        self.update_manager
            .add_dirty_region(self.target_id, rect.unwrap_or(self.bounds));
    }

    fn invalidate(&mut self) {
        self.update_manager.add_invalid_figure(self.target_id);
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use novadraw::{
        command::{LineCap, LineJoin},
        Bounded, Color, FigureGraph, MouseEvent, NovadrawContext, Shape, Updatable,
    };

    use super::*;
    use crate::scene_manager::SceneType;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    struct TestFigureState {
        hovered: bool,
        pressed: bool,
        selected: bool,
    }

    struct TestInteractiveFigure {
        bounds: Rectangle,
        state: Arc<Mutex<TestFigureState>>,
    }

    impl TestInteractiveFigure {
        fn new(bounds: Rectangle, state: Arc<Mutex<TestFigureState>>) -> Self {
            Self { bounds, state }
        }
    }

    impl Bounded for TestInteractiveFigure {
        fn bounds(&self) -> Rectangle {
            self.bounds
        }

        fn set_bounds(&mut self, x: f64, y: f64, width: f64, height: f64) {
            self.bounds = Rectangle::new(x, y, width, height);
        }

        fn name(&self) -> &'static str {
            "TestInteractiveFigure"
        }
    }

    impl Updatable for TestInteractiveFigure {
        fn validate(&mut self) {}

        fn invalidate(&mut self) {}
    }

    impl Shape for TestInteractiveFigure {
        fn stroke_color(&self) -> Option<Color> {
            None
        }

        fn stroke_width(&self) -> f64 {
            0.0
        }

        fn fill_color(&self) -> Option<Color> {
            None
        }

        fn line_cap(&self) -> LineCap {
            LineCap::default()
        }

        fn line_join(&self) -> LineJoin {
            LineJoin::default()
        }

        fn fill_shape(&self, _gc: &mut NdCanvas) {}

        fn outline_shape(&self, _gc: &mut NdCanvas) {}

        fn wants_mouse_events(&self) -> bool {
            true
        }

        fn on_mouse_pressed(&self, _event: &MouseEvent, _ctx: &mut dyn NovadrawContext) -> bool {
            let mut state = self.state.lock().unwrap();
            state.pressed = true;
            true
        }

        fn on_mouse_released(&self, _event: &MouseEvent, _ctx: &mut dyn NovadrawContext) -> bool {
            let mut state = self.state.lock().unwrap();
            state.pressed = false;
            state.selected = true;
            true
        }

        fn on_mouse_entered(&self, _event: &MouseEvent, _ctx: &mut dyn NovadrawContext) -> bool {
            let mut state = self.state.lock().unwrap();
            state.hovered = true;
            true
        }

        fn on_mouse_exited(&self, _event: &MouseEvent, _ctx: &mut dyn NovadrawContext) -> bool {
            let mut state = self.state.lock().unwrap();
            state.hovered = false;
            state.pressed = false;
            true
        }
    }

    fn build_test_core() -> (EditorInteractionCore, Arc<Mutex<TestFigureState>>, BlockId) {
        let state = Arc::new(Mutex::new(TestFigureState::default()));
        let mut scene = FigureGraph::new();
        let root_id = scene.set_contents(Box::new(novadraw::RectangleFigure::new_with_color(
            0.0,
            0.0,
            400.0,
            300.0,
            Color::rgba(0.0, 0.0, 0.0, 0.0),
        )));
        let target_id = scene.add_child_to(
            root_id,
            Box::new(TestInteractiveFigure::new(
                Rectangle::new(100.0, 100.0, 100.0, 100.0),
                Arc::clone(&state),
            )),
        );
        let mut core = EditorInteractionCore::new();
        core.scene_manager = SceneManager {
            scene,
            current_scene: SceneType::DpiTest,
        };
        (core, state, target_id)
    }

    #[test]
    fn test_raw_pointer_conversion() {
        let logical = EditorInteractionCore::logical_from_raw(RawPointerInput::new(300.0, 200.0, 2.0));
        assert_eq!(logical, LogicalPointerPosition { x: 150.0, y: 100.0 });
    }

    #[test]
    fn test_hover_script_hits_expected_target() {
        let (mut core, state, target_id) = build_test_core();
        let report = core.run_interaction_script(&[InteractionStep::Hover {
            input: RawPointerInput::new(300.0, 300.0, 2.0),
            duration_ms: 0,
        }]);

        assert_eq!(report.traces.len(), 1);
        assert_eq!(report.traces[0].logical, LogicalPointerPosition { x: 150.0, y: 150.0 });
        assert_eq!(report.traces[0].hit_target_before, Some(target_id));
        assert_eq!(report.traces[0].mouse_target_after, Some(target_id));
        assert_eq!(*state.lock().unwrap(), TestFigureState { hovered: true, pressed: false, selected: false });
    }

    #[test]
    fn test_click_script_updates_figure_state() {
        let (mut core, state, target_id) = build_test_core();
        let report = core.run_interaction_script(&[InteractionStep::Click {
            input: RawPointerInput::new(300.0, 300.0, 2.0),
            button: MouseButton::Left,
        }]);

        assert_eq!(report.traces.len(), 2);
        assert_eq!(report.traces[0].hit_target_before, Some(target_id));
        assert_eq!(report.traces[1].hit_target_before, Some(target_id));
        assert_eq!(report.traces[1].mouse_target_after, Some(target_id));
        assert_eq!(*state.lock().unwrap(), TestFigureState { hovered: true, pressed: false, selected: true });
    }

    #[test]
    fn test_release_after_dragging_outside_still_reaches_pressed_target() {
        let (mut core, state, target_id) = build_test_core();
        let report = core.run_interaction_script(&[
            InteractionStep::Move(RawPointerInput::new(300.0, 300.0, 2.0)),
            InteractionStep::Press {
                input: RawPointerInput::new(300.0, 300.0, 2.0),
                button: MouseButton::Left,
            },
            InteractionStep::Move(RawPointerInput::new(20.0, 20.0, 2.0)),
            InteractionStep::Release {
                input: RawPointerInput::new(20.0, 20.0, 2.0),
                button: MouseButton::Left,
            },
        ]);

        assert_eq!(report.traces.len(), 4);
        assert_eq!(report.traces[1].captured_after, Some(target_id));
        assert_eq!(report.traces[2].mouse_target_after, Some(target_id));
        assert_eq!(report.traces[3].hit_target_before, None);
        assert_eq!(report.traces[3].mouse_target_before, Some(target_id));
        assert_eq!(report.traces[3].captured_after, None);
        assert_eq!(report.traces[3].mouse_target_after, None);
        assert_eq!(
            *state.lock().unwrap(),
            TestFigureState {
                hovered: false,
                pressed: false,
                selected: true,
            }
        );
    }
}
