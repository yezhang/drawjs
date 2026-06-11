# Architecture Worklog

按轮记录架构改进工作，确保中断后可以快速恢复上下文。

## Entry Template

```text
## 2026-04-10 / AD-001

- Goal:
- Root Cause:
- Files:
- Verification:
- Delta Verification:
- Baseline Verification:
- Decision:
- Split Decision:
- Post-Execution Reflection:
- New Candidate Deltas:
- Next Step:
```

## Entries

## 2026-06-11 / AD-020

- Goal: 启动 M1，补齐 Graphics state stack / clip-transform command snapshot 的最小契约实现与测试。
- Root Cause: `NdCanvas` 只生成 `PushState/RestoreState/PopState` 命令但不维护 Graphics 状态，`clip_depth()` 恒为 0，`set_transform/reset_transform` 被编码为 concat/no-op；因此 M1 要求的 Graphics state stack nesting 与 clip/transform command snapshot 无法在命令层稳定验证。
- Minimal Fix: 在 `NdCanvas` 中新增 `GraphicsState` 与 `state_stack`，维护 fill/stroke/line/transform/clip_depth；新增 `SetTransform`、`ResetTransform`、`ResetClip` 命令；Vello backend 改为按 clip depth 恢复 scene layer，PushState 不再重复推 clip layer。
- Tests: 新增 3 个 `novadraw-render` 单元测试，覆盖嵌套状态恢复、`set_transform/reset_transform` 快照命令、clip reset/restore 命令序列。
- Files: `novadraw-render/src/context.rs`, `novadraw-render/src/command.rs`, `novadraw-render/src/backend/vello/mod.rs`, `agent/draw2d-core-milestones.yaml`, `agent/goal-roadmap.md`, `agent/outer-loop-delta-backlog.yaml`, `agent/inner-loop-checkpoint.md`, `agent/inner-loop-worklog.md`
- Delta Verification: cargo fmt --check ✅, cargo test -p novadraw-render ✅, cargo check --workspace ✅
- Decision: M1 从 `not_started` 推进为 `in_progress`；本轮只收口 Graphics 状态栈和 clip/transform 快照，不把 M1 标记为 `contract_aligned`。
- Split Decision: 不处理 Dimension/PointList/precision geometry、文本/图像/alpha，也不修复 Viewport visual clipping 或 M3 render traversal probes。
- Post-Execution Reflection: 本轮更接近 M1，因为 Graphics 命令层现在可以表达并验证状态栈、clip 与 transform 的确定性组合，为 M3 绘制裁剪闭环提供稳定 substrate。
- New Candidate Deltas: M1 geometry missing types audit；M1 text/image/alpha command support；M3 recursive/iterative command equivalence probe。
- Next Step: 继续 M1，优先补 geometry missing types 或 Graphics text/image/alpha 中的一个最小 delta；不要直接跳到 M3 complete。

## 2026-06-11 / Milestone Assessment + WF-001

- Goal: 按最新 `workflow-continuous` 先执行 BOOTSTRAP / ASSESS，检查 M1-M10 达成情况，并补齐 M0 工作流前置校验能力。
- Milestone Assessment: YAML 与 `goal-roadmap.md` 均显示 M1-M10 全部 `not_started`；代码中已有 M2/M4/M5/M6/M8/M10 等历史局部能力，但未完成 probes / product deliverables / demo matrix 三层验收，因此不直接升级 milestone 状态。
- Root Cause: 新里程碑体系已经定义状态机、同步规则和 M0 `workflow doctor checks`，但仓库缺少自动化控制器来检测 milestone / roadmap / backlog / checkpoint / debt 的状态漂移，状态一致性仍依赖人工审计。
- Minimal Fix: 新增 `agent/workflow-doctor.rb`，校验 M0/M1-M10 必填字段、状态机、companion 文件、goal-roadmap 同步、demo matrix 覆盖、backlog id/status/milestone_id、baseline debt 状态、checkpoint schema 与 current delta；将 doctor 接入 `agent/workflow-verify.sh`。
- Workflow Finding: 首次运行 doctor 发现历史 `candidate_items` 使用 `promoted`，但 backlog 状态定义未声明该状态；本轮将 `promoted` 纳入 `outer-loop-delta-backlog.yaml` 状态定义，保持历史候选迁移语义不变。
- Files: `agent/workflow-doctor.rb`, `agent/workflow-verify.sh`, `agent/outer-loop-delta-backlog.yaml`, `agent/draw2d-core-milestones.yaml`, `agent/workflow-continuous.md`, `agent/workflow-run-continuous.sh`, `agent/README.md`, `agent/quality-workflow-readiness.md`, `agent/inner-loop-checkpoint.md`, `agent/inner-loop-worklog.md`
- Delta Verification: `ruby agent/workflow-doctor.rb` ✅
- Baseline Verification: `bash agent/workflow-verify.sh` 运行到 `cargo clippy -- -D warnings` 时失败于既有 `apps/vello-app/src/main.rs` 两处 needless borrow；已登记 `BASELINE-002`，不混入 WF-001 修复。
- Decision: M0 从 `not_started` 推进为 `in_progress`；M1-M10 保持 `not_started`，后续必须通过具体 milestone delta 和 probes 推进。
- Split Decision: 本轮只做 workflow doctor 与状态模型补齐，不执行 M1 Graphics、M3 裁剪、M6 输入状态机或 M8 Viewport 修复，避免把 workflow gate 与业务能力混在一个 delta。
- Post-Execution Reflection: 本轮更接近最新工作流目标，因为状态文件不再只靠人工自律，baseline verification 会先检查 milestone/backlog/checkpoint 的机器可检出漂移，为后续 M1-M10 推进提供控制器基础。
- New Candidate Deltas: 建议新增 M1 Graphics state stack and clip/transform snapshot parity delta（下一轮 REVIEW 后决定正式编号）。
- Next Step: 回到 REVIEW，选择首个挂 M1-M10 的最小 delta；建议从 M1 Graphics 状态栈与 clip/transform 命令快照开始。

## 2026-06-10 / AD-019B

- Goal: 按用户要求一次性完成当前可安全调整的 `novadraw-scene` 目录边界，并判断是否创建新 crate。
- Root Cause: `novadraw-scene/src` 根层仍平铺 `scene/update/context/event/mutation/system/viewport/border` 等不同职责域，物理目录没有完整表达 `figure / graph / runtime / host / container` 子域边界。
- Minimal Fix: 完成 `scene -> graph`、`context/event/mutation/system/update -> runtime`、`viewport.rs -> container/viewport.rs`、`border -> figure/border`；保留 root facade alias 兼容旧外部入口。
- Crate Decision: 不创建新 crate。依赖扫描显示 `graph/runtime/context/update/mutation` 仍是内部协作闭环，提前拆 crate 会制造循环依赖或迫使 `PendingMutation`、`UpdateManager`、`Context` 等内部协议公开化。
- Internal Path Decision: 内部模块引用改向新子域路径，避免继续依赖 `lib.rs` root re-export facade；外部 API 通过 `novadraw_scene::*` 保持兼容。
- Render Guardrail: `render_recursive.rs` 与 `render_iterative.rs` 仅随 `graph/` 移动路径，未修改主循环逻辑。
- Files: `novadraw-scene/src/figure/border/**`, `novadraw-scene/src/graph/**`, `novadraw-scene/src/runtime/**`, `novadraw-scene/src/container/**`, `novadraw-scene/src/lib.rs`, `agent/outer-loop-delta-backlog.yaml`, `agent/governance-contract-coverage.md`, `agent/inner-loop-checkpoint.md`, `agent/inner-loop-worklog.md`
- Delta Verification: cargo fmt --check ✅, cargo check --workspace ✅, cargo test -p novadraw-scene 146/146 + 3 doctests ✅
- Decision: CAD-010 的代码目录重组已闭环；后续只做文档历史路径清扫，不继续大迁移。
- Post-Execution Reflection: 本轮更接近理想架构，因为物理目录已经表达 Figure 树核心、运行时服务、宿主边界和容器扩展边界；未来 crate 拆分可以在依赖方向进一步稳定后成为机械迁移。

## 2026-06-10 / AD-019A

