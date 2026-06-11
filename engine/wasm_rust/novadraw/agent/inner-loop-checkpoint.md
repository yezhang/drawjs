# Session Checkpoint

## Metadata

- schema_version: 1
- updated_at: 2026-06-11
- checkpoint_kind: architecture-loop

## Current Delta

- AD-023 Graphics text image alpha command support

## Current Status

- verified（AD-023 已补齐 Graphics text/image/alpha 命令层语义，并通过 delta verification）

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

### AD-013 Editor interaction hot-path logging cleanup（本轮，verified）
- **根因分析**：editor 默认交互路径仍在 mouse move / press / release、raw pointer dispatch、Winit CursorMoved、interactive entered/exited 中打印 info 级日志；这些路径属于平台输入、事件入口或 Figure 高频回调。
- **最小修复**：
  - 移除 `EditorInteractionCore` mouse/raw pointer 默认日志。
  - 移除 `WindowEvent::CursorMoved` 默认日志。
  - 移除 `InteractiveRectFigure::on_mouse_entered/exited` 默认日志和无用 import。
  - 保留 DPI 测试场景探针日志与 UpdateListener 通知日志，不扩大为通知体系或调试能力重构。
- **Coverage Decision**：C-05 提升为 aligned；EventDispatcher 本身仍只负责分发，editor 默认交互热路径不再打印运行时日志。
- **验证**：cargo fmt --check ✅，cargo check ✅，cargo test -p editor 6/6 ✅，目标日志标识 rg 检查无匹配 ✅。

### CAD-004 Composition root residual public read surface audit（本轮 REVIEW）
- **Review Scope**：只评估 editor 组合根残余只读面，未改运行时代码。
- **Root Cause**：`WinitNovadrawSystem::scene_manager()` 向 `app_window` 暴露整个 `SceneManager` 只读引用；`app_window` 通过 `system.scene_manager().current_scene` 判断 DPI 探针与 T 键平移门禁，仍依赖组合根内部结构。
- **Additional Evidence**：`app_window` 直接调用 `EditorInteractionCore::logical_from_raw` 为 DPI 探针取得逻辑坐标，说明平台窗口层仍复用 editor 输入核心内部 helper，而不是调用组合根命名能力或独立平台输入适配 API。
- **Promote Decision**：`CAD-004` 提升为 `AD-014 Composition root residual read surface audit`。
- **Split Decision**：AD-014 只处理组合根只读 escape hatch 和平台输入适配边界；不混入 CAD-005 理想文档清扫、CAD-006 PendingMutation 生产边界或 render strategy wiring。
- **Verification**：REVIEW 阶段仅做静态证据审计；未运行 Cargo 验证。

### AD-014 Composition root residual read surface audit（本轮，verified）
- **根因分析**：`WinitNovadrawSystem::scene_manager()` 暴露整个 `SceneManager` 只读引用，`app_window` 通过它读取 `current_scene`；`app_window` 还直接调用 `EditorInteractionCore::logical_from_raw`，把 editor 输入核心内部 helper 泄漏给平台窗口层。
- **最小修复**：
  - 删除 `EditorInteractionCore::scene_manager()` 与 `WinitNovadrawSystem::scene_manager()`。
  - 新增 `WinitNovadrawSystem::is_scene()` 与 `translate_contents_if_scene()`，让 `app_window` 使用组合根命名 query/action。
  - 将 raw pointer 坐标换算迁移到 `RawPointerInput::logical_position()`，作为平台输入适配数据自身的命名能力。
  - `app_window` 不再引用 `EditorInteractionCore`，也不再读取 `SceneManager.current_scene`。
- **Architecture Review**：Diff review 结论 Go；本轮未引入新的 manager escape hatch，事件分发、坐标换算结果、PendingMutation apply 时机和 UpdateManager 调度语义保持不变。
- **Coverage Decision**：C-07 提升为 aligned；C-09 仍 partially_aligned，由 CAD-005 理想架构旧表述继续追踪。
- **验证**：cargo fmt --check ✅，cargo check ✅，cargo test -p editor 6/6 ✅，目标 escape hatch / helper rg 检查无匹配 ✅。

