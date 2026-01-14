//! 变换模块
//!
//! 2D 仿射变换，封装矩阵和平移分量。

use novadraw_math::Mat3;
use serde::{Deserialize, Serialize};

use super::Vec2;

mod affine;
pub use affine::AffineTransform;

/// 2D 仿射变换
///
/// # 数学定义
///
/// 仿射变换由线性部分 (Mat3) 和平移部分 (Vec2) 组成：`p' = M * p + t`
///
/// # 运算符
///
/// `A * B` 表示矩阵乘法 `A × B`
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(from = "TransformSerde", into = "TransformSerde")]
pub struct Transform {
    matrix: Mat3,
    translation: Vec2,
}

impl Transform {
    /// 单位变换
    pub const IDENTITY: Transform = Transform {
        matrix: Mat3::IDENTITY,
        translation: Vec2::ZERO,
    };

    /// 创建变换
    #[inline]
    pub fn new(matrix: Mat3, translation: Vec2) -> Self {
        Self { matrix, translation }
    }

    /// 从平移创建
    #[inline]
    pub fn from_translation(x: f64, y: f64) -> Self {
        Self {
            matrix: Mat3::IDENTITY,
            translation: Vec2::new(x, y),
        }
    }

    /// 从平移创建 (Vec2)
    #[inline]
    pub fn from_translation_vec(translation: Vec2) -> Self {
        Self {
            matrix: Mat3::IDENTITY,
            translation,
        }
    }

    /// 从缩放创建
    #[inline]
    pub fn from_scale(x: f64, y: f64) -> Self {
        Self {
            matrix: Mat3::from_scale(x, y),
            translation: Vec2::ZERO,
        }
    }

    /// 从统一缩放创建
    #[inline]
    pub fn from_uniform_scale(s: f64) -> Self {
        Self::from_scale(s, s)
    }

    /// 从旋转创建 (弧度，绕原点)
    #[inline]
    pub fn from_rotation(radians: f64) -> Self {
        Self {
            matrix: Mat3::from_rotation(radians),
            translation: Vec2::ZERO,
        }
    }

    /// 变换点
    #[inline]
    pub fn transform_point(self, point: Vec2) -> Vec2 {
        let result = self.matrix.0.mul_vec3(glam::DVec3::new(point.x(), point.y(), 1.0));
        Vec2::new(result.x + self.translation.x(), result.y + self.translation.y())
    }

    /// 变换向量 (不包含平移)
    #[inline]
    pub fn transform_vector(self, vector: Vec2) -> Vec2 {
        let result = self.matrix.0.mul_vec3(glam::DVec3::new(vector.x(), vector.y(), 0.0));
        Vec2::new(result.x, result.y)
    }

    /// 变换组合：`self * other = self × other`
    ///
    /// 语义：先应用 other，再应用 self
    #[inline]
    pub fn multiply(self, other: Transform) -> Transform {
        let matrix = self.matrix * other.matrix;
        let result = self.matrix.0.mul_vec3(glam::DVec3::new(other.translation.x(), other.translation.y(), 0.0));
        let scaled_trans = Vec2::new(result.x, result.y);
        let translation = self.translation + scaled_trans;
        Transform { matrix, translation }
    }

    /// 求逆变换
    #[inline]
    pub fn inverse(self) -> Option<Transform> {
        self.matrix.inverse().map(|inv_matrix| {
            let result = inv_matrix.0.mul_vec3(glam::DVec3::new(self.translation.x(), self.translation.y(), 0.0));
            let inv_translation = Vec2::new(-result.x, -result.y);
            Transform {
                matrix: inv_matrix,
                translation: inv_translation,
            }
        })
    }

    /// 平移分量
    #[inline]
    pub fn translation(self) -> Vec2 {
        self.translation
    }

    /// 缩放分量
    #[inline]
    pub fn scale(self) -> Vec2 {
        let (sx, sy) = self.matrix.scale();
        Vec2::new(sx, sy)
    }

