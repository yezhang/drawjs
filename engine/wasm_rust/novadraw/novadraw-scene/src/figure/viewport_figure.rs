//! 视口 Figure
//!
//! 参考 Eclipse Draw2D Viewport 设计，实现可滚动的视口容器。

use glam::DVec2;
use novadraw_render::NdCanvas;

use super::{Figure, FigureRendererRef};
use crate::scene::{BlockId, PaintTask, Rect};

/// 视口 Figure
///
/// 作为场景的根容器，标识这是一个视口。
/// 在 paint_figure() 中应用平移和缩放变换。
///
/// # 原理
///
/// - is_viewport_container() 返回 true，标识这是一个视口容器
/// - paint_figure() 应用 translate(-origin) 和 scale(zoom)
/// - 子元素在此变换下绘制
///
/// # 使用方式
///
/// ```
/// use novadraw_scene::figure::{Figure, ViewportFigure};
///
/// let mut viewport = ViewportFigure::new();
/// ```
#[derive(Clone, Debug)]
pub struct ViewportFigure {
    origin: DVec2,
    zoom: f64,
}

impl ViewportFigure {
    /// 创建新视口
    pub fn new() -> Self {
        Self {
            origin: DVec2::ZERO,
            zoom: 1.0,
        }
    }

    /// 设置原点
    pub fn set_origin(&mut self, x: f64, y: f64) {
        self.origin = DVec2::new(x, y);
    }

    /// 获取原点
    pub fn origin(&self) -> DVec2 {
        self.origin
    }

    /// 设置缩放
    pub fn set_zoom(&mut self, zoom: f64) {
        self.zoom = zoom.max(0.01);
    }

    /// 获取缩放
    pub fn zoom(&self) -> f64 {
        self.zoom
    }

    /// 平移（屏幕坐标增量）
    pub fn pan(&mut self, dx: f64, dy: f64) {
        self.origin.x -= dx / self.zoom;
        self.origin.y -= dy / self.zoom;
    }

    /// 以屏幕点为中心缩放
    pub fn zoom_at(&mut self, factor: f64, screen_center: DVec2) {
        let world_before = self.screen_to_world(screen_center);
        self.zoom *= factor;
        let world_after = self.screen_to_world(screen_center);
        self.origin += world_before - world_after;
    }

    /// 屏幕转世界坐标
    pub fn screen_to_world(&self, screen: DVec2) -> DVec2 {
        (screen / self.zoom) + self.origin
    }

    /// 世界转屏幕坐标
    pub fn world_to_screen(&self, world: DVec2) -> DVec2 {
        (world - self.origin) * self.zoom
    }
}

impl Default for ViewportFigure {
    fn default() -> Self {
        Self::new()
    }
}

impl Figure for ViewportFigure {
    fn bounds(&self) -> Rect {
        Rect::new(0.0, 0.0, 0.0, 0.0)
    }

    fn paint(&self, _gc: &mut NdCanvas) {}

    fn paint_figure(&self, gc: &mut NdCanvas) {
        // 对应 Draw2D Viewport.paintClientArea():
        gc.save();
        gc.translate(-self.origin.x, -self.origin.y);
        gc.scale(self.zoom, self.zoom);
        // 子元素由渲染循环在此 save/restore 内绘制
        // 注意：restore() 由渲染循环处理
    }

    fn paint_border(&self, _gc: &mut NdCanvas) {}

    fn paint_highlight(&self, _gc: &mut NdCanvas) {}

    fn is_viewport_container(&self) -> bool {
        true
    }

    fn name(&self) -> &'static str {
        "ViewportFigure"
    }

    fn as_viewport_figure_mut(&mut self) -> Option<&mut super::ViewportFigure> {
        Some(self)
    }

    fn paint_client_area(
        &self,
        renderer: &FigureRendererRef<'_>,
        block_id: BlockId,
    ) -> Vec<PaintTask> {
        let mut tasks = Vec::new();
        let _block = match renderer.block(block_id) {
            Some(b) if b.is_visible => b,
            _ => return tasks,
        };

        tasks.push(PaintTask::Save);

        tasks.push(PaintTask::Translate {
            x: -self.origin.x,
            y: -self.origin.y,
        });

        tasks.push(PaintTask::Scale {
            x: self.zoom,
            y: self.zoom,
        });

        tasks.extend(self.paint_children(renderer, block_id));

        tasks.push(PaintTask::Restore);

        tasks
    }
}
