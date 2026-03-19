# Eclipse Draw2D Figure 核心概念全景图

本文档系统整理 Eclipse Draw2D Figure 系统的核心概念，帮助理解 Figure 的设计原理和实现要点。

## 1. 核心概念全景图

```
┌─────────────────────────────────────────────────────────────────────────┐
│                          Figure 核心概念                                 │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                         │
│  ┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐     │
│  │  树状结构       │    │  Bounds 系统    │    │  坐标系统       │     │
│  │  Tree Structure │    │  Bounds System  │    │  Coordinate     │     │
│  ├─────────────────┤    ├─────────────────┤    ├─────────────────┤     │
│  │ • parent/child  │    │ • bounds (Rect) │    │ • local         │     │
│  │ • sibling       │    │ • setBounds()   │    │ • parent        │     │
│  │ • Z-order       │    │ • insets        │    │ • absolute      │     │
│  │ • add/remove    │    │ • clientArea    │    │ • coordinate    │     │
│  │                 │    │                 │    │   root          │     │
│  └────────┬────────┘    └────────┬────────┘    └────────┬────────┘     │
│           │                      │                      │               │
│           └──────────────────────┼──────────────────────┘               │
│                                  ▼                                      │
│  ┌─────────────────────────────────────────────────────────────────┐   │
│  │                    七阶段渲染流程                                 │   │
│  │  InitProperties → EnterState → PaintSelf → ResetState           │   │
│  │                 → PaintChildren → PaintBorder → ExitState       │   │
│  └─────────────────────────────────────────────────────────────────┘   │
│                                  │                                      │
│           ┌──────────────────────┼──────────────────────┐               │
│           ▼                      ▼                      ▼               │
│  ┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐     │
│  │  布局管理器     │    │  命中测试       │    │  事件通知       │     │
│  │  LayoutManager  │    │  Hit Test       │    │  Event System   │     │
│  ├─────────────────┤    ├─────────────────┤    ├─────────────────┤     │
│  │ • layout()      │    │ • containsPoint │    │ • figureMoved   │     │
│  │ • invalidate()  │    │ • findFigureAt  │    │ • coordinate    │     │
│  │ • revalidate()  │    │ • TreeSearch    │    │   changed       │     │
│  └─────────────────┘    └─────────────────┘    └─────────────────┘     │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
```

## 2. 树状结构 (Tree Structure)

### 2.1 基本概念

Figure 系统采用树形层次结构，每个 Figure 可以有零个或多个子节点：

```java
// Draw2D 源码结构
public class Figure {
    protected Figure parent;
    protected List<IFigure> children = new ArrayList<>();
}
```

### 2.2 Z-order 机制

后添加的子节点在视觉上位于上层（遮挡先添加的）：

```java
// 添加顺序：A → B → C
// 渲染顺序：A → B → C（先添加的先渲染，在下层）
// 视觉层级：C 在最上层（遮挡 B 和 A）

// 命中测试时逆序遍历，后添加的优先命中
for (IFigure fig : getChildrenRevIterable()) {
    if (fig.containsPoint(x, y)) {
        return fig.findMouseEventTargetAt(x, y);
    }
}
```

### 2.3 层次遍历（禁止递归）

Draw2D 使用迭代器模式遍历，避免递归栈溢出：

```java
// 禁止：递归遍历
void visit(Figure node) {
    visit(node.children);
}

// 正确：使用栈迭代
void visitIterative(Figure root) {
    Stack<Figure> stack = new Stack<>();
    stack.push(root);
    while (!stack.isEmpty()) {
        Figure node = stack.pop();
        stack.addAll(node.getChildren());  // 逆序入栈保持正序处理
    }
}
```

## 3. Bounds 系统 (Bounds System)

### 3.1 Bounds 的定义与含义

```java
// Figure.java:77
protected Rectangle bounds = new Rectangle(0, 0, 0, 0);
```