### AD-015 Ideal architecture composition-root document cleanup（本轮，verified）
- **根因分析**：`doc/理想架构设计.md` 仍把 `NovadrawSystem (trait)` 描述为持有 `scene/update_manager/dispatcher/scene_host`，并保留 `NovadrawSystem.update_manager` / `NovadrawSystem.dispatcher` 与 `WinitEventDispatcher` 旧平台入口表述。
- **最小修复**：
  - 将组合根持有关系改为 `NovadrawSystem` 平台实现内部装配 `FigureGraph` / `UpdateManager` / `EventDispatcher` / `SceneHost`。
  - 明确公开 `NovadrawSystem trait` 只暴露 `render()` / `viewport_size()` / `request_update()`。
  - 将组合根/事件流中的旧 `WinitEventDispatcher` 入口改为 `app_window` 平台输入适配 + `BasicEventDispatcher` 引擎无状态分发。
- **Coverage Decision**：C-09 提升为 aligned；C-08 仍 partially_aligned，由 CAD-006 继续追踪。
- **验证**：目标旧组合根关键词 rg 检查通过；git diff --check ✅。

### AD-016 PendingMutation production boundary audit（本轮，verified）
- **根因分析**：`PendingMutation` / `PendingMutations::enqueue` / `MutationContext` 曾作为公开构造与 enqueue 面暴露，`FigureGraph::apply_pending_mutations()` 接收任意 `Vec<PendingMutation>`；`PendingMutation::AddChild { child: BlockId }` 还允许把既有节点 ID 附加到底层图结构。
- **最小修复**：
  - `PendingMutation` / `PendingMutationKind` / `MutationContext` / `PendingMutations::enqueue` 收窄为 crate 内部。
  - 删除既有节点 `AddChild` 变体；新增 child 只通过 `AddChildFigure` 携带 `Box<dyn Figure>`，由 apply 阶段内部 allocate block。
  - `PendingMutations::drain()` 返回不可外部伪造的 `PendingMutationBatch`；`FigureGraph::apply_pending_mutations()` 只接受 batch，不接受任意 Vec。
  - `FigureGraph::allocate_block()` 收窄为 `pub(crate)`。
- **Architecture Review**：Go；本轮没有改变 dispatch 后 apply 的运行时事务顺序，只把 mutation 生产能力收回引擎上下文，未把业务逻辑放入 dispatcher 或 UpdateManager。
- **Coverage Decision**：C-08 提升为 aligned；C-01 到 C-10 当前均 aligned。
- **验证**：cargo fmt --check ✅，cargo check ✅，cargo test -p novadraw-scene 139/139 + 3 doctests ✅，cargo test -p editor 6/6 ✅，API 残留 rg 检查通过 ✅。

### AD-017 PendingMutation reparent cycle invariant audit（本轮，verified）
- **根因分析**：`apply_reparent_mutation()` 已校验 child、old_parent、new_parent 存在，但没有拒绝 reparent 到自身或子孙节点；自定义 Figure 可通过 `NovadrawContext::reparent_later()` 申请该变更，若 apply 阶段形成环，会破坏 FigureGraph 树不变量。
- **最小修复**：
  - 在 `FigureGraph` 内新增迭代式祖先链校验，使用 blocks 长度作为上界，避免既有损坏图导致无限循环。
  - `apply_reparent_mutation()` 在 detach/attach 前拒绝 `child == new_parent` 与 `new_parent` 位于 child 子树内。
  - 新增 self reparent 与 descendant reparent 无副作用回归测试，确认失败后 parent/children 关系保持不变。
- **Coverage Decision**：C-08 恢复为 aligned；PendingMutation apply 阶段维护 FigureGraph 树不变量。
- **验证**：cargo fmt --check ✅，cargo check ✅，cargo test -p novadraw-scene 143/143 + 3 doctests ✅。