- Goal: 以最小风险启动 `novadraw-scene` 目录边界拆分，让平台宿主职责进入 `host` 子域。
- Root Cause: `SceneHost` 已被契约定义为极薄平台调度层，但文件仍平铺在 `novadraw-scene/src/scene_host.rs`，目录结构未表达 host 与 graph/runtime/container 的职责边界。
- Minimal Fix: 新增 `novadraw-scene/src/host/mod.rs`，将 `scene_host.rs` 移动到 `host/scene_host.rs`，并保持 `novadraw_scene::SceneHost` facade re-export 不变。
- Files: `novadraw-scene/src/host/mod.rs`, `novadraw-scene/src/host/scene_host.rs`, `novadraw-scene/src/lib.rs`, `agent/outer-loop-delta-backlog.yaml`, `agent/governance-contract-coverage.md`, `agent/inner-loop-checkpoint.md`, `agent/inner-loop-worklog.md`
- Delta Verification: cargo fmt --check ✅, cargo check -p novadraw-scene ✅, cargo test -p novadraw-scene 146/146 + 3 doctests ✅
- Decision: C-06 保持 aligned，并补充 AD-019A 作为目录边界证据；本轮只移动 host，不拆 crate。
- Split Decision: 不迁移 runtime，不执行 `scene -> graph` 大迁移，不移动暂停中的 `viewport.rs`，不修改 `render_recursive.rs` / `render_iterative.rs` 主循环。
- Post-Execution Reflection: 本轮更接近理想架构，因为 `SceneHost` 的物理目录位置与“极薄平台宿主”职责一致，后续 `WinitSceneHost / WebSceneHost / HeadlessSceneHost` 有自然归属。
- New Candidate Deltas: AD-019B runtime boundary directory split；AD-019C graph boundary directory split；AD-019D container boundary directory split（待 Viewport 恢复后）。
- Next Step: 如继续模块拆分，优先评估 AD-019B runtime 边界；保持 facade crate `novadraw` 不变。

## 2026-06-10 / Viewport Visual Follow-up Paused

- Goal: 冻结 Viewport 可视化验证现场，按用户要求暂停 Viewport 后续开发。
- What Was Finished: 已新增 `apps/viewport-app`，包含 `clip_to_viewport`、`origin_scroll`、`zoomed_content`、`nested_viewports` 四个可视化场景；已新增 `--screenshot-clip` 自动截图入口。
- Visual Verification Result: `cargo run -p viewport-app -- --screenshot-clip` 成功生成 `apps/viewport-app/screenshot/viewport-app_clip_to_viewport_1781071623.png`，但截图未通过，黄色原点块和蓝/绿 content 网格绘制到黑色 Viewport 边框外。
- Current Hypothesis: 问题不应先归因到 `render_recursive.rs` / `render_iterative.rs` 主循环；恢复时优先审查 `NdCanvas::clip_rect` 到 `VelloRenderer::push_clip_layer()` 的 clip layer / transform 映射，以及 `perform_update -> repair -> render_to_iterative` 截图路径。
- Guardrail: `render_recursive.rs` 与 `render_iterative.rs` 主循环是保护区，除非已对标 draw2d 证明主流程不符，否则不要改主循环逻辑。
- Exact Restart Point: 重新运行 `cargo run -p viewport-app -- --screenshot-clip`，打开最新截图，对照黑色 Viewport 边框检查 content 裁剪；然后从 Vello clip 映射和 UpdateManager repair 调用路径开始排查。
- Decision: 用户要求 Viewport 后续开发暂时搁置；当前不继续修复 `clip_to_viewport`，不推进 ScrollPane / mouse wheel / auto-expose。
- Next Step: 若继续架构主线，回到 discovery/review，选择非 Viewport 的最小 delta。

## 2026-06-10 / AD-018

- Goal: 按 draw2d 对标，把 Viewport 从 standalone math helper 推进为 Figure 树中的坐标根和裁剪容器。
- Root Cause: g2 中 `Viewport` / `ScrollPane` 位于 `org.eclipse.draw2d`，GEF 只提供 viewer/helper/policy 集成；Novadraw 当前 `Viewport` 尚未作为 Figure 节点参与 render / hit-test / damage repair，若在 apps/editor 或 SceneHost 保存滚动状态，会重新引入 Figure 树外全局坐标特判。
- Minimal Fix: 新增 `ChildTransform` 作为 Figure 子树坐标协议；新增 `ViewportFigure`，用 `origin` / `zoom` 计算 content -> parent 变换，并用 content-domain `client_area()` 驱动裁剪；render recursive / iterative、hit-test、damage repair 均消费同一协议。
- Files: `novadraw-scene/src/figure/mod.rs`, `novadraw-scene/src/viewport.rs`, `novadraw-scene/src/scene/mod.rs`, `novadraw-scene/src/scene/render_recursive.rs`, `novadraw-scene/src/scene/render_iterative.rs`, `novadraw-scene/src/update/repair.rs`, `novadraw-scene/src/scene/bounds_test.rs`, `novadraw-scene/src/scene/update_integration_test.rs`, `novadraw-scene/src/lib.rs`, `novadraw/src/lib.rs`, `agent/outer-loop-delta-backlog.yaml`, `agent/governance-contract-coverage.md`, `agent/inner-loop-checkpoint.md`, `agent/inner-loop-worklog.md`
- Delta Verification: cargo fmt ✅, cargo check ✅, cargo test -p novadraw-scene 146/146 + 3 doctests ✅, backlog YAML parse ✅, git diff --check ✅
- Baseline Verification: `cargo clippy -- -D warnings` 发现非本轮 clippy debt（apps/vello-app needless borrow、旧 novadraw-scene derivable impl / clone-on-copy 等），未混入 AD-018 修复。
- Decision: C-03 / C-09 保持 aligned；Viewport 核心语义归属 `novadraw-scene`，apps/editor 和 SceneHost 不拥有滚动/视口图语义状态。
- Split Decision: 不实现 ScrollBar UI、mouse wheel、auto-expose、selection feedback 或完整 ScrollPane layout；这些属于后续独立 delta。
- Post-Execution Reflection: 本轮更接近理想架构，因为 Viewport 现在沿 Figure 树父链坐标协议闭合，render / hit-test / damage repair 不需要各自维护特殊分支，也没有把 draw2d Figure 级语义上移到 GEF-like/editor 层。
- New Candidate Deltas: ScrollPane layout and range model integration；editor mouse-wheel viewport policy；Viewport auto-expose helper。
- Next Step: 提交 AD-018；如继续迭代，回到 discovery/review 选择 ScrollPane 或 editor policy 中的一个最小项。

## 2026-06-10 / AD-017

- Goal: 收敛 PendingMutation reparent apply 阶段的树不变量，防止 Figure 回调申请形成 FigureGraph 环。
- Root Cause: `apply_reparent_mutation()` 已校验 child、old_parent、new_parent 存在，但没有拒绝 `child == new_parent` 或 `new_parent` 位于 child 子树；一旦 detach/attach 执行，会破坏 render/event/layout/update 遍历依赖的树结构前提。
- Minimal Fix: 在 `FigureGraph` 内新增迭代式祖先链校验，并在 detach/attach 前拒绝 self reparent 与 descendant reparent；使用 blocks 长度作为遍历上界，避免既有损坏图导致无限循环。
- Files: `novadraw-scene/src/scene/mod.rs`, `novadraw-scene/src/scene/update_integration_test.rs`, `agent/outer-loop-delta-backlog.yaml`, `agent/governance-contract-coverage.md`, `agent/inner-loop-checkpoint.md`, `agent/inner-loop-worklog.md`
- Delta Verification: cargo fmt --check ✅, cargo check ✅, cargo test -p novadraw-scene 143/143 + 3 doctests ✅
- Decision: C-08 恢复为 aligned；PendingMutation apply 阶段现在在产生结构副作用前维护 FigureGraph 树不变量。
- Split Decision: 不处理 `EditorInteractionCore::scene_manager_mut()`、`FigureGraph::get_block()` 或 Viewport/ScrollPane 集成；这些是相邻候选，不属于本轮 reparent 防环根因。
- Post-Execution Reflection: 本轮更接近理想架构，因为结构性变更不仅通过 PendingMutation 延迟应用，还在消费 batch 的唯一图级入口统一维护树不变量，失败路径保持无副作用。
- New Candidate Deltas: 无。
- Next Step: 提交 AD-017；如继续迭代，回到 discovery/review 选择新的最小 delta。

## 2026-06-09 / New Cycle Discovery / AD-017

- Goal: 在 AD-016 follow-up 提交后启动新一轮 architecture delta discovery。
- Audited Contracts: C-01/C-02 Figure/FigureBlock ownership, C-03/C-04 FigureGraph vs UpdateManager, C-05 EventDispatcher, C-06 SceneHost, C-07 Composition Root, C-08 PendingMutation Timing, C-09 Interface Boundary。
- Checked Entrypoints: `novadraw-scene/src/scene/mod.rs`, `novadraw-scene/src/context/mod.rs`, `novadraw-scene/src/mutation/mod.rs`, `novadraw-scene/src/event/mod.rs`, `novadraw-scene/src/update/mod.rs`, `novadraw-scene/src/update/deferred.rs`, `apps/editor/src/system.rs`, `apps/editor/src/scene_manager/scene_host.rs`, `novadraw/src/lib.rs`, `novadraw-scene/src/figure/**`。
- Candidate Deltas: CAD-008 / AD-017 promoted；另记录但不提升：`EditorInteractionCore::scene_manager_mut()` 内部逃生口、`FigureGraph::get_block()` 只读类型暴露面。
- Root Cause: `apply_reparent_mutation` 已校验节点存在和 invalid parent 失败路径，但没有防止把 child reparent 到自身或自身子孙；这会破坏 FigureGraph 树不变量，并影响 render/event/layout/update 遍历前提。
- Decision: 提升 AD-017，优先修 C-08 的树不变量缺口；其他发现暂缓，避免一次 delta 混入组合根 API 与只读查询 API 收敛。
- Next Step: 在 detach/attach 前补 `child == new_parent` 与 descendant guard，并添加无副作用回归测试。

