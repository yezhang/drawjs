# Eclipse Draw2D Figure 盒模型分析

> 基于 Eclipse GEF Classic 源码分析

## 概述

d2 Figure 的盒模型（Box Model）借鉴了 CSS 盒模型的概念，但有其特定的实现方式。该模型定义了图形元素的结构层次，用于布局和渲染。

## 盒模型层次结构

```
┌────────────────────────────────────────────────────────────────┐
│                        Figure Bounds                            │
│                                                                │
│  ┌──────────────────────────────────────────────────────────┐ │
│  │                      Border (装饰边框)                    │ │
│  │  ┌────────────────────────────────────────────────────┐ │ │
│  │  │                                                    │ │ │
│  │  │  ┌────────────────────────────────────────────┐  │ │ │
│  │  │  │           Client Area (客户区域)            │  │ │ │
│  │  │  │                                            │  │ │ │
│  │  │  │     子元素在这里布局和绘制                   │  │ │ │
│  │  │  │                                            │  │ │ │
│  │  │  └────────────────────────────────────────────┘  │ │ │
│  │  │                                                    │ │ │
│  │  └────────────────────────────────────────────────────┘ │ │
│  │                                                            │ │
│  └──────────────────────────────────────────────────────────┘ │
│                                                                │
└────────────────────────────────────────────────────────────────
```

## 核心概念定义

### 1. Bounds（边界）

Figure 占用的完整矩形区域，包括所有视觉元素。

```java
// Figure.java
protected Rectangle bounds = new Rectangle(0, 0, 0, 0);

public Rectangle getBounds() {
    return bounds;
}
```

**特点**：
- 图形占用的完整区域
- 包含所有视觉元素：内容 + 边框 + 描边
- 是图形定位的基础坐标

### 2. Insets（内边距）

Border 声明占用的空间，用于收缩 ClientArea。

```java
// Figure.java
public Insets getInsets() {
    if (getBorder() != null) {
        return getBorder().getInsets(this);
    }
    return NO_INSETS;
}
```

**计算公式**：
```java
// LineBorder.java
public Insets getInsets(IFigure figure) {
    return new Insets(getWidth());  // insets = border width
}
```

### 3. Client Area（客户区域）

子元素可以布局和绘制的区域。

```java
// Figure.java
public Rectangle getClientArea(Rectangle rect) {
    rect.setBounds(getBounds());      // 从 bounds 开始
    rect.shrink(getInsets());        // 向内收缩 insets
    if (useLocalCoordinates()) {
        rect.setLocation(0, 0);      // 本地坐标时置零
    }
    return rect;
}
```

**计算公式**：
```
clientArea = bounds - insets
```

### 4. Border（装饰边框）

可选的装饰性边框，独立于 Shape 的 outline。

**重要：Border 绘制在 bounds 内部！**

```java
// Border.java
public interface Border {
    Insets getInsets(IFigure figure);  // 返回占用的空间
    void paint(IFigure figure, Graphics g, Insets insets);  // 绘制
}

// Figure.java - paintBorder()
protected void paintBorder(Graphics graphics) {
    if (getBorder() != null) {
        // 注意：传入的是 NO_INSETS (全0)！
        getBorder().paint(this, graphics, NO_INSETS);
    }
}

// AbstractBorder.java - getPaintRectangle
protected Rectangle getPaintRectangle(IFigure figure, Insets insets) {
    tempRect.setBounds(figure.getBounds());
    return tempRect.shrink(insets);  // 根据传入的 insets 收缩
}

// LineBorder.java - paint
public void paint(IFigure figure, Graphics graphics, Insets insets) {
    Rectangle r = getPaintRectangle(figure, insets);  // bounds - insets
    r.shrink(getWidth() / 2, getWidth() / 2);         // 再向内收缩 half-width
    graphics.drawRectangle(r);
}
```

**关键发现**：
- `paintBorder()` 调用时传入的是 `NO_INSETS`（全是0）
- 所以 Border 绘制在 **bounds 内部**，而不是外部
- Border 和 Outline 一样，都是向内收缩绘制的

### 5. Outline（轮廓描边）

Shape 自身的描边（与 Border 类似，都在 bounds 内部绘制）。

