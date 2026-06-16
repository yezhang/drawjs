# Session Checkpoint

## Metadata

- schema_version: 1
- updated_at: 2026-06-16
- checkpoint_kind: architecture-loop

## Current Delta

- AD-035 M3 paint versus hit-test consistency tests

## Current Status

- verified（AD-035 已补齐 paint clip、hit-test descent 与 mouse event target descent 共享 border-inset clientArea 的契约测试；M3 已推进到 `contract_aligned`）

## Context Boundary

- 当前 M2-M7 讨论只围绕通用 Figure 树、父链坐标协议、绘制、布局、事件与通知。
- 已剥离主题不得作为每轮 next step、阻塞项或残余风险反复带出。

## What Was Done

### AD-035 M3 paint versus hit-test consistency tests（本轮，verified）
- **根因分析**：AD-034 已让 Border insets 进入 render clientArea clip，但 M3 仍缺少同一 clientArea 同时约束 paint、hit-test 与 mouse event target descent 的契约测试。
- **最小修复**：
  - 新增 `test_paint_clip_and_hit_test_share_border_inset_client_area`，验证 parent border-inset clientArea clip 与 hit-test descent 一致。
  - 新增 `test_mouse_event_target_uses_same_border_inset_client_area_as_paint`，验证 mouse event target 不会进入 paint 不可见的 child 区域。
  - M3 从 `in_progress` 推进到 `contract_aligned`；不推进到 `behavior_verified`，因为 demo/视觉验证尚未闭合。
- **验证**：cargo fmt ✅，cargo test -p novadraw-scene paint_clip_and_hit_test ✅，cargo test -p novadraw-scene mouse_event_target_uses_same_border ✅
- **下一步**：M3 product visual verification via shapes-demo。

### WF-005 Workflow SSOT cleanup（本轮，verified）
- **根因分析**：工作流已经由 `workflow-doctor.rb`、`workflow-verify.sh` 和 continuous controller 驱动，但旧 readiness、run-once、map、inbox 仍被当作热路径入口。
- **最小修复**：
  - `workflow-verify.sh --gate=ready` 接管 readiness hard gate。
  - `inner-loop-checkpoint.md` 新增 `Interruptions` 小节，中断不再写独立 inbox。
  - 删除 `workflow-run-once.sh`、`workflow-map.md`、`quality-workflow-readiness.md`、`interruptions-inbox.md`。
- **验证**：ruby agent/workflow-doctor.rb ✅，./agent/workflow-verify.sh --fast --gate=ready ✅，git diff --check ✅
- **下一步**：回到 M3 代码主线。

### AD-034 M3 border insets client-area clipping（本轮，verified）
- **根因分析**：`RectangleFigure::with_border(...)` 已支持 Border 装饰器，但 `Bounded::insets()` 未从 border 读取 `get_insets()`，导致产品图元的 border insets 不会收窄 clientArea，children 可进入应由 border/insets 隔离的绘制区域。
- **最小修复**：
  - `RectangleFigure` 的 `Bounded::insets()` 返回 border insets；无 border 时保持 `(0,0,0,0)`。
  - `graph/mod.rs` 新增 `test_border_insets_define_client_area_clip_for_children`，验证 parent clientArea clip 使用 border insets、border 在 children 之后绘制且使用 inset-adjusted bounds。
  - 新增 `test_iterative_render_matches_recursive_nested_border_insets_clipping`，验证 nested border/insets 场景下 recursive/iterative render signature 等价。
- **验证**：cargo fmt --check ✅，cargo test -p novadraw-scene ✅，cargo clippy -p novadraw-scene -- -D warnings ✅
- **状态推进**：M3 保持 `in_progress`；不推进到 `contract_aligned`。
- **下一步**：M3 paint versus hit-test consistency tests。

