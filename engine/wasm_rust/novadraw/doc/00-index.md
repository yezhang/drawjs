# Novadraw 设计文档索引

本文档对 doc 目录下所有设计文档进行分类说明，便于查阅和理解。

---

## 文档分类结构

```text
doc/
├── 00-index.md                        # 文档索引（本文档）
│
├── 01-architecture/                  # 架构设计
│   ├── gef_principle.md              # GEF 框架核心原理
│   ├── displaylist_design.md          # DisplayList 中间层设计
│   ├── draw2d-history.md             # draw2d 历史与架构演变
│   └── swt-gc-analysis.md           # SWT GC 底层绘制 API 分析
│
├── 02-figure/                        # Figure 核心
│   ├── figure_core_concepts.md       # Figure 核心概念
│   ├── figure_bounds.md               # Figure 边界机制
│   ├── figure_tree_position.md       # Figure 树父子位置
│   ├── ifigure_interface.md          # g2 IFigure 接口分析
│   ├── figure_implementation.md      # g2 Figure 实现分析
│   ├── figure_tree_operations.md     # g2 Figure 树操作机制分析
│   ├── figure_box_model.md           # 盒模型分析
│   └── layout-constraints.md         # 布局约束机制
│
├── 03-rendering/                      # 渲染管线
│   ├── rendering_pipeline.md          # 渲染管线概览
│   ├── update_manager_pipeline.md    # g2 UpdateManager + 渲染管线分析
│   ├── update_manager_design.md      # Novadraw UpdateManager 设计
│   ├── trampoline_rendering.md       # Trampoline 渲染任务管理
│   ├── displaylist_detailed.md       # DisplayList 详细设计
│   ├── graphics_api.md               # Graphics API 参考
│   └── clip_principle.md             # Clip 裁剪原理
│
├── 04-coordinates/                   # 坐标系与变换
│   └── coordinates.md                 # 坐标系统原理
│
└── 05-java-rust/                     # Java to Rust 迁移
    ├── java_to_rust_oo.md            # Java OOP 特性等价实现
    └── java_to_rust_migration.md     # 迁移步骤指南 + 多态支持
```

---

## 文档详细说明

### 1. 架构设计

| 文档 | 主题 | 关键内容 |
|------|------|----------|
| `gef_principle.md` | GEF 框架架构 | MVC 模式、EditPart 控制器、Command 模式、Request/EditPolicy 机制、连接支持 |
| `displaylist_design.md` | DisplayList 设计 | crate 设计决策、协议定义、与渲染层解耦方案 |
| `draw2d-history.md` | draw2d 历史 | draw2d 架构演变、设计决策背景 |
| `swt-gc-analysis.md` | SWT GC 分析 | SWT GC 底层绘制 API、IServerOcr2d 接口 |

### 2. Figure 核心

| 文档 | 主题 | 关键内容 |
|------|------|----------|
| `figure_core_concepts.md` | Figure 基础 | Figure 接口、paint 流程、GeometryHolder、父子关系 |
| `figure_bounds.md` | 边界机制 | bounds 定义、preferredSize、validate 流程、布局触发 |
| `figure_tree_position.md` | 父子位置 | 坐标系转换、translateToParent/useLocalCoordinates、嵌套变换 |
| `ifigure_interface.md` | IFigure 接口 | 接口方法分类、设计意图、上帝接口模式分析 |
| `figure_implementation.md` | Figure 实现 | 核心数据结构、paint/setBounds 实现、关键设计模式 |
| `figure_tree_operations.md` | 树操作机制 | 树遍历方法分类、递归/迭代机制、传播方向分析 |
| `figure_box_model.md` | 盒模型 | Bounds/Insets/ClientArea/Border/Outline 关系 |
| `layout-constraints.md` | 布局约束 | 约束系统、LayoutManager 接口、布局约束机制 |

### 3. 渲染管线

| 文档 | 主题 | 关键内容 |
|------|------|----------|
| `rendering_pipeline.md` | 管线概览 | 三环节验证：IR 层、后端层、场景层 |
| `update_manager_pipeline.md` | g2 UpdateManager | g2 LightweightSystem、UM 两阶段、Figure.paint、Graphics、EventDispatcher |
| `update_manager_design.md` | Novadraw UpdateManager | Novadraw SceneUpdateManager 实现、设计决策、与 g2 差异 |
| `trampoline_rendering.md` | 任务遍历 | Trampoline 模式、任务队列、避免递归栈溢出 |
| `displaylist_detailed.md` | 详细实现 | RenderCommand 结构、场景图到命令的转换 |
| `graphics_api.md` | API 参考 | Graphics 状态管理、绘制 API、变换 API |
| `clip_principle.md` | 裁剪机制 | Clip 架构、LazyState、IServerOcr2d 接口 |

### 4. 坐标系与变换

| 文档 | 主题 | 关键内容 |
|------|------|----------|
| `coordinates.md` | 坐标系统 | 屏幕坐标、逻辑坐标、世界坐标、视口/内容变换、Figure 树变换 |

### 5. Java to Rust 迁移

| 文档 | 主题 | 关键内容 |
|------|------|----------|
| `java_to_rust_oo.md` | OOP 等价实现 | 20 种 Java OOP 特性与 Rust 对应关系 |
| `java_to_rust_migration.md` | 迁移指南 | 迁移步骤、多态调用支持、决策流程 |

---

## 阅读路径建议

### 新人入门

```text
1. gef_principle.md              # 理解整体架构
2. figure_core_concepts.md       # 理解 Figure 模型
3. coordinates.md                # 理解坐标系
```

### 渲染开发

```text
1. rendering_pipeline.md            # 了解管线结构
2. update_manager_pipeline.md      # g2 UpdateManager + EventDispatcher 机制
3. trampoline_rendering.md        # 理解遍历机制
4. graphics_api.md               # 熟悉绘图 API
5. displaylist_detailed.md       # 深入实现细节
```

### 特性开发

| 场景 | 推荐文档 |
|------|----------|
| 裁剪功能 | `clip_principle.md` |
| 边界布局 | `figure_bounds.md` + `figure_tree_position.md` |
| 连接线 | `gef_principle.md` (连接章节) + `graphics_api.md` |
| 撤销重做 | `gef_principle.md` (Command 章节) |
| UpdateManager | `update_manager_pipeline.md` (g2 参考) + `update_manager_design.md` (本项目) |

### Java to Rust 迁移

```text
1. java_to_rust_oo.md              # 理解 Java OOP 在 Rust 中的等价物
2. java_to_rust_migration.md       # 掌握迁移步骤和多态支持
3. ifigure_interface.md            # 分析源接口
4. figure_implementation.md        # 分析实现类
```

---

## 文档命名规范

- 使用小写字母和下划线
- 语义清晰，体现主题
- 避免过长的文件名
- 相关文档使用相似前缀以便分组

---

## 贡献指南

新增文档时请：

1. 确定所属分类，放置到对应子目录
2. 遵循现有文档的格式风格
3. 通过 markdownlint 检查
4. 更新本文档索引