### AD-018 Viewport Figure-tree integration（本轮，verified / visual follow-up paused）
- **根因分析**：g2 中 `Viewport` / `ScrollPane` 是 draw2d Figure 级组件，GEF 只提供 viewer/helper/policy 集成；Novadraw 当前 `Viewport` 仍是 standalone math helper，尚未通过 Figure 树父链协议参与 render / hit-test / damage repair。
- **最小修复**：
  - 新增 `ChildTransform`，将普通坐标根和 Viewport 的 child -> parent 变换统一为 Figure 协议。
  - 新增 `ViewportFigure`，自身 bounds 位于父坐标域，子节点处于 content 坐标域，`origin` / `zoom` 通过 `child_transform()` 与 `client_area()` 接入父链。
  - render recursive / iterative、hit-test、damage repair 统一消费 Figure 的子树变换协议。
  - 新增 Viewport hit-test、render content clip、damage repair origin/zoom 映射回归测试。
- **Coverage Decision**：C-03 / C-09 保持 aligned；Viewport 核心语义位于 `novadraw-scene`，未回流到 apps/editor 或 SceneHost。
- **验证**：cargo fmt ✅，cargo check ✅，cargo test -p novadraw-scene 146/146 + 3 doctests ✅。
- **可视化跟进（暂停）**：
  - 已新增 `apps/viewport-app` 和 `--screenshot-clip` 自动截图入口，用于验证 `clip_to_viewport`。
  - 最新截图路径：`apps/viewport-app/screenshot/viewport-app_clip_to_viewport_1781071623.png`。
  - 截图结论：`clip_to_viewport` 未通过，黄色原点块、蓝/绿网格块出现在黑色 Viewport 边框外，说明 Vello 截图路径中 Viewport content 裁剪未生效。
  - 用户已要求 Viewport 后续开发暂时搁置；不要继续排查或推进 ScrollPane / Viewport 后续能力。
  - 恢复时的最小入口：先审查 `NdCanvas::clip_rect` 到 `VelloRenderer::push_clip_layer()` 的状态栈/clip layer 映射，以及 `perform_update -> repair -> render_to_iterative` 调用路径；不要优先修改 `render_recursive.rs` / `render_iterative.rs` 主循环，除非已对标 draw2d 证明主流程不符。

### AD-019A Host boundary directory split（本轮，verified）
- **根因分析**：`SceneHost` 已被契约定义为极薄平台调度层，但文件仍平铺在 `novadraw-scene/src/scene_host.rs`，目录结构未表达 host 与 graph/runtime/container 的职责边界。
- **最小修复**：
  - 新增 `novadraw-scene/src/host/mod.rs`。
  - 将 `novadraw-scene/src/scene_host.rs` 移动到 `novadraw-scene/src/host/scene_host.rs`。
  - `novadraw-scene/src/lib.rs` 改为 `pub mod host`，并继续 `pub use host::SceneHost`，保持外部 API 不变。
- **Coverage Decision**：C-06 保持 aligned，并补充 AD-019A 作为目录边界证据。
- **验证**：cargo fmt --check ✅，cargo check -p novadraw-scene ✅，cargo test -p novadraw-scene 146/146 + 3 doctests ✅。

### AD-019B novadraw-scene domain directory realignment（本轮，verified）
- **根因分析**：`novadraw-scene/src` 根层仍平铺 `scene/update/context/event/mutation/system/viewport/border` 等不同职责域，物理目录没有完整表达 `graph/runtime/container/figure` 边界。
- **最小修复**：
  - `scene -> graph`。
  - `context/event/mutation/system/update -> runtime`。
  - `viewport.rs -> container/viewport.rs`。
  - `border -> figure/border`。
  - `lib.rs` 保留 root facade alias，兼容 `novadraw_scene::scene/update/context/event/mutation/system/viewport/border` 旧入口。
  - 内部模块引用改向新子域路径，避免继续依赖 root re-export facade。
- **Crate Decision**：暂不创建新 crate；`graph/runtime/context/update/mutation` 仍是内部协作闭环，提前拆 crate 会制造循环依赖或迫使内部协议公开化。
- **Coverage Decision**：C-03 / C-09 / C-10 保持 aligned，并补充 AD-019B 作为目录边界证据。
- **验证**：cargo fmt --check ✅，cargo check --workspace ✅，cargo test -p novadraw-scene 146/146 + 3 doctests ✅。

