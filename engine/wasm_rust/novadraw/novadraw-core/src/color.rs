//! 颜色类型

use serde::{Deserialize, Serialize};

/// RGBA 颜色类型
///
/// 使用 f64 精度，每个分量范围 [0.0, 1.0]。
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Color {
    /// 红色分量 [0.0, 1.0]
    pub r: f64,
    /// 绿色分量 [0.0, 1.0]
    pub g: f64,
    /// 蓝色分量 [0.0, 1.0]
    pub b: f64,
    /// 透明度分量 [0.0, 1.0]
    pub a: f64,
}

impl Color {
    /// 从十六进制颜色码创建颜色
    ///
    /// 支持 6 位 (#RRGGBB) 和 8 位 (#RRGGBBAA) 格式。
    ///
    /// # 示例
    ///
    /// ```
    /// let red = novadraw_core::Color::hex("#ff0000");
    /// let with_alpha = novadraw_core::Color::hex("#ff000080"); // 50% 透明度
    /// ```
    #[inline]
    pub fn hex(hex: &str) -> Self {
        let hex = hex.trim_start_matches('#');
        let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0) as f64 / 255.0;
        let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0) as f64 / 255.0;
        let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0) as f64 / 255.0;
        let a = if hex.len() == 8 {
            u8::from_str_radix(&hex[6..8], 16).unwrap_or(255) as f64 / 255.0
        } else {
            1.0
        };
        Self { r, g, b, a }
    }

    /// 从 RGBA 分量创建颜色
    ///
    /// 每个分量范围 [0.0, 1.0]。
    #[inline]
    pub fn rgba(r: f64, g: f64, b: f64, a: f64) -> Self {
        Self { r, g, b, a }
    }

    /// 红色
    pub const RED: Color = Color { r: 1.0, g: 0.0, b: 0.0, a: 1.0 };

    /// 绿色
    pub const GREEN: Color = Color { r: 0.0, g: 1.0, b: 0.0, a: 1.0 };

    /// 蓝色
    pub const BLUE: Color = Color { r: 0.0, g: 0.0, b: 1.0, a: 1.0 };

    /// 白色
    pub const WHITE: Color = Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 };

    /// 黑色
    pub const BLACK: Color = Color { r: 0.0, g: 0.0, b: 0.0, a: 1.0 };

    /// 透明
    pub const TRANSPARENT: Color = Color { r: 0.0, g: 0.0, b: 0.0, a: 0.0 };

    /// 设置透明度
    #[inline]
    pub fn with_alpha(self, alpha: f64) -> Self {
        Self { a: alpha, ..self }
    }

    /// 检查是否完全透明
    #[inline]
    pub fn is_transparent(self) -> bool {
        self.a <= 0.0
    }

    /// 检查是否完全不透明
    #[inline]
    pub fn is_opaque(self) -> bool {
        self.a >= 1.0
    }
}

impl Default for Color {
    #[inline]
    fn default() -> Self {
        Color::BLACK
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_6_digits() {
        let red = Color::hex("#ff0000");
        assert!((red.r - 1.0).abs() < 1e-10);
        assert!((red.g - 0.0).abs() < 1e-10);
        assert!((red.b - 0.0).abs() < 1e-10);
        assert!((red.a - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_hex_8_digits() {
        let red_half = Color::hex("#ff000080");
        assert!((red_half.r - 1.0).abs() < 1e-10);
        assert!((red_half.a - 128.0 / 255.0).abs() < 1e-10);
    }

    #[test]
    fn test_rgba() {
        let color = Color::rgba(0.5, 0.25, 0.75, 0.8);
        assert!((color.r - 0.5).abs() < 1e-10);
        assert!((color.g - 0.25).abs() < 1e-10);
        assert!((color.b - 0.75).abs() < 1e-10);
        assert!((color.a - 0.8).abs() < 1e-10);
    }

    #[test]
    fn test_with_alpha() {
        let red = Color::hex("#ff0000");
        let red_half = red.with_alpha(0.5);
        assert!((red_half.a - 0.5).abs() < 1e-10);
        assert!((red_half.r - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_constants() {
        assert_eq!(Color::RED.r, 1.0);
        assert_eq!(Color::GREEN.g, 1.0);
        assert_eq!(Color::BLUE.b, 1.0);
        assert_eq!(Color::WHITE.r, 1.0);
        assert_eq!(Color::BLACK.r, 0.0);
    }
}