```java
// Shape.java
protected abstract void outlineShape(Graphics graphics);

// RectangleFigure.java
protected void outlineShape(Graphics g) {
    // 考虑线宽，向内收缩 half-width
    float lineInset = Math.max(1.0f, getLineWidthFloat()) / 2.0f;
    Rectangle r = getBounds().getCopy();
    r.x += lineInset;
    r.y += lineInset;
    r.width -= lineInset * 2;
    r.height -= lineInset * 2;
    g.drawRectangle(r);
}
```

## 绘制顺序

```
paint(Graphics)
    │
    ├── 1. paintFigure()
    │       │
    │       ├── isOpaque() ? fillRectangle(bounds)  ← 填充背景
    │       │
    │       └── border instanceof AbstractBackground ? paintBackground()
    │
    ├── 2. paintClientArea()
    │       │
    │       ├── translate(bounds.x + insets.left, bounds.y + insets.top)
    │       ├── clipRect(clientArea)              ← 裁剪到客户区
    │       │
    │       └── for child : children
    │               child.paint()
    │
    └── 3. paintBorder()
            │
            └── border.paint(figure, g, insets)  ← 绘制装饰边框（在 bounds 内）
```

## Border vs Outline vs Fill

| 概念 | 来源 | 作用 | 绘制位置 |
|------|------|------|----------|
| **fillShape** | Shape | 填充图形内部 | bounds 整个范围 |
| **outlineShape** | Shape | 描边图形轮廓 | bounds 向内收缩 half-lineWidth |
| **border** | Figure (setBorder) | 装饰性边框 | bounds 向内收缩 border-width |

### 关键发现：Border 和 Outline 都在 bounds 内部绘制

```
┌─────────────────────────────────────────────────────────┐
│                    Figure Bounds                         │
│  ┌─────────────────────────────────────────────────┐   │
│  │                                                 │   │
│  │  ┌─────────────────────────────────────────┐  │   │
│  │  │                                         │  │   │
│  │  │  ┌─────────────────────────────────┐  │  │   │
│  │  │  │                                 │  │  │   │
│  │  │  │  ┌───────────────────────┐   │  │  │   │
│  │  │  │  │                       │   │  │  │   │
│  │  │  │  │      FILL            │   │  │  │   │ ← bounds 整个范围
│  │  │  │  │  ┌─────────────────┐ │   │  │  │   │
│  │  │  │  │  │               │ │   │  │  │   │
│  │  │  │  │  │   OUTLINE     │ │   │  │  │   │ ← 向内收缩
│  │  │  │  │  │               │ │   │  │  │   │
│  │  │  │  │  └─────────────────┘ │   │  │  │   │
│  │  │  │  │                       │   │  │  │   │
│  │  │  │  └───────────────────────┘   │  │  │   │
│  │  │  │                                 │  │  │   │
│  │  │  └─────────────────────────────────┘  │  │   │
│  │  │                                         │  │   │
│  │  └─────────────────────────────────────────┘  │   │
│  │                                             │   │
│  │              BORDER (装饰)                    │ ← 向内收缩
│  │                                             │   │
│  └─────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────┘
```

### Border vs Outline 区别

| | Border | Outline |
|---|--------|---------|
| 来源 | `setBorder(Border)` | Shape 属性 (fill/outline) |
| 绘制位置 | bounds 内，向内收缩 | bounds 内，向内收缩 |
| insets 作用 | **不决定** Border 绘制位置 | 不适用 |
| 影响 clientArea | 是（收缩子元素布局区域） | 否 |

### 重要：Insets 的真正作用

insets 用于收缩 **ClientArea**（子元素布局区域），但 **不决定** Border 的绘制位置：

```java
// Border 绘制：使用传入的 insets 参数（通常为 NO_INSETS）
getBorder().paint(this, graphics, NO_INSETS);

// ClientArea 计算：使用 figure 的 insets
public Rectangle getClientArea() {
    rect.shrink(getInsets());  // insets 收缩客户区
}
```

## 实际案例：带边框的矩形

```java
RectangleFigure rect = new RectangleFigure();
rect.setBounds(new Rectangle(10, 10, 100, 50));
rect.setBorder(new LineBorder(Color.BLACK, 2));

// 几何数据：
// bounds      = (10, 10, 100, 50)
// border insets = 2 (border width)
// clientArea  = bounds - insets = (12, 12, 96, 46)
```

