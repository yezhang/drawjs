//! 视口管理
//!
//! 提供 viewport 坐标域与 content 坐标域之间的变换。
//!
//! 这里的 `content` 不是 Figure 树外的统一全局空间，而是某个 viewport
//! 管理的内容坐标域。未来如果 Viewport 作为 Figure 节点接入树结构，应通过
//! `translate_to_parent` / `translate_from_parent` 协议加入父链，而不是在事件或渲染入口
//! 额外添加全局空间特判。

use glam::DVec2;
use novadraw_geometry::Transform;

/// 视口
///
/// 管理 content 坐标域的可见区域，支持平移和缩放。
///
/// `origin` 表示 viewport 左上角对应的 content 坐标。
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

    /// viewport 坐标转 content 坐标。
    ///
    /// 对齐 draw2d `Viewport.translateFromParent()` 的方向：从父/viewport 坐标进入内容坐标。
    pub fn viewport_to_content(&self, point: DVec2) -> DVec2 {
        (point / self.zoom) + self.origin
    }

    /// content 坐标转 viewport 坐标。
    ///
    /// 对齐 draw2d `Viewport.translateToParent()` 的方向：从内容坐标回到父/viewport 坐标。
    pub fn content_to_viewport(&self, point: DVec2) -> DVec2 {
        (point - self.origin) * self.zoom
    }

    /// 将点从内容坐标转换到父/viewport 坐标。
    pub fn translate_to_parent(&self, point: &mut DVec2) {
        *point = self.content_to_viewport(*point);
    }

    /// 将点从父/viewport 坐标转换到内容坐标。
    pub fn translate_from_parent(&self, point: &mut DVec2) {
        *point = self.viewport_to_content(*point);
    }

    /// 平移
    pub fn pan(&mut self, dx: f64, dy: f64) {
        self.origin -= DVec2::new(dx, dy) / self.zoom;
    }

    /// 以指定中心点缩放
    pub fn zoom_at(&mut self, factor: f64, center: DVec2) {
        let content_center_before = self.viewport_to_content(center);
        self.zoom *= factor;
        let content_center_after = self.viewport_to_content(center);
        let offset = content_center_before - content_center_after;
        self.origin += offset;
    }

    /// 缩放以适应矩形
    pub fn zoom_to_fit(
        &mut self,
        rect: &crate::Rectangle,
        viewport_width: f64,
        viewport_height: f64,
        padding: f64,
    ) {
        if rect.width <= 0.0 || rect.height <= 0.0 {
            return;
        }
        let scale_x = (viewport_width - padding * 2.0) / rect.width;
        let scale_y = (viewport_height - padding * 2.0) / rect.height;
        self.zoom = scale_x.min(scale_y);
        self.origin = DVec2::new(rect.x - padding / self.zoom, rect.y - padding / self.zoom);
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
    /// 变换公式: viewport = (content - origin) * zoom
    /// 即: 先平移 `-origin`，再缩放
    /// 使用 `*` 运算符：T(translate) * S(scale) = 先 S，后 T
    pub fn to_transform(&self) -> Transform {
        let scale = Transform::from_scale(self.zoom, self.zoom);
        let translate = Transform::from_translation(-self.origin.x, -self.origin.y);
        scale * translate // S * T = 先平移 origin，后缩放
    }

    /// 转换为逆变换
    ///
    /// 逆变换公式: content = viewport / zoom + origin
    pub fn to_inverse_transform(&self) -> Transform {
        let inv_zoom = 1.0 / self.zoom;
        let scale = Transform::from_scale(inv_zoom, inv_zoom);
        let translate = Transform::from_translation(self.origin.x, self.origin.y);
        translate * scale // T * S = 先缩放回 content 增量，后加 origin
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
    fn test_viewport_content_conversion() {
        let viewport = Viewport::new().with_origin(100.0, 200.0).with_zoom(2.0);
        let content = DVec2::new(150.0, 250.0);
        let viewport_point = viewport.content_to_viewport(content);
        // viewport = (content - origin) * zoom
        // zoom=2, origin=(100, 200), content=(150, 250)
        // viewport = (150-100, 250-200) * 2 = (100, 100)
        assert_eq!(viewport_point, DVec2::new(100.0, 100.0));
        let back = viewport.viewport_to_content(viewport_point);
        assert_eq!(back, content);
    }

    #[test]
    fn test_translate_parent_protocol() {
        let viewport = Viewport::new().with_origin(100.0, 200.0).with_zoom(2.0);

        let mut point = DVec2::new(150.0, 250.0);
        viewport.translate_to_parent(&mut point);
        assert_eq!(point, DVec2::new(100.0, 100.0));

        viewport.translate_from_parent(&mut point);
        assert_eq!(point, DVec2::new(150.0, 250.0));
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
        let transformed = transform.transform_point(point.x, point.y);
        assert!((transformed.0 - point.x).abs() < 1e-10);
        assert!((transformed.1 - point.y).abs() < 1e-10);
    }

    #[test]
    fn test_to_transform_scale() {
        let viewport = Viewport::new().with_origin(0.0, 0.0).with_zoom(2.0);
        let transform = viewport.to_transform();
        let point = glam::DVec2::new(100.0, 200.0);
        let transformed = transform.transform_point(point.x, point.y);
        // viewport = (content - origin) * zoom = (100-0, 200-0) * 2 = (200, 400)
        assert_eq!(transformed.0, 200.0);
        assert_eq!(transformed.1, 400.0);
    }

    #[test]
    fn test_to_transform_with_non_zero_origin() {
        let viewport = Viewport::new().with_origin(100.0, 200.0).with_zoom(2.0);
        let transform = viewport.to_transform();
        let inverse = viewport.to_inverse_transform();

        let content = DVec2::new(150.0, 250.0);
        let transformed = transform.transform_point(content.x, content.y);
        assert_eq!(transformed, (100.0, 100.0));

        let restored = inverse.transform_point(transformed.0, transformed.1);
        assert_eq!(restored, (150.0, 250.0));
    }
}
