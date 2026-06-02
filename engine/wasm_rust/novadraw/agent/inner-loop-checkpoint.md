# Session Checkpoint

## Metadata

- schema_version: 1
- updated_at: 2026-06-01
- checkpoint_kind: architecture-loop

## Current Delta

- AD-013

## Current Status

- review-complete（CAD-003 已提升为 AD-013，等待执行）

## What Was Done

### AD-006 通知体系基础设施（前一 delta，verified）
- UpdateListener 三方法 + effect 队列 + listener dispatch + 事务边界 flush
- 端到端链路打通（editor TraceUpdateListener）

### AD-007 坐标模型对齐（本轮）
- **根因分析**：渲染链路仍存在两个 client area 定义，`Bounded::client_area()` 未在坐标根下重置原点，render false 分支直接裁剪完整 bounds，可能让 children 绘制进入 border/insets 区域。
- **最小修复**：
  - `Bounded::client_area()` 成为统一 SSOT：坐标根返回 `(0,0,width,height)`，非坐标根返回 `bounds + insets`。
  - `FigureGraph::get_container_bounds()` 改为复用 `figure.client_area()`。
  - recursive / iterative 渲染路径的非坐标根分支统一按 client area 裁剪。
  - 新增测试覆盖坐标根 client area 原点归零，以及 recursive / iterative 渲染对非坐标根 insets 的 client-area 裁剪。
- **验证**：cargo test -p novadraw-scene ✅，136/136 tests + 3 doctests ✅

### 本轮会话额外工作（已纳入 AD-007）
- `bounds` 语义已从早期“全绝对坐标”过渡契约修正为 draw2d 的“相对最近坐标根的绝对值”。
- dirty / repair、hit-test、layout、mouse event dispatch、render client area 已逐步接入同一父链坐标协议。
- `MouseEvent::entry_point()` 只读保留入口域点，`MouseEvent.x/y` 在引擎层转换为 target/source Figure 坐标域点。
- `Viewport` 已从 `screen/world` 命名收口为 `viewport/content` 命名；`origin` 表示 viewport 左上角对应的 content 坐标。
- `Viewport::to_transform()` / `to_inverse_transform()` 已按 `viewport = (content - origin) * zoom` 修正非零 origin 下的矩阵组合。
- `Viewport` 当前仍是 standalone math helper；未来接入 Figure 树时必须通过 `translate_to_parent` / `translate_from_parent` 协议进入父链。

### AD-001C 更新调度边界（本轮，verified）
- **根因分析**：redraw / queue / next-cycle 的 transition 逻辑分散在多个 editor wrapper，`SceneHost` 文档仍描述未实现的 `about_to_wait` 每帧驱动，且 host queued flag 与 UpdateManager queued flag 缺少自愈同步路径。
- **最小修复**：
  - `WinitNovadrawSystem::run_update_transaction()` 统一包住事件/脚本入口，集中处理 `was_queued -> dispatch -> request_update`。
  - `WinitSceneHost` 文档改为 request-driven redraw；`execute_update()` 在 host flag 漏同步但 UpdateManager 已 queued 时仍执行更新。
  - `NovadrawSystem` 裸访问器注释为高级逃生口，直接写入 invalid/dirty 时必须位于组合根调度事务中或显式 `request_update()`。
- **验证**：cargo check ✅，cargo check -p editor ✅，cargo test -p editor 7/7 ✅

### AD-002 SceneHost thin boundary audit（本轮，verified）
- **根因分析**：`WinitSceneHost` 持有 `use_iterative_render`，这是 editor/demo 的渲染策略状态，不属于平台调度层；同时 SceneHost 文档仍有旧 `WinitEventDispatcher` 映射，容易混淆平台事件与 paint entry。
- **最小修复**：
  - `WinitSceneHost` 移除 `use_iterative_render` 字段和 getter/setter，只保留 window proxy 与 redraw pending 标记。
  - `WinitNovadrawSystem` 持有 `use_iterative_render`，保留 I 键切换语义，不污染 SceneHost。
  - `novadraw-scene/src/scene_host.rs` 修正文档映射：Canvas / paint entry 对应 `SceneHost + RenderBackend`。
- **验证**：cargo check ✅，cargo check -p editor ✅，cargo test -p editor 7/7 ✅