## 2026-06-09 / AD-016 Review Follow-up

- Goal: 修复提交级 Review 发现的 PendingMutation apply 失败路径副作用问题。
- Root Cause: `apply_reparent_mutation()` 在确认 `new_parent` 存在前先 detach 原 child；`apply_add_mutation()` 在确认 `parent` 存在前先 allocate block。自定义 Figure 可通过公开 `NovadrawContext` 传入无效 `BlockId`，导致失败路径仍污染图结构。
- Minimal Fix: `reparent` 在 detach 前确认 child、old_parent、new_parent 都存在；`add child` 在 allocate 前确认 parent 存在。
- Tests: 新增 invalid parent add-child 无副作用测试，确认 blocks/uuid_map 不增长；新增 invalid new_parent reparent 无副作用测试，确认 child 仍挂在原 parent 下。
- Verification: cargo fmt --check ✅, cargo check ✅, cargo test -p novadraw-scene 141/141 + 3 doctests ✅
- Result: Review finding closed；AD-016 的 PendingMutation 事务边界仍为 verified。

## 2026-06-09 / Completion Baseline

- Goal: 在 AD-016 后执行完成基线验证，确认当前架构循环可以进入 complete-ready。
- Checks: backlog YAML 解析 ✅, git diff --check ✅, backlog 无 `status: open` / `status: candidate` ✅, coverage 无 `partially_aligned` / `unassessed` / `drifting` ✅, cargo fmt --check ✅, cargo check ✅, cargo test ✅
- Result: C-01 到 C-10 均为 aligned；BASELINE-001 已通过本轮 `cargo fmt` 收敛；当前架构循环状态推进到 complete-ready。
- Next Step: 整理并按主题提交当前未提交改动，或开启新的 architecture delta discovery。

## 2026-06-09 / AD-016

- Goal: 收敛 PendingMutation 生产阶段类型边界，确保结构性变更只能通过受控上下文 enqueue，并在顶层分发后以 batch 应用。
- Root Cause: `PendingMutation` / `PendingMutations::enqueue` / `MutationContext` 曾作为公开构造与 enqueue 面暴露，`FigureGraph::apply_pending_mutations()` 接收任意 `Vec<PendingMutation>`；同时 `PendingMutation::AddChild { child: BlockId }` 允许携带既有节点 ID 附加到底层图结构，类型层面保留了低层 attach 能力。
- Minimal Fix: `PendingMutation` / `PendingMutationKind` / `MutationContext` / `PendingMutations::enqueue` 收窄为 crate 内部；删除既有节点 `AddChild` 变体；新增 child 仅通过 `AddChildFigure` 携带 `Box<dyn Figure>` 并在 apply 阶段内部 allocate；`PendingMutations::drain()` 返回 `PendingMutationBatch`，`FigureGraph::apply_pending_mutations()` 只接受 batch；`FigureGraph::allocate_block()` 收窄为 `pub(crate)`。
- Files: `novadraw-scene/src/mutation/mod.rs`, `novadraw-scene/src/scene/mod.rs`, `novadraw-scene/src/context/mod.rs`, `novadraw-scene/src/lib.rs`, `novadraw/src/lib.rs`, `novadraw-scene/src/scene/update_integration_test.rs`
- Delta Verification: cargo fmt --check ✅, cargo check ✅, cargo test -p novadraw-scene 139/139 + 3 doctests ✅, cargo test -p editor 6/6 ✅, API residual grep ✅
- Architecture Review: Go. 本轮没有改变 dispatch 后 apply 的运行时事务顺序，只把 mutation 生产能力收回引擎上下文，消除了公开类型伪造和既有节点 AddChild attach 面。
- Coverage Update: C-08 从 partially_aligned 提升为 aligned；当前 C-01 到 C-10 均为 aligned。
- Split Decision: 不处理 Viewport/ScrollPane 真实 Figure-tree 集成，不重新打开 AD-014/AD-015。
- Post-Execution Reflection: 本轮更接近理想架构，因为结构性变更现在由 `NovadrawContext`/`SceneDispatchContext` 产生，`FigureGraph` 只在稳定事务边界消费不可外部伪造的 batch，类型系统更直接地表达了 PendingMutation 契约。
- New Candidate Deltas: 无。
- Next Step: 进入 completion baseline verification；若 `cargo test` 全量通过，可将当前架构循环标记为 complete-ready。

## 2026-06-09 / AD-015

- Goal: 清理理想架构文档中的组合根旧表述，使文档与 AD-010 / AD-014 后的公开接口边界一致。
- Root Cause: `doc/理想架构设计.md` 仍把 `NovadrawSystem (trait)` 描述为持有 `scene/update_manager/dispatcher/scene_host`，并保留 `NovadrawSystem.update_manager` / `NovadrawSystem.dispatcher` 与 `WinitEventDispatcher` 旧平台入口表述；这会把已移除的公开逃生口重新写成理想架构。
- Minimal Fix: 将相关段落统一改为“NovadrawSystem 平台实现内部装配 FigureGraph / UpdateManager / EventDispatcher / SceneHost；公开 trait 只暴露 render / viewport_size / request_update”；将组合根/事件流中的旧 `WinitEventDispatcher` 入口改为 `app_window` 平台输入适配 + `BasicEventDispatcher` 引擎无状态分发。
- Files: `doc/理想架构设计.md`, `agent/outer-loop-delta-backlog.yaml`, `agent/governance-contract-coverage.md`, `agent/inner-loop-checkpoint.md`, `agent/inner-loop-worklog.md`
- Delta Verification: `rg "WinitEventDispatcher|NovadrawSystem\\.update_manager|NovadrawSystem\\.dispatcher|dispatcher: Arc|update_manager: Arc|scene_host: Arc|FigureGraph 持有树结构和 UpdateManager|NovadrawSystem \\(trait\\)|全局组合根，持有 UpdateManager" doc/理想架构设计.md` 无匹配 ✅, git diff --check ✅
- Coverage Update: C-09 从 partially_aligned 提升为 aligned；C-08 仍 partially_aligned，由 CAD-006 追踪。
- Split Decision: 不处理 CAD-006 PendingMutation 生产阶段边界；不进行运行时代码修改。
- Post-Execution Reflection: 本轮更接近理想架构，因为文档不再把已删除的公开 manager 逃生口描述为目标形态，后续实现将以“公开 trait 稳定入口 + 平台实现内部装配 + 组合根命名动作”为准。
- New Candidate Deltas: 无。
- Next Step: 回到 REVIEW，优先评估 `CAD-006 PendingMutation production boundary audit`。

## 2026-06-09 / AD-014

- Goal: 收敛 editor 组合根残余只读面，让 `app_window` 只通过组合根命名 query/action 或平台输入适配 API 与系统交互。
- Root Cause: `WinitNovadrawSystem::scene_manager()` 暴露整个 `SceneManager` 只读引用，`app_window` 借此读取 `current_scene`；同时 `app_window` 直接调用 `EditorInteractionCore::logical_from_raw`，把 editor 输入核心内部 helper 暴露给平台窗口层。
- Minimal Fix: 删除 `EditorInteractionCore::scene_manager()` 与 `WinitNovadrawSystem::scene_manager()`；新增 `WinitNovadrawSystem::is_scene()` 和 `translate_contents_if_scene()`；将 raw pointer 坐标换算改为 `RawPointerInput::logical_position()`；`app_window` 改用这些命名 API。
- Files: `apps/editor/src/system.rs`, `apps/editor/src/app_window.rs`, `agent/outer-loop-delta-backlog.yaml`, `agent/governance-contract-coverage.md`, `agent/inner-loop-checkpoint.md`, `agent/inner-loop-worklog.md`
- Delta Verification: cargo fmt --check ✅, cargo check ✅, cargo test -p editor 6/6 ✅, `rg "scene_manager\\(|EditorInteractionCore::logical_from_raw|logical_from_raw" apps/editor/src` 无匹配 ✅
- Architecture Review: Diff review 结论 Go；`app_window` 不再触达组合根内部 `SceneManager`，`RawPointerInput::logical_position()` 属于平台输入适配数据的命名能力，没有引入新的 manager escape hatch。
- Coverage Update: C-07 从 partially_aligned 提升为 aligned；C-09 仍 partially_aligned，由 CAD-005 理想架构旧表述继续追踪。
- Split Decision: 不处理 CAD-005 理想架构文档旧表述，不处理 CAD-006 PendingMutation 生产阶段边界，不重构 render strategy wiring。
- Post-Execution Reflection: 本轮更接近理想架构，因为平台窗口层从“读取组合根内部结构并复用内部 helper”收敛为“调用组合根命名能力与平台输入适配方法”，组合根边界更窄且意图更明确。
- New Candidate Deltas: 无。
- Next Step: 回到 REVIEW，优先评估 `CAD-005 Ideal architecture stale composition-root document cleanup`。

## 2026-06-09 / CAD-004 REVIEW

