//! 3x3 矩阵类型
//!
//! 用于 2D 仿射变换的纯数学类型。
//! 采用列向量约定：`v' = M × v`
//!
//! # 运算符语义
//!
//! | 运算符 | 含义 | 示例 |
//! |--------|------|------|
//! | `*` | 矩阵乘法：`A * B = A × B` | `combined = parent * child` |

use glam::DMat3;
use serde::{Deserialize, Serialize};

/// 3x3 矩阵
///
/// # 数学定义
///
/// 列向量约定下，变换公式为：`v' = M × v`
///
/// 矩阵乘法满足：`A * B = A × B`
/// - 语义：先应用 `B`，再应用 `A`
/// - 与数学公式一致
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(from = "Mat3Serde", into = "Mat3Serde")]
pub struct Mat3(pub DMat3);

impl Mat3 {
    /// 单位矩阵
    pub const IDENTITY: Mat3 = Mat3(DMat3::IDENTITY);

    /// 从行优先参数创建
    ///
    /// 矩阵布局:
    /// ```text
    /// | m00 m01 m02 |
    /// | m10 m11 m12 |
    /// | m20 m21 m22 |
    /// ```
    #[inline]
    pub fn new(
        m00: f64, m01: f64, m02: f64,
        m10: f64, m11: f64, m12: f64,
        m20: f64, m21: f64, m22: f64,
    ) -> Self {
        Self(DMat3::from_cols(
            glam::DVec3::new(m00, m10, m20),
            glam::DVec3::new(m01, m11, m21),
            glam::DVec3::new(m02, m12, m22),
        ))
    }

    /// 从数组创建 (行优先)
    #[inline]
    pub fn from_array(arr: [[f64; 3]; 3]) -> Self {
        Self(DMat3::from_cols_array(&[
            arr[0][0], arr[1][0], arr[2][0],
            arr[0][1], arr[1][1], arr[2][1],
            arr[0][2], arr[1][2], arr[2][2],
        ]))
    }

    /// 转换为数组 (行优先)
    #[inline]
    pub fn to_array(self) -> [[f64; 3]; 3] {
        [
            [self.0.x_axis.x, self.0.y_axis.x, self.0.z_axis.x],
            [self.0.x_axis.y, self.0.y_axis.y, self.0.z_axis.y],
            [self.0.x_axis.z, self.0.y_axis.z, self.0.z_axis.z],
        ]
    }

    /// 从缩放创建
    #[inline]
    pub fn from_scale(sx: f64, sy: f64) -> Self {
        Self(DMat3::from_scale(glam::DVec2::new(sx, sy)))
    }

    /// 从统一缩放创建
    #[inline]
    pub fn from_uniform_scale(s: f64) -> Self {
        Self::from_scale(s, s)
    }

    /// 从平移创建
    #[inline]
    pub fn from_translation(tx: f64, ty: f64) -> Self {
        Self::new(1.0, 0.0, tx, 0.0, 1.0, ty, 0.0, 0.0, 1.0)
    }

    /// 从旋转创建 (弧度，绕原点)
    ///
    /// 标准旋转矩阵:
    /// ```text
    /// | cos -sin  0 |
    /// | sin  cos  0 |
    /// |   0    0  1 |
    /// ```
    #[inline]
    pub fn from_rotation(radians: f64) -> Self {
        let (s, c) = radians.sin_cos();
        Self::new(c, -s, 0.0, s, c, 0.0, 0.0, 0.0, 1.0)
    }

    /// 行列式
    #[inline]
    pub fn determinant(self) -> f64 {
        self.0.determinant()
    }

    /// 逆矩阵
    #[inline]
    pub fn inverse(self) -> Option<Self> {
        let det = self.determinant();
        if det.abs() < 1e-12 {
            None
        } else {
            Some(Mat3(self.0.inverse()))
        }
    }

    /// 转置
    #[inline]
    pub fn transpose(self) -> Self {
        Mat3(self.0.transpose())
    }

