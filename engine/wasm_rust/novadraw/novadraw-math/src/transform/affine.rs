//! 仿射变换类型

use super::{Mat3, Transform, Vec2};

/// 2D 仿射变换
///
/// 提供更完整的 2D 仿射变换操作。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AffineTransform {
    a: f64, b: f64, c: f64, d: f64,
    tx: f64, ty: f64,
}

impl AffineTransform {
    /// 单位变换
    pub const IDENTITY: AffineTransform = AffineTransform {
        a: 1.0, b: 0.0, c: 0.0, d: 1.0,
        tx: 0.0, ty: 0.0,
    };

    /// 创建单位变换
    #[inline]
    pub fn new() -> Self {
        Self::IDENTITY
    }

    /// 从变换矩阵创建
    ///
    /// 矩阵布局：
    /// | a  b  tx |
    /// | c  d  ty |
    /// | 0  0  1  |
    #[inline]
    pub fn from_matrix(a: f64, b: f64, c: f64, d: f64, tx: f64, ty: f64) -> Self {
        Self { a, b, c, d, tx, ty }
    }

    /// 从 Transform 转换
    #[inline]
    pub fn from_transform(t: Transform) -> Self {
        let m = t.matrix.to_array();
        Self {
            a: m[0][0], b: m[0][1],
            c: m[1][0], d: m[1][1],
            tx: t.translation.x(),
            ty: t.translation.y(),
        }
    }

    /// 转换为 Transform
    #[inline]
    pub fn to_transform(self) -> Transform {
        Transform::new(
            Mat3::new(self.a, self.b, 0.0, self.c, self.d, 0.0, 0.0, 0.0, 1.0),
            Vec2::new(self.tx, self.ty),
        )
    }

    /// 创建平移变换
    #[inline]
    pub fn translate(self, tx: f64, ty: f64) -> Self {
        Self {
            a: self.a, b: self.b, c: self.c, d: self.d,
            tx: self.a * tx + self.b * ty + self.tx,
            ty: self.c * tx + self.d * ty + self.ty,
        }
    }

    /// 创建缩放变换
    #[inline]
    pub fn scale_transform(self, sx: f64, sy: f64) -> Self {
        Self {
            a: self.a * sx, b: self.b * sy,
            c: self.c * sx, d: self.d * sy,
            tx: self.tx, ty: self.ty,
        }
    }

    /// 创建统一缩放变换
    #[inline]
    pub fn scale_uniform(self, s: f64) -> Self {
        self.scale_transform(s, s)
    }

    /// 创建旋转变换（弧度）
    #[inline]
    pub fn rotate(self, angle: f64) -> Self {
        let (s, c) = angle.sin_cos();
        Self {
            a: self.a * c + self.b * s,
            b: -self.a * s + self.b * c,
            c: self.c * c + self.d * s,
            d: -self.c * s + self.d * c,
            tx: self.tx,
            ty: self.ty,
        }
    }

    /// 创建斜切变换
    #[inline]
    pub fn shear(self, shx: f64, shy: f64) -> Self {
        Self {
            a: self.a + self.b * shy,
            b: self.b + self.a * shx,
            c: self.c + self.d * shy,
            d: self.d + self.c * shx,
            tx: self.tx,
            ty: self.ty,
        }
    }

    /// 组合变换（self 后应用 other）
    #[inline]
    pub fn multiply(self, other: AffineTransform) -> Self {
        Self {
            a: self.a * other.a + self.b * other.c,
            b: self.a * other.b + self.b * other.d,
            c: self.c * other.a + self.d * other.c,
            d: self.c * other.b + self.d * other.d,
            tx: self.a * other.tx + self.b * other.ty + self.tx,
            ty: self.c * other.tx + self.d * other.ty + self.ty,
        }
    }

    /// 变换点
    #[inline]
    pub fn transform_point(self, x: f64, y: f64) -> (f64, f64) {
        (
            self.a * x + self.b * y + self.tx,
            self.c * x + self.d * y + self.ty,
        )
    }

    /// 变换点（Vec2）
    #[inline]
    pub fn multiply_point(self, point: Vec2) -> Vec2 {
        let (x, y) = self.transform_point(point.x(), point.y());
        Vec2::new(x, y)
    }

    /// 变换向量（不含平移）
    #[inline]
    pub fn multiply_vector(self, vector: Vec2) -> Vec2 {
        Vec2::new(
            self.a * vector.x() + self.b * vector.y(),
            self.c * vector.x() + self.d * vector.y(),
        )
    }

    /// 求逆变换
    #[inline]
    pub fn inverse(self) -> Option<Self> {
        let det = self.a * self.d - self.b * self.c;
        if det.abs() < 1e-12 {
            return None;
        }
        let inv_det = 1.0 / det;
        Some(Self {
            a: self.d * inv_det,
            b: -self.b * inv_det,
            c: -self.c * inv_det,
            d: self.a * inv_det,
            tx: (self.b * self.ty - self.d * self.tx) * inv_det,
            ty: (self.c * self.tx - self.a * self.ty) * inv_det,
        })
    }

    /// 行列式
    #[inline]
    pub fn determinant(self) -> f64 {
        self.a * self.d - self.b * self.c
    }

    /// 检查是否可逆
    #[inline]
    pub fn is_invertible(self) -> bool {
        self.determinant().abs() > 1e-12
    }

    /// 获取平移分量
    #[inline]
    pub fn translation(self) -> (f64, f64) {
        (self.tx, self.ty)
    }

    /// 获取缩放分量
    #[inline]
    pub fn get_scale(self) -> (f64, f64) {
        let sx = (self.a.powi(2) + self.c.powi(2)).sqrt();
        let sy = (self.b.powi(2) + self.d.powi(2)).sqrt();
        (sx, sy)
    }

    /// 获取旋转角度（弧度）
    #[inline]
    pub fn rotation(self) -> f64 {
        self.c.atan2(self.a)
    }
}

impl Default for AffineTransform {
    #[inline]
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl std::ops::Mul for AffineTransform {
    type Output = AffineTransform;

    #[inline]
    fn mul(self, other: AffineTransform) -> AffineTransform {
        self.multiply(other)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identity() {
        let identity = AffineTransform::IDENTITY;
        let p = (5.0, 10.0);
        assert_eq!(identity.transform_point(p.0, p.1), p);
    }

    #[test]
    fn test_translate() {
        let t = AffineTransform::new().translate(10.0, 20.0);
        let p = t.transform_point(5.0, 5.0);
        assert_eq!(p, (15.0, 25.0));
    }

    #[test]
    fn test_scale() {
        let t = AffineTransform::new().scale_transform(2.0, 3.0);
        let p = t.transform_point(5.0, 10.0);
        assert_eq!(p, (10.0, 30.0));
    }

    #[test]
    fn test_rotation() {
        let t = AffineTransform::new().rotate(std::f64::consts::FRAC_PI_2);
        let p = t.transform_point(1.0, 0.0);
        assert!((p.0 - 0.0).abs() < 1e-10);
        assert!((p.1 - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_inverse() {
        let t = AffineTransform::new().translate(10.0, 20.0);
        let inv = t.inverse().unwrap();
        let p = inv.transform_point(15.0, 25.0);
        assert_eq!(p, (5.0, 5.0));
    }
}