### AD-033 M3 iterative render client-area state equivalence（本轮，verified）
- **根因分析**：recursive render 在 paintClientArea 设置 parent clientArea clip 后会 `push_state()`，child clip 结束后 `restore_state()` 回到 parent clientArea；iterative render 缺少该保存点，`ExitChild` 会恢复到更外层状态，导致 sibling clip state 与递归路径不等价。
- **最小修复**：
  - `render_iterative.rs` 在 `EnterClientArea` 完成 parent clientArea transform/clip 后新增 `push_state()`，保存 parent clientArea 状态。
  - `ExitClientArea` 改为 `pop_state()` 后再 `restore_state()`，对齐 recursive paintClientArea 状态栈语义。
  - `graph/mod.rs` 新增 render command signature helper 和 `test_iterative_render_matches_recursive_clip_state_for_siblings`，验证 sibling 场景下 recursive/iterative command signature 一致。
- **验证**：cargo fmt --check ✅，cargo test -p novadraw-scene iterative_render ✅，cargo test -p novadraw-scene 162/162 + 3 integration + 3 doctests ✅，cargo clippy -p novadraw-scene -- -D warnings ✅，ruby agent/workflow-doctor.rb ✅，bash agent/workflow-verify.sh --fast ✅
- **状态推进**：M3 从 `not_started` 推进到 `in_progress`；不推进到 `contract_aligned`。
- **下一步**：M3 nested clipping and border/insets rendering tests。

### WF-004 Execution momentum gate（本轮，verified）
- **根因分析**：AD-031/AD-032 让用户感知到近期工作偏向 md/status 文件；现有 workflow 有拆分、active/recent 生命周期和 milestone 状态门禁，但没有阻止连续 documentation-only delta 的执行动量规则。
- **最小修复**：
  - `agent/README.md` 新增“执行动量门禁”：最近两个终态 delta 不得同时为 documentation-only；文档型 delta 之后必须回到产品代码、运行时代码、可执行 workflow code 或 tests。
  - `agent/backlog/schema.yaml` 同步记录该硬约束。
  - `agent/workflow-doctor.rb` 新增 recent momentum 校验：读取 recent 对应 archive item 的 evidence/files，若最近两个终态 delta 都只包含 `agent/` 或 `doc/` 下 `.md/.yaml/.yml` 证据则失败。
  - backlog current delta 更新为 `WF-004`，next recommended delta 保持 `M3 render traversal and clipping contract audit`。
- **验证**：ruby agent/workflow-doctor.rb ✅，git diff --check ✅
- **下一步**：进入 M3 render traversal and clipping contract audit，不继续追加 M2 状态总结。

### AD-032 M2 product-layer existence checks（本轮，verified）
- **根因分析**：M2 已经 contract_aligned，但 `product-deliverables.md` 中 5 基础图元、Figure/FigureBlock/FigureGraph 产品角色与三段式 paint 协议缺少 crate 外部存在性检查，因此不能推进到 `behavior_verified`。
- **最小修复**：
  - 新增 `novadraw-scene/tests/m2_product_existence.rs`，从 crate 外部构造 5 个产品图元并装箱为 `Box<dyn Figure>`。
  - 覆盖 `FigureGraph` 产品 API：树挂载、block 只读查询、child order、z-index、hit-test、visible/enabled 有效状态。
  - 通过 marker Figure 验证三段式 paint 顺序：parent figure -> child figure -> child border -> parent border。
  - 新增 `agent/m2-product-existence-checks.md`，记录产品层证据、验证命令与 residual risks。
- **验证**：cargo fmt --check ✅，cargo test -p novadraw-scene 161/161 + 3 integration + 3 doctests ✅，cargo clippy -p novadraw-scene -- -D warnings ✅，ruby agent/workflow-doctor.rb ✅，bash agent/workflow-verify.sh --fast ✅
- **状态推进**：M2 从 `contract_aligned` 推进到 `behavior_verified`；不推进到 `complete`，因为 `shapes-demo` 端到端验证仍未执行。
- **下一步**：进入 M3 render traversal and clipping contract audit。

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
- 已剥离主题的历史细节不进入当前恢复摘要。

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
  - 明确公开 `NovadrawSystem trait` 只暴露 `render()` / `redacted_topic_size()` / `request_update()`。
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

### 历史剥离条目 Historical Redacted Topic（verified / paused）
- **说明**：该历史主题已从当前工作流热路径剥离；具体名称、恢复入口与失败现场不进入 checkpoint。
- **验证**：历史代码验证曾通过；当前主线不以该主题作为 next step、阻塞项或残余风险。

