use crate::core::transform::Transform;
use glam::DVec2;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Viewport {
    pub origin: DVec2,
    pub zoom: f64,
}

impl Viewport {
    pub fn new() -> Self {
        Self {
            origin: DVec2::ZERO,
            zoom: 1.0,
        }
    }

    pub fn with_origin(mut self, x: f64, y: f64) -> Self {
        self.origin = DVec2::new(x, y);
        self
    }

    pub fn with_zoom(mut self, zoom: f64) -> Self {
        self.zoom = zoom;
        self
    }

    pub fn screen_to_world(&self, screen: DVec2) -> DVec2 {
        (screen / self.zoom) + self.origin
    }

    pub fn world_to_screen(&self, world: DVec2) -> DVec2 {
        (world - self.origin) * self.zoom
    }

    pub fn pan(&mut self, dx: f64, dy: f64) {
        self.origin -= DVec2::new(dx, dy) / self.zoom;
    }

    pub fn zoom_at(&mut self, factor: f64, center: DVec2) {
        let world_center_before = self.screen_to_world(center);
        self.zoom *= factor;
        let world_center_after = self.screen_to_world(center);
        let offset = world_center_before - world_center_after;
        self.origin += offset;
    }

    pub fn zoom_to_fit(&mut self, rect: &super::scene::Rect, viewport_width: f64, viewport_height: f64, padding: f64) {
        if rect.width <= 0.0 || rect.height <= 0.0 {
            return;
        }
        let scale_x = (viewport_width - padding * 2.0) / rect.width;
        let scale_y = (viewport_height - padding * 2.0) / rect.height;
        self.zoom = scale_x.min(scale_y);
        self.origin = DVec2::new(
            rect.x - padding / self.zoom,
            rect.y - padding / self.zoom,
        );
    }

    pub fn zoom_in(&mut self, factor: f64) {
        self.zoom *= factor;
    }

    pub fn zoom_out(&mut self, factor: f64) {
        self.zoom /= factor;
    }

    pub fn set_origin(&mut self, x: f64, y: f64) {
        self.origin = DVec2::new(x, y);
    }

    pub fn set_zoom(&mut self, zoom: f64) {
        self.zoom = zoom;
    }

    pub fn to_transform(&self) -> Transform {
        let scale = Transform::from_scale_2d(self.zoom, self.zoom);
        let translation = Transform::from_translation_2d(self.origin.x, self.origin.y);
        scale.compose(&translation)
    }

    pub fn to_inverse_transform(&self) -> Transform {
        let inv_zoom = 1.0 / self.zoom;
        Transform::from_scale_2d(inv_zoom, inv_zoom)
    }
}

impl Default for Viewport {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_screen_world_conversion() {
        let viewport = Viewport::new().with_origin(100.0, 200.0).with_zoom(2.0);
        let world = DVec2::new(150.0, 250.0);
        let screen = viewport.world_to_screen(world);
        assert_eq!(screen, DVec2::new(100.0, 100.0));
        let back = viewport.screen_to_world(screen);
        assert_eq!(back, world);
    }

    #[test]
    fn test_pan() {
        let mut viewport = Viewport::new().with_origin(100.0, 100.0).with_zoom(2.0);
        viewport.pan(100.0, 100.0);
        assert_eq!(viewport.origin, DVec2::new(50.0, 50.0));
    }

    #[test]
    fn test_zoom_at() {
        let mut viewport = Viewport::new().with_origin(0.0, 0.0).with_zoom(1.0);
        viewport.zoom_at(2.0, DVec2::new(100.0, 100.0));
        assert_eq!(viewport.zoom, 2.0);
    }

    #[test]
    fn test_zoom_in_out() {
        let mut viewport = Viewport::new().with_zoom(1.0);
        viewport.zoom_in(2.0);
        assert_eq!(viewport.zoom, 2.0);
        viewport.zoom_out(2.0);
        assert_eq!(viewport.zoom, 1.0);
    }

    #[test]
    fn test_to_transform_identity() {
        let viewport = Viewport::new().with_origin(0.0, 0.0).with_zoom(1.0);
        let transform = viewport.to_transform();
        let point = glam::DVec2::new(100.0, 200.0);
        let transformed = transform.multiply_point_2d(point);
        assert!((transformed.x - point.x).abs() < 1e-10);
        assert!((transformed.y - point.y).abs() < 1e-10);
    }

    #[test]
    fn test_to_transform_scale() {
        let viewport = Viewport::new().with_zoom(2.0);
        let transform = viewport.to_transform();
        let point = glam::DVec2::new(100.0, 200.0);
        let transformed = transform.multiply_point_2d(point);
        assert_eq!(transformed.x, 200.0);
        assert_eq!(transformed.y, 400.0);
    }
}
