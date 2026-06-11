# Novadraw Goal Roadmap

> 工作流判定"任务是否对原目标负责"的**人读 SSOT**。
> 机器可读的结构化 milestone 数据见 `agent/draw2d-core-milestones.yaml`。
> 两份文件互相引用：本文解释"为什么 / 何时"，YAML 定义"如何度量"。

## 原目标（来自 CLAUDE.md）

> "使用 Rust + WebGPU 技术栈实现的高性能绘图引擎工具包，参考 eclipse draw2d/GEF 架构设计，**目标是扩展为通用图形框架**。"

## Year 1 主题

**毕业演示（M11）**：可工作的节点编辑器 demo — 创建/拖拽/连接/删除节点，验证 M1-M10 全部能力串联可用。

**总测试增量目标**：+1,200（当前 146 → ~1,350）。

**Year 1 收尾判据**：M1-M10 全部 done + M11 至少 50% 交互流程跑通（不要求 60fps 全过）。

## 与 milestones YAML 的关系

- **本文件（MD）**：人读、解释性、按"用户感知优先级"组织（视觉 → 能力 → 架构 → 收口）
- **`draw2d-core-milestones.yaml`**：机器可读、按"draw2d 自底向上依赖"组织、5 状态机（`not_started` / `in_progress` / `contract_aligned` / `behavior_verified` / `complete`）
- **同步原则**：每个 milestone 完成时，**两个文件都更新**；状态字段必须保持一致

## Milestone 编号映射（MD ↔ YAML）

| MD | YAML | 标题（MD） | 标题（YAML） | 差异说明 |
|----|------|-----------|-------------|----------|
| M0 | M0 | 几何基础 | 目标与验收框架 | MD 把几何当 M0，YAML 把 M0 当作 workflow_prerequisite |
| M1 | M1 | Figure 协议 + 5 图元 | 几何与 Graphics 基础 | 标题对调：MD 的 Figure 协议 = YAML 的 M2 |
| M2 | M2 | 容器 + 6 布局 | Figure 树与盒模型 | MD 标题强调布局，YAML 强调盒模型与树 |
| M3 | M3 | 边框系统 | 绘制遍历与裁剪闭环 | 内容主题不同 |
| M4 | M4 | 文本 + 图像 | 坐标域与变换闭环 | 内容主题不同 |
| M5 | M5 | UpdateManager 端到端 | Layout + Validation + UpdateManager | 范围不同（MD 更聚焦 update，YAML 包含 layout） |
| M6 | M6 | 事件系统 | 事件分发与交互状态机 | 标题一致 |
| M7 | M7 | LayeredPane + Freeform | 通知语义分层 | 内容主题不同 |
| M8 | M8 | ScrollPane 完整 | Viewport / Scroll / Zoom | 范围接近 |
| M9 | M9 | Connection 体系（简化版） | Connection / Anchor / Router | 范围接近 |
| **M10** | **M10** | **Tooltip + Accessible 基础** | **常用 Figure 与文本/控件** | **⚠️ 内容差异最大，待用户决策** |
| M11 | — | 节点编辑器毕业 demo | — | MD 独有 |

> **M10 冲突说明**：本文 M10 = Tooltip + Accessible；YAML M10 = 常用 Figure + 文本（MD 的 M4 内容）。
> 解决方向二选一：
> 1. 重新编号本文 M10 → M12（demo 前置），YAML M10 内容合并入本文 M1
> 2. 保持现状，承认两文件 M10 含义不同，服务不同目的（MD = 主题年，YAML = 依赖序列）
> 当前**默认走方向 2**，避免改文件时连带改 milestone 编号影响 delta 引用。

## Status 严格语义（本文专用）

- `pending` → 首个 delta 启动前
- `in_progress` → 至少一个 delta 启动，且内部尚有 delta 未 `done`
- `done` → 内部所有 delta `done` 且 Done When 全部满足

> 本文的 `done` ≠ YAML 的 `complete`。YAML 还要求 `contract_aligned` + `behavior_verified` 之后才能 `complete`。