### 绘制结果

```
┌──────────────────────────────┐
│ ■■■■■■■■■■■■■■■■■■ │  ← Border (2px，在 bounds 内)
│ ■                          ■ │
│ ■  ┌────────────────────┐  ■ │  ← fillShape + outlineShape
│ ■  │                    │  ■ │
│ ■  │   Client Area     │  ■ │     (子元素布局区域)
│ ■  │                    │  ■ │
│ ■  └────────────────────┘  ■ │
│ ■                          ■ │
│ ■■■■■■■■■■■■■■■■■■■■ │  ← Border
└──────────────────────────────┘
```

## 与 CSS 盒模型对比

| CSS 盒模型 | d2 Figure | 说明 |
|------------|-----------|------|
| content | clientArea | 内容区域 |
| padding | - | d2 无对应 |
| border | Border + outline | 边框区域（在 bounds 内） |
| margin | - | d2 无对应 |
| box-sizing | - | d2 固定为 border-box 模式 |

**关键区别**：
- d2 使用 border-box 模式（bounds 包含所有）
- d2 没有 margin（外边距由父容器管理）
- Border 和 Outline 都在 bounds 内部绘制
- insets 只用于收缩 ClientArea，不决定 Border 绘制位置

## 关键设计意图

### 1. Border 与 Shape 分离

Border 是**装饰性**的，独立于图形几何：
- 一个 Border 可以用于任何 Figure
- Border 改变不影响图形几何（只影响 clientArea）

### 2. Outline 是 Shape 属性

Outline（描边）是**图形几何**的一部分：
- 与 fill（填充）成对出现
- 在 bounds 内部绘制

### 3. Border 和 Outline 都在 bounds 内部绘制

两者都向内收缩，不占用额外空间：
- Border 向内收缩 border-width
- Outline 向内收缩 line-width/2

### 4. Insets 的真正作用

insets 的作用是**收缩 ClientArea**（子元素布局区域），而不是决定 Border 绘制位置。

```java
// Border 绘制位置 = bounds 内部（由传入的 insets 决定，通常为 NO_INSETS）
// ClientArea 收缩 = 使用 getInsets()
```

## 图示总结

```
┌─────────────────────────────────────────────────────────────────┐
│                         BOUNDS                                  │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │                        INSETS                             │  │
│  │     (收缩 ClientArea，不决定 Border 绘制位置)              │  │
│  │  ┌─────────────────────────────────────────────────────┐  │  │
│  │  │                                                     │  │  │
│  │  │  ┌───────────────────────────────────────────────┐  │  │ │
│  │  │  │                                               │  │  │ │
│  │  │  │              CLIENT AREA                        │  │  │ │
│  │  │  │         (子元素布局区域)                        │  │  │ │
│  │  │  │         = bounds - insets                       │  │  │ │
│  │  │  │                                               │  │  │ │
│  │  │  │  ┌─────────────────────────────────────────┐  │  │  │ │
│  │  │  │  │                                         │  │  │  │ │
│  │  │  │  │              FILL                        │  │  │  │ │
│  │  │  │  │  ┌───────────────────────────────────┐  │  │  │  │ │
│  │  │  │  │  │                                   │  │  │  │  │ │
│  │  │  │  │  │           OUTLINE                 │  │  │  │  │ │
│  │  │  │  │  │                                   │  │  │  │  │ │
│  │  │  │  │  └───────────────────────────────────┘  │  │  │  │ │
│  │  │  │  │                                         │  │  │  │ │
│  │  │  │  └─────────────────────────────────────────┘  │  │  │ │
│  │  │  │                                                 │  │  │ │
│  │  │  └───────────────────────────────────────────────┘  │  │ │
│  │  │                                                     │  │ │
│  │  └─────────────────────────────────────────────────────┘  │  │
│  │                                                           │  │
│  │              BORDER (在 bounds 内绘制)                    │  │
│  │              = bounds - NO_INSETS - borderWidth/2        │  │
│  └───────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
```

## 参考

- 源码: `org.eclipse.draw2d.Figure`
- 源码: `org.eclipse.draw2d.Shape`
- 源码: `org.eclipse.draw2d.Border`
- 源码: `org.eclipse.draw2d.LineBorder`