### AD-019A Host boundary directory split（本轮，verified）
- **根因分析**：`SceneHost` 已被契约定义为极薄平台调度层，但文件仍平铺在 `novadraw-scene/src/scene_host.rs`，目录结构未表达 host 与 graph/runtime/container 的职责边界。
- **最小修复**：
  - 新增 `novadraw-scene/src/host/mod.rs`。
  - 将 `novadraw-scene/src/scene_host.rs` 移动到 `novadraw-scene/src/host/scene_host.rs`。
  - `novadraw-scene/src/lib.rs` 改为 `pub mod host`，并继续 `pub use host::SceneHost`，保持外部 API 不变。
- **Coverage Decision**：C-06 保持 aligned，并补充 AD-019A 作为目录边界证据。
- **验证**：cargo fmt --check ✅，cargo check -p novadraw-scene ✅，cargo test -p novadraw-scene 146/146 + 3 doctests ✅。

### AD-019B novadraw-scene domain directory realignment（本轮，verified）
- **根因分析**：`novadraw-scene/src` 根层仍平铺 `scene/update/context/event/mutation/system/redacted_topic/border` 等不同职责域，物理目录没有完整表达 `graph/runtime/container/figure` 边界。
- **最小修复**：
  - `scene -> graph`。
  - `context/event/mutation/system/update -> runtime`。
  - `redacted_topic.rs -> container/redacted_topic.rs`。
  - `border -> figure/border`。
  - `lib.rs` 保留 root facade alias，兼容 `novadraw_scene::scene/update/context/event/mutation/system/redacted_topic/border` 旧入口。
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

### 2026-06-11 / AD-024 M1 contract probes summary（本轮，verified）
- **Milestone**：M1 几何与 Graphics 基础。
- **根因分析**：M1 已完成 AD-020/021/022/023，但状态推进必须按 YAML probes 逐项映射自动化证据，而不是按主观完成感判断。
- **最小修复**：
  - 新增 `agent/m1-contract-probes-summary.md`。
  - 汇总 M1 scope、contracts、probes、delta evidence、verification 与 residual risks。
  - 确认 geometry operation、Graphics state stack nesting、clip/transform command snapshots、text/image/alpha command snapshots 均已有测试证据。
  - 将 M1 从 `in_progress` 推进到 `contract_aligned`。
- **边界判断**：不推进到 `behavior_verified` 或 `complete`；产品层 existence checks 尚未建立。
- **验证**：cargo test -p novadraw-geometry 42/42 ✅，cargo test -p novadraw-render 7/7 ✅。

### 2026-06-11 / WF-003 Active backlog lifecycle compaction（本轮，verified）
- **根因分析**：WF-002 已把单体 backlog 拆为 manifest / active / archive，但 `active.yaml` 仍保留大量 `verified` 终态条目，导致热路径文件继续增长，仍会污染 Agent 默认上下文。
- **最小修复**：
  - `agent/backlog/active.yaml` 只保留非终态工作，当前已压缩为 `items: []`。
  - 原 active 终态条目迁入 `agent/backlog/archive/2026-06.yaml`，并追加 WF-003 归档记录。
  - 新增 `agent/backlog/recent.yaml`，只保留最近 5 个终态 delta 摘要。
  - `workflow-doctor.rb` 新增 active lifecycle 校验：active 禁止终态、active 最大 10 项、recent 最多 5 项且必须为终态。
  - workflow 文档和启动脚本默认读取 `index.yaml` / `active.yaml` / `recent.yaml`，archive 继续作为冷历史。
- **验证**：ruby -c agent/workflow-doctor.rb ✅，ruby agent/workflow-doctor.rb ✅，backlog YAML parse ✅，git diff --check ✅。

