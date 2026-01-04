use glam::{DMat4, DVec2, DVec3};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Transform {
    matrix: DMat4,
}

impl Transform {
    pub fn identity() -> Self {
        Self {
            matrix: DMat4::IDENTITY,
        }
    }

    pub fn from_translation_2d(tx: f64, ty: f64) -> Self {
        Self {
            matrix: DMat4::from_translation(DVec3::new(tx, ty, 0.0)),
        }
    }

    pub fn from_translation_3d(tx: f64, ty: f64, tz: f64) -> Self {
        Self {
            matrix: DMat4::from_translation(DVec3::new(tx, ty, tz)),
        }
    }

    pub fn from_scale_2d(sx: f64, sy: f64) -> Self {
        Self {
            matrix: DMat4::from_scale(DVec3::new(sx, sy, 1.0)),
        }
    }

    pub fn from_scale_3d(sx: f64, sy: f64, sz: f64) -> Self {
        Self {
            matrix: DMat4::from_scale(DVec3::new(sx, sy, sz)),
        }
    }

    pub fn from_scale_uniform(s: f64) -> Self {
        Self::from_scale_2d(s, s)
    }

    pub fn from_rotation_z(angle: f64) -> Self {
        Self {
            matrix: DMat4::from_rotation_z(angle),
        }
    }

    pub fn from_rotation_x(angle: f64) -> Self {
        Self {
            matrix: DMat4::from_rotation_x(angle),
        }
    }

    pub fn from_rotation_y(angle: f64) -> Self {
        Self {
            matrix: DMat4::from_rotation_y(angle),
        }
    }

    pub fn from_rotation_2d_around(angle: f64, center: DVec2) -> Self {
        let translate_to_origin = Self::from_translation_2d(-center.x, -center.y);
        let rotate = Self::from_rotation_z(angle);
        let translate_back = Self::from_translation_2d(center.x, center.y);
        translate_to_origin * rotate * translate_back
    }

    pub fn from_scale_2d_around(sx: f64, sy: f64, center: DVec2) -> Self {
        let translate_to_origin = Self::from_translation_2d(-center.x, -center.y);
        let scale = Self::from_scale_2d(sx, sy);
        let translate_back = Self::from_translation_2d(center.x, center.y);
        translate_to_origin * scale * translate_back
    }

    pub fn compose(&self, other: &Self) -> Self {
        Self {
            matrix: other.matrix * self.matrix,
        }
    }

    pub fn multiply_point_2d(&self, point: DVec2) -> DVec2 {
        let transformed = self.matrix.mul_vec4(DVec3::new(point.x, point.y, 1.0).extend(1.0));
        DVec2::new(transformed.x, transformed.y)
    }

    pub fn multiply_point_3d(&self, point: DVec3) -> DVec3 {
        self.matrix.mul_vec4(point.extend(1.0)).truncate()
    }

    pub fn multiply_vector_2d(&self, vector: DVec2) -> DVec2 {
        let transformed = self.matrix.mul_vec4(DVec3::new(vector.x, vector.y, 0.0).extend(0.0));
        DVec2::new(transformed.x, transformed.y)
    }

    pub fn multiply_vector_3d(&self, vector: DVec3) -> DVec3 {
        self.matrix.mul_vec4(vector.extend(0.0)).truncate()
    }

    pub fn inverse(&self) -> Self {
        Self {
            matrix: self.matrix.inverse(),
        }
    }

    pub fn translation_2d(&self) -> DVec2 {
        let t = self.matrix.w_axis.truncate();
        DVec2::new(t.x, t.y)
    }

    pub fn translation_3d(&self) -> DVec3 {
        self.matrix.w_axis.truncate()
    }

    pub fn scale_2d(&self) -> DVec2 {
        DVec2::new(
            self.matrix.x_axis.length(),
            self.matrix.y_axis.length(),
        )
    }

    pub fn scale_3d(&self) -> DVec3 {
        DVec3::new(
            self.matrix.x_axis.length(),
            self.matrix.y_axis.length(),
            self.matrix.z_axis.length(),
        )
    }

    pub fn to_mat4(&self) -> DMat4 {
        self.matrix
    }

    pub fn to_array(&self) -> [[f64; 4]; 4] {
        self.matrix.to_cols_array_2d()
    }
}

impl std::ops::Mul for Transform {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        self.compose(&other)
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self::identity()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TransformStack {
    stack: Vec<Transform>,
}

impl TransformStack {
    pub fn new() -> Self {
        Self {
            stack: vec![Transform::identity()],
        }
    }

    pub fn push(&mut self, transform: Transform) {
        let current = self.current();
        self.stack.push(current.compose(&transform));
    }

    pub fn pop(&mut self) -> Option<Transform> {
        if self.stack.len() > 1 {
            Some(self.stack.pop().unwrap())
        } else {
            None
        }
    }