    /// 旋转角度 (弧度)
    #[inline]
    pub fn rotation(self) -> f64 {
        self.matrix.rotation()
    }

    /// 内部矩阵 (用于低级操作)
    #[inline]
    pub fn matrix(self) -> Mat3 {
        self.matrix
    }
}

impl Default for Transform {
    fn default() -> Self {
        Transform::IDENTITY
    }
}

/// 变换乘法：`A * B = A × B`
///
/// 语义：先应用 B，再应用 A
impl std::ops::Mul for Transform {
    type Output = Transform;

    #[inline]
    fn mul(self, other: Transform) -> Self::Output {
        self.multiply(other)
    }
}

impl std::ops::MulAssign for Transform {
    #[inline]
    fn mul_assign(&mut self, other: Transform) {
        *self = *self * other;
    }
}

impl std::fmt::Display for Transform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Transform({}, {})", self.matrix, self.translation)
    }
}

#[derive(Serialize, Deserialize)]
struct TransformSerde {
    matrix: [[f64; 3]; 3],
    translation: [f64; 2],
}

impl From<TransformSerde> for Transform {
    fn from(val: TransformSerde) -> Self {
        Transform {
            matrix: Mat3::from_array(val.matrix),
            translation: Vec2::from(val.translation),
        }
    }
}

impl From<Transform> for TransformSerde {
    fn from(val: Transform) -> Self {
        TransformSerde {
            matrix: val.matrix.to_array(),
            translation: [val.translation.x(), val.translation.y()],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identity() {
        let identity = Transform::IDENTITY;
        let p = Vec2::new(5.0, 10.0);
        assert_eq!(identity.transform_point(p), p);
    }

    #[test]
    fn test_translation() {
        let t = Transform::from_translation(10.0, 20.0);
        let p = Vec2::new(5.0, 5.0);
        assert_eq!(t.transform_point(p), Vec2::new(15.0, 25.0));
    }

    #[test]
    fn test_scale() {
        let t = Transform::from_scale(2.0, 3.0);
        let p = Vec2::new(5.0, 10.0);
        assert_eq!(t.transform_point(p), Vec2::new(10.0, 30.0));
    }

    #[test]
    fn test_multiplication_order() {
        let parent = Transform::from_translation(10.0, 0.0);
        let child = Transform::from_scale(2.0, 2.0);

        // parent * child = parent × child
        // 语义：先 child(缩放)，后 parent(平移)
        let combined = parent * child;
        let p = Vec2::new(5.0, 5.0);
        let result = combined.transform_point(p);

        // 缩放后：(10, 10)，平移后：(20, 10)
        assert_eq!(result, Vec2::new(20.0, 10.0));
    }

    #[test]
    fn test_reverse_order() {
        let parent = Transform::from_translation(10.0, 0.0);
        let child = Transform::from_scale(2.0, 2.0);

        // child * parent = child × parent
        // 语义：先 parent(平移)，后 child(缩放)
        let combined = child * parent;
        let p = Vec2::new(5.0, 5.0);
        let result = combined.transform_point(p);

        // 平移后：(15, 5)，缩放后：(30, 10)
        assert_eq!(result, Vec2::new(30.0, 10.0));
    }

    #[test]
    fn test_rotation() {
        let t = Transform::from_rotation(std::f64::consts::FRAC_PI_2);
        let p = Vec2::new(0.0, 1.0);
        let rotated = t.transform_point(p);
        // 逆时针旋转90度: (0, 1) -> (-1, 0)
        assert!((rotated.x() + 1.0).abs() < 1e-10);
        assert!((rotated.y() - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_inverse() {
        let t = Transform::from_translation(10.0, 20.0);
        let inv = t.inverse().unwrap();
        let p = Vec2::new(15.0, 25.0);
        assert_eq!(inv.transform_point(p), Vec2::new(5.0, 5.0));
    }
}