- Goal: 评估 `Composition root residual public read surface audit` 是否应提升为正式 delta，只做 REVIEW，不改运行时代码。
- Evidence Checked: `apps/editor/src/app_window.rs`, `apps/editor/src/system.rs`, `apps/editor/src/scene_manager/mod.rs`, `agent/outer-loop-delta-backlog.yaml`, `agent/governance-contract-coverage.md`, `agent/inner-loop-checkpoint.md`
- Root Cause: `WinitNovadrawSystem::scene_manager()` 暴露整个 `SceneManager` 只读引用，`app_window` 通过 `system.scene_manager().current_scene` 判断 DPI 探针和 T 键平移门禁，仍依赖组合根内部结构；`app_window` 还直接调用 `EditorInteractionCore::logical_from_raw`，复用 editor 输入核心内部 helper 来完成 DPI 探针坐标换算。
- Duplicate Check: 不重复 AD-004/AD-010。AD-004 已把场景切换和平移动作收回组合根，AD-010 移除了公开 trait 可变逃生口；本项处理的是 editor 具体组合根上的残余只读 escape hatch 与平台输入适配边界。
- Promote Decision: CAD-004 提升为 `AD-014 Composition root residual read surface audit`。
- Split Decision: AD-014 只处理 `WinitNovadrawSystem::scene_manager()` 暴露面和 `EditorInteractionCore::logical_from_raw` 调用边界；不处理 CAD-005 理想架构文档旧表述，不处理 CAD-006 PendingMutation 生产阶段边界，不重构 render strategy wiring。
- Next Step: 从 EXECUTE 开始执行 AD-014，最小修复应让 `app_window` 改用组合根命名 query/action 或明确的平台输入适配 API，并验证 editor 行为不变。

## 2026-06-01 / AD-013

- Goal: 清理 editor interaction 默认热路径日志，保证平台输入、事件入口和示例 Figure 高频回调默认不打印运行时日志。
- Root Cause: `apps/editor/src/system.rs` 的 mouse/raw pointer dispatch、`apps/editor/src/app_window.rs` 的 `WindowEvent::CursorMoved`、`apps/editor/src/scene_manager/interactive_figure.rs` 的 entered/exited 回调仍使用 info 级日志；这些路径属于默认交互热路径。
- Files: `apps/editor/src/system.rs`, `apps/editor/src/app_window.rs`, `apps/editor/src/scene_manager/interactive_figure.rs`, `agent/outer-loop-delta-backlog.yaml`, `agent/governance-contract-coverage.md`, `agent/inner-loop-checkpoint.md`, `agent/inner-loop-worklog.md`
- Delta Verification: cargo fmt --check ✅, cargo check ✅, cargo test -p editor 6/6 ✅, `rg "\[(MouseEvent|RawPointer|Winit)\]|interactive_rect (entered|exited)" apps/editor/src` 无匹配 ✅
- Baseline Verification: 未运行全仓 `cargo test`；本轮只执行 delta scope 验证。
- Decision: 只删除默认交互热路径日志调用，不改变事件 target 选择、坐标转换、InteractionTrace、selection/hover/pressed 状态写入或 PendingMutation apply 时机。
- Split Decision: 不处理 CAD-004/CAD-005/CAD-006；保留 DPI 测试场景探针日志与 TraceUpdateListener 通知日志，避免把本轮扩大为通知/调试体系重构。
- Post-Execution Reflection: 本轮减少了应用层默认热路径对运行时日志的依赖，让 C-05 从“dispatcher 主干已对齐但 editor 残留热路径日志”收敛为 aligned。
- New Candidate Deltas: 无。
- Next Step: 进入 REVIEW，优先评估 `CAD-004 Composition root residual public read surface audit`。

## 2026-05-28 / Continuous Workflow Cycle 1

- Goal: 运行 `workflow-continuous` 的 BOOTSTRAP / ASSESS / REVIEW / EXECUTE / VERIFY / RECORD，推进当前最高价值架构收敛项
- Bootstrap: 已读取 `AGENTS.md`、`CLAUDE.md`、`doc/理想架构设计.md`、`workflow-continuous.md`、架构契约、backlog、checkpoint、coverage 与 readiness
- Assess: checkpoint schema 与 backlog YAML 有效；inbox 无阻塞；coverage 中 C-01/C-02 仍为 unassessed，C-04/C-09/C-10 为 partially_aligned；checkpoint 明确建议先 review backlog
- Review Decision: `AD-001B` repair boundary 已满足 done_when，只是状态滞后；`AD-001A/B/C` 均已 verified，因此父项 `AD-001` 标记为 done，C-04 提升为 aligned
- Promoted Delta: 新增 `AD-009 Figure and FigureBlock formal contract audit`，用于正式评估 C-01 / C-02
- Root Cause: C-01 / C-02 长期处于 unassessed，阻塞持续工作流的理想完成态；审计发现 `FigureBlock::paint_selection_overlay()` 把渲染行为放进节点状态容器，属于轻微职责回流
- Minimal Fix: 将 selection overlay 绘制从 `FigureBlock` inherent method 移为 scene 渲染遍历 helper；recursive / iterative render 读取 block state 并调用 helper，FigureBlock 只保留运行时状态
- Files: `novadraw-scene/src/scene/mod.rs`, `novadraw-scene/src/scene/render_recursive.rs`, `novadraw-scene/src/scene/render_iterative.rs`, `agent/outer-loop-delta-backlog.yaml`, `agent/governance-contract-coverage.md`, `agent/inner-loop-checkpoint.md`, `agent/inner-loop-worklog.md`
- Delta Verification: cargo fmt ✅, cargo check ✅, cargo test -p novadraw-scene 139/139 + 3 doctests ✅, cargo test -p editor 6/6 ✅
- Decision: C-01 / C-02 标记为 aligned；`Figure` 只保留内在能力，`FigureBlock` 保持节点运行时状态容器
- Split Decision: 不拆分；本轮只处理 C-01/C-02 正式审计与发现的最小职责回流，不处理 focus state machine 或 render strategy wiring
- Post-Execution Reflection: Contract coverage 已无 unassessed；剩余 partially_aligned 为 C-09/C-10，下一轮应先 review 是否通过持续工作流记录实践提升 C-10，或继续处理 C-09 的接口职责边界风险
- New Candidate Deltas: 无
- Next Step: 下一轮从 ASSESS 开始，优先 review `C-09` / `C-10` 的 partially_aligned 状态，或选择 focus state machine / render strategy wiring 中最小职责项

## 2026-05-28 / Continuous Workflow Cycle 2

- Goal: 继续持续工作流第 2 轮 ASSESS / REVIEW，判断剩余 `C-09` / `C-10` partially_aligned 是否可直接收敛
- Assess: contract coverage 已无 unassessed；C-09 / C-10 仍为 partially_aligned；backlog 无新的 in_progress 项
- Review Decision: C-10 属于长期执行纪律，当前 worklog 已持续记录 Root Cause / Decision / Split Decision / Reflection，但仍保留观察，不在本轮主观提升为 aligned
- Review Decision: C-09 仍有真实审计价值，`NovadrawSystem` 高级 escape hatch 和 `NovadrawContext` 默认 panic 的能力边界可能存在接口表达不够精确的问题
- Promoted Delta: 新增 `AD-010 Interface boundary escape hatch audit`
- Split Decision: 本轮不直接修改接口；先把 C-09 风险建模为独立 delta，避免在 review 阶段跨边界改代码
- Verification: workflow state YAML/checkpoint schema 校验通过；未修改运行时代码
- Next Step: 下次从 `AD-010` 开始，先审计 escape hatch 与 context optional capability 是否是职责回流

## 2026-05-28 / AD-010

- Goal: 审计并收敛 interface boundary escape hatch，避免新接口为了兼容现状引入职责回流
- Root Cause: `NovadrawSystem` 在公开 trait 上暴露 `scene()` / `update_manager()` / `dispatcher()` 三个可变逃生口，虽然有注释门禁，但类型层面仍允许绕过组合根事务、pending mutation 和 redraw scheduling；`NovadrawContext` 用默认 panic 表达 selection / deferred mutation 可选能力，会把能力缺失推迟到运行时
- Minimal Fix: 从 `NovadrawSystem` trait 和 `WinitNovadrawSystem` impl 中移除三个公开逃生口，只保留 `render()` / `viewport_size()` / `request_update()`；`NovadrawContext` 的 `set_selected()` / `add_child_later()` / `remove_child_later()` / `reparent_later()` 改为必须显式实现的方法
- Documentation: `doc/理想架构设计.md` 的多平台架构图与使用示例不再展示 `system.scene()` / `system.dispatcher()` / `system.update_manager()`，改为通过组合根命名动作进入
- Files: `novadraw-scene/src/system/mod.rs`, `apps/editor/src/system.rs`, `novadraw-scene/src/context/mod.rs`, `doc/理想架构设计.md`, `agent/outer-loop-delta-backlog.yaml`, `agent/governance-contract-coverage.md`
- Verification: cargo fmt ✅, cargo check ✅, cargo test -p novadraw-scene 139/139 + 3 doctests ✅, cargo test -p editor 6/6 ✅, GetDiagnostics ✅
- Decision: C-09 更新为 aligned；公开接口不再提供绕过职责边界的便利入口
- Split Decision: 不处理 `apps/editor/src/system.rs` 中仍存在的交互 trace 日志；这是 hot-path logging 的另一个候选问题，和本轮接口边界不同
- Residual Risk: C-10 仍保持 partially_aligned，作为长期执行纪律观察项；focus state machine、render strategy wiring 与文档旧图示清扫仍是后续候选
- Next Step: 从 ASSESS 开始，优先 review C-10 是否仍需保持 partially_aligned，或选择 focus state machine / render strategy wiring / editor trace logging cleanup 中的一个最小职责项