    /// 提取缩放分量 (假设均匀缩放)
    #[inline]
    pub fn scale(self) -> (f64, f64) {
        let m = self.to_array();
        let sx = (m[0][0].powi(2) + m[1][0].powi(2)).sqrt();
        let sy = (m[0][1].powi(2) + m[1][1].powi(2)).sqrt();
        (sx, sy)
    }

    /// 提取旋转角度 (弧度)
    #[inline]
    pub fn rotation(self) -> f64 {
        let m = self.to_array();
        m[1][0].atan2(m[0][0])
    }

    /// 提取平移分量
    #[inline]
    pub fn translation(self) -> (f64, f64) {
        let m = self.to_array();
        (m[0][2], m[1][2])
    }
}

impl Default for Mat3 {
    fn default() -> Self {
        Mat3::IDENTITY
    }
}

/// 矩阵乘法：`A * B = A × B`
///
/// 语义：先应用 `B`，再应用 `A`
impl std::ops::Mul for Mat3 {
    type Output = Mat3;

    #[inline]
    fn mul(self, other: Mat3) -> Self::Output {
        Mat3(self.0 * other.0)
    }
}

impl std::ops::MulAssign for Mat3 {
    #[inline]
    fn mul_assign(&mut self, other: Mat3) {
        *self = *self * other;
    }
}

impl From<DMat3> for Mat3 {
    fn from(dmat3: DMat3) -> Self {
        Mat3(dmat3)
    }
}

impl From<Mat3> for DMat3 {
    fn from(mat3: Mat3) -> Self {
        mat3.0
    }
}

impl std::fmt::Display for Mat3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // 按数学行布局显示，底层 glam 为列向量约定
        // v' = M × v
        let m = self.to_array();
        write!(
            f,
            "Mat3({:.4}, {:.4}, {:.4}\n     {:.4}, {:.4}, {:.4}\n     {:.4}, {:.4}, {:.4})",
            m[0][0], m[0][1], m[0][2],
            m[1][0], m[1][1], m[1][2],
            m[2][0], m[2][1], m[2][2]
        )
    }
}

#[derive(Serialize, Deserialize)]
struct Mat3Serde(pub [[f64; 3]; 3]);

impl From<Mat3Serde> for Mat3 {
    fn from(val: Mat3Serde) -> Self {
        Mat3::from_array(val.0)
    }
}

impl From<Mat3> for Mat3Serde {
    fn from(val: Mat3) -> Self {
        Mat3Serde(val.to_array())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identity() {
        let identity = Mat3::IDENTITY;
        let arr = identity.to_array();
        assert_eq!(arr[0][0], 1.0);
        assert_eq!(arr[1][1], 1.0);
        assert_eq!(arr[2][2], 1.0);
    }

    #[test]
    fn test_multiplication_order() {
        let scale = Mat3::from_scale(2.0, 2.0);
        let translate = Mat3::from_translation(10.0, 0.0);

        let combined = translate * scale;
        let arr = combined.to_array();
        assert!((arr[0][0] - 2.0).abs() < 1e-10);
        assert!((arr[0][2] - 10.0).abs() < 1e-10);
    }

    #[test]
    fn test_reverse_order() {
        let scale = Mat3::from_scale(2.0, 2.0);
        let translate = Mat3::from_translation(10.0, 0.0);

        let combined = scale * translate;
        let arr = combined.to_array();
        assert!((arr[0][0] - 2.0).abs() < 1e-10);
        assert!((arr[0][2] - 20.0).abs() < 1e-10);
    }

    #[test]
    fn test_rotation() {
        let rotate = Mat3::from_rotation(std::f64::consts::FRAC_PI_2);
        let arr = rotate.to_array();
        assert!((arr[0][0] - 0.0).abs() < 1e-10);
        assert!((arr[0][1] - (-1.0)).abs() < 1e-10);
        assert!((arr[1][0] - 1.0).abs() < 1e-10);
        assert!((arr[1][1] - 0.0).abs() < 1e-10);
    }
}
