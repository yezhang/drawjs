//! 视口管理
//!
//! 提供视口变换和坐标转换功能。

use glam::DVec2;
use novadraw_math::Transform;

/// 视口
///
/// 管理世界的可见区域，支持平移和缩放。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Viewport {
    pub origin: DVec2,
    pub zoom: f64,
}

impl Viewport {
    /// 创建新视口
    pub fn new() -> Self {
        Self {
            origin: DVec2::ZERO,
            zoom: 1.0,
        }
    }

    /// 设置原点
    pub fn with_origin(mut self, x: f64, y: f64) -> Self {
        self.origin = DVec2::new(x, y);
        self
    }

    /// 设置缩放
    pub fn with_zoom(mut self, zoom: f64) -> Self {
        self.zoom = zoom;
        self
    }

    /// 屏幕坐标转世界坐标
    pub fn screen_to_world(&self, screen: DVec2) -> DVec2 {
        let world = (screen / self.zoom) + self.origin;
        world
    }

    /// 世界坐标转屏幕坐标
    pub fn world_to_screen(&self, world: DVec2) -> DVec2 {
        (world - self.origin) * self.zoom
    }

    /// 平移
    pub fn pan(&mut self, dx: f64, dy: f64) {
        self.origin -= DVec2::new(dx, dy) / self.zoom;
    }

    /// 以指定中心点缩放
    pub fn zoom_at(&mut self, factor: f64, center: DVec2) {
        let world_center_before = self.screen_to_world(center);
        self.zoom *= factor;
        let world_center_after = self.screen_to_world(center);
        let offset = world_center_before - world_center_after;
        self.origin += offset;
    }

    /// 缩放以适应矩形
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

    /// 放大
    pub fn zoom_in(&mut self, factor: f64) {
        self.zoom *= factor;
    }

    /// 缩小
    pub fn zoom_out(&mut self, factor: f64) {
        self.zoom /= factor;
    }

    /// 设置原点
    pub fn set_origin(&mut self, x: f64, y: f64) {
        self.origin = DVec2::new(x, y);
    }

    /// 设置缩放
    pub fn set_zoom(&mut self, zoom: f64) {
        self.zoom = zoom;
    }

    /// 转换为变换矩阵
    ///
    /// 变换公式: screen = (world - origin) * zoom
    /// 即: 先平移 origin，再缩放
    /// 使用 `*` 运算符：T(translate) * S(scale) = 先 S，后 T
    pub fn to_transform(&self) -> Transform {
        let scale = Transform::from_scale(self.zoom, self.zoom);
        let translate = Transform::from_translation(
            -self.origin.x * self.zoom,
            -self.origin.y * self.zoom,
        );
        scale * translate  // S * T = 先平移 origin，后缩放
    }

    /// 转换为逆变换
    ///
    /// 逆变换公式: world = screen / zoom + origin
    pub fn to_inverse_transform(&self) -> Transform {
        let inv_zoom = 1.0 / self.zoom;
        let scale = Transform::from_scale(inv_zoom, inv_zoom);
        let translate = Transform::from_translation(
            self.origin.x,
            self.origin.y,
        );
        scale * translate  // S * T = 先平移 origin，后缩放
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

    #[inline]
    fn dvec2_to_vec2(v: DVec2) -> novadraw_math::Vec2 {
        novadraw_math::Vec2::new(v.x, v.y)
    }

    #[inline]
    fn vec2_to_dvec2(v: novadraw_math::Vec2) -> DVec2 {
        DVec2::new(v.x(), v.y())
    }

    #[test]
    fn test_screen_world_conversion() {
        let viewport = Viewport::new().with_origin(100.0, 200.0).with_zoom(2.0);
        let world = DVec2::new(150.0, 250.0);
        let screen = viewport.world_to_screen(world);
        // screen = (world - origin) * zoom
        // zoom=2, origin=(100, 200), world=(150, 250)
        // screen = (150-100, 250-200) * 2 = (100, 100)
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
        let transformed = vec2_to_dvec2(transform.transform_point(dvec2_to_vec2(point)));
        assert!((transformed.x - point.x).abs() < 1e-10);
        assert!((transformed.y - point.y).abs() < 1e-10);
    }

    #[test]
    fn test_to_transform_scale() {
        let viewport = Viewport::new().with_origin(0.0, 0.0).with_zoom(2.0);
        let transform = viewport.to_transform();
        let point = glam::DVec2::new(100.0, 200.0);
        let transformed = vec2_to_dvec2(transform.transform_point(dvec2_to_vec2(point)));
        // screen = (world - origin) * zoom = (100-0, 200-0) * 2 = (200, 400)
        assert_eq!(transformed.x, 200.0);
        assert_eq!(transformed.y, 400.0);
    }
}