### AD-003 PendingMutation timing audit（本轮，verified）
- **根因分析**：`PendingMutation` 队列和 `apply_pending_mutations()` 时机已存在，但 Figure 回调上下文没有 mutation 生产链路，未来交互式结构变更容易绕回直接修改 `FigureGraph`。
- **最小修复**：
  - `PendingMutation` 新增 `AddChildFigure`，允许 Figure 回调申请新增 child 而不在回调期间挂树。
  - `MutationContext` / `NovadrawContext` 暴露 `add_child_later()` / `remove_child_later()` / `reparent_later()`。
  - `SceneDispatchContext` 接收 `PendingMutations`，`SceneNovadrawContext` 在回调里只 enqueue mutation。
  - editor 顶层输入边界继续在分发结束后调用 `apply_pending_mutations()`。
- **验证**：cargo check ✅，cargo test -p novadraw-scene 139/139 + 3 doctests ✅，cargo test -p editor 7/7 ✅

### AD-004 NovadrawSystem composition root audit（本轮，verified）
- **根因分析**：`app_window` 作为 winit 事件适配层，直接通过 `scene_manager_mut()` 切换场景、执行 `prim_translate()` 并手动 `request_update()`，导致组合根职责不集中。
- **最小修复**：
  - `WinitNovadrawSystem` 新增 `switch_scene()` / `translate_contents()` / `toggle_iterative_render()` 组合根动作。
  - `app_window` 键盘入口只把平台输入映射为组合根动作，不再直接修改 `SceneManager` / `FigureGraph`。
  - 顺手清理无效代码：CGEvent mouse simulator 链路、`core-graphics` 依赖、旧 wrapper、无效 layout 变量、测试 mock warning、vello demo 未用 resize。
  - `novadraw-scene` 补齐 `debug_render` feature 声明。
- **验证**：cargo check ✅，cargo test -p editor 5/5 ✅，cargo test -p novadraw-scene 139/139 + 3 doctests ✅

### AD-005 Interaction state ownership audit（本轮，verified）
- **根因分析**：`mouse_target` / `focus_owner` / `captured` 已归属 `FigureGraph`，但 editor 示例 `InteractiveRectFigure` 仍自持 `Mutex<InteractiveState>` 保存 hovered / pressed / selected，其中 selected 与 `FigureBlock::is_selected` 重复。
- **最小修复**：
  - `FigureBlock` 新增 `is_hovered` / `is_pressed` 运行时状态，`FigureGraph` 提供 accessor。
  - `BasicEventDispatcher` 通过 `DispatchContext` 写 hovered / pressed，自己仍不持有状态。
  - `SceneDispatchContext` 将状态写入转发到 `FigureGraph`。
  - editor 示例 Figure 移除内部 `InteractiveState`，测试改为断言 `FigureGraph` 状态。
  - 场景构建从直接写 `block.is_selected` 改为 `FigureGraph::set_selected(Some(id))`。
- **验证**：cargo check ✅，cargo test -p novadraw-scene 139/139 + 3 doctests ✅，cargo test -p editor 5/5 ✅

### AD-008 Hot-path logging boundary cleanup（本轮，verified）
- **根因分析**：`BasicEventDispatcher::refresh_mouse_target()` 和 `FigureGraph::find_mouse_event_target_at/from()` 属于鼠标移动、target transition 与 hit-test 高频路径，但仍保留 `info` / `debug` / `trace` 日志，违反项目“热路径禁止日志”约束。
- **最小修复**：
  - 删除 event dispatch 热路径中的 target transition 运行时日志。
  - 删除 hit-test 递归下降过程中的 contains / visibility / wants_mouse_events 运行时日志。
  - 保持 `captured -> hit_target -> next_target`、entered/exited 分发、hovered/pressed 状态写入和父链坐标转换语义不变。
- **验证**：cargo fmt ✅，cargo check ✅，cargo test -p novadraw-scene 139/139 + 3 doctests ✅，cargo test -p editor 5/5 ✅