## 2026-05-28 / C-10 ASSESS

- Goal: 只运行下一轮 ASSESS / REVIEW，判断 C-10 “任何架构改动都要说明为何更接近理想架构” 是否仍需保持 partially_aligned
- Evidence: `agent/workflow-continuous.md` 已强制每轮在 REFLECT 阶段判断是否真正减少架构差距；`inner-loop-worklog.md` 最近多轮真实 delta 持续记录 Root Cause / Minimal Fix / Decision / Split Decision / Post-Execution Reflection / Verification
- Review Decision: C-10 已不再是未落地流程风险，而是工作流强制项并已被多轮真实 delta 验证
- Coverage Update: C-10 从 partially_aligned 收敛为 aligned
- Split Decision: 本轮不执行代码 delta；这是 coverage 状态收敛，不与 focus state machine、render strategy wiring 或 editor trace logging cleanup 混合
- Verification: checkpoint schema 与 backlog YAML 校验通过；GetDiagnostics 为空
- Next Step: 从持续工作流 ASSESS 继续；coverage 已全部 aligned，下一步应检查 backlog 中是否还有 architecture/parity 类型 pending/proposed/in_progress/split/blocked 条目

## 2026-05-28 / Completion Audit Attempt

- Goal: 执行 `workflow-continuous` 的 completion audit 前置检查，判断是否可以进入 COMPLETE
- Inputs: `doc/理想架构设计.md`, `agent/governance-architecture-contracts.md`, `agent/governance-contract-coverage.md`, `agent/outer-loop-delta-backlog.yaml`, `agent/quality-discover-smoke-test.md`, 核心代码入口
- Audited Contracts: C-01 到 C-10 均被覆盖；按 discover smoke 要求至少抽查 Figure/FigureBlock/FigureGraph、Update/Event/Mutation、SceneHost/System、理想架构文档入口
- Checked Entrypoints: `novadraw-scene/src/figure/**`, `novadraw-scene/src/scene/mod.rs`, `novadraw-scene/src/update/**`, `novadraw-scene/src/event/mod.rs`, `novadraw-scene/src/context/mod.rs`, `novadraw-scene/src/mutation/mod.rs`, `apps/editor/src/system.rs`, `apps/editor/src/app_window.rs`, `apps/editor/src/scene_manager/scene_host.rs`, `doc/理想架构设计.md`
- Discover Smoke Result: 通过，重新发现多个 residual candidates；因此不能进入 COMPLETE
- Candidate Deltas: 新增 CAD-002 FigureGraph/FigureBlock public mutation surface audit、CAD-003 editor interaction hot-path logging cleanup、CAD-004 composition root residual public read surface audit、CAD-005 ideal architecture stale composition-root document cleanup、CAD-006 PendingMutation production boundary audit
- Coverage Update: C-02 / C-03 / C-05 / C-07 / C-08 / C-09 从 aligned 回退为 partially_aligned，指向对应 candidate；C-01 / C-04 / C-06 / C-10 仍保持 aligned
- Stop Reason: Completion audit 发现新 architecture candidates，未运行最终 baseline verification，必须先回到 REVIEW
- Verification: backlog YAML 校验通过；candidate_count=5
- Next Step: 进入 `review-delta-backlog`，优先判断 CAD-002 是否应提升为下一正式 delta

## 2026-05-28 / CAD-002 REVIEW

- Goal: 详细评估 `CAD-002 FigureGraph and FigureBlock public mutation surface audit`，只做 REVIEW，不改运行时代码
- Evidence Checked: `novadraw-scene/src/scene/mod.rs`, `novadraw-scene/src/context/mod.rs`, `novadraw-scene/src/scene/render_recursive.rs`, `novadraw-scene/src/scene/render_iterative.rs`, `novadraw-scene/src/lib.rs`, `novadraw/src/lib.rs`
- Root Cause: `FigureGraph` 是树关系与图级状态 owner，但 `blocks` / `uuid_map` 是 public 字段，crate 外可直接改 SlotMap / HashMap；这绕过 `new_block_with_parent`、`attach_child`、`detach_child`、validation path、notification effects、dirty/update 协议
- Scope Finding: 原 CAD-002 同时覆盖 `FigureGraph` 存储面和 `FigureBlock` 字段/方法公开面，范围过大；其中 `FigureGraph.blocks` / `uuid_map` 是最小且最高优先级的边界根因
- Duplicate Check: 不重复 AD-009；AD-009 评估并修复 FigureBlock 渲染行为回流，本轮发现的是 public mutation surface。与 AD-010 相关但不重复；AD-010 移除了 system trait escape hatch，本轮是 graph storage escape hatch
- Promote Decision: CAD-002 提升为 `AD-011 FigureGraph storage encapsulation audit`
- Split Decision: 本轮只建议先处理 `FigureGraph.blocks` / `uuid_map` 公开存储面；`FigureBlock` 字段/方法收窄留作 AD-011 后重新评估，避免一次性跨 render/event/layout 大改
- Next Step: 执行 AD-011 时先设计受控 accessor，再把 `blocks` / `uuid_map` 收窄为私有或 crate-private，并验证 event/render/layout 行为不变

## 2026-06-01 / AD-011

- Goal: 封装 `FigureGraph.blocks` / `uuid_map` 存储面，防止 crate 外绕过图级不变量直接修改 SlotMap 或 UUID 映射
- Root Cause: `FigureGraph` 是树关系、图级交互状态和命中测试 owner，但公开 `blocks` / `uuid_map` 会让外部 crate 直接插入、删除或修改节点，绕过 `new_block_with_parent()`、parent/children 维护、notification effects、dirty/update 协议
- Minimal Fix: 将 `FigureGraph.blocks` / `uuid_map` 收窄为私有字段；新增 `figure_bounds()` / `set_visible()` 图级命名方法，并给 crate 内 context 使用只读 `block()` accessor；apps/editor 与 apps/update-app 不再直接访问 `scene.blocks`
- Files: `novadraw-scene/src/scene/mod.rs`, `novadraw-scene/src/context/mod.rs`, `apps/editor/src/scene_manager/mod.rs`, `apps/update-app/src/main.rs`, `agent/outer-loop-delta-backlog.yaml`, `agent/governance-contract-coverage.md`, `agent/inner-loop-checkpoint.md`, `agent/inner-loop-worklog.md`
- Delta Verification: cargo fmt --check ✅, cargo check ✅, cargo check -p editor -p update-app ✅, cargo test -p novadraw-scene 139/139 + 3 doctests ✅, cargo test -p editor 6/6 ✅, GetDiagnostics ✅
- Decision: C-03 提升为 aligned；`FigureGraph` 存储不再是 crate 外 public mutation surface，树结构、UUID 映射和图级状态必须通过 FigureGraph 命名 API 进入
- Split Decision: 不收窄所有 `FigureBlock` 字段/方法；该范围更大且会触碰 render/event/layout 内部遍历，已登记为 `CAD-007 FigureBlock public mutation surface audit`
- Post-Execution Reflection: 本轮更接近理想架构，因为图级存储重新回到 `FigureGraph` 的单一所有权边界，外部 app 只能表达图级意图而不能直接操作内部 SlotMap
- New Candidate Deltas: `CAD-007 FigureBlock public mutation surface audit`
- Next Step: 下一轮从 REVIEW 开始，优先评估 `CAD-007` 是否应提升为 `AD-012`；若不提升，再回到 CAD-003/CAD-004/CAD-005/CAD-006

## 2026-06-01 / CAD-007 REVIEW

- Goal: 评估 `FigureBlock public mutation surface audit` 是否应提升为正式 delta，只做 REVIEW，不改运行时代码
- Evidence Checked: `novadraw-scene/src/scene/mod.rs`, `novadraw-scene/src/lib.rs`, `novadraw/src/lib.rs`, `novadraw-scene/src/update/repair.rs`, `novadraw-scene/src/scene/render_recursive.rs`, `novadraw-scene/src/scene/render_iterative.rs`
- Root Cause: `FigureBlock` 是节点运行时状态容器，但当前作为 public 类型从 `novadraw-scene` 和 `novadraw` facade 导出，且 parent/children/figure/selection/visibility/validity/layout 等字段均为 public；外部可直接修改节点状态而不经过 `FigureGraph` 的图级不变量、dirty/update、notification 和坐标传播协议
- Scope Finding: 该问题与 AD-011 不重复；AD-011 封装的是 `FigureGraph` 存储，CAD-007 关注的是取得 `FigureBlock` 后仍可直接改节点状态。问题也不应与 render/event/layout 内部遍历重构混在一起
- Promote Decision: CAD-007 提升为 `AD-012 FigureBlock public mutation surface audit`
- Split Decision: AD-012 先处理 crate 外公开可变面与 facade 导出面；内部 render/event/layout 所需读取能力应通过 crate 内 helper 或只读 query 保留，不在本轮改写全部内部遍历
- Next Step: 从 EXECUTE 开始执行 AD-012，先设计 `FigureBlock` 的只读/内部访问边界，再收窄 public 字段、public mutator 和 facade re-export

