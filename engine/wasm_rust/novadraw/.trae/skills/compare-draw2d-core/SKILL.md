---
name: "compare-draw2d-core"
description: "Audits Novadraw against Draw2D core capabilities. Invoke when asked to compare current code with draw2d/GEF core or find missing parity."
---

# Compare Draw2D Core

把 Novadraw 当前实现与 Eclipse Draw2D 核心能力做结构化对标，输出缺口、证据、优先级和推荐下一步。

## When To Use

- 用户要求“检查本项目代码，对比 draw2d 核心能力还差什么”
- 用户要求盘点 Draw2D parity、核心能力缺口、M1-M10 完成情况
- 用户要求判断某个架构 delta 是否属于 Draw2D 核心能力缺失
- 完成一个 architecture/parity delta 后，需要重新评估剩余核心缺口

## Non Scope

- 不做 GEF 层能力评估：`EditPart`、`EditPolicy`、`Tool`、`Command`、`Request`、`Viewer`、Palette、Selection provider、Undo/Redo 不计入 Draw2D 核心缺口。
- 不直接实现修复。若用户要继续落地，切换到 `execute-architecture-delta` 或先创建/选择一个最小 delta。
- 不把 `render_iterative.rs` 或 I 键切换作为当前核心门禁；迭代渲染 POC 已归档，除非明确进入性能专项。

## Required Inputs

启动和边界：

- `AGENTS.md`
- `CLAUDE.md`
- 项目记忆：`project_memory.md` 或会话提供的 memory context

Draw2D 核心 SSOT：

- `agent/draw2d-core-milestones.yaml`
- `doc/01-architecture/draw2d_api_coverage.md`
- `agent/goal-roadmap.md`
- `doc/06-roadmap/product-deliverables.md`
- `doc/06-roadmap/demo-matrix.md`
- `doc/01-architecture/draw2d_design_axioms.md`
- `doc/01-architecture/draw2d_notification_design.md`

参考源码：

- `/Users/bytedance/Documents/code/GitHub/gef-classic/org.eclipse.draw2d/src/org/eclipse/draw2d`

Novadraw 代码入口按需检查：

- `novadraw-core/src/**`
- `novadraw-geometry/src/**`
- `novadraw-render/src/**`
- `novadraw-scene/src/figure/**`
- `novadraw-scene/src/graph/**`
- `novadraw-scene/src/layout/**`
- `novadraw-scene/src/runtime/**`
- `novadraw-scene/src/container/**`
- `apps/*/src/**`，仅用于确认是否有职责回流，不作为核心能力归属层

## Procedure

1. 完成启动门禁：读取 `AGENTS.md` 和 `CLAUDE.md`，确认本次是“现状对标/落地审计”，因此允许扫描本项目实现。
2. 明确对标边界：只评估 Draw2D 核心，不把 GEF 编辑框架能力计入缺口。
3. 读取 `agent/draw2d-core-milestones.yaml`，用 M1-M10 作为唯一编号来源，不发明新 milestone 编号。
4. 读取 `doc/01-architecture/draw2d_api_coverage.md`，按受影响 API family 检查语义覆盖，不只看类型名是否存在。
5. 读取 `goal-roadmap.md`、`product-deliverables.md`、`demo-matrix.md`，区分协议层、产品层、端到端验证层。
6. 对 Draw2D 参考源码做按需取证，至少覆盖以下 family：
   - `IFigure` / `Figure`
   - `Graphics` / `SWTGraphics`
   - `LayoutManager`
   - `EventDispatcher`
   - `UpdateManager` / `DeferredUpdateManager`
   - `Border`
   - `Connection` / `PolylineConnection` / Anchor / Router
   - Viewport / Scroll / Zoom 相关类
   - Label / Text / Image / Button-like reusable figures
7. 扫描 Novadraw 当前代码，按同样 family 建立现状矩阵。不要只读文档；每个关键判断至少给一个代码入口证据。
8. 对每个能力给出状态：
   - `verified`: 已有实现且有契约/测试/文档或 demo 证据
   - `partial`: 有骨架或部分行为，但缺语义闭环、测试或产品清单
   - `stub`: API 或命令存在，但后端/运行时未真正执行
   - `missing`: 没有可定位实现
   - `out_of_scope`: 属于 GEF 或其他非 Draw2D 核心层
