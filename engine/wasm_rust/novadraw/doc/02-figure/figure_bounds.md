# Eclipse Draw2D Figure Bounds 分析

本文档分析 Eclipse Draw2D 中 Figure 的 bounds 概念，包括其含义、作用、坐标系统以及在布局和绘制中的应用。

## 1. Bounds 的定义与含义

### 1.1 基本定义

```java
// Figure.java:77
protected Rectangle bounds = new Rectangle(0, 0, 0, 0);
```

Bounds 是一个 `Rectangle`，存储 Figure 的**位置和尺寸**：(x, y, width, height)。

### 1.2 准确含义：绝对坐标

Bounds 的 (x, y) 是**绝对坐标**，但这个"绝对"是相对于**坐标根**的：

- **默认模式**（`useLocalCoordinates() = false`）：父节点移动时，bounds 会自动传播到子节点
- **本地坐标模式**（`useLocalCoordinates() = true`）：bounds 是相对于父节点的，不自动传播

```java
// Figure.java:1390-1397 - primTranslate
protected void primTranslate(int dx, int dy) {
    bounds.x += dx;
    bounds.y += dy;

    if (useLocalCoordinates()) {
        // 本地坐标模式：不传播偏移到子节点
        fireCoordinateSystemChanged();
        return;
    }
    // 默认模式：递归传播偏移到所有子节点
    children.forEach(child -> child.translate(dx, dy));
}
```

**效果**：默认模式下，父节点移动时，子节点自动跟着移动，最终所有节点的 bounds 都是**绝对坐标**。

## 2. Bounds 的具体作用

Bounds 在 Draw2D 中有多种关键用途：

| 用途 | 代码示例 | 说明 |
|------|----------|------|
| **命中测试** | `containsPoint(x, y) → bounds.contains(x, y)` | 判断点是否在图形内 |
| **绘制位置** | `graphics.fillRectangle(getBounds())` | 绘制图形的背景 |
| **裁剪区域** | `clippingStrategy.getClip(child)` | 确定子节点的绘制区域 |
| **重绘区域** | `repaint(getBounds())` | 需要重绘的区域 |
| **坐标转换** | `translateFromParent/ToParent` | 父子坐标系转换 |
| **布局计算** | `layout.setConstraints(this, constraint)` | 布局管理器计算位置 |

### 2.1 绘制中的使用

```java
// Figure.java:1373-1375 - paintFigure
protected void paintFigure(Graphics graphics) {
    if (isOpaque()) {
        graphics.fillRectangle(getBounds());  // 使用绝对坐标绘制
    }
}

// Figure.java:1333-1334 - paintClientArea (子节点绘制)
if (useLocalCoordinates()) {
    graphics.translate(getBounds().x, getBounds().y);  // 移动到子节点位置
    // ... 绘制子节点
}
```

### 2.2 命中测试中的使用

```java
// Figure.java:367-368 - containsPoint
public boolean containsPoint(int x, int y) {
    return getBounds().contains(x, y);  // x, y 是绝对坐标
}

// Figure.java:500-517 - findMouseEventTargetInDescendantsAt
protected IFigure findMouseEventTargetInDescendantsAt(int x, int y) {
    PRIVATE_POINT.setLocation(x, y);
    translateFromParent(PRIVATE_POINT);  // 转换到父节点坐标系

    if (!getClientArea(Rectangle.SINGLETON).contains(PRIVATE_POINT)) {
        return null;  // 剪枝：不在父节点内，跳过
    }

    for (IFigure fig : getChildrenRevIterable()) {  // 逆序遍历
        if (fig.containsPoint(x, y)) {  // 检查子节点 bounds
            fig = fig.findMouseEventTargetAt(x, y);
            if (fig != null) return fig;
        }
    }
    return this;
}
```

### 2.3 坐标转换