### 2026-06-11 / AD-025 M1 product-layer existence checks（本轮，verified）
- **Milestone**：M1 几何与 Graphics 基础。
- **根因分析**：M1 已经 `contract_aligned`，但 `product-deliverables.md` 的 M1 几何类型清单与 Graphics API 清单缺少独立存在性检查，不能推进到 `behavior_verified`。
- **最小修复**：
  - 新增 `PrecisionPoint`、`PrecisionRectangle`、`PrecisionDimension`、`Vector`、`AffineTransform` 兼容别名。
  - 新增 M1 geometry product existence 集成测试，覆盖产品层类型导入、PointList、Insets 与 ApproxEq 兼容语义。
  - 新增 Graphics product-layer snake_case 入口，覆盖 rectangle、oval、polygon、text/string、clip、alpha 与 style setters。
  - 新增 `LineStyle` 并录入 stroke command snapshot，使线型状态成为可回放命令语义。
  - 新增 M1 render product existence 集成测试，覆盖 shape/style、text/image/clip/alpha 命令输出。
  - 新增 `agent/m1-product-existence-checks.md`，将 M1 从 `contract_aligned` 推进到 `behavior_verified`。
- **边界判断**：不推进到 `complete`；M1 无独立 demo，但仍需后续路线图/文档闭环后再判断 complete。
- **验证**：cargo fmt --check ✅，cargo test -p novadraw-geometry 44/44 ✅，cargo test -p novadraw-render 9/9 ✅，cargo check --workspace ✅，cargo clippy -p novadraw-geometry -p novadraw-render -- -D warnings ✅，cargo check -p novadraw-render --features vello ✅。

### 2026-06-11 / BASELINE-002 cleanup（本轮，verified）
- **根因分析**：`workflow-verify.sh` 原先在 `cargo clippy -- -D warnings` 阶段失败；直接登记点是 `apps/vello-app/src/main.rs` 两处 `needless_borrows_for_generic_args`，修复后又继续暴露 scene 与 demo apps 的既有 clippy style debt。
- **最小修复**：
  - `vello-app` 移除 `scene.fill()` 颜色参数的不必要引用。
  - `novadraw-scene` 清理 Copy 类型 clone、derive Default、collapsible-if、needless-borrow 与 Copy event clone。
  - demo apps 清理冗余零参闭包、复杂 scene entry 类型别名、无效 cast、无效 return、range loop；示例函数参数过多处保留局部 allow。
- **边界判断**：本轮只清理 clippy 基线债务，不改变 M1 状态机，不继续 已剥离主题 视觉验证。
- **验证**：cargo fmt ✅，cargo fmt --check ✅，cargo clippy -- -D warnings ✅，bash agent/workflow-verify.sh ✅。

### 2026-06-11 / AD-026 M2 topology add entry guard（本轮，verified）
- **Milestone**：M2 Figure 树与盒模型。
- **根因分析**：M2 要求 Figure tree 是 ownership 与 topology 的运行时骨架；延迟 add/reparent 路径已有 invalid-parent 防御，但公开直接 add 入口仍通过 `self.blocks[parent_id]` 访问 parent，parent 无效时会 panic，且与 pending mutation 路径的无副作用契约不一致。
- **最小修复**：
  - `add_child_to` 改为复用 `try_add_child_to`，无效 parent 时返回 `BlockId::null()`，不分配 block、不写 uuid_map、不修改 validation path。
  - 新增显式可失败入口 `try_add_child_to(parent, figure) -> Option<BlockId>`。
  - `add_child_with_bounds` 与交互式 `add_child` 同步接入无效 parent 防御。
  - 交互式 `add_child` 无效 parent 时不触发 layout/repaint queue。
  - 新增 invalid-parent 定向测试，覆盖 pending add、直接 add、try add 与 update-manager add。
- **边界判断**：不处理 remove dispose 语义、不做 z-order API、不做 visible/enabled effective propagation、不接入 border insets。
- **验证**：cargo fmt ✅，cargo test -p novadraw-scene invalid_parent 5/5 ✅，cargo test -p novadraw-scene 149/149 + 3 doctests ✅，cargo clippy -p novadraw-scene -- -D warnings ✅，bash agent/workflow-verify.sh ✅。