---

## 里程碑（12 个）

### M0 — 几何基础
- **Status**: pending
- **能力**: `Point` / `PrecisionPoint` / `Rectangle` / `PrecisionRectangle` / `Dimension` / `Insets` / `Vector` / `Transform` / `AffineTransform`
- **交付**: 类型补齐报告、跨包一致测试、draw2d 等价性测试
- **Done When**: `novadraw-geometry` 公开 API 与 draw2d `geometry` 包名 + 行为对齐
- **测试增量**: +30
- **依赖**: 无

### M1 — Figure 协议 + 基础图形
- **Status**: pending
- **能力**:
  - `RectangleFigure` / `EllipseFigure` / `PolygonFigure`（3 基础）
  - `RoundedRectangleFigure` / `TriangleFigure`（2 常用）
  - `paintFigure` / `paintBorder` / `paintClientArea` 三段式
- **交付**: `apps/shapes-demo`（5 图元 + paint 分层可视化）
- **Done When**: 5 图元 + 三段式 paint + 截图断言全过
- **测试增量**: +60
- **依赖**: M0

### M2 — 容器 + 布局系统
- **Status**: pending
- **能力**: `Container` 复合图元；`LayoutManager` trait；`FlowLayout`（验证）/`BorderLayout`（验证）/`GridLayout` / `ToolbarLayout` / `XYLayout` / `StackLayout`；`LayoutConstraint`
- **交付**: `apps/layouts-demo`（6 布局并排对比 + draw2d 反向等价测试）
- **Done When**: 6 布局 + 容器 + 约束 + demo 截图全过
- **测试增量**: +150
- **依赖**: M1

### M3 — 边框系统
- **Status**: pending
- **能力**: `LineBorder`（验证）/ `MarginBorder`（验证）/ `TitleBarBorder` / `CompoundBorder` / `EtchedBorder` / `BevelBorder` / `FocusedBorder`；`BorderedFigure` 协议
- **交付**: `apps/borders-demo`
- **Done When**: 6 边框 + 复合 + demo 全过
- **测试增量**: +80
- **依赖**: M1

### M4 — 文本 + 图像
- **Status**: pending
- **能力**: `Label`（cosmic-text 集成）/ 文本布局 + 截断 + align / `ImageFigure` / 字体管理 / 文本与边框布局交互
- **交付**: `apps/text-image-demo`
- **Done When**: 文本渲染 + 图像 + 交互 + demo 全过
- **测试增量**: +100
- **依赖**: M1, M2, M3

### M5 — UpdateManager 端到端
- **Status**: pending
- **能力**: 区域失效 / clip 失效 / 整图失效 / repaint coalescing / damage repair 全图元覆盖 / 1k+ Figure stress
- **交付**: `apps/update-stress-demo`（帧率 + 区域断言）
- **Done When**: 三种粒度 + 全图元 + stress + 帧率断言全过
- **测试增量**: +120
- **依赖**: M1-M4

### M6 — 事件系统
- **Status**: pending
- **能力**: `MouseListener` / `MouseMotionListener` / `KeyListener` / `FocusListener` / `FigureListener`（bounds/visibility/add/remove）/ hit-test 全图元覆盖
- **交付**: `apps/events-demo`
- **Done When**: 4 类监听 + hit-test + demo 全过
- **测试增量**: +100
- **依赖**: M1, M2

### M7 — LayeredPane + Freeform
- **Status**: pending
- **能力**: `LayeredPane`（z-order paint）/ `Layer` / `FreeformFigure` / `FreeformLayer` / `FreeformLayout` / auto-extend bounds
- **交付**: `apps/layered-pane-demo`
- **Done When**: 分层 + 自由形式 + auto-extend + demo 全过
- **测试增量**: +80
- **依赖**: M2, M5

### M8 — ScrollPane 完整
- **Status**: pending
- **能力**: `ScrollPane` / `ScrollBar` (H+V) / 鼠标滚轮 / `AutoexposeHelper` / **收口 AD-018 暂停的 `apps/viewport-app` 4 场景可视化**
- **交付**: `apps/scroll-pane-demo` + `apps/viewport-app` 4 场景全过
- **Done When**: ScrollBar + 滚轮 + autoexpose + 4 场景可视化全过
- **测试增量**: +100
- **依赖**: M7

