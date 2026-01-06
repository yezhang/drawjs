//! 3D 向量类型

use glam::DVec3;
use serde::{Deserialize, Serialize};

/// 3D 向量类型
///
/// 基于 `glam::DVec3` 的包装类型，用于 3D 坐标和向量计算。
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(from = "Vec3Serde", into = "Vec3Serde")]
pub struct Vec3(pub DVec3);

impl Vec3 {
    /// 零向量
    pub const ZERO: Vec3 = Vec3(DVec3::ZERO);

    /// 创建新向量
    #[inline]
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3(DVec3::new(x, y, z))
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

    /// Z 分量
    #[inline]
    pub fn z(self) -> f64 {
        self.0.z
    }

    /// 向量长度（模）
    #[inline]
    pub fn length(self) -> f64 {
        self.0.length()
    }

    /// 向量长度的平方
    #[inline]
    pub fn length_squared(self) -> f64 {
        self.0.length_squared()
    }

    /// 归一化
    #[inline]
    pub fn normalize(self) -> Self {
        Vec3(self.0.normalize())
    }

    /// 点积
    #[inline]
    pub fn dot(self, other: Vec3) -> f64 {
        self.0.dot(other.0)
    }

    /// 叉积
    #[inline]
    pub fn cross(self, other: Vec3) -> Self {
        Vec3(self.0.cross(other.0))
    }
}

impl Default for Vec3 {
    #[inline]
    fn default() -> Self {
        Vec3::ZERO
    }
}

impl std::ops::Add for Vec3 {
    type Output = Vec3;

    #[inline]
    fn add(self, other: Vec3) -> Vec3 {
        Vec3(self.0 + other.0)
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Vec3;

    #[inline]
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3(self.0 - other.0)
    }
}

impl std::ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    #[inline]
    fn mul(self, scalar: f64) -> Vec3 {
        Vec3(self.0 * scalar)
    }
}

impl std::ops::Div<f64> for Vec3 {
    type Output = Vec3;

    #[inline]
    fn div(self, scalar: f64) -> Vec3 {
        Vec3(self.0 / scalar)
    }
}

impl From<DVec3> for Vec3 {
    #[inline]
    fn from(dvec3: DVec3) -> Self {
        Vec3(dvec3)
    }
}

impl From<Vec3> for DVec3 {
    #[inline]
    fn from(vec3: Vec3) -> Self {
        vec3.0
    }
}

impl From<(f64, f64, f64)> for Vec3 {
    #[inline]
    fn from((x, y, z): (f64, f64, f64)) -> Self {
        Vec3::new(x, y, z)
    }
}

#[derive(Serialize, Deserialize)]
struct Vec3Serde(f64, f64, f64);

impl From<Vec3Serde> for Vec3 {
    fn from(val: Vec3Serde) -> Self {
        Vec3::new(val.0, val.1, val.2)
    }
}

impl From<Vec3> for Vec3Serde {
    fn from(val: Vec3) -> Self {
        Vec3Serde(val.x(), val.y(), val.z())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v.x(), 1.0);
        assert_eq!(v.y(), 2.0);
        assert_eq!(v.z(), 3.0);
    }

    #[test]
    fn test_cross() {
        let a = Vec3::new(1.0, 0.0, 0.0);
        let b = Vec3::new(0.0, 1.0, 0.0);
        let c = a.cross(b);
        assert!((c.x() - 0.0).abs() < 1e-10);
        assert!((c.y() - 0.0).abs() < 1e-10);
        assert!((c.z() - 1.0).abs() < 1e-10);
    }
}