### 2026-06-12 / AD-027 M2 effective visible/enabled propagation（本轮，verified）
- **Milestone**：M2 Figure 树与盒模型。
- **根因分析**：`FigureBlock.is_visible` / `is_enabled` 只是节点本地状态；render 和 hit-test 通过父节点遍历能自然跳过隐藏子树，但 repaint 与 validation 可直接以 child id 作为入口，仍只检查目标节点本地标志，导致隐藏/禁用祖先下的 child 仍可能进入更新语义。
- **最小修复**：
  - `FigureGraph` 新增本地查询 `is_visible` / `is_enabled`。
  - `FigureGraph` 新增父链查询 `is_effectively_visible` / `is_effectively_enabled`。
  - 新增 `set_enabled`，并让 `set_visible(false)` / `set_enabled(false)` 清理子树交互状态。
  - `repaint` 改为按有效可见性过滤；validation phase 改为按有效可见性与有效启用状态过滤。
  - 新增 M2 定向测试覆盖父链传播、隐藏祖先 repaint、隐藏/禁用祖先 validation queue drain。
- **边界判断**：不处理 z-order API、不做 remove dispose 语义、不推进 M3 绘制裁剪闭环。
- **验证**：cargo fmt --check ✅，cargo test -p novadraw-scene 154/154 + 3 doctests ✅，cargo clippy -p novadraw-scene -- -D warnings ✅，ruby agent/workflow-doctor.rb ✅，bash agent/workflow-verify.sh ✅。

### 2026-06-12 / AD-028 M2 child order and z-order contract audit（本轮，verified）
- **Milestone**：M2 Figure 树与盒模型。
- **根因分析**：`children` 存储顺序已被 render 正向遍历与 hit-test 反向遍历隐式使用，但 crate 外没有图级命名 API 查询或调整 sibling z-order；继续依赖内部 `children` 字段会破坏 FigureGraph 封装与树不变量。
- **最小修复**：
  - 对标 Draw2D：children 正序绘制，reverse children 优先用于 findFigureAt / findMouseEventTargetAt。
  - `FigureGraph` 新增 `child_order` / `child_z_index` 只读查询，明确 index 越大越靠顶层。
  - `FigureGraph` 新增 `move_child_to_index` / `bring_child_to_front` / `send_child_to_back`。
  - 重排 API 只允许直接 child；非法 parent、非直接 child、越界 index 与 no-op 均无副作用返回 false。
  - 新增 M2 契约测试覆盖 add append 顺序、z-index 查询、重排后 topmost hit-test 变化，以及非法重排无副作用。
- **边界判断**：不处理 bounds/insets/clientArea 一致性、不做 remove dispose 语义、不推进 M3 绘制裁剪闭环。
- **验证**：cargo fmt --check ✅，cargo test -p novadraw-scene z_order ✅，cargo test -p novadraw-scene 157/157 + 3 doctests ✅，cargo clippy -p novadraw-scene -- -D warnings ✅，ruby agent/workflow-doctor.rb ✅，bash agent/workflow-verify.sh ✅。

### 2026-06-12 / AD-029 M2 bounds/insets/clientArea consistency audit（本轮，verified）
- **Milestone**：M2 Figure 树与盒模型。
- **根因分析**：`Bounded::client_area()` 与 render/layout 已大体对齐 Draw2D，但 hit-test 与 mouse target 子树下降只检查父 Figure bounds，未按 Draw2D `findDescendantAtExcluding` / `findMouseEventTargetInDescendantsAt` 先用父 clientArea 过滤 children 搜索。
- **最小修复**：
  - 对标 Draw2D：`getClientArea()` 基于 bounds shrink insets；`useLocalCoordinates()` 时原点重置为 `(0,0)`。
  - `hit_test_from` 在父 Figure 自身按 bounds 命中后，只在转换后的点落入父 clientArea 时继续搜索 children。
  - `find_mouse_event_target_from` 同步采用父 clientArea 门禁；父自身是否作为 target 仍由 `wants_mouse_events()` 决定。
  - 新增 M2 契约测试覆盖点位于父 bounds 但落在 clientArea 外时不会命中 child，进入 clientArea 后正常命中 child / mouse target。
  - 顺手修复 `workflow-doctor.rb` 默认外部/内部编码为 UTF-8，避免普通 `ruby agent/workflow-doctor.rb` 在中文 roadmap 上触发 US-ASCII 读取错误。
