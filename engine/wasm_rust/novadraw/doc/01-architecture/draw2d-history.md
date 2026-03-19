# Eclipse Draw2D 演进历史分析

## 项目起源

- **创建时间**: 2000 年
- **创建者**: IBM Corporation
- **许可证**: Eclipse Public License 2.0
- **最早 Git Commit**: 2002-06-04 (Commit ID: `4da32d966f3986a1313518d9e02489764d11b8dd`)

**注意**: 虽然 Git 历史显示最早提交为 2002 年，但源代码文件中的版权声明为 "Copyright (c) 2000"，说明项目实际始于 2000 年，2002 年是代码迁移到当前版本控制系统的时间点。

## 核心架构设计

Draw2D 的设计深受 **Smalltalk 图形系统** 影响，采用以下核心模式：

1. **轻量级组件模型**: Figure 不继承 SWT Widget，独立渲染
2. **组合模式 (Composite Pattern)**: 支持图形嵌套形成树形结构
3. **策略模式 (Strategy Pattern)**: 布局管理器可插拔
4. **观察者模式 (Observer Pattern)**: 属性变更通知机制

## 最初版本 (2000-2002) 的核心能力

### 1. 核心图形系统

```
├── Figure (核心基类)
├── IFigure (接口契约)
├── LightweightSystem (连接 SWT Canvas 与 Draw2d)
└── UpdateManager (管理重绘和更新)
```

**关键类**:
- `Figure`: 所有可视对象的根类，包含边界、布局、子图形管理
- `IFigure`: 定义图形的基本契约
- `LightweightSystem`: SWT 与 Draw2D 之间的桥梁
- `UpdateManager`: 管理图形更新和重绘

### 2. 几何系统

```
├── Rectangle (矩形区域)
├── Point (点坐标)
├── Dimension (尺寸)
├── Insets (边距)
└── Translatable (坐标变换接口)
```

**关键特性**:
- 整数坐标系统 (区别于浮点)
- 支持坐标变换
- 边界计算和包含检测

### 3. 布局管理器 (原始版本)

| 布局器 | 约束类型 | 用途 |
|--------|----------|------|
| **XYLayout** | Rectangle (x, y, width, height) | 绝对定位 |
| **BorderLayout** | Integer (TOP/LEFT/CENTER/RIGHT/BOTTOM) | 五区边框布局 |
| **FlowLayout** | (无约束) | 流式排列 |
| **DelegatingLayout** | Locator | 子图形自定位 |

**核心接口**:
- `LayoutManager`: 定义布局契约
- `AbstractLayout`: 布局基类
- `AbstractConstraintLayout`: 约束布局基类

### 4. 基础图形实现

```
├── RectangleFigure (矩形)
├── Ellipse (椭圆)
├── Polyline (折线)
├── Polygon (多边形)
├── Label (文本标签)
└── ImageFigure (图像)
```

### 5. 连接与路由系统

```
├── Connection (连接线接口)
├── ConnectionAnchor (连接锚点接口)
├── AbstractConnectionAnchor (锚点基类)
├── ChopboxAnchor (边界框锚点)
└── AbstractRouter (路由基类)
```

**关键特性**:
- 锚点用于定义连接的附着点
- 路由器负责计算连接线路径
- 支持正交和折线路由

### 6. 事件处理系统

```
├── EventDispatcher (事件分发)
├── MouseListener/MouseMotionListener (鼠标事件)
├── KeyListener (键盘事件)
├── FigureListener (图形变化通知)
├── CoordinateListener (坐标变换通知)
└── PropertyChangeListener (属性变更通知)
```

### 7. 基础渲染支持

```
├── Graphics (抽象绘制上下文接口)
├── SWTGraphics (基于 SWT GC 的实现)
├── ColorProvider (颜色管理)
└── Font/Color 资源管理
```

## 演进时间线

| 时期 | 关键新增能力 |
|------|-------------|
| **2000-2002** | 核心架构建立，基础图形、布局、连接系统 |
| **2002-2004** | GridLayout 加入，图形算法库扩充，动画支持 |
| **2004-2006** | 文本布局增强 (Flow 文本)，Zest 可视化组件 |
| **2006+** | GEF 与 draw2d 分离，独立发展 |

## 设计遗产

Draw2D 的原始设计影响了后续众多图形框架：

1. **GEF (Graphical Editing Framework)**: 基于 draw2d 构建可视化编辑器
2. **Zest**: 可视化工具包 (图表、网络图)
3. **Sirius**: 基于 EMF 的建模工具
4. **Graphene**: Eclipse 新一代图形框架

其核心思想——**轻量级图形、组合模式、布局分离**——至今仍是图形 UI 设计的最佳实践。

## 相关 Git Commit 信息

| 信息项 | 值 |
|--------|-----|
| 最早 Commit ID | `4da32d966f3986a1313518d9e02489764d11b8dd` |
| 最早 Commit 日期 | 2002-06-04 |
| 当前 HEAD Commit | `4463d9d0ce13c19d10fbe769d29f28b7345a8cba` |
| 项目版权声明 | Copyright (c) 2000, IBM Corporation |

---

*文档创建: 基于对 Eclipse GEF 代码库 git://git.eclipse.org/gitroot/gef/org.eclipse.gef.git 的分析*