### 2026-06-11 / Milestone Assessment + WF-001（本轮，verified）
- **BOOTSTRAP**：已读取 `AGENTS.md`、`CLAUDE.md`、`agent/draw2d-core-milestones.yaml`、`agent/goal-roadmap.md`、`agent/workflow-continuous.md`、backlog、checkpoint、coverage、readiness 与 `doc/06-roadmap/`。
- **Milestone Assessment**：M1-M10 在 YAML 与 roadmap 中全部仍为 `not_started`；代码已有历史能力雏形，但未达到 milestone 的 probes / 产品清单 / demo 三层验收闭环。
- **关键判断**：M0 `目标与验收框架` 是当前最新 workflow 的前置能力；仓库缺少可自动检测 milestone / backlog / checkpoint / debt 状态漂移的 doctor。
- **最小修复**：
  - 新增 `agent/workflow-doctor.rb`。
  - 校验 M0/M1-M10 必填字段与状态、goal-roadmap 同步、demo matrix 覆盖、backlog id/status/milestone_id、baseline debt 状态、checkpoint schema 与 current delta。
  - 将 doctor 接入 `agent/workflow-verify.sh`。
  - 补齐 backlog 中历史 `promoted` candidate 状态定义。
- **M0 状态**：`agent/draw2d-core-milestones.yaml` 中 M0 从 `not_started` 推进为 `in_progress`；M1-M10 不做乐观升级。
- **验证**：`ruby agent/workflow-doctor.rb` ✅。
- **Baseline Verification**：`bash agent/workflow-verify.sh` 在 `cargo clippy -- -D warnings` 失败于既有 `apps/vello-app/src/main.rs` needless borrow；已登记 `BASELINE-002`，不混入本轮修复。

### 2026-06-11 / AD-020 Graphics state stack and clip-transform command snapshot（本轮，verified）
- **Milestone**：M1 几何与 Graphics 基础。
- **根因分析**：`NdCanvas` 只生成状态命令但不维护 Graphics 状态，`clip_depth()` 恒为 0，`set_transform/reset_transform` 被编码为 concat/no-op，导致 M1 的 state stack nesting 与 clip/transform snapshot 缺少命令层验收点。
- **最小修复**：
  - `NdCanvas` 新增 `GraphicsState` 与 `state_stack`，维护 fill/stroke/line/transform/clip_depth。
  - `RenderCommandKind` 新增 `SetTransform`、`ResetTransform`、`ResetClip`。
  - Vello backend 改为按 clip depth 解释 restore/pop/reset，不在 PushState 时重复推 clip layer。
  - 新增 3 个 `novadraw-render` 单元测试覆盖 M1 probes。
- **M1 状态**：`agent/draw2d-core-milestones.yaml` 中 M1 从 `not_started` 推进为 `in_progress`；不标记 `contract_aligned`。
- **验证**：cargo fmt --check ✅，cargo test -p novadraw-render ✅，cargo check --workspace ✅。

### 2026-06-11 / AD-021 Seal mutable render command escape hatch（本轮，verified）
- **Milestone**：M1 几何与 Graphics 基础。
- **根因分析**：AD-020 后 `NdCanvas` 维护内部 GraphicsState；公开 `commands_mut()` 会允许外部直接修改命令流，但不会同步 state/clip_depth/transform，破坏录制回放与后端替换需要的命令流确定性。
- **调用面审计**：`commands_mut()` 只有定义自身，没有跨 crate 或 crate 内调用。
- **最小修复**：先收窄为 `pub(crate)`，验证出现 dead_code warning；最终直接移除该接口，只保留 `commands()` 只读快照和 `to_submission()`。
- **验证**：cargo fmt --check ✅，cargo test -p novadraw-render ✅，cargo check --workspace ✅。