- **边界判断**：不处理 remove dispose 语义、不扩大到 M3 绘制裁剪闭环、不改 recursive / iterative render 主流程。
- **验证**：cargo fmt --check ✅，cargo test -p novadraw-scene parent_client_area ✅，cargo test -p novadraw-scene bounds_test 26/26 ✅，cargo test -p novadraw-scene 159/159 + 3 doctests ✅，cargo clippy -p novadraw-scene -- -D warnings ✅，ruby agent/workflow-doctor.rb ✅，bash agent/workflow-verify.sh --fast ✅。

### 2026-06-12 / AD-030 M2 remove and reparent lifecycle contract audit（本轮，verified）
- **Milestone**：M2 Figure 树与盒模型。
- **根因分析**：`apply_reparent_mutation` 已拒绝 invalid parent、self reparent 与 descendant reparent，但 detach 旧父发生在 attach 新父之前；若新父已异常持有 child，attach 会失败并留下 orphan，违反 remove/reparent no partial write 契约。
- **最小修复**：
  - 对标 Draw2D：add/reparent 会先保证不会形成 cycle，remove 会只接受真实 direct child，并触发重绘/重验证。
  - `FigureGraph` 新增 `contains_direct_child` 内部查询。
  - `apply_reparent_mutation` 在任何 detach/attach 写入前校验旧父确实持有 child、新父尚未持有 child。
  - 新增 wrong-parent remove 无副作用测试：不 detach child、不清交互状态、不排队 invalid/dirty。
  - 新增 duplicate-new-parent reparent 无副作用测试：不 detach 旧父、不改 parent、不排队 invalid/dirty。
- **边界判断**：不新增公开 remove/reparent API，不改变 pending mutation 时序，不处理 M3 绘制闭环。
- **验证**：cargo fmt --check ✅，cargo test -p novadraw-scene apply_pending 9/9 ✅，cargo test -p novadraw-scene 161/161 + 3 doctests ✅，cargo clippy -p novadraw-scene -- -D warnings ✅，ruby agent/workflow-doctor.rb ✅，bash agent/workflow-verify.sh --fast ✅。

### 2026-06-12 / AD-031 M2 contract alignment summary（本轮，verified）
- **Milestone**：M2 Figure 树与盒模型。
- **根因分析**：M2 已完成多个局部 delta，但 milestone 状态仍停留在 `in_progress`；需要将 add/remove/reparent、z-order、bounds/clientArea、visible/enabled 四类 YAML probes 与自动化证据逐项对齐，避免状态推进依赖口头判断。
- **最小修复**：
  - 新增 `agent/m2-contract-alignment-summary.md`，逐项登记 M2 scope、contracts、probes、delta evidence、verification 与 residual risks。
  - 确认 AD-026 到 AD-030 已覆盖 M2 YAML probes。
  - 将 M2 从 `in_progress` 推进到 `contract_aligned`。
  - 明确本轮不推进 `behavior_verified` 或 `complete`，产品层存在性检查留到下一步。
- **边界判断**：不新增代码能力，不启动 M3，不做 M2 product-layer checks。
- **验证**：cargo test -p novadraw-scene ✅，cargo clippy -p novadraw-scene -- -D warnings ✅，ruby agent/workflow-doctor.rb ✅，bash agent/workflow-verify.sh --fast ✅。

## Current Hypothesis