Bounds 是一个 `Rectangle`，存储 Figure 的位置和尺寸：(x, y, width, height)。

**关键理解**：bounds 的 (x, y) 是**绝对坐标**，但这个"绝对"是相对于**坐标根**的。

### 3.2 两种坐标模式

| 模式 | useLocalCoordinates() | bounds 含义 | 传播行为 |
|------|----------------------|-------------|----------|
| 默认模式 | `false` | 绝对坐标（相对于坐标根） | 父节点移动时自动传播 |
| 本地坐标模式 | `true` | 相对坐标（相对于父节点） | 不自动传播 |

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

### 3.3 Bounds 的具体作用

| 用途 | 代码示例 | 说明 |
|------|----------|------|
| 命中测试 | `containsPoint(x, y) → bounds.contains(x, y)` | 判断点是否在图形内 |
| 绘制位置 | `graphics.fillRectangle(getBounds())` | 绘制图形的背景 |
| 裁剪区域 | `clippingStrategy.getClip(child)` | 确定子节点的绘制区域 |
| 重绘区域 | `repaint(getBounds())` | 需要重绘的区域 |
| 坐标转换 | `translateFromParent/ToParent` | 父子坐标系转换 |
| 布局计算 | `layout.setConstraints(this, constraint)` | 布局管理器计算位置 |

### 3.4 setBounds() 完整语义

```java
// Figure.java:1674-1698
@Override
public void setBounds(Rectangle rect) {
    int x = bounds.x, y = bounds.y;

    boolean resize = (rect.width != bounds.width) || (rect.height != bounds.height);
    boolean translate = (rect.x != x) || (rect.y != y);

    // 1. 擦除旧位置（如果可见且位置/大小变化）
    if ((resize || translate) && isVisible()) {
        erase();
    }

    // 2. 移动 bounds 并传播到子节点
    if (translate) {
        int dx = rect.x - x;
        int dy = rect.y - y;
        primTranslate(dx, dy);
    }

    // 3. 更新宽高
    bounds.width = rect.width;
    bounds.height = rect.height;

    // 4. 布局失效和重绘
    if (translate || resize) {
        if (resize) {
            invalidate();  // 使布局无效，需要重新布局
        }
        fireFigureMoved();
        repaint();  // 在新位置重绘
    }
}
```

### 3.5 clientArea 与 insets

```java
// clientArea = bounds - insets
public Rectangle getClientArea(Rectangle rect) {
    rect.setBounds(bounds);
    rect.x += insets.left;
    rect.y += insets.top;
    rect.width -= insets.left + insets.right;
    rect.height -= insets.top + insets.bottom;
    return rect;
}
```

## 4. 坐标系统 (Coordinate System)

### 4.1 坐标层级

```
画布原点 (0,0)
    │
    ├── ScalableFreeformLayeredPane (isCoordinateSystem() = true) ──┐
    │     │                                                     │
    │     ├── Panel A (useLocalCoordinates = false) ──────┐     │
    │     │     │                                           │     │
    │     │     └── Button X (bounds: 10,10,100,50) ───────┼─────┘
    │     │                                                   │
    │     └── Panel B (useLocalCoordinates = true) ─────────┼──┐
    │           │                                            │  │
    │           └── Button Y (bounds: 20,20,100,50) ─────────┼──┤
    │                                                            │
    └── Viewport (isCoordinateSystem() = true) ────────────────┘  │
                                                                │
    Button X 的绝对坐标 = (10, 10)                               │
    Button Y 的绝对坐标 = (20, 20) 相对于 Panel B，而非画布原点  │
```

### 4.2 坐标转换方法

