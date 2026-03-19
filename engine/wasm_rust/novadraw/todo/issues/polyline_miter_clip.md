# Issue: Polyline/Polygon 描边尖角被边界裁剪

## 问题描述

当 PolylineFigure 或 PolygonFigure 使用 Miter 连接时，在尖锐角度处的描边尖角会超出 bounds 边界，导致被裁剪。

## 复现步骤

1. 运行 shape-app 场景 3 (Polyline)
2. 观察带尖角的折线（如山峰形状）
3. 尖角顶部被边界裁剪

## 根因分析

1. **Bounds 计算**: `calculate_bounds()` 使用 `stroke_width / 2` 扩展边界
2. **Miter 特性**: 尖锐角度处 miter 尖角可以延伸很远，超过 `stroke_width / 2` 的范围
3. **渲染坐标**: 直接在原始坐标绘制，未考虑向内收缩

## g2 对比

| Figure 类型 | g2 处理方式 |
|------------|------------|
| RectangleFigure | 向内收缩绘制: `lineInset = max(1.0, lineWidth) / 2.0` |
| PolylineShape | 不做特殊处理 |
| PolygonShape | 不做特殊处理 |

**结论**: g2 也存在同样问题。

## 解决方案

参考 RectangleFigure 的做法（在 `outline_shape` 中向内收缩）:

```rust
// line_inset = max(1.0, stroke_width) / 2.0
let line_inset = 1.0_f64.max(self.stroke_width) / 2.0;

// 向内收缩绘制
let x = self.bounds.x + line_inset;
let y = self.bounds.y + line_inset;
let width = self.bounds.width - line_inset * 2.0;
let height = self.bounds.height - line_inset * 2.0;

// 在收缩后的区域绘制
```

## 相关文件

- `novadraw-scene/src/figure/polyline.rs`
- `novadraw-scene/src/figure/polygon.rs`
- `novadraw-scene/src/figure/rounded_rectangle.rs` (已有正确实现)

## 优先级

中等 - 影响描边渲染正确性

## 记录时间

2026-03-12