- ✅ 核心坐标模型主干已闭合：bounds / dirty / hit-test / layout / render / mouse event 均遵守相对最近坐标根语义。
- ✅ client area 已统一到 `Bounded::client_area()`，布局与渲染不再各自推导。
- ✅ 当前核心工作流只保留通用 Figure runtime 主线；已剥离主题不参与恢复摘要。
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
- ✅ 历史暂停上下文不作为当前 M2-M7 的阻塞项、next step 或残余风险。
- ✅ AD-019A 已完成：`SceneHost` 进入 `host` 子域，facade 导出保持不变；本轮没有触碰 已剥离主题、runtime、scene->graph 或渲染主循环。
- ✅ AD-019B 已完成：`novadraw-scene/src` 已收敛为 `figure / graph / runtime / host / container / layout / log / lib.rs`；未创建新 crate，未修改渲染主循环逻辑。
- ✅ M1 已进入 `behavior_verified`：契约层 probes 与产品层 existence checks 均有自动化证据。
- ✅ M2 已进入 `in_progress`：AD-026 已收口公开 add 入口的无效 parent 防御；M3-M10 仍保持 `not_started`。
- ✅ WF-001 已完成：workflow doctor 初版可检测 milestone、roadmap、backlog、checkpoint 与 debt 的基础状态漂移，并已接入 `workflow-verify.sh`。
- ✅ AD-020 已完成：命令层可验证 Graphics 状态栈嵌套、set/reset transform 快照、clip reset/restore 快照。
- ✅ AD-021 已完成：`NdCanvas` 不再暴露可变命令 Vec 入口，外部只能通过 Graphics API 生成命令并通过只读快照/提交读取录制结果。
- ✅ AD-022 已完成：M1 geometry 补齐 `Dimension`、`PointList` 与 precision geometry。
- ✅ WF-002 已完成：backlog 热路径已拆为 manifest / index / active，冷历史进入 archive，doctor 仍可全量校验。
- ✅ AD-023 已完成：M1 Graphics text/image/alpha 进入命令层快照，`NdCanvas` 不再把相关 API 保持为 no-op。
- ✅ AD-024 已完成：M1 contract probes summary 确认 YAML probes 全部有自动化证据，M1 状态已推进到 `contract_aligned`。
- ✅ WF-003 已完成：`active.yaml` 只保存非终态工作，终态 delta 进入 archive，`recent.yaml` 提供最近 5 个终态摘要，doctor 强制校验该生命周期。
- ✅ AD-025 已完成：M1 product-layer existence checks 覆盖几何类型清单与 Graphics API 清单，M1 状态已推进到 `behavior_verified`。
- ✅ AD-026 已完成：M2 topology add entry guard 覆盖直接 add / try add / update-manager add 的 invalid-parent 无副作用契约。
- ✅ AD-027 已完成：M2 effective visible/enabled propagation 覆盖本地状态查询、父链有效状态查询、repaint 过滤与 validation 过滤。
- ✅ AD-028 已完成：M2 child order / z-order 覆盖 sibling 顺序查询、z-index 查询、重排 API 与 topmost hit-test 契约。
- ✅ AD-029 已完成：M2 bounds/insets/clientArea 覆盖 hit-test / mouse target 子树下降的父 clientArea 门禁。
- ✅ AD-030 已完成：M2 remove/reparent 覆盖 wrong-parent remove 与 duplicate-new-parent reparent 的无副作用契约。
- ✅ AD-031 已完成：M2 contract summary 确认 YAML probes 全部有自动化证据，M2 已推进到 `contract_aligned`。

## Next Small Step

- 下一步：继续 M2 product-layer existence checks；不要直接跳到 M3 complete。

## Blockers