### Continuous Workflow Cycle 1 / AD-009（本轮，verified）
- **BOOTSTRAP / ASSESS**：checkpoint schema 与 backlog YAML 有效，inbox 无阻塞；coverage 中 C-01 / C-02 为 unassessed，触发 review / formal audit gate。
- **REVIEW**：`AD-001B` repair boundary 已满足 done_when，状态从 in_progress 收敛为 verified；`AD-001A/B/C` 均已 verified，父项 `AD-001` 收敛为 done；C-04 提升为 aligned。
- **AD-009 根因分析**：C-01 / C-02 长期未正式评估；审计发现 `FigureBlock::paint_selection_overlay()` 把渲染行为放入节点运行时状态容器，属于轻微职责回流。
- **最小修复**：selection overlay 绘制迁移为 scene 渲染遍历 helper，recursive / iterative render 调用 helper，FigureBlock 只保存节点运行时状态。
- **验证**：cargo fmt ✅，cargo check ✅，cargo test -p novadraw-scene 139/139 + 3 doctests ✅，cargo test -p editor 6/6 ✅

### Continuous Workflow Cycle 2 / AD-010（review 完成，pending）
- **ASSESS / REVIEW**：coverage 已无 unassessed；C-09 / C-10 仍 partially_aligned。
- **Review Decision**：C-10 属于长期执行纪律，当前 worklog 已持续记录 Root Cause / Decision / Split Decision / Reflection，但仍保留观察。
- **Promoted Delta**：C-09 仍有真实审计价值，已新增 `AD-010 Interface boundary escape hatch audit`。
- **下一步**：先审计 `NovadrawSystem` 高级 escape hatch 与 `NovadrawContext` 默认 panic 的可选能力表达，判断是否存在职责回流。

### AD-010 Interface boundary escape hatch audit（本轮，verified）
- **根因分析**：`NovadrawSystem` 公开 trait 暴露 `scene()` / `update_manager()` / `dispatcher()` 三个可变逃生口，类型层面允许绕过组合根事务、PendingMutation 与 redraw scheduling；`NovadrawContext` 默认 panic 将 selection / deferred mutation 能力缺失推迟到运行时。
- **最小修复**：移除 `NovadrawSystem` 公开逃生口；`NovadrawContext` 的 selection 与 deferred mutation 方法改为实现方必须显式提供；`WinitNovadrawSystem` impl 同步删除逃生口实现。
- **文档同步**：`doc/理想架构设计.md` 不再展示 `system.scene()` / `system.dispatcher()` / `system.update_manager()`，改为组合根命名动作。
- **验证**：cargo fmt ✅，cargo check ✅，cargo test -p novadraw-scene 139/139 + 3 doctests ✅，cargo test -p editor 6/6 ✅

### C-10 ASSESS（本轮）
- **ASSESS / REVIEW**：`workflow-continuous` 已强制每轮 REFLECT 判断是否减少架构差距，`inner-loop-worklog.md` 近几轮真实 delta 稳定记录 Root Cause / Minimal Fix / Decision / Split Decision / Reflection / Verification。
- **Coverage Decision**：C-10 不再保持 partially_aligned，已收敛为 aligned。
- **执行边界**：本轮不执行代码 delta，只做 contract coverage 状态收敛。

### Completion Audit Attempt（本轮，blocked）
- **审计范围**：按 `workflow-continuous` 完成审计要求，重新检查理想架构、契约、coverage、backlog，并抽查 C-01 到 C-10 对应代码入口。
- **Discover Smoke**：通过，重新发现多个 residual candidate，而不是输出 0 candidate。
- **新增候选**：
  - `CAD-002` FigureGraph / FigureBlock public mutation surface audit
  - `CAD-003` editor interaction hot-path logging cleanup
  - `CAD-004` composition root residual public read surface audit
  - `CAD-005` ideal architecture stale composition-root document cleanup
  - `CAD-006` PendingMutation production boundary audit
- **Coverage Decision**：C-02 / C-03 / C-05 / C-07 / C-08 / C-09 回退为 partially_aligned；C-01 / C-04 / C-06 / C-10 保持 aligned。
- **Stop Reason**：completion audit 发现新的 architecture candidates，不能进入 COMPLETE，未运行最终 baseline verification。

### CAD-002 REVIEW（本轮）
- **Review Scope**：详细评估 `FigureGraph` / `FigureBlock` public mutation surface，未改运行时代码。
- **Root Cause**：`FigureGraph.blocks` / `uuid_map` 是 public 字段，crate 外可直接改 SlotMap / HashMap，绕过 parent/children、uuid、validation path、notification effects、dirty/update 协议。
- **Promote Decision**：`CAD-002` 提升为 `AD-011 FigureGraph storage encapsulation audit`。
- **Split Decision**：原 CAD-002 过大；AD-011 先处理 `FigureGraph.blocks` / `uuid_map` 公开存储面，`FigureBlock` 字段/方法公开面留待 AD-011 后评估是否拆出后续 delta。