```java
// 绝对坐标 → 父节点坐标系
public void translateFromParent(Translatable t) {
    if (useLocalCoordinates()) {
        t.performTranslate(-bounds.x - insets.left, -bounds.y - insets.top);
    }
}

// 父节点坐标系 → 子节点坐标系
public void translateToParent(Translatable t) {
    if (useLocalCoordinates()) {
        t.performTranslate(bounds.x + insets.left, bounds.y + insets.top);
    }
}

// 相对坐标 → 绝对坐标
public final void translateToAbsolute(Translatable t) {
    if (getParent() != null) {
        Translatable tPrecise = toPreciseShape(t);
        getParent().translateToParent(tPrecise);
        getParent().translateToAbsolute(tPrecise);
        fromPreciseShape(tPrecise, t);
    }
}
```

## 3. 坐标根（Coordinate Root）

### 3.1 定义

**坐标根**是视图树中最近的使用 `useLocalCoordinates() = true`（或 `isCoordinateSystem() = true`）的祖先节点。

### 3.2 形式化定义

```
absolute_bounds(F) =
    如果 P = 最近的使用 isCoordinateSystem() = true 的祖先(F)
    则 absolute_bounds(F) = relative_bounds(F) 相对于 P
    否则 absolute_bounds(F) = relative_bounds(F) 相对于画布原点
```

其中：
- `F` 是当前 Figure
- `P` 是 `F` 的祖先中最近的使用 `isCoordinateSystem() = true` 的节点
- 如果没有这样的祖先 `P`，则相对于画布原点

### 3.3 关键源码

```java
// IFigure.java:662-670
/**
 * Returns <code>true</code> if this figure is capable of applying a local
 * coordinate system which affects its children.
 *
 * @since 3.1
 * @return <code>true</code> if this figure provides local coordinates to
 *         children
 */
boolean isCoordinateSystem();

// Figure.java:1128-1133
@Override
public boolean isCoordinateSystem() {
    return useLocalCoordinates();
}

// Figure.java:2045-2048
@Override
public void translateFromParent(Translatable t) {
    if (useLocalCoordinates()) {
        t.performTranslate(-getBounds().x - getInsets().left,
                           -getBounds().y - getInsets().top);
    }
}
```

### 3.4 常见坐标根

在 Draw2D 中，以下容器类默认是坐标根：

| 类名 | 用途 |
|------|------|
| `ScalableFreeformLayeredPane` | 可缩放的自由表单层叠面板 |
| `Viewport` | 视口，支持滚动 |
| `FreeformLayer` | 自由表单层 |
| `FreeformLayeredPane` | 自由表单层叠面板 |
| `ScalableLayeredPane` | 可缩放层叠面板 |

```java
// ScalableFreeformLayeredPane.java:67-72
@Override
public boolean isCoordinateSystem() {
    return true;  // 总是作为坐标根
}

// Viewport.java:172-177
@Override
public boolean isCoordinateSystem() {
    return useGraphicsTranslate() || super.isCoordinateSystem();
}
```

### 3.5 图示

```
画布原点 (0,0)
    │
    ├── ScalableFreeformLayeredPane (isCoordinateSystem() = true) ─┐
    │     │                                                     │
    │     ├── Panel A (useLocalCoordinates = false) ──────┐     │
    │     │     │                                           │     │
    │     │     └── Button X (bounds: 10,10,100,50) ───────┼─────┘
    │     │                                                   │
    │     └── Panel B (useLocalCoordinates = true) ─────────┼──┐
    │           │                                            │  │
    │           └── Button Y (bounds: 20,20,100,50) ─────────┼──┤
    │                                                          │  │
    └── Viewport (isCoordinateSystem() = true) ────────────────┘  │
                                                                │
    Button X 的绝对坐标 = (10, 10)                               │
    Button Y 的绝对坐标 = (20, 20) 相对于 Panel B，而非画布原点  │
```

### 3.6 总结表