```java
// 绝对坐标 → 父节点坐标系
public void translateFromParent(Translatable t) {
    if (useLocalCoordinates()) {
        t.performTranslate(-bounds.x - insets.left, -bounds.y - insets.top);
    }
}

// 父节点坐标系 → 绝对坐标
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

### 4.3 坐标根 (Coordinate Root)

**坐标根**是视图树中最近的使用 `useLocalCoordinates() = true` 的祖先节点。

形式化定义：
```
absolute_bounds(F) =
    如果 P = 最近的使用 isCoordinateSystem() = true 的祖先(F)
    则 absolute_bounds(F) = relative_bounds(F) 相对于 P
    否则 absolute_bounds(F) = relative_bounds(F) 相对于画布原点
```

常见坐标根容器：

| 类名 | 用途 |
|------|------|
| `ScalableFreeformLayeredPane` | 可缩放的自由表单层叠面板 |
| `Viewport` | 视口，支持滚动 |
| `FreeformLayer` | 自由表单层 |
| `FreeformLayeredPane` | 自由表单层叠面板 |
| `ScalableLayeredPane` | 可缩放层叠面板 |

```java
// ScalableFreeformLayeredPane.java
@Override
public boolean isCoordinateSystem() {
    return true;  // 总是作为坐标根
}

// Viewport.java
@Override
public boolean isCoordinateSystem() {
    return useGraphicsTranslate() || super.isCoordinateSystem();
}
```

## 5. 七阶段渲染流程

### 5.1 流程图

```
┌─────────────────────────────────────────────────────────────────────┐
│                    七阶段渲染流程                                     │
├─────────────────────────────────────────────────────────────────────┤
│                                                                     │
│  1. InitProperties    → 初始化属性（子节点创建时调用）               │
│     │                                                               │
│     ▼                                                               │
│  2. EnterState        → 进入状态（设置 clip、transform）             │
│     │                                                               │
│     ▼                                                               │
│  3. PaintSelf         → 绘制自身内容                                 │
│     │                                                               │
│     ▼                                                               │
│  4. ResetState        → 重置状态（pop transform/clip）               │
│     │                                                               │
│     ▼                                                               │
│  5. PaintChildren     → 递归绘制子节点                               │
│     │                                                               │
│     ▼                                                               │
│  6. PaintBorder       → 绘制边框                                     │
│     │                                                               │
│     ▼                                                               │
│  7. ExitState         → 退出状态（清理资源）                         │
│                                                                     │
└─────────────────────────────────────────────────────────────────────┘
```

### 5.2 关键点

1. **Figure 只负责自身**：PaintChildren 由外部（SceneGraph）控制调用
2. **状态管理**：EnterState/ResetState 成对出现，管理 Graphics 状态
3. **PaintBorder 在子节点之后**：确保边框绘制在最上层

### 5.3 Draw2D 源码实现

```java
// Figure.java:1307-1329
public final void paint(Graphics graphics) {
    paintFigure(graphics);  // 3. PaintSelf
    paintChildren(graphics);  // 5. PaintChildren（由外部调用）

    // Figure 自身不直接调用 PaintBorder
}

// 外部（LightweightSystem）控制完整流程
public void paint() {
    initProperties();      // 1. InitProperties
    erase();               // 擦除
    paintFigure(graphics); // 3. PaintSelf
    paintBorder(graphics); // 6. PaintBorder（选中高亮等）
}
```

## 6. 布局管理器 (LayoutManager)

### 6.1 LayoutManager 接口

```java
public interface LayoutManager {
    void layout(IFigure container);
    Object getConstraint(IFigure child);
    void setConstraint(IFigure child, Object constraint);
}
```

### 6.2 布局失效机制

```java
// Figure.java:1580-1590
protected boolean isValid = false;

public void invalidate() {
    isValid = false;
    if (getParent() != null) {
        getParent().invalidate();
    }
}