### M9 — Connection 体系（简化版）
- **Status**: pending
- **能力**: `PolylineConnection` / `ConnectionAnchor`（Chopbox / Ellipse / Slope / Label）/ `ConnectionRouter`（Bendpoint / Manhattan / Fan）/ **不做** `ShortestPathRouter`
- **交付**: `apps/connections-demo`（4 anchor × 3 router 组合）
- **Done When**: Connection + Anchor + 3 Router + demo 全过
- **测试增量**: +150
- **依赖**: M2, M5

### M10 — Tooltip + 可访问性基础
- **Status**: pending
- **能力**: `TooltipHelper` / 悬停延迟 + show/hide / `Accessible` bridge **基础**（接口骨架 + 键盘可达性，不做完整 ARIA/无障碍属性）
- **交付**: `apps/tooltip-demo`
- **Done When**: Tooltip + Accessible 基础 + demo 全过
- **测试增量**: +40
- **依赖**: M6

### M11 — 节点编辑器毕业 demo
- **Status**: pending
- **能力**: 创建节点 / 拖拽 / 连接 / 删除 / 滚动 + 缩放 / Tooltip
- **交付**: `apps/node-editor-demo`（可视化断言 + 60fps / 100 节点性能基线）
- **Done When**: 全部交互 + 60fps 性能 + 可视化全过
- **测试增量**: +200
- **依赖**: M1-M10 全部

---

## Draw2d Parity Checklist

- [ ] IFigure 协议（M1）
- [ ] 基础图元 Rectangle / Ellipse / Polygon（M1）
- [ ] 常用图元 RoundedRectangle / Triangle（M1）
- [ ] 容器 + 6 种布局（M2）
- [ ] 6 种边框（M3）
- [ ] 文本 + 图像（M4）
- [ ] UpdateManager 端到端（M5）
- [ ] 事件系统（M6）
- [ ] LayeredPane（M7）
- [ ] Freeform（M7）
- [ ] ScrollPane（M8）
- [ ] Connection + Anchor + Router（M9）
- [ ] Tooltip（M10）
- [ ] Accessible bridge 基础（M10）
- [ ] 节点编辑器毕业 demo（M11）

## Demo Checklist

- [ ] M1 `apps/shapes-demo`
- [ ] M2 `apps/layouts-demo`
- [ ] M3 `apps/borders-demo`
- [ ] M4 `apps/text-image-demo`
- [ ] M5 `apps/update-stress-demo`
- [ ] M6 `apps/events-demo`
- [ ] M7 `apps/layered-pane-demo`
- [ ] M8 `apps/scroll-pane-demo` + `apps/viewport-app`
- [ ] M9 `apps/connections-demo`
- [ ] M10 `apps/tooltip-demo`
- [ ] M11 `apps/node-editor-demo`

## 状态快照（2026-06-11）

- **已完成**: 无（AD-019B Viewport 集成已 verify 但视觉验证暂停，状态记为 partial）
- **进行中**: 无
- **总测试数**: 146
- **目标测试数**: ~1,350
- **完成进度**: 0 / 12
- **阻塞项**:
  - `INBOX-20260610-01` Viewport 视觉验证暂停（M8 阻塞）
  - M10 含义与 YAML 冲突，默认走方向 2（双 M10 各自服务不同目的）

## 状态更新规则

工作流每完成一个 milestone 必须：

1. 更新本文对应 `Status`: pending → in_progress → done
2. 在 Draw2d Parity Checklist 勾选
3. 在 Demo Checklist 勾选
4. 更新 `总测试数` 与 `完成进度`
5. **同步更新 `agent/draw2d-core-milestones.yaml` 对应项**（M10 例外，含义不同）
6. 若阻塞，记录到 `阻塞项` + `agent/interruptions-inbox.md`