| 问题 | 答案 |
|------|------|
| bounds 存储什么？ | (x, y, width, height) - 位置和尺寸 |
| x, y 的含义是什么？ | **绝对坐标**（相对于坐标根） |
| 什么是坐标根？ | 使用 `useLocalCoordinates() = true` 的 Figure |
| 默认模式下如何变成绝对坐标？ | 父节点移动时自动 `translate()` 传播到子节点 |
| `useLocalCoordinates()` 的作用？ | 设为 `true` 时，bounds 是相对坐标，不自动传播 |
| 命中测试的坐标？ | 使用**绝对坐标**，点在全局坐标系中 |
| 绘制时如何使用 bounds？ | `graphics.fillRectangle(getBounds())` - 绝对坐标 |
| 子节点如何定位？ | 遍历时通过 `translateFromParent()` 转换坐标 |
| 常见坐标根有哪些？ | ScalableFreeformLayeredPane、Viewport、FreeformLayer 等 |
| 如果没有坐标根？ | 相对于画布原点 |

## 4. Bounds 与布局的关系

### 4.1 布局过程中的 bounds 设置

```java
// Figure.java:1674-1698 - setBounds
@Override
public void setBounds(Rectangle rect) {
    int x = bounds.x, y = bounds.y;

    boolean resize = (rect.width != bounds.width) || (rect.height != bounds.height);
    boolean translate = (rect.x != x) || (rect.y != y);

    if ((resize || translate) && isVisible()) {
        erase();  // 擦除旧位置
    }
    if (translate) {
        int dx = rect.x - x;
        int dy = rect.y - y;
        primTranslate(dx, dy);  // 移动 bounds 并传播到子节点
    }

    bounds.width = rect.width;
    bounds.height = rect.height;

    if (translate || resize) {
        if (resize) {
            invalidate();  // 使布局无效，需要重新布局
        }
        fireFigureMoved();
        repaint();  // 在新位置重绘
    }
}
```

### 4.2 布局管理器的作用

布局管理器负责计算子 Figure 的 bounds：

```java
// LayoutManager 接口
public interface LayoutManager {
    void layout(IFigure container);
    Object getConstraint(IFigure child);
    void setConstraint(IFigure child, Object constraint);
}
```

布局计算完成后，每个子 Figure 的 bounds 被设置为相对于父 Figure 的位置。

## 5. Bounds 与命中的测试流程

### 5.1 命中测试的完整流程

```
点击点 (global_x, global_y)
        │
        ▼
┌─────────────────────────────────────┐
│ findMouseEventTargetAt              │
│ 坐标转换: global → parent           │
│ containsPoint(parent)?              │
└─────────────────────────────────────┘
        │
        ▼
    逆序遍历 children
        │
        ├─→ Child1 (后添加，在上层)
        │       │
        │       ▼
        │   containsPoint(child1)?
        │       │
        │       ├──→ 是 → 递归检测 Child1 的子节点
        │       └──→ 否 → 继续下一个
        │
        └─→ Child2 (先添加，在下层)
                │
                ▼
            containsPoint(child2)?
                │
                └──→ 否 → 继续

返回最深层的命中节点
```

### 5.2 关键点

1. **逆序遍历**：`getChildrenRevIterable()` 确保后添加的节点（视觉上层）先被检测
2. **坐标转换**：使用 `translateFromParent()` 将全局坐标转换为本地坐标
3. **剪枝**：`!getClientArea().contains(point)` 跳过不在父节点内的整个子树
4. **递归**：找到最深层的命中节点即返回

## 6. 参考源码

| 文件 | 主要内容 |
|------|----------|
| `Figure.java` | 核心 Figure 类，包含 bounds 操作 |
| `IFigure.java` | Figure 接口，定义坐标相关方法 |
| `ScalableFreeformLayeredPane.java` | 可缩放自由表单层叠面板 |
| `Viewport.java` | 视口实现 |
| `FreeformLayer.java` | 自由表单层 |
| `LayoutManager.java` | 布局管理器接口 |

---

*本文档基于 Eclipse Draw2D 源码分析，版本：org.eclipse.draw2d.source_3.10.100.201606061308*