### 2026-06-11 / AD-022 Geometry missing types foundation（本轮，verified）
- **Milestone**：M1 几何与 Graphics 基础。
- **根因分析**：M1 要求 `Point, Dimension, Rectangle, Insets, PointList, precision geometry, Transform`；当前 geometry 缺少正式 `Dimension` 命名、点序列封装和统一 precision primitive。
- **最小修复**：
  - `Dimension` 成为正式尺寸类型，`Size` 保持兼容别名。
  - 新增 `PointList`，支持 bounds、transform、Translatable、迭代和 serde。
  - 新增 `Precision`、`DEFAULT_EPSILON`、`ApproxEq`，覆盖 `f64`、`Point`、`Rectangle`、`Transform`。
- **验证**：cargo fmt --check ✅，cargo test -p novadraw-geometry 42/42 ✅，cargo check --workspace ✅。

### 2026-06-11 / WF-002 Backlog manifest split（本轮，verified）
- **根因分析**：`agent/outer-loop-delta-backlog.yaml` 约 1000 行，混合规则、候选项、基线债务、当前 delta 与历史记录；Agent 每轮全量读取会把冷历史和旧路径证据带入上下文。
- **最小修复**：
  - `outer-loop-delta-backlog.yaml` 降级为 manifest。
  - 新增 `agent/backlog/schema.yaml`、`index.yaml`、`active.yaml`、`candidates.yaml`、`baseline-debts.yaml`、`archive/2026-06.yaml`。
  - `workflow-doctor.rb` 改为通过 manifest 聚合校验 active/candidates/archive/debts。
  - workflow 启动脚本和说明文档改为默认读取 index/active，archive 仅审计追溯时读取。
- **验证**：ruby -c agent/workflow-doctor.rb ✅，ruby agent/workflow-doctor.rb ✅，backlog YAML parse ✅，git diff --check ✅。

### 2026-06-11 / AD-023 Graphics text image alpha command support（本轮，verified）
- **Milestone**：M1 几何与 Graphics 基础。
- **根因分析**：M1 scope 要求 Graphics text/image/alpha；`NdCanvas` 已有 `font`、`fill_text`、`stroke_text`、`draw_image`、`global_alpha` API，但这些入口仍是 no-op 或不进入命令流，无法用于录制回放和后端替换验证。
- **最小修复**：
  - `GraphicsState` 新增 `font`、`font_size`、`global_alpha`，随 push/restore/pop 一起作用域化。
  - `RenderCommandKind` 新增 `SetGlobalAlpha`；`FillText` / `StrokeText` 携带 font、font_size、color；`Image` 携带 alpha。
  - `NdCanvas::fill_text`、`stroke_text`、`draw_image`、`draw_image_with_size`、`global_alpha` 生成可回放命令。
  - 形状、路径、文字命令在录制时应用当前 global alpha。
- **验证**：cargo fmt --check ✅，cargo test -p novadraw-render 7/7 ✅，cargo check --workspace ✅，cargo check -p novadraw-render --features vello ✅。

## Current Hypothesis