## 2026-06-01 / AD-012

- Goal: 收窄 `FigureBlock` 对 crate 外的 public mutation surface，确保节点运行时状态只能通过 `FigureGraph` 图级协议修改
- Root Cause: `FigureBlock` 字段和 mutator 曾对 crate 外公开，并通过 `novadraw-scene` / `novadraw` facade re-export；外部调用方可直接修改 parent/children、figure bounds、selection/hover/pressed、visibility/enabled/validity 与 layout 状态，绕过 `FigureGraph` 的树不变量、dirty/update、notification 和坐标传播协议
- Minimal Fix: 将 `FigureBlock` 字段收窄为 `pub(crate)`；删除未使用的 public mutator；保留 `id()` / `uuid()` / `children_count()` / `figure_bounds()` / size getters 等只读 query；从 `novadraw-scene` 与 `novadraw` facade re-export 中移除 `FigureBlock`
- Split Decision: 不重构 render/event/layout 内部遍历；这些模块仍作为 crate 内实现细节读取 `FigureBlock` 字段。后续若要进一步收窄 crate 内可变面，应另建 delta，避免扩大本轮范围
- Verification: cargo fmt --check passed; cargo check passed; cargo test -p novadraw-scene passed 139/139 + 3 doctests; cargo test -p editor passed 6/6
- Coverage Update: C-02 从 partially_aligned 提升为 aligned；C-09 仍 partially_aligned，由 CAD-004 / CAD-005 继续追踪组合根只读面与理想文档旧表述
- Next Step: 下一轮从 REVIEW 开始，优先评估 `CAD-003 editor interaction hot-path logging cleanup`

## 2026-06-01 / CAD-003 REVIEW

- Goal: 评估 `Editor interaction hot-path logging cleanup` 是否应提升为正式 delta，只做 REVIEW，不改运行时代码
- Evidence Checked: `apps/editor/src/system.rs`, `apps/editor/src/app_window.rs`, `apps/editor/src/scene_manager/interactive_figure.rs`, `apps/editor/Cargo.toml`
- Root Cause: editor 默认交互路径仍在 mouse move、raw pointer dispatch、Winit CursorMoved、interactive entered/exited 中打印 info 级日志；这些路径位于平台输入、事件分发入口或 Figure 回调热路径，违反“高频路径默认不打印运行时日志”约束
- Duplicate Check: 不重复 AD-008；AD-008 处理的是 engine `BasicEventDispatcher` 与 `FigureGraph` hit-test 热路径，本项只处理 editor 层残留日志
- Promote Decision: CAD-003 提升为 `AD-013 Editor interaction hot-path logging cleanup`
- Split Decision: AD-013 只移除或隔离 editor interaction 默认热路径日志；不处理 CAD-004 的组合根只读面，不重构通知体系，也不改变 InteractionTrace 返回值
- Next Step: 从 EXECUTE 开始执行 AD-013，保持事件 target 选择、坐标转换、selection/hover/pressed 状态写入和 PendingMutation apply 时机不变


## 2026-05-27 / AD-008

- Root Cause: `BasicEventDispatcher::refresh_mouse_target()` 与 `FigureGraph::find_mouse_event_target_at/from()` 位于鼠标移动和命中测试高频路径，却保留 `info` / `debug` / `trace` 日志；这会把诊断行为带入运行时热路径，并与引擎层事件分发职责混在一起
- Files: `novadraw-scene/src/event/mod.rs`, `novadraw-scene/src/scene/mod.rs`, `agent/outer-loop-delta-backlog.yaml`, `agent/inner-loop-checkpoint.md`, `agent/inner-loop-worklog.md`
- Delta Verification: cargo fmt ✅, cargo check ✅, cargo test -p novadraw-scene 139/139 + 3 doctests ✅, cargo test -p editor 5/5 ✅
- Baseline Verification: 全仓 `cargo check` 通过；IDE 诊断仍可能显示 stale `debug_render` cfg warning，但 fresh cargo 已通过
- Decision: event dispatch / hit-test 默认路径不打印运行时日志；如后续需要可观测性，应通过显式 debug feature、测试断言或外层 profiling 工具实现，不回流到热路径
- Split Decision: 不拆分；本轮只清理 event / hit-test 热路径日志，不处理 focus state machine、render strategy wiring 或 Viewport/ScrollPane 集成
- Post-Execution Reflection: 清理后事件语义保持不变，`BasicEventDispatcher` 仍只计算 target transition 并通过 `DispatchContext` 写图状态，`FigureGraph` 仍负责父链降域 hit-test
- New Candidate Deltas: Focus state machine audit；Render strategy wiring audit；文档旧 `WinitEventDispatcher` 图示清扫
- Next Step: 回到 backlog review，优先选择 `AD-001B repair boundary review`、`C-01/C-02 formal audit` 或 focus state machine 中的一个最小项


## 2026-05-25 / AD-005

- Root Cause: `mouse_target` / `focus_owner` / `captured` 已在 `FigureGraph`，但 editor 示例 `InteractiveRectFigure` 自持 `Mutex<InteractiveState>` 保存 hovered / pressed / selected；其中 selected 还与 `FigureBlock::is_selected` 重复，形成第二套状态源
- Files: `novadraw-scene/src/event/mod.rs`, `novadraw-scene/src/context/mod.rs`, `novadraw-scene/src/scene/mod.rs`, `novadraw-scene/src/scene/update_integration_test.rs`, `apps/editor/src/system.rs`, `apps/editor/src/scene_manager/interactive_figure.rs`, `apps/editor/src/scene_manager/mod.rs`, `doc/理想架构设计.md`, `agent/outer-loop-delta-backlog.yaml`, `agent/inner-loop-checkpoint.md`, `agent/inner-loop-worklog.md`, `agent/governance-contract-coverage.md`
- Delta Verification: cargo fmt ✅, cargo check ✅, cargo test -p novadraw-scene 139/139 + 3 doctests ✅, cargo test -p editor 5/5 ✅
- Baseline Verification: 全仓 `cargo check` 通过；IDE 诊断仍有 stale warning，但 fresh cargo 无 warning
- Decision: `FigureBlock` 承载 hovered / pressed 运行时状态；`BasicEventDispatcher` 通过 `DispatchContext` 写入状态但不持有状态；editor Figure 不再保存通用交互状态
- Split Decision: 不在基础 dispatcher 中自动切换 selection；选择策略属于工具/应用语义，当前只消除重复状态源并保留 `FigureGraph::set_selected(Some(id))` 作为图级 API
- Post-Execution Reflection: 交互状态现在沿 `EventDispatcher -> DispatchContext -> FigureGraph` 单向写入，Figure 回调只能响应事件和请求 repaint/invalidate，不再成为状态 owner
- New Candidate Deltas: Focus state machine audit（`focus_owner` 已归属 FigureGraph，但 key/focus gained/lost 完整状态机仍未实现）
- Next Step: AD-006 已 verified，可进入下一未完成 delta 或先做 backlog review


## 2026-05-25 / AD-004

- Root Cause: `app_window` 作为 winit 事件适配层，直接通过 `scene_manager_mut()` 切换场景、执行 `prim_translate()` 并手动 `request_update()`；这让子系统协作散落在平台入口，组合根边界不清晰
- Files: `apps/editor/src/system.rs`, `apps/editor/src/app_window.rs`, `apps/editor/src/scene_manager/mod.rs`, `apps/editor/src/scene_manager/interactive_figure.rs`, `apps/editor/Cargo.toml`, `apps/vello-app/src/main.rs`, `novadraw-scene/Cargo.toml`, `novadraw-scene/src/layout/flow_layout.rs`, `novadraw-scene/src/layout/border_layout.rs`, `agent/outer-loop-delta-backlog.yaml`, `agent/inner-loop-checkpoint.md`, `agent/inner-loop-worklog.md`, `agent/governance-contract-coverage.md`
- Delta Verification: cargo fmt ✅, cargo check ✅, cargo test -p editor 5/5 ✅, cargo test -p novadraw-scene 139/139 + 3 doctests ✅
- Baseline Verification: 全仓 `cargo check` 通过且无 warning；未运行 `workflow-verify.sh`
- Decision: `WinitNovadrawSystem` 暴露 `switch_scene()` / `translate_contents()` / `toggle_iterative_render()` 作为组合根动作；`app_window` 只把键盘输入映射为组合根动作
- Cleanup: 移除未接入的 CGEvent mouse simulator、`core-graphics` 依赖、无调用 logical wrapper、无效 layout 统计变量、测试 mock warning 和 vello demo 未使用 resize 方法；补齐 `debug_render` feature 声明
- Split Decision: 不处理 `NovadrawSystem` trait 高级 escape hatch 的进一步收窄；它已在 AD-001C 标注风险，是否拆分需后续 API 稳定性评估
- Post-Execution Reflection: 平台入口现在只负责事件适配和 UI 快捷键映射，子系统依赖方向回到 app_window -> WinitNovadrawSystem -> SceneManager/FigureGraph/SceneHost
- New Candidate Deltas: Render strategy wiring audit（`use_iterative_render` 是否真正选择 render path 仍需确认）
- Next Step: AD-005 Interaction state ownership audit


