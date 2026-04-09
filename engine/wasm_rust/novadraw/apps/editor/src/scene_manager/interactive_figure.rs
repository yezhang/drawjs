use std::sync::Mutex;

use novadraw::{
    Bounded, Color, MouseEvent, NovadrawContext, Rectangle, Shape, Updatable,
    command::{LineCap, LineJoin},
};
use novadraw::NdCanvas;
use tracing::info;

struct InteractiveState {
    hovered: bool,
    pressed: bool,
    selected: bool,
}

pub struct InteractiveRectFigure {
    bounds: Rectangle,
    normal_fill: Color,
    hover_fill: Color,
    pressed_fill: Color,
    normal_stroke: Color,
    selected_stroke: Color,
    state: Mutex<InteractiveState>,
}

impl InteractiveRectFigure {
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self::with_palette(
            x,
            y,
            width,
            height,
            Color::rgba(0.20, 0.47, 0.86, 1.0),
            Color::rgba(0.18, 0.72, 0.64, 1.0),
            Color::rgba(0.95, 0.56, 0.18, 1.0),
            Color::rgba(0.10, 0.16, 0.24, 1.0),
            Color::rgba(0.98, 0.86, 0.22, 1.0),
        )
    }

    pub fn with_palette(
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        normal_fill: Color,
        hover_fill: Color,
        pressed_fill: Color,
        normal_stroke: Color,
        selected_stroke: Color,
    ) -> Self {
        Self {
            bounds: Rectangle::new(x, y, width, height),
            normal_fill,
            hover_fill,
            pressed_fill,
            normal_stroke,
            selected_stroke,
            state: Mutex::new(InteractiveState {
                hovered: false,
                pressed: false,
                selected: false,
            }),
        }
    }

    fn fill(&self) -> Color {
        let state = self.state.lock().unwrap();
        if state.pressed {
            self.pressed_fill
        } else if state.hovered {
            self.hover_fill
        } else {
            self.normal_fill
        }
    }

    fn stroke(&self) -> Color {
        let state = self.state.lock().unwrap();
        if state.selected {
            self.selected_stroke
        } else {
            self.normal_stroke
        }
    }
}

impl Bounded for InteractiveRectFigure {
    fn bounds(&self) -> Rectangle {
        self.bounds
    }

    fn set_bounds(&mut self, x: f64, y: f64, width: f64, height: f64) {
        self.bounds = Rectangle::new(x, y, width, height);
    }

    fn name(&self) -> &'static str {
        "InteractiveRectFigure"
    }
}

impl Updatable for InteractiveRectFigure {
    fn validate(&mut self) {}

    fn invalidate(&mut self) {}
}

impl Shape for InteractiveRectFigure {
    fn stroke_color(&self) -> Option<Color> {
        Some(self.stroke())
    }

    fn stroke_width(&self) -> f64 {
        4.0
    }

    fn fill_color(&self) -> Option<Color> {
        Some(self.fill())
    }

    fn line_cap(&self) -> LineCap {
        LineCap::default()
    }

    fn line_join(&self) -> LineJoin {
        LineJoin::default()
    }

    fn fill_shape(&self, gc: &mut NdCanvas) {
        let bounds = self.bounds;
        gc.fill_rect(
            bounds.x,
            bounds.y,
            bounds.width,
            bounds.height,
            self.fill(),
        );
    }

    fn outline_shape(&self, gc: &mut NdCanvas) {
        let bounds = self.bounds;
        gc.stroke_rect(
            bounds.x + 2.0,
            bounds.y + 2.0,
            (bounds.width - 4.0).max(0.0),
            (bounds.height - 4.0).max(0.0),
            self.stroke(),
            4.0,
            LineCap::default(),
            LineJoin::default(),
        );
    }

    fn wants_mouse_events(&self) -> bool {
        true
    }

    fn on_mouse_pressed(&self, _event: &MouseEvent, ctx: &mut dyn NovadrawContext) -> bool {
        let mut state = self.state.lock().unwrap();
        state.pressed = true;
        drop(state);
        ctx.repaint(None);
        true
    }

    fn on_mouse_released(&self, _event: &MouseEvent, ctx: &mut dyn NovadrawContext) -> bool {
        let mut state = self.state.lock().unwrap();
        state.pressed = false;
        state.selected = !state.selected;
        drop(state);
        ctx.repaint(None);
        true
    }

    fn on_mouse_entered(&self, _event: &MouseEvent, ctx: &mut dyn NovadrawContext) -> bool {
        let mut state = self.state.lock().unwrap();
        state.hovered = true;
        drop(state);
        info!("interactive_rect entered: target={:?}, bounds={:?}", ctx.target_id(), self.bounds);
        ctx.repaint(None);
        true
    }

    fn on_mouse_exited(&self, _event: &MouseEvent, ctx: &mut dyn NovadrawContext) -> bool {
        let mut state = self.state.lock().unwrap();
        state.hovered = false;
        state.pressed = false;
        drop(state);
        info!("interactive_rect exited: target={:?}, bounds={:?}", ctx.target_id(), self.bounds);
        ctx.repaint(None);
        true
    }
}