- BASELINE-001（历史 cargo fmt drift）已通过本轮 `cargo fmt` / `cargo fmt --check` 收敛
- 当前无新的硬阻塞。

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
- 历史剥离条目 cargo fmt: passed ✅
- 历史剥离条目 cargo check: passed ✅
- 历史剥离条目 cargo test -p novadraw-scene: 146/146 + 3 doctests passed ✅
- 历史剥离条目 workflow YAML parse: passed ✅
- 历史剥离条目 git diff --check: passed ✅
- Full cargo clippy -- -D warnings: failed on existing non-历史剥离条目 clippy debt in apps/vello-app and older novadraw-scene modules; not mixed into this delta
- Historical paused visual follow-up: not part of current recovery path
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
- AD-024 cargo test -p novadraw-geometry: 42/42 tests passed ✅
- AD-024 cargo test -p novadraw-render: 7/7 tests passed ✅
- WF-003 ruby -c agent/workflow-doctor.rb: passed ✅
- WF-003 ruby agent/workflow-doctor.rb: passed ✅
- WF-003 backlog YAML parse: passed ✅
- WF-003 git diff --check: passed ✅
- AD-025 cargo fmt --check: passed ✅
- AD-025 cargo test -p novadraw-geometry: 44/44 tests passed ✅
- AD-025 cargo test -p novadraw-render: 9/9 tests passed ✅
- AD-025 cargo check --workspace: passed ✅
- AD-025 cargo clippy -p novadraw-geometry -p novadraw-render -- -D warnings: passed ✅
- AD-025 cargo check -p novadraw-render --features vello: passed ✅
- AD-025 ruby agent/workflow-doctor.rb: passed ✅
- AD-025 git diff --check: passed ✅
- AD-025 baseline verification `bash agent/workflow-verify.sh`: failed on existing BASELINE-002 (`apps/vello-app` clippy needless borrow) ⚠️
- BASELINE-002 cargo fmt --check: passed ✅
- BASELINE-002 cargo clippy -- -D warnings: passed ✅
- BASELINE-002 bash agent/workflow-verify.sh: passed ✅
- AD-026 cargo fmt: passed ✅
- AD-026 cargo test -p novadraw-scene invalid_parent: 5/5 tests passed ✅
- AD-026 cargo test -p novadraw-scene: 149/149 + 3 doctests passed ✅
- AD-026 cargo clippy -p novadraw-scene -- -D warnings: passed ✅
- AD-026 bash agent/workflow-verify.sh: passed ✅
- AD-027 cargo fmt --check: passed ✅
- AD-027 cargo test -p novadraw-scene: 154/154 + 3 doctests passed ✅
- AD-027 cargo clippy -p novadraw-scene -- -D warnings: passed ✅
- AD-027 ruby agent/workflow-doctor.rb: passed ✅
- AD-027 bash agent/workflow-verify.sh: passed ✅
- AD-028 cargo fmt --check: passed ✅
- AD-028 cargo test -p novadraw-scene z_order: passed ✅
- AD-028 cargo test -p novadraw-scene: 157/157 + 3 doctests passed ✅
- AD-028 cargo clippy -p novadraw-scene -- -D warnings: passed ✅
- AD-028 ruby agent/workflow-doctor.rb: passed ✅
- AD-028 bash agent/workflow-verify.sh: passed ✅
- AD-029 cargo fmt --check: passed ✅
- AD-029 cargo test -p novadraw-scene parent_client_area: passed ✅
- AD-029 cargo test -p novadraw-scene bounds_test: 26/26 passed ✅
- AD-029 cargo test -p novadraw-scene: 159/159 + 3 doctests passed ✅
- AD-029 cargo clippy -p novadraw-scene -- -D warnings: passed ✅
- AD-029 ruby agent/workflow-doctor.rb: passed ✅
- AD-029 bash agent/workflow-verify.sh --fast: passed ✅
- AD-030 cargo fmt --check: passed ✅
- AD-030 cargo test -p novadraw-scene apply_pending: 9/9 passed ✅
- AD-030 cargo test -p novadraw-scene: 161/161 + 3 doctests passed ✅
- AD-030 cargo clippy -p novadraw-scene -- -D warnings: passed ✅
- AD-030 ruby agent/workflow-doctor.rb: passed ✅
- AD-030 bash agent/workflow-verify.sh --fast: passed ✅
- AD-031 cargo test -p novadraw-scene: passed ✅
- AD-031 cargo clippy -p novadraw-scene -- -D warnings: passed ✅
- AD-031 ruby agent/workflow-doctor.rb: passed ✅
- AD-031 bash agent/workflow-verify.sh --fast: passed ✅
- WF-005 ruby agent/workflow-doctor.rb: passed ✅
- WF-005 ./agent/workflow-verify.sh --fast --gate=ready: passed ✅
- WF-005 git diff --check: passed ✅
- AD-035 cargo fmt: passed ✅
- AD-035 cargo test -p novadraw-scene paint_clip_and_hit_test: passed ✅
- AD-035 cargo test -p novadraw-scene mouse_event_target_uses_same_border: passed ✅
- AD-035 ./agent/workflow-verify.sh --fast: passed ✅

## Interruptions

- 当前无 active interruption。
- 突发任务不再写入独立 inbox；如阻塞主线，直接在本小节记录来源、影响范围、是否阻塞当前 delta 和建议动作。

## Resume Prompt

```text
AD-035 M3 paint versus hit-test consistency tests 已完成。M3 已推进到 `contract_aligned`；`agent/backlog/active.yaml` 仍为空。下一轮建议进入 M3 product visual verification via shapes-demo，不要直接推进 M3 到 `behavior_verified`。
```