## 2026-05-25 / AD-003

- Root Cause: PendingMutation 队列和 `apply_pending_mutations()` 时机已存在，但 `SceneNovadrawContext` 没有接入 mutation 生产链路，Figure 回调无法通过引擎上下文申请 add/remove/reparent，未来容易绕回直接改 `FigureGraph`
- Files: `novadraw-scene/src/mutation/mod.rs`, `novadraw-scene/src/context/mod.rs`, `novadraw-scene/src/scene/mod.rs`, `apps/editor/src/system.rs`, `doc/理想架构设计.md`, `agent/outer-loop-delta-backlog.yaml`, `agent/inner-loop-checkpoint.md`, `agent/inner-loop-worklog.md`, `agent/governance-contract-coverage.md`
- Delta Verification: cargo fmt ✅, cargo check ✅, cargo test -p novadraw-scene 139/139 + 3 doctests ✅, cargo test -p editor 7/7 ✅
- Baseline Verification: 未运行全仓 `workflow-verify.sh`；本轮执行 AD-003 delta scope 验证
- Decision: Figure 回调通过 `add_child_later` / `remove_child_later` / `reparent_later` enqueue `PendingMutation`；`SceneDispatchContext` 只在回调中记录 mutation，editor 顶层输入边界继续负责 `apply_pending_mutations()`
- Split Decision: 不处理 `set_contents` 作为应用级场景切换入口；不处理 layout manager / constraint mutation 类型，留给后续布局约束 delta
- Post-Execution Reflection: 现在结构变更有了生产链路与 apply 时机闭环，核心边界是“回调期间只记录，分发完成后才改树”
- New Candidate Deltas: Layout constraint mutation audit（如果引入 constraint/layout manager 运行期变更，需要扩展 PendingMutation 类型）
- Next Step: AD-004 NovadrawSystem composition root audit


## 2026-05-25 / AD-002

- Root Cause: `WinitSceneHost` 持有 `use_iterative_render`，这是 editor/demo 的渲染策略状态，不属于平台 redraw 调度；`novadraw-scene/src/scene_host.rs` 文档仍把 Canvas 映射到旧 `WinitEventDispatcher`，容易混淆事件平台适配与 paint entry
- Files: `apps/editor/src/scene_manager/scene_host.rs`, `apps/editor/src/system.rs`, `novadraw-scene/src/scene_host.rs`, `agent/outer-loop-delta-backlog.yaml`, `agent/inner-loop-checkpoint.md`, `agent/inner-loop-worklog.md`
- Delta Verification: cargo fmt ✅, cargo check ✅, cargo check -p editor ✅, cargo test -p editor 7/7 ✅
- Baseline Verification: 未运行全仓 `workflow-verify.sh`；本轮执行 AD-002 delta scope 验证
- Decision: `WinitSceneHost` 只保留 window proxy 与 redraw pending；editor/render 策略状态上移到组合根 `WinitNovadrawSystem`
- Split Decision: 不处理 `use_iterative_render` 是否真正切换 editor 渲染路径；这属于 render strategy wiring，不属于 SceneHost thin boundary
- Post-Execution Reflection: SceneHost 可执行 `execute_update(scene, update_manager, renderer)` 作为平台 redraw 入口协调，但不应持有业务/图状态或 demo 策略状态
- New Candidate Deltas: Render strategy wiring audit（确认 editor 的 iterative/recursive 切换是否仍应存在，以及应由谁选择 render path）
- Next Step: AD-003 PendingMutation timing audit


## 2026-05-25 / AD-001C

- Root Cause: 事件入口的 `was_queued -> dispatch -> request_update` transition 样板分散在多个 wrapper；`SceneHost` 文档仍描述未实现的 `about_to_wait` 每帧驱动；host queued flag 与 UpdateManager queued flag 之间缺少漏同步自愈路径；`NovadrawSystem` 裸访问器未说明绕过调度事务的风险
- Files: `apps/editor/src/system.rs`, `apps/editor/src/scene_manager/scene_host.rs`, `novadraw-scene/src/scene_host.rs`, `novadraw-scene/src/system/mod.rs`, `agent/outer-loop-delta-backlog.yaml`, `agent/inner-loop-checkpoint.md`, `agent/inner-loop-worklog.md`
- Delta Verification: cargo fmt ✅, cargo check ✅, cargo check -p editor ✅, cargo test -p editor 7/7 ✅
- Baseline Verification: 未运行全仓 `workflow-verify.sh`；本轮执行 AD-001C delta scope 验证
- Decision: `WinitNovadrawSystem::run_update_transaction()` 是组合根事务边界，统一负责检测 UpdateManager 队列从空到非空并触发 `SceneHost.request_update()`；`UpdateManager` 继续只负责队列与两阶段更新
- Split Decision: 不拆分；本轮只处理 scheduling boundary，不处理 SceneHost 是否过厚、demo dead code warning、debug_render feature warning
- Post-Execution Reflection: 双 queued flag 可以保留为两层语义：UpdateManager 表示核心待更新，SceneHost 表示平台 redraw 已挂起；`execute_update()` 通过读取 UpdateManager queued 进行自愈，降低裸访问或未来入口漏调度的风险
- New Candidate Deltas: 无
- Next Step: AD-002 SceneHost thin boundary audit，检查 host 是否仍保持极薄平台调度层


## 2026-05-25 / AD-007 Viewport coordinate-domain audit

- Root Cause: `Viewport` 仍使用 `screen_to_world` / `world_to_screen` 命名，虽然当前只是数学 helper，但 API 会暗示存在 Figure 树外的全局 world 坐标；同时 transform 组合只在 origin=0 用例下被覆盖，非零 origin 时公式风险未被测试锁住
- Files: `novadraw-scene/src/viewport.rs`, `apps/transform-app/src/main.rs`, `doc/04-coordinates/coordinates.md`, `doc/理想架构设计.md`, `agent/outer-loop-delta-backlog.yaml`, `agent/inner-loop-checkpoint.md`, `agent/inner-loop-worklog.md`
- Delta Verification: cargo fmt ✅, cargo test -p novadraw-scene 138/138 + 3 doctests ✅
- Baseline Verification: 未运行全仓 `workflow-verify.sh`；本轮只执行 delta scope 验证
- Decision: `Viewport` 统一使用 viewport/content 坐标域；`origin` 表示 viewport 左上角对应的 content 坐标；转换公式为 `content = viewport_point / zoom + origin` 与 `viewport_point = (content - origin) * zoom`
- Split Decision: 不将真实 Viewport/ScrollPane Figure-tree 集成混入本轮；当前只收口 standalone helper 的语义、公式与文档
- Post-Execution Reflection: Viewport 应作为父链坐标协议的未来扩展点，而不是事件或渲染入口的特殊全局通道；`translate_to_parent` / `translate_from_parent` 已作为协议方向锚点保留
- New Candidate Deltas: Viewport/ScrollPane Figure-tree integration（通过 Figure 节点、client area 裁剪、hit-test、damage repair 接入父链坐标协议）
- Next Step: AD-007 主干与 Viewport audit 均已 verified，建议切回 AD-001C scheduling boundary audit


## 2026-05-20 / AD-007

- Root Cause: `Bounded::client_area()` 对 `use_local_coordinates=true` 仍返回 `bounds.x/y + insets`，没有像 draw2d `getClientArea()` 一样把坐标根 client area 原点重置到 `(0,0)`；同时 recursive / iterative 渲染的非坐标根分支直接 clip 完整 bounds，忽略 insets，导致 children 可绘制进 border 区域
- Files: `novadraw-scene/src/figure/mod.rs`, `novadraw-scene/src/scene/mod.rs`, `novadraw-scene/src/scene/render_recursive.rs`, `novadraw-scene/src/scene/render_iterative.rs`, `novadraw-scene/src/scene/bounds_test.rs`, `agent/outer-loop-delta-backlog.yaml`, `agent/inner-loop-checkpoint.md`, `agent/inner-loop-worklog.md`
- Delta Verification: cargo fmt ✅, cargo test -p novadraw-scene 136/136 + 3 doctests ✅
- Baseline Verification: 未运行全仓 `workflow-verify.sh`；本轮只执行 delta scope 验证
- Decision: `Bounded::client_area()` 成为 client area SSOT；坐标根返回 `(0,0,width,height)`，非坐标根返回 `bounds + insets`；布局和渲染统一复用该语义
- Split Decision: 不拆分；本轮只处理 render/client-area 闭环，Viewport/scroll/scale 留作下一轮独立 delta
- Post-Execution Reflection: 早期 AD-007 backlog 中“全绝对坐标”契约已经过期，当前正确契约是 `bounds` 表示相对最近坐标根的绝对值；render 修复后主干链路已基本闭合
- New Candidate Deltas: Viewport coordinate-domain audit（确认 screen/world API 是否为独立视口抽象，以及如何接入 draw2d translateToParent/fromParent 协议）
- Next Step: 审计 `novadraw-scene/src/viewport.rs` 与 viewport 文档，避免 viewport 概念重新引入全局/世界坐标假设