### AD-011 FigureGraph storage encapsulation audit（本轮，verified）
- **根因分析**：`FigureGraph` 是树关系与图级状态 owner，但公开 `blocks` / `uuid_map` 会让 crate 外绕过 `new_block_with_parent()`、parent/children 维护、notification effects、dirty/update 协议直接修改 SlotMap 或 UUID 映射。
- **最小修复**：
  - `FigureGraph.blocks` / `uuid_map` 收窄为私有字段。
  - 新增 `FigureGraph::figure_bounds()` / `set_visible()` 图级命名方法。
  - `SceneDispatchContext` 改用 crate 内只读 `block()` accessor 与 `figure_bounds()`。
  - apps/editor 与 apps/update-app 不再直接访问 `scene.blocks`。
- **Coverage Decision**：C-03 提升为 aligned；C-02 仍 partially_aligned，并新增 `CAD-007 FigureBlock public mutation surface audit` 承接后续 REVIEW。
- **验证**：cargo fmt --check ✅，cargo check ✅，cargo check -p editor -p update-app ✅，cargo test -p novadraw-scene 139/139 + 3 doctests ✅，cargo test -p editor 6/6 ✅。

### CAD-007 REVIEW（本轮）
- **Review Scope**：评估 `FigureBlock public mutation surface audit`，未改运行时代码。
- **Root Cause**：`FigureBlock` 是节点运行时状态容器，但仍作为 public 类型从 `novadraw-scene` 与 `novadraw` facade 导出，且 parent / children / figure / selection / visibility / validity / layout 等字段均为 public；外部可直接修改节点状态而不经过 FigureGraph 图级不变量、dirty/update、notification 和坐标传播协议。
- **Promote Decision**：`CAD-007` 提升为 `AD-012 FigureBlock public mutation surface audit`。
- **Split Decision**：AD-012 先处理 crate 外公开可变面与 facade 导出面；内部 render/event/layout 所需读取能力通过 crate 内 helper 或只读 query 保留，不在本轮改写全部内部遍历。

### AD-012 FigureBlock public mutation surface audit（本轮，verified）
- **根因分析**：`FigureBlock` 是节点运行时状态容器，但其字段和 mutator 曾对 crate 外公开，且通过 `novadraw-scene` / `novadraw` facade re-export，允许外部绕过 `FigureGraph` 修改 parent/children、figure bounds、selection/hover/pressed、visibility/enabled/validity 与 layout 状态。
- **最小修复**：
  - `FigureBlock` 字段收窄为 `pub(crate)`，内部 render/event/layout 可继续读取，不扩展本轮重构范围。
  - 删除未使用的 `FigureBlock` public mutator：`new()` / `add_child()` / size setters / `set_visible()` / `set_enabled()` / `set_figure()` / `set_bounds()`。
  - 保留必要只读 query：`id()` / `uuid()` / `children_count()` / `figure_bounds()` / size getters。
  - `novadraw-scene` 与 `novadraw` facade 不再 re-export `FigureBlock`。
- **Coverage Decision**：C-02 提升为 aligned；C-09 仍 partially_aligned，继续由 CAD-004 / CAD-005 追踪组合根只读面和理想文档旧表述。
- **验证**：cargo fmt --check ✅，cargo check ✅，cargo test -p novadraw-scene 139/139 + 3 doctests ✅，cargo test -p editor 6/6 ✅。

### CAD-003 REVIEW（本轮）
- **Review Scope**：评估 `Editor interaction hot-path logging cleanup`，未改运行时代码。
- **Root Cause**：editor 默认交互路径仍在鼠标移动、raw pointer dispatch、Winit CursorMoved、interactive entered/exited 中打印 info 级日志；这些路径位于平台输入、事件分发入口或 Figure 回调热路径。
- **Duplicate Check**：不重复 AD-008；AD-008 已清理 engine `BasicEventDispatcher` 与 `FigureGraph` hit-test 热路径，本项只处理 editor 层残留日志。
- **Promote Decision**：`CAD-003` 提升为 `AD-013 Editor interaction hot-path logging cleanup`。
- **Split Decision**：AD-013 先移除或隔离 editor interaction 默认热路径日志；不处理 CAD-004 的组合根只读面，也不重构通知体系。

