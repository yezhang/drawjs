//! 2D 向量类型
//!
//! 纯数学类型，用于表示 2D 坐标和向量。

use glam::DVec2;
use serde::{Deserialize, Serialize};

/// 2D 向量类型
///
/// 基于 `glam::DVec2` 的包装类型，使用 `f64` 精度。
/// 遵循标准数学运算语义。
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(from = "Vec2Serde", into = "Vec2Serde")]
pub struct Vec2(pub DVec2);

impl Vec2 {
    /// 零向量
    pub const ZERO: Vec2 = Vec2(DVec2::ZERO);

    /// X 轴单位向量
    pub const X: Vec2 = Vec2(DVec2::X);

    /// Y 轴单位向量
    pub const Y: Vec2 = Vec2(DVec2::Y);

    /// 创建向量
    #[inline]
    pub fn new(x: f64, y: f64) -> Self {
        Vec2(DVec2::new(x, y))
    }

    /// X 分量
    #[inline]
    pub fn x(self) -> f64 {
        self.0.x
    }

    /// Y 分量
    #[inline]
    pub fn y(self) -> f64 {
        self.0.y
    }

    /// 向量长度
    #[inline]
    pub fn length(self) -> f64 {
        self.0.length()
    }

    /// 长度平方
    #[inline]
    pub fn length_squared(self) -> f64 {
        self.0.length_squared()
    }

    /// 归一化
    #[inline]
    pub fn normalize(self) -> Self {
        Vec2(self.0.normalize())
    }

    /// 点积
    #[inline]
    pub fn dot(self, other: Vec2) -> f64 {
        self.0.dot(other.0)
    }

    /// 叉积 (2D 叉乘结果为标量)
    #[inline]
    pub fn cross(self, other: Vec2) -> f64 {
        self.0.x * other.0.y - self.0.y * other.0.x
    }

    /// 旋转向量
    #[inline]
    pub fn rotate(self, angle: f64) -> Self {
        let (s, c) = angle.sin_cos();
        Vec2(DVec2::new(
            self.0.x * c + self.0.y * s,
            -self.0.x * s + self.0.y * c,
        ))
    }

    /// 线性插值
    #[inline]
    pub fn lerp(self, other: Vec2, t: f64) -> Vec2 {
        Vec2(self.0.lerp(other.0, t))
    }

    /// 到另一点的距离
    #[inline]
    pub fn distance(self, other: Vec2) -> f64 {
        self.0.distance(other.0)
    }
}

impl Default for Vec2 {
    fn default() -> Self {
        Vec2::ZERO
    }
}

impl std::ops::Add for Vec2 {
    type Output = Vec2;

    #[inline]
    fn add(self, other: Vec2) -> Self::Output {
        Vec2(self.0 + other.0)
    }
}

impl std::ops::Sub for Vec2 {
    type Output = Vec2;

    #[inline]
    fn sub(self, other: Vec2) -> Self::Output {
        Vec2(self.0 - other.0)
    }
}

impl std::ops::Mul<f64> for Vec2 {
    type Output = Vec2;

    #[inline]
    fn mul(self, scalar: f64) -> Self::Output {
        Vec2(self.0 * scalar)
    }
}

impl std::ops::Mul<Vec2> for Vec2 {
    type Output = f64;

    #[inline]
    fn mul(self, other: Vec2) -> Self::Output {
        self.dot(other)
    }
}

impl std::ops::Div<f64> for Vec2 {
    type Output = Vec2;

    #[inline]
    fn div(self, scalar: f64) -> Self::Output {
        Vec2(self.0 / scalar)
    }
}

impl std::ops::Neg for Vec2 {
    type Output = Vec2;

    #[inline]
    fn neg(self) -> Self::Output {
        Vec2(-self.0)
    }
}

impl std::ops::AddAssign for Vec2 {
    #[inline]
    fn add_assign(&mut self, other: Vec2) {
        self.0 += other.0;
    }
}

impl std::ops::SubAssign for Vec2 {
    #[inline]
    fn sub_assign(&mut self, other: Vec2) {
        self.0 -= other.0;
    }
}

impl std::ops::MulAssign<f64> for Vec2 {
    #[inline]
    fn mul_assign(&mut self, scalar: f64) {
        self.0 *= scalar;
    }
}

impl std::ops::DivAssign<f64> for Vec2 {
    #[inline]
    fn div_assign(&mut self, scalar: f64) {
        self.0 /= scalar;
    }
}

impl From<DVec2> for Vec2 {
    fn from(dvec2: DVec2) -> Self {
        Vec2(dvec2)
    }
}

impl From<Vec2> for DVec2 {
    fn from(vec2: Vec2) -> Self {
        vec2.0
    }
}

impl From<(f64, f64)> for Vec2 {
    fn from((x, y): (f64, f64)) -> Self {
        Vec2::new(x, y)
    }
}

impl From<[f64; 2]> for Vec2 {
    fn from([x, y]: [f64; 2]) -> Self {
        Vec2::new(x, y)
    }
}

impl std::fmt::Display for Vec2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vec2({:.4}, {:.4})", self.x(), self.y())
    }
}

#[derive(Serialize, Deserialize)]
struct Vec2Serde(f64, f64);

impl From<Vec2Serde> for Vec2 {
    fn from(val: Vec2Serde) -> Self {
        Vec2::new(val.0, val.1)
    }
}

impl From<Vec2> for Vec2Serde {
    fn from(val: Vec2) -> Self {
        Vec2Serde(val.x(), val.y())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(Vec2::new(1.0, 2.0) + Vec2::new(3.0, 4.0), Vec2::new(4.0, 6.0));
    }

    #[test]
    fn test_sub() {
        assert_eq!(Vec2::new(5.0, 6.0) - Vec2::new(2.0, 3.0), Vec2::new(3.0, 3.0));
    }

    #[test]
    fn test_mul_scalar() {
        assert_eq!(Vec2::new(2.0, 3.0) * 2.0, Vec2::new(4.0, 6.0));
    }

    #[test]
    fn test_mul_vec() {
        assert_eq!(Vec2::new(2.0, 3.0) * Vec2::new(4.0, 5.0), 23.0); // 2*4 + 3*5
    }

    #[test]
    fn test_div() {
        assert_eq!(Vec2::new(6.0, 8.0) / 2.0, Vec2::new(3.0, 4.0));
    }

    #[test]
    fn test_neg() {
        assert_eq!(-Vec2::new(3.0, -4.0), Vec2::new(-3.0, 4.0));
    }
}