- Goal: 通知体系最小基础设施 — 扩展 UpdateListener、接入运行时主干、打通端到端 flush 链路
- Root Cause: `UpdateListener` 只有 `()` no-op 实现；FigureGraph 和 SceneUpdateManager 各有独立 effect 队列但从未被 flush 到任何 listener；ADR-002 要求 effect 在事务边界统一 drain/flush 但缺失这一步
- Files: `novadraw-scene/src/update/listener.rs`, `novadraw-scene/src/update/deferred.rs`, `novadraw/src/lib.rs`, `apps/editor/src/system.rs`, `agent/inner-loop-checkpoint.md`, `agent/inner-loop-worklog.md`
- Delta Verification: cargo check ✅, cargo test -p novadraw-scene 126/126 ✅, cargo check -p editor ✅
- Baseline Verification: cargo fmt --check 因 BASELINE-001 失败（非本轮引入）
- Decision: UpdateListener 扩展为三个独立方法（on_update_event / on_figure_event / on_notify）；listener 由 SceneUpdateManager 内部持有；flush 在 perform_update() 末尾（clear_dirty_and_flag 之后）执行，确保 listener 观察稳定状态；双队列（FigureGraph + SceneUpdateManager）统一合并分发
- Split Decision: 无新增拆分；AD-006 最小基础设施已收敛，后续 PropertyChanged 事件、subscription 过滤机制可独立作为子 delta
- Post-Execution Reflection: 设计保持了 Draw2D 的语义分层（FigureEvent / UpdateEvent）与 Zed 的执行模型（effect 队列 + 延迟 flush），listener 不吞并图状态或调度职责
- New Candidate Deltas: 无
- Next Step: AD-006 最小基础设施已收敛；下一轮处理 AD-001B（repair boundary）或 AD-001C（scheduling boundary）

## 2026-05-19 / AD-007

- Goal: 统一 bounds 的正式语义，消除文档/代码中“局部坐标”与“绝对坐标”两套并存定义
- Root Cause: `coordinates.md` 持续使用“局部坐标”/“相对于父 Figure”来描述 bounds，与代码实现（全绝对坐标）不一致
- Files: `doc/04-coordinates/coordinates.md`, `novadraw-scene/src/figure/mod.rs`, `novadraw-scene/src/scene/mod.rs`, `novadraw-scene/src/update/repair.rs`, `novadraw-scene/src/update/deferred.rs`, `agent/inner-loop-checkpoint.md`, `agent/inner-loop-worklog.md`, `agent/outer-loop-delta-backlog.yaml`
- Delta Verification: cargo check ✅, cargo test -p novadraw-scene 126/126 ✅
- Baseline Verification: cargo fmt --check 因 BASELINE-001 失败（非本轮引入）
- Decision: 明确项目采用“全绝对坐标”模型（所有 bounds 处于同一全局坐标空间），与 g2 的坐标根 offset 语义有设计差异但内部自洽；`use_local_coordinates` 只控制 translate 传播和渲染 offset，不创建独立坐标域
- Split Decision: 无新增拆分
- Post-Execution Reflection: 尝试在 repair 中引入坐标根 offset 翻译后发现与现有测试语义冲突（测试期望 damage rect 不做 offset 转换），确认当前全绝对坐标模型是刻意的设计选择，应在契约级别显式记录而非强行“修复”
- New Candidate Deltas: 无
- Next Step: AD-007 坐标模型统一已基本完成；恢复 AD-006 通知体系基础设施，或处理 AD-001B/AD-001C

## 2026-04-29 / Coordinate Delta Switch

- Goal: 决定是否把坐标模型统一提升为当前主线，并暂停通知体系基础设施
- Root Cause: 代码、文档、测试同时存在“局部坐标”与“相对最近坐标根的绝对坐标”两套 bounds 语义，继续推进 `AD-006` 会让通知体系建立在不稳定坐标契约上
- Files: `agent/outer-loop-delta-backlog.yaml`, `agent/inner-loop-checkpoint.md`, `novadraw-scene/src/scene/mod.rs`, `novadraw-scene/src/update/repair.rs`, `doc/02-figure/figure_bounds.md`, `doc/04-coordinates/coordinates.md`
- Verification: g2/draw2d 坐标源码与仓库文档再次核对完成；当前代码验证进行中
- Decision: 新增 `AD-007 Coordinate model alignment with g2` 作为新的当前主线；`AD-006` 保留为后续 pending 条目
- Split Decision: 无新增拆分；属于主线切换而非继续拆分
- Post-Execution Reflection: 通知体系比坐标模型更上层；若 bounds / dirty / transform 语义继续混用，后续 repair、hit-test、viewport、通知都会持续漂移
- New Candidate Deltas: 无
- Next Step: 以 `AD-007` 为主线，先统一 bounds 正式定义，再逐步收口 translate API 与 repair/damage 主链

## 2026-04-28 / Backlog Review

- Goal: 评估 `AD-001` 是否应继续作为单一主线，以及是否需要先切换到通知体系基础设施
- Root Cause: `AD-001` 已同时承载 validation、repair、scheduling 三类边界问题，继续在同一 delta 下推进会失焦；而通知体系又是 repair / viewport / scroll 等后续机制的公共底座
- Files: `agent/outer-loop-delta-backlog.yaml`, `agent/inner-loop-checkpoint.md`, `agent/inner-loop-worklog.md`, `novadraw-scene/src/update/listener.rs`, `novadraw-scene/src/scene/mod.rs`
- Verification: backlog review 完成；checkpoint schema 仍满足 v1
- Decision: 将 `AD-001` 正式拆分为 `AD-001A validation boundary`、`AD-001B repair boundary`、`AD-001C scheduling boundary`；新增 `AD-006 notification foundation audit` 作为新的主线候选
- Split Decision: `AD-001` 转为 `split`，其中 validation 子项标记为 `verified`，repair / scheduling 保持 `pending`
- Post-Execution Reflection: 通知体系不应继续作为 `CAD-001` 这种窄候选保留，而应提升为正式架构条目；否则后续 viewport / scroll / router 仍缺少稳定基础设施
- New Candidate Deltas: 无；原 `CAD-001` 已提升并扩展为 `AD-006`
- Next Step: 以 `AD-006` 作为当前主线，先定义最小通知分层和第一轮接入的单条通知链路

## 2026-04-23 / AD-001

- Goal: 收口 UpdateManager 在 validation phase 的职责边界，避免图级语义继续留在更新服务里
- Root Cause: `SceneUpdateManager::perform_validation()` 直接读取 `FigureGraph.blocks` 并决定可见性/启用语义，导致 UpdateManager 从“两阶段编排器”回流为“图语义执行者”
- Files: `novadraw-scene/src/update/mod.rs`, `novadraw-scene/src/update/deferred.rs`, `novadraw-scene/src/scene/mod.rs`, `novadraw-scene/src/scene/update_integration_test.rs`, `agent/outer-loop-delta-backlog.yaml`, `agent/inner-loop-checkpoint.md`
- Verification: `cargo test -p novadraw-scene` 通过；`cargo check` 通过；`./agent/workflow-verify.sh` 在 `cargo fmt --check` 阶段因仓库既有格式化漂移失败
- Decision: 本轮只把 validation 图语义回收到 `FigureGraph`，不同时触碰 repair phase 和 system 调度，保持单轮单 delta 的最小切口
- Split Decision: 暂不拆分，下一轮先由 backlog review 判断
- Post-Execution Reflection: `AD-001` 仍未完全收敛，至少还剩 repair phase 与上层调度样板两个子问题；继续用一个大 delta 承载会开始失焦
- New Candidate Deltas: 可考虑把 `AD-001` 拆成 “validation boundary” / “repair boundary” / “update scheduling boundary” 三个更小条目
- Next Step: 下一轮先判断是否拆分 `AD-001`；若不拆，优先审计 repair phase 是否仍让 UpdateManager 持有不必要的图语义

## 2026-04-10 / Workflow Bootstrap

- Goal: 建立 solo coder 可恢复的架构改进工作流
- Root Cause: 长任务容易被中断，恢复成本高，导致每轮都要重新梳理上下文
- Files: agent/README.md, agent/governance-architecture-contracts.md, agent/outer-loop-delta-backlog.yaml, agent/inner-loop-checkpoint.md, agent/interruptions-inbox.md
- Verification: 结构检查待执行
- Decision: 采用 `agent/` 状态文件加 `.trae/skills` 技能目录的兼容方案
- Next Step: 用 AD-001 开始第一轮架构审计