public void revalidate() {
    if (!isValid) {
        invalidate();
        getParent().layout();
        isValid = true;
    }
}
```

### 6.3 布局管理器类型

| 类型 | 用途 |
|------|------|
| `FillLayout` | 所有子元素填充容器 |
| `XYLayout` | 子元素在指定位置垂直堆叠 |
| `FlowLayout` | 水平流式排列 |
| `GridLayout` | 网格排列 |
| `BorderLayout` | 东南西北中布局 |

## 7. 命中测试 (Hit Test)

### 7.1 完整流程

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

### 7.2 关键点

1. **逆序遍历**：`getChildrenRevIterable()` 确保后添加的节点（视觉上层）先被检测
2. **坐标转换**：使用 `translateFromParent()` 将全局坐标转换为本地坐标
3. **剪枝**：`!getClientArea().contains(point)` 跳过不在父节点内的整个子树
4. **递归**：找到最深层的命中节点即返回

### 7.3 Draw2D 源码

```java
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

## 8. 事件通知 (Event System)

### 8.1 Figure 事件

| 事件 | 触发条件 | 用途 |
|------|----------|------|
| `fireFigureMoved()` | 位置或大小变化 | 通知父节点重新布局 |
| `fireCoordinateSystemChanged()` | 坐标根变化 | 通知子节点更新坐标 |
| `fireRequestLayout()` | 需要重新布局 | 请求父节点布局 |

### 8.2 FigureListener 接口

```java
public interface FigureListener {
    void figureMoved(IFigure source);
    void figureResized(IFigure source);
}
```

## 9. 核心概念依赖关系

```
                    ┌─────────────────────────────────────┐
                    │          Tree Structure             │
                    │     (parent/children Z-order)       │
                    └──────────────────┬──────────────────┘
                                       │
                    ┌──────────────────▼──────────────────┐
                    │          Bounds System              │
                    │    (bounds = 绝对坐标)              │
                    │    • setBounds()                    │
                    │    • erase/repaint                  │
                    └──────────────────┬──────────────────┘
                                       │
         ┌─────────────────────────────┼─────────────────────────────┐
         │                             │                             │
┌────────▼────────┐        ┌───────────▼──────────┐        ┌────────▼────────┐
│  Coordinate      │        │  Layout Manager      │        │  Event System   │
│  System          │        │  • invalidate()      │        │  • moved event  │
│  • primTranslate │        │  • revalidate()      │        │  • coord change │
│  • translate*()  │        │  • layout()          │        │                 │
└────────┬─────────┘        └───────────────────────┘        └─────────────────┘
         │                             │
         │                             ▼
         │              ┌───────────────────────────────┐
         │              │      Seven-Phase Render       │
         │              │   (Init → Enter → Paint → ...)│
         │              └───────────────────────────────┘
         │                             │
         │                             ▼
         │              ┌───────────────────────────────┐
         └─────────────►│         Hit Test              │
                        │  • containsPoint()            │
                        │  • findFigureAt()             │
                        │  • TreeSearch 过滤            │
                        └───────────────────────────────┘
```

## 10. 渐进式实现建议

### 10.1 实现优先级

| 优先级 | 概念 | 说明 |
|--------|------|------|
| P0 | 树状结构 | SceneGraph、parent/children |
| P1 | bounds 绝对坐标 | 统一坐标语义 |
| P2 | primTranslate | 坐标传播 |
| P3 | 七阶段渲染 | 渲染流程 |
| P4 | containsPoint | 命中测试 |
| P5 | setBounds | 位置设置 |
| P6 | translateFromParent | 坐标转换 |
| P7 | 布局管理器 | 可选，按需实现 |

### 10.2 手动布局 vs 自动布局

| 能力 | 手动布局 | 自动布局 |
|------|----------|----------|
| **位置设置** | `figure.set_bounds(rect)` | `layout.layout(container)` |
| **尺寸调整** | 手动计算并调用 set_bounds | 布局器自动计算 |
| **响应变化** | 外部监听并主动更新 | 自动监听 invalidate |
| **复杂度** | 低 | 高 |
| **适用场景** | 简单场景、自由绘图 | 表单、列表、规则排列 |

**建议**：编辑器类应用优先实现手动布局，LayoutManager 在真正需要时再添加。

## 11. 参考源码

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