9. 对 roadmap 状态和代码证据分开说明。代码提前存在不等于 milestone complete；`behavior_verified` 也不等于 `complete`。
10. 输出推荐推进顺序。默认优先级是 M4 坐标域、M5 Layout/Update、M6 Event、M7 Notification，再进入 M8 Viewport、M9 Connection、M10 reusable figures。

## Audit Matrix

至少覆盖这些审计项：

| Area | Draw2D Core Question | Typical Novadraw Evidence |
|------|----------------------|---------------------------|
| M1 Geometry / Graphics | 几何、Graphics 状态栈、clip/transform/text/image 是否可复用且后端可执行 | `novadraw-geometry`, `novadraw-render/src/context.rs`, `command.rs`, backend |
| M2 Figure Tree / Box Model | Figure、Block、Graph、bounds、insets、clientArea、visible/enabled、z-order 是否闭合 | `figure/mod.rs`, `graph/mod.rs` |
| M3 Paint / Clip | paint template、parent clientArea clip、child bounds clip、hit-test 是否一致 | `graph/render_recursive.rs`, graph tests |
| M4 Coordinates | coordinate root、translateToParent/Absolute/Relative、event point reduction 是否统一 | `figure/mod.rs`, `graph/mod.rs`, `runtime/event` |
| M5 Layout / Update | LayoutManager、constraints、preferred/min/max、Validation before Repair、damage repair 是否闭合 | `layout/**`, `runtime/update/**`, `graph/mod.rs` |
| M6 Event | mouse/capture/focus/hover/wheel/key/listeners 是否由引擎层分发 | `runtime/event/**`, `apps/editor/src/system.rs` |
| M7 Notification | Figure/Coordinate/Property/Ancestor/Layout/Input/UpdateListener 是否分层 | `runtime/update/listener.rs`, graph notification code |
| M8 Viewport | Viewport/ScrollPane/RangeModel/ScrollBar/ScalableFigure 是否进入 Figure tree 语义 | `container/viewport.rs`, demos |
| M9 Connection | Connection/Anchor/Router/Bendpoint/Decoration/ConnectionLayer 是否存在 | `novadraw-scene/src/**` search |
| M10 Reusable Figures | Label/Image/Text/Border/Button/Tooltip/Accessible 是否可用并走核心协议 | `figure/**`, `render/backend/**`, demos |

## Evidence Rules

- 使用 `rg` / `rg --files` 定位文件和符号。
- 输出中引用本地文件时给出文件路径和行号。
- 发现 “TODO / `_ => {}` / API 存在但未实现” 时，标为 `stub` 或 `partial`，不要写成已完成。
- 若某项只在 `apps/*` 层实现，检查是否违反“通用机制下沉引擎层”。
- 若某项在文档中标 `not_started`，但代码已有雏形，写成“代码提前存在，但 milestone 未闭环”。
- 若某项属于 GEF 层，明确标 `out_of_scope`，不要把它转成核心缺口。

## Output Format

输出使用中文，包含：

- Direct Result
  - 总体判断：当前离 Draw2D 核心完整还差什么
  - M1-M10 状态表：roadmap 状态、代码证据状态、主要缺口
  - Top Gaps：按阻塞程度排序的缺口
  - Existing Strengths：已经具备或接近闭合的能力
  - Evidence：关键文件和行号
  - Recommended Next Delta：建议下一个最小 delta
- Deeper Interaction
  - 哪些看似重要但不应先做
  - 是否存在职责回流或错误分层风险
  - 是否需要更新 `draw2d_api_coverage.md`、roadmap 或 backlog

## Guardrails

- 不要只基于 `goal-roadmap.md` 下结论，必须交叉检查代码。
- 不要只基于代码存在下结论，必须对齐 milestone、API 语义账本和 demo 完成规则。
- 不要混用旧 M 编号；所有 `M{n}` 必须指 `agent/draw2d-core-milestones.yaml`。
- 不要把节点编辑器、编辑策略、工具、命令栈计入 Draw2D 核心毕业判据。
- 不要修改源码或文档，除非用户明确要求把审计结果落成 backlog、roadmap 或 delta。
- 不要在渲染热路径中建议添加日志作为方案。