    pub fn current(&self) -> Transform {
        self.stack.last().copied().unwrap_or(Transform::identity())
    }

    pub fn reset(&mut self) {
        self.stack.clear();
        self.stack.push(Transform::identity());
    }

    pub fn depth(&self) -> usize {
        self.stack.len()
    }
}

impl Default for TransformStack {
    fn default() -> Self {
        Self::new()
    }
}

pub fn screen_to_world_2d(screen: DVec2, viewport_origin: DVec2, zoom: f64) -> DVec2 {
    (screen / zoom) + viewport_origin
}

pub fn world_to_screen_2d(world: DVec2, viewport_origin: DVec2, zoom: f64) -> DVec2 {
    (world - viewport_origin) * zoom
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identity() {
        let t = Transform::identity();
        let point = DVec2::new(10.0, 20.0);
        assert_eq!(t.multiply_point_2d(point), point);
    }

    #[test]
    fn test_translation_2d() {
        let t = Transform::from_translation_2d(100.0, 200.0);
        let point = DVec2::new(10.0, 20.0);
        assert_eq!(t.multiply_point_2d(point), DVec2::new(110.0, 220.0));
    }

    #[test]
    fn test_scale_2d() {
        let t = Transform::from_scale_2d(2.0, 3.0);
        let point = DVec2::new(10.0, 20.0);
        assert_eq!(t.multiply_point_2d(point), DVec2::new(20.0, 60.0));
    }

    #[test]
    fn test_inverse() {
        let t = Transform::from_translation_2d(100.0, 200.0);
        let inverse = t.inverse();
        let point = DVec2::new(10.0, 20.0);
        let translated = t.multiply_point_2d(point);
        let back = inverse.multiply_point_2d(translated);
        assert!((back.x - point.x).abs() < 1e-10);
        assert!((back.y - point.y).abs() < 1e-10);
    }

    #[test]
    fn test_composition() {
        let t1 = Transform::from_translation_2d(100.0, 0.0);
        let t2 = Transform::from_scale_2d(2.0, 2.0);
        let composed = t1 * t2;
        let point = DVec2::new(10.0, 10.0);
        assert_eq!(composed.multiply_point_2d(point), DVec2::new(220.0, 20.0));
    }

    #[test]
    fn test_rotation_z() {
        let t = Transform::from_rotation_z(std::f64::consts::PI / 2.0);
        let point = DVec2::new(1.0, 0.0);
        let rotated = t.multiply_point_2d(point);
        assert!((rotated.x).abs() < 1e-10);
        assert!((rotated.y - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_screen_world_conversion() {
        let viewport_origin = DVec2::new(100.0, 200.0);
        let zoom = 2.0;
        let world = DVec2::new(150.0, 250.0);
        let screen = world_to_screen_2d(world, viewport_origin, zoom);
        assert_eq!(screen, DVec2::new(100.0, 100.0));
        let back = screen_to_world_2d(screen, viewport_origin, zoom);
        assert_eq!(back, world);
    }

    #[test]
    fn test_rotation_2d_around() {
        let center = DVec2::new(100.0, 100.0);
        let t = Transform::from_rotation_2d_around(std::f64::consts::PI / 2.0, center);
        let point = DVec2::new(100.0, 200.0);
        let rotated = t.multiply_point_2d(point);
        let expected = DVec2::new(0.0, 100.0);
        assert!((rotated - expected).length() < 1e-10);
    }

    #[test]
    fn test_scale_2d_around() {
        let center = DVec2::new(100.0, 100.0);
        let t = Transform::from_scale_2d_around(2.0, 2.0, center);
        let point = DVec2::new(150.0, 150.0);
        let scaled = t.multiply_point_2d(point);
        let expected = DVec2::new(200.0, 200.0);
        assert_eq!(scaled, expected);
    }

    #[test]
    fn test_3d_translation() {
        let t = Transform::from_translation_3d(10.0, 20.0, 30.0);
        let point = DVec3::new(1.0, 2.0, 3.0);
        let transformed = t.multiply_point_3d(point);
        assert_eq!(transformed, DVec3::new(11.0, 22.0, 33.0));
    }

    #[test]
    fn test_3d_rotation() {
        let t = Transform::from_rotation_x(std::f64::consts::PI / 2.0);
        let point = DVec3::new(0.0, 1.0, 0.0);
        let rotated = t.multiply_point_3d(point);
        assert!((rotated.x).abs() < 1e-10);
        assert!((rotated.z - 1.0).abs() < 1e-10);
        assert!((rotated.y).abs() < 1e-10);
    }

    #[test]
    fn test_3d_scale() {
        let t = Transform::from_scale_3d(2.0, 3.0, 4.0);
        let point = DVec3::new(1.0, 2.0, 3.0);
        let scaled = t.multiply_point_3d(point);
        assert_eq!(scaled, DVec3::new(2.0, 6.0, 12.0));
    }
}