- ✅ 核心坐标模型主干已闭合：bounds / dirty / hit-test / layout / render / mouse event 均遵守相对最近坐标根语义。
- ✅ client area 已统一到 `Bounded::client_area()`，布局与渲染不再各自推导。
- ✅ `Viewport` 已完成 coordinate-domain audit：API、注释、测试均改为 viewport/content 坐标域，未继续暴露 screen/world 语义。
- ✅ AD-018 已完成引擎最小验证；⚠️ 后续可视化验证发现 `clip_to_viewport` 截图未通过，Viewport 开发按用户要求暂停。
- ✅ AD-001C 已完成：调度触发属于组合根 / SceneHost，UpdateManager 不承担平台 redraw 调度。
- ✅ AD-002 已完成：SceneHost 保持极薄平台调度层，editor/render 策略状态位于组合根。
- ✅ AD-003 已完成：Figure 回调只记录 pending mutation，结构性改树发生在顶层分发结束后。
- ✅ AD-004 已完成：平台事件层只做输入映射，子系统协作由 `WinitNovadrawSystem` 组合根承载。
- ✅ AD-005 已完成：hovered / pressed / mouse_target / focus_owner / captured 等通用交互状态归属 `FigureGraph`。
- ✅ AD-008 已完成：event / hit-test 热路径不再打印运行时日志，默认路径符合“热路径禁止日志”约束。
- ✅ AD-009 已完成：Figure 只保留内在能力；AD-011 已封装 FigureGraph 存储面，C-03 回到 aligned。
- ✅ AD-012 已完成：FigureBlock 字段与 mutator 不再形成 crate 外 public mutation surface，C-02 回到 aligned。
- ✅ AD-001 已收敛：AD-001A validation、AD-001B repair、AD-001C scheduling 均 verified，父项已 done；C-04 已 aligned。
- ✅ AD-010 已完成：公开可变系统逃生口与默认 panic 能力边界已收敛；C-09 已 aligned。
- ✅ C-10 已收敛：架构改动说明“为何更接近理想架构”已成为持续工作流强制项，并有多轮真实 delta 证据。
- ⚠️ 文档中仍可能存在历史图示或 WinitEventDispatcher 旧命名，需要后续全量清扫。
- ⚠️ `focus_owner` 只有基础 owner 字段，完整 focus gained/lost/key state machine 仍需后续 delta。
- ✅ Contract coverage 当前 C-01 到 C-10 均为 aligned。
- ✅ `AD-011 FigureGraph storage encapsulation audit` 已 verified；`FigureGraph.blocks` / `uuid_map` 不再是 crate 外 public mutation surface。
- ✅ `AD-013 Editor interaction hot-path logging cleanup` 已 verified；editor 默认交互热路径日志已清理。
- ✅ CAD-006 已提升并完成为 AD-016。
- ✅ AD-014 已完成：editor 组合根残余只读面已收敛，C-07 已 aligned。
- ✅ AD-015 已完成：理想架构文档组合根旧表述已清理，C-09 已 aligned。
- ✅ Completion baseline verification 已通过：backlog 无 open/candidate，coverage 无 partially_aligned/unassessed/drifting，`cargo test` 全量通过。
- ✅ AD-016 Review follow-up 已完成：invalid add/reparent mutation 不再污染图结构。
- ✅ AD-017 已完成：`apply_reparent_mutation` 在 detach/attach 前拒绝 self reparent 与 descendant reparent，C-08 已恢复 aligned。
- ⚠️ AD-018 后续视觉验证暂停：Viewport 核心语义已在引擎测试中通过，但 `apps/viewport-app` 的 `clip_to_viewport` 自动截图显示 content 裁剪未生效；恢复时优先查 NdCanvas/Vello clip 与 perform_update repair 路径，不先动渲染主循环。
- ✅ AD-019A 已完成：`SceneHost` 进入 `host` 子域，facade 导出保持不变；本轮没有触碰 Viewport、runtime、scene->graph 或渲染主循环。
- ✅ AD-019B 已完成：`novadraw-scene/src` 已收敛为 `figure / graph / runtime / host / container / layout / log / lib.rs`；未创建新 crate，未修改渲染主循环逻辑。
- ✅ M1 已进入 `in_progress`：AD-020 已收口 Graphics state stack / clip-transform command snapshot；M2-M10 仍保持 `not_started`。
- ✅ WF-001 已完成：workflow doctor 初版可检测 milestone、roadmap、backlog、checkpoint 与 debt 的基础状态漂移，并已接入 `workflow-verify.sh`。
- ✅ AD-020 已完成：命令层可验证 Graphics 状态栈嵌套、set/reset transform 快照、clip reset/restore 快照。
- ✅ AD-021 已完成：`NdCanvas` 不再暴露可变命令 Vec 入口，外部只能通过 Graphics API 生成命令并通过只读快照/提交读取录制结果。
- ✅ AD-022 已完成：M1 geometry 补齐 `Dimension`、`PointList` 与 precision geometry。
- ✅ WF-002 已完成：backlog 热路径已拆为 manifest / index / active，冷历史进入 archive，doctor 仍可全量校验。
- ✅ AD-023 已完成：M1 Graphics text/image/alpha 进入命令层快照，`NdCanvas` 不再把相关 API 保持为 no-op。

## Next Small Step

- 当前不要继续排查 Viewport `clip_to_viewport`，该项随 M8 收口。
- 下一步继续 M1：执行 `M1 contract probes summary`，检查 YAML M1 probes 是否都已有测试与证据，再决定是否推进到 `contract_aligned`；不要直接跳到 M3 complete。