## Current Hypothesis

- ✅ 核心坐标模型主干已闭合：bounds / dirty / hit-test / layout / render / mouse event 均遵守相对最近坐标根语义。
- ✅ client area 已统一到 `Bounded::client_area()`，布局与渲染不再各自推导。
- ✅ `Viewport` 已完成 coordinate-domain audit：API、注释、测试均改为 viewport/content 坐标域，未继续暴露 screen/world 语义。
- ✅ AD-001C 已完成：调度触发属于组合根 / SceneHost，UpdateManager 不承担平台 redraw 调度。
- ✅ AD-002 已完成：SceneHost 保持极薄平台调度层，editor/render 策略状态位于组合根。
- ✅ AD-003 已完成：Figure 回调只记录 pending mutation，结构性改树发生在顶层分发结束后。
- ✅ AD-004 已完成：平台事件层只做输入映射，子系统协作由 `WinitNovadrawSystem` 组合根承载。
- ✅ AD-005 已完成：hovered / pressed / mouse_target / focus_owner / captured 等通用交互状态归属 `FigureGraph`。
- ✅ AD-008 已完成：event / hit-test 热路径不再打印运行时日志，默认路径符合“热路径禁止日志”约束。
- ✅ AD-009 已完成：Figure 只保留内在能力；AD-011 已封装 FigureGraph 存储面，C-03 回到 aligned。
- ✅ AD-012 已完成：FigureBlock 字段与 mutator 不再形成 crate 外 public mutation surface，C-02 回到 aligned。
- ✅ AD-001 已收敛：AD-001A validation、AD-001B repair、AD-001C scheduling 均 verified，父项已 done；C-04 已 aligned。
- ✅ AD-010 已完成：公开接口逃生口与默认 panic 能力边界已收敛；C-09 已 aligned。
- ✅ C-10 已收敛：架构改动说明“为何更接近理想架构”已成为持续工作流强制项，并有多轮真实 delta 证据。
- ⚠️ 文档中仍可能存在历史图示或 WinitEventDispatcher 旧命名，需要后续全量清扫。
- ⚠️ `focus_owner` 只有基础 owner 字段，完整 focus gained/lost/key state machine 仍需后续 delta。
- ⚠️ Contract coverage 当前不再全部 aligned：C-05 / C-07 / C-08 / C-09 为 partially_aligned。
- ✅ `AD-011 FigureGraph storage encapsulation audit` 已 verified；`FigureGraph.blocks` / `uuid_map` 不再是 crate 外 public mutation surface。
- ⚠️ `AD-013 Editor interaction hot-path logging cleanup` 已 proposed，应作为下一轮最小 delta 执行。
- ⚠️ Backlog 仍有 CAD-004 / CAD-005 / CAD-006 candidates，需要后续 REVIEW。

## Next Small Step

- 下一轮进入 EXECUTE，执行 `AD-013 Editor interaction hot-path logging cleanup`。只移除或隔离 editor 鼠标移动、raw pointer、Winit CursorMoved、interactive enter/exit 默认热路径日志；不要混入 CAD-004/CAD-005/CAD-006。
- Viewport/ScrollPane 的真实 Figure-tree 集成仍应作为后续独立 delta，不与 SceneHost 边界混在一起。

## Blockers

- BASELINE-001（历史 cargo fmt drift）仍记录在 backlog；本轮已运行 `cargo fmt`
- 当前无新的硬阻塞

## Verification State

- cargo fmt --check: passed ✅
- cargo check: passed ✅
- cargo test -p novadraw-scene: 139/139 + 3 doctests passed ✅
- cargo test -p editor: 6/6 passed ✅

## Resume Prompt

```text
请按 agent/workflow-continuous.md 从 EXECUTE 继续，当前 delta 为 AD-013 Editor interaction hot-path logging cleanup。CAD-003 REVIEW 已完成并提升为 AD-013：editor 默认交互路径仍在鼠标移动、raw pointer dispatch、Winit CursorMoved、interactive entered/exited 中打印 info 级日志。执行时只移除或隔离这些默认热路径日志，保持事件 target 选择、坐标转换、InteractionTrace、selection/hover/pressed 状态写入和 PendingMutation apply 时机不变；不要混入 CAD-004/CAD-005/CAD-006。
```
