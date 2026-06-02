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