## Blockers

- BASELINE-001（历史 cargo fmt drift）已通过本轮 `cargo fmt` / `cargo fmt --check` 收敛
- 当前无新的硬阻塞；M8 仍有 `INBOX-20260610-01` 暂停项，随 M8 收口。

## Verification State

- cargo fmt --check: passed ✅
- cargo check: passed ✅
- cargo test -p novadraw-scene: 139/139 + 3 doctests passed ✅
- cargo test -p editor: 6/6 passed ✅
- AD-013 log target grep: passed ✅
- CAD-004 REVIEW: static evidence audit passed ✅
- AD-014 escape hatch grep: passed ✅
- AD-014 architecture diff review: Go ✅
- AD-015 stale composition-root doc grep: passed ✅
- AD-016 PendingMutation API residual grep: passed ✅
- AD-016 architecture diff review: Go ✅
- completion state consistency grep: passed ✅
- cargo test: passed ✅
- AD-016 follow-up cargo test -p novadraw-scene: 141/141 + 3 doctests passed ✅
- AD-017 cargo fmt --check: passed ✅
- AD-017 cargo check: passed ✅
- AD-017 cargo test -p novadraw-scene: 143/143 + 3 doctests passed ✅
- AD-018 cargo fmt: passed ✅
- AD-018 cargo check: passed ✅
- AD-018 cargo test -p novadraw-scene: 146/146 + 3 doctests passed ✅
- AD-018 workflow YAML parse: passed ✅
- AD-018 git diff --check: passed ✅
- Full cargo clippy -- -D warnings: failed on existing non-AD-018 clippy debt in apps/vello-app and older novadraw-scene modules; not mixed into this delta
- viewport-app clip_to_viewport visual screenshot: failed / paused ⚠️
- AD-019A cargo fmt --check: passed ✅
- AD-019A cargo check -p novadraw-scene: passed ✅
- AD-019A cargo test -p novadraw-scene: 146/146 + 3 doctests passed ✅
- AD-019B cargo fmt --check: passed ✅
- AD-019B cargo check --workspace: passed ✅
- AD-019B cargo test -p novadraw-scene: 146/146 + 3 doctests passed ✅
- WF-001 ruby agent/workflow-doctor.rb: passed ✅
- WF-001 baseline verification `bash agent/workflow-verify.sh`: failed on existing BASELINE-002 (`apps/vello-app` clippy needless borrow) ⚠️
- AD-020 cargo fmt --check: passed ✅
- AD-020 cargo test -p novadraw-render: 3/3 tests passed ✅
- AD-020 cargo check --workspace: passed ✅
- AD-021 cargo fmt --check: passed ✅
- AD-021 cargo test -p novadraw-render: 3/3 tests passed ✅
- AD-021 cargo check --workspace: passed ✅
- AD-022 cargo fmt --check: passed ✅
- AD-022 cargo test -p novadraw-geometry: 42/42 tests passed ✅
- AD-022 cargo check --workspace: passed ✅
- WF-002 ruby -c agent/workflow-doctor.rb: passed ✅
- WF-002 ruby agent/workflow-doctor.rb: passed ✅
- WF-002 backlog YAML parse: passed ✅
- WF-002 git diff --check: passed ✅
- AD-023 cargo fmt --check: passed ✅
- AD-023 cargo test -p novadraw-render: 7/7 tests passed ✅
- AD-023 cargo check --workspace: passed ✅
- AD-023 cargo check -p novadraw-render --features vello: passed ✅

## Resume Prompt

```text
AD-023 Graphics text image alpha command support 已完成并通过验证。`NdCanvas` 的 `font`、`fill_text`、`stroke_text`、`draw_image`、`global_alpha` 已进入可回放命令流；M1 仍为 `in_progress`。下一轮执行 M1 contract probes summary，检查 geometry operation、Graphics state stack nesting、clip/transform command snapshots、text/image/alpha command snapshots 是否证据齐全，再决定是否推进到 `contract_aligned`；Viewport 后续开发仍暂停，不要继续排查 `clip_to_viewport`。
```
