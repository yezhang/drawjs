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

## 2026-06-12 / AD-033

- Goal: 启动 M3 代码主线，先收敛 recursive/iterative render 在 sibling child clipping state 上的不等价。
- Root Cause: recursive render 在 paintClientArea 设置 parent clientArea clip 后会 `push_state()`，child clip 结束后 `restore_state()` 回到 parent clientArea；iterative render 缺少该保存点，`ExitChild` 会恢复到更外层状态，导致 sibling clip state 与递归路径不等价。
- Minimal Fix:
  - `render_iterative.rs` 在 `EnterClientArea` 完成 parent clientArea transform/clip 后新增 `push_state()`，保存 parent clientArea 状态。
  - `ExitClientArea` 改为 `pop_state()` 后再 `restore_state()`，对齐 recursive paintClientArea 状态栈语义。
  - `graph/mod.rs` 新增 render command signature helper 和 `test_iterative_render_matches_recursive_clip_state_for_siblings`，验证 sibling 场景下 recursive/iterative command signature 一致。
- Files: `novadraw-scene/src/graph/render_iterative.rs`, `novadraw-scene/src/graph/mod.rs`, `agent/draw2d-core-milestones.yaml`, `agent/goal-roadmap.md`, `agent/backlog/index.yaml`, `agent/backlog/recent.yaml`, `agent/backlog/archive/2026-06.yaml`, `agent/inner-loop-checkpoint.md`, `agent/inner-loop-worklog.md`
- Delta Verification: cargo fmt --check ✅, cargo test -p novadraw-scene iterative_render ✅, cargo test -p novadraw-scene 162/162 + 3 integration + 3 doctests ✅, cargo clippy -p novadraw-scene -- -D warnings ✅
- Baseline Verification: ruby agent/workflow-doctor.rb ✅, git diff --check ✅, bash agent/workflow-verify.sh --fast ✅
- Decision: AD-033 状态 `verified`；M3 从 `not_started` 推进到 `in_progress`。
- Split Decision: 本轮只修复 iterative clientArea state equivalence；nested clipping、border/insets、paint-vs-hit-test 作为后续 M3 delta，不混入本轮。
- Post-Execution Reflection: 该修复触碰 protected render_iterative 文件，但属于对标 Draw2D paintChildren 状态恢复的主流程偏差修正；下一步继续补 M3 clipping probes。
- Next Step: M3 nested clipping and border/insets rendering tests。

## 2026-06-12 / WF-004

- Goal: 给 workflow 增加执行动量门禁，防止连续 documentation-only delta 替代代码主线推进。
- Root Cause: AD-031/AD-032 让用户感知到近期工作偏向 md/status 文件；现有 workflow 有拆分、active/recent 生命周期和 milestone 状态门禁，但没有阻止连续 documentation-only delta 的规则。
- Minimal Fix:
  - `agent/README.md` 新增“执行动量门禁”：最近两个终态 delta 不得同时为 documentation-only；文档型 delta 之后必须回到产品代码、运行时代码、可执行 workflow code 或 tests。
  - `agent/backlog/schema.yaml` 同步记录 documentation-only delta 约束。
  - `agent/workflow-doctor.rb` 新增 recent momentum 校验，根据 archive/recent 的 evidence/files 判断最近两个终态 delta 是否都是 documentation-only。
  - backlog current delta 更新为 `WF-004`，next recommended delta 保持 `M3 render traversal and clipping contract audit`。
- Files: `agent/workflow-doctor.rb`, `agent/README.md`, `agent/backlog/schema.yaml`, `agent/backlog/index.yaml`, `agent/backlog/recent.yaml`, `agent/backlog/archive/2026-06.yaml`, `agent/inner-loop-checkpoint.md`, `agent/inner-loop-worklog.md`
- Delta Verification: ruby agent/workflow-doctor.rb ✅, git diff --check ✅
- Baseline Verification: 本轮是 workflow 门禁 delta，未运行全仓 cargo；执行了 workflow doctor 与 diff whitespace gate。
- Decision: WF-004 状态 `verified`；documentation-only 连续执行现在会被 workflow doctor 拦截。
- Split Decision: 不继续扩展成“每个 milestone 的最大 summary 数量”自动推断；当前先用 recent 连续 documentation-only 作为低成本硬门禁。
- Post-Execution Reflection: 该规则把用户担心转化为可执行门禁，同时保留必要的单次 summary/status 收口能力；下一步必须回到 M3 code-bearing 主线。
- Next Step: M3 render traversal and clipping contract audit。

## 2026-06-12 / AD-032

- Goal: M2 product-layer existence checks，确认 Figure 树与盒模型不仅 contract-aligned，也能通过 crate 外部产品 API 被使用。
- Root Cause: M2 已经通过 AD-026 至 AD-031 覆盖 YAML probes 与契约对齐，但 `product-deliverables.md` 的 5 基础图元、Figure/FigureBlock/FigureGraph 角色、三段式 paint 顺序缺少独立产品层存在性检查。
- Minimal Fix:
  - 新增 `novadraw-scene/tests/m2_product_existence.rs`，从 crate 外部构造 5 个产品图元并装箱为 `Box<dyn Figure>`。
  - 覆盖 `FigureGraph` 产品 API：树挂载、block 只读查询、child order、z-index、hit-test、visible/enabled 有效状态。
  - 通过 marker Figure 验证三段式 paint 顺序：parent figure -> child figure -> child border -> parent border。
  - 新增 `agent/m2-product-existence-checks.md`，记录产品层证据、验证命令与 residual risks。
- Files: `novadraw-scene/tests/m2_product_existence.rs`, `agent/m2-product-existence-checks.md`, `agent/draw2d-core-milestones.yaml`, `agent/goal-roadmap.md`, `agent/backlog/index.yaml`, `agent/backlog/recent.yaml`, `agent/backlog/archive/2026-06.yaml`, `agent/inner-loop-checkpoint.md`, `agent/inner-loop-worklog.md`
- Delta Verification: cargo fmt --check ✅, cargo test -p novadraw-scene 161/161 + 3 integration + 3 doctests ✅, cargo clippy -p novadraw-scene -- -D warnings ✅
- Baseline Verification: ruby agent/workflow-doctor.rb ✅, bash agent/workflow-verify.sh --fast ✅
- Decision: AD-032 状态 `verified`；M2 从 `contract_aligned` 推进到 `behavior_verified`。
- Split Decision: 不把 M2 推进到 `complete`；`shapes-demo` 截图/端到端验证属于 demo 层，后续独立收口。
- Post-Execution Reflection: M2 现在同时具备契约层 probes 与产品层存在性证据，可作为 M3/M4/M6 的更稳固依赖；下一步应进入 M3 render traversal and clipping，而不是继续扩大 M2 范围。
- Next Step: M3 render traversal and clipping contract audit。

## 2026-06-12 / AD-031

- Goal: 汇总 M2 Figure 树与盒模型的 contract alignment，判断是否可从 `in_progress` 推进到 `contract_aligned`。
- Root Cause: M2 已完成多个局部 delta，但 milestone 状态仍停留在 `in_progress`；需要将 add/remove/reparent、z-order、bounds/clientArea、visible/enabled 四类 YAML probes 与自动化证据逐项对齐，避免状态推进依赖口头判断。
- Minimal Fix:
  - 新增 `agent/m2-contract-alignment-summary.md`，逐项登记 M2 scope、contracts、probes、delta evidence、verification 与 residual risks。
  - 确认 AD-026 到 AD-030 已覆盖 M2 YAML probes。
  - 将 M2 从 `in_progress` 推进到 `contract_aligned`。
  - 明确本轮不推进 `behavior_verified` 或 `complete`，产品层存在性检查留到下一步。
- Files: `agent/m2-contract-alignment-summary.md`, `agent/draw2d-core-milestones.yaml`, `agent/backlog/index.yaml`, `agent/backlog/recent.yaml`, `agent/backlog/archive/2026-06.yaml`, `agent/goal-roadmap.md`, `agent/inner-loop-checkpoint.md`, `agent/inner-loop-worklog.md`
- Delta Verification: cargo test -p novadraw-scene ✅, cargo clippy -p novadraw-scene -- -D warnings ✅, ruby agent/workflow-doctor.rb ✅, bash agent/workflow-verify.sh --fast ✅
- Decision: AD-031 状态 `verified`；M2 推进到 `contract_aligned`。
- Split Decision: 本轮不新增代码能力，不启动 M3，不做 M2 product-layer checks。
- Post-Execution Reflection: M2 当前已经具备作为 M3/M4/M6 契约层依赖的条件，但仍不能被描述为产品行为完成。
- Next Step: M2 product-layer existence checks。

## 2026-06-12 / AD-030

- Goal: 推进 M2 Figure 树与盒模型的 remove / reparent 生命周期契约审计。
- Root Cause: `apply_reparent_mutation` 已拒绝 invalid parent、self reparent 与 descendant reparent，但 detach 旧父发生在 attach 新父之前；若新父已异常持有 child，attach 会失败并留下 orphan，违反 remove/reparent no partial write 契约。
- Minimal Fix:
  - 对标 Draw2D：add/reparent 会先保证不会形成 cycle，remove 会只接受真实 direct child，并触发重绘/重验证。
  - `FigureGraph` 新增 `contains_direct_child` 内部查询。
  - `apply_reparent_mutation` 在任何 detach/attach 写入前校验旧父确实持有 child、新父尚未持有 child。
  - 新增 wrong-parent remove 无副作用测试：不 detach child、不清交互状态、不排队 invalid/dirty。
  - 新增 duplicate-new-parent reparent 无副作用测试：不 detach 旧父、不改 parent、不排队 invalid/dirty。
- Files: `novadraw-scene/src/graph/mod.rs`, `novadraw-scene/src/graph/update_integration_test.rs`, `agent/backlog/index.yaml`, `agent/backlog/recent.yaml`, `agent/backlog/archive/2026-06.yaml`, `agent/goal-roadmap.md`, `agent/inner-loop-checkpoint.md`, `agent/inner-loop-worklog.md`
- Delta Verification: cargo fmt --check ✅, cargo test -p novadraw-scene apply_pending 9/9 ✅, cargo test -p novadraw-scene 161/161 + 3 doctests ✅, cargo clippy -p novadraw-scene -- -D warnings ✅, ruby agent/workflow-doctor.rb ✅, bash agent/workflow-verify.sh --fast ✅
- Decision: AD-030 状态 `verified`；M2 保持 `in_progress`。
- Split Decision: 本轮不新增公开 remove/reparent API，不改变 pending mutation 时序，不处理 M3 绘制闭环。
- Post-Execution Reflection: remove/reparent 的核心风险不是单个 guard 缺失，而是结构 mutation 的原子性；所有拒绝路径必须在写入前完成判断。
- Next Step: M2 contract alignment summary。

## 2026-06-12 / AD-029

- Goal: 推进 M2 Figure 树与盒模型的 bounds/insets/clientArea 一致性审计。
- Root Cause: `Bounded::client_area()` 与 render/layout 已大体对齐 Draw2D，但 hit-test 与 mouse target 子树下降只检查父 Figure bounds，未按 Draw2D `findDescendantAtExcluding` / `findMouseEventTargetInDescendantsAt` 先用父 clientArea 过滤 children 搜索。
- Minimal Fix:
  - 对标 Draw2D：`getClientArea()` 基于 bounds shrink insets；`useLocalCoordinates()` 时原点重置为 `(0,0)`。
  - `hit_test_from` 在父 Figure 自身按 bounds 命中后，只在转换后的点落入父 clientArea 时继续搜索 children。
  - `find_mouse_event_target_from` 同步采用父 clientArea 门禁；父自身是否作为 target 仍由 `wants_mouse_events()` 决定。
  - 新增 M2 契约测试覆盖点位于父 bounds 但落在 clientArea 外时不会命中 child，进入 clientArea 后正常命中 child / mouse target。
  - 顺手修复 `workflow-doctor.rb` 默认外部/内部编码为 UTF-8，避免普通 `ruby agent/workflow-doctor.rb` 在中文 roadmap 上触发 US-ASCII 读取错误。
- Files: `novadraw-scene/src/graph/mod.rs`, `agent/workflow-doctor.rb`, `agent/backlog/index.yaml`, `agent/backlog/recent.yaml`, `agent/backlog/archive/2026-06.yaml`, `agent/goal-roadmap.md`, `agent/inner-loop-checkpoint.md`, `agent/inner-loop-worklog.md`
- Delta Verification: cargo fmt --check ✅, cargo test -p novadraw-scene parent_client_area ✅, cargo test -p novadraw-scene bounds_test 26/26 ✅, cargo test -p novadraw-scene 159/159 + 3 doctests ✅, cargo clippy -p novadraw-scene -- -D warnings ✅, ruby agent/workflow-doctor.rb ✅, bash agent/workflow-verify.sh --fast ✅
- Decision: AD-029 状态 `verified`；M2 保持 `in_progress`。
- Split Decision: 本轮不处理 remove dispose 语义、不扩大到 M3 绘制裁剪闭环、不改 recursive / iterative render 主流程。
- Post-Execution Reflection: clientArea 不只是绘制裁剪区域，也必须是子树命中搜索的边界；父 Figure 自身可按 bounds 命中，但 children 不能越过父 clientArea 被命中。
- Next Step: M2 remove and reparent lifecycle contract audit。

## 2026-06-12 / AD-028

- Goal: 推进 M2 Figure 树与盒模型的 child order / z-order 契约审计。
- Root Cause: `children` 存储顺序已被 render 正向遍历与 hit-test 反向遍历隐式使用，但 crate 外没有图级命名 API 查询或调整 sibling z-order；继续依赖内部 `children` 字段会破坏 FigureGraph 封装与树不变量。
- Minimal Fix:
  - 对标 Draw2D：children 正序绘制，reverse children 优先用于 findFigureAt / findMouseEventTargetAt。
  - `FigureGraph` 新增 `child_order` / `child_z_index` 只读查询，明确 index 越大越靠顶层。
  - `FigureGraph` 新增 `move_child_to_index` / `bring_child_to_front` / `send_child_to_back`。
  - 重排 API 只允许直接 child；非法 parent、非直接 child、越界 index 与 no-op 均无副作用返回 false。
  - 新增 M2 契约测试覆盖 add append 顺序、z-index 查询、重排后 topmost hit-test 变化，以及非法重排无副作用。
- Files: `novadraw-scene/src/graph/mod.rs`, `agent/backlog/index.yaml`, `agent/backlog/recent.yaml`, `agent/backlog/archive/2026-06.yaml`, `agent/goal-roadmap.md`, `agent/inner-loop-checkpoint.md`, `agent/inner-loop-worklog.md`
- Delta Verification: cargo fmt --check ✅, cargo test -p novadraw-scene z_order ✅, cargo test -p novadraw-scene 157/157 + 3 doctests ✅, cargo clippy -p novadraw-scene -- -D warnings ✅, ruby agent/workflow-doctor.rb ✅, bash agent/workflow-verify.sh ✅
- Decision: AD-028 状态 `verified`；M2 保持 `in_progress`。
- Split Decision: 本轮不处理 bounds/insets/clientArea 一致性、不做 remove dispose 语义、不推进 M3 绘制裁剪闭环。
- Post-Execution Reflection: z-order 是 FigureGraph 的 sibling 顺序语义，不应通过暴露 `children` 可变存储解决；命名 API 能保留树不变量并让 hit-test/render 的顺序契约可测试。
- Next Step: M2 bounds/insets/clientArea consistency audit。

## 2026-06-12 / AD-027

- Goal: 推进 M2 Figure 树与盒模型的 visible/enabled 父链有效状态传播。
- Root Cause: `FigureBlock.is_visible` / `is_enabled` 是节点本地状态；render 和 hit-test 从父节点遍历时会自然跳过隐藏/禁用路径，但 `repaint` 与 validation 可直接从 child id 进入，若只看 child 本地标志，就会绕过祖先状态。
- Minimal Fix:
  - `FigureGraph` 新增 `is_visible` / `is_enabled` 本地查询。
  - `FigureGraph` 新增 `is_effectively_visible` / `is_effectively_enabled` 父链查询。
  - 新增 `set_enabled`，并让 `set_visible(false)` / `set_enabled(false)` 清理子树交互状态。
  - `repaint` 改为按有效可见性过滤；validation phase 改为按有效可见性与有效启用状态过滤。
  - 新增 M2 定向测试覆盖父链传播、隐藏祖先 repaint、隐藏/禁用祖先 validation queue drain。
- Files: `novadraw-scene/src/graph/mod.rs`, `novadraw-scene/src/graph/update_integration_test.rs`, `agent/backlog/index.yaml`, `agent/backlog/recent.yaml`, `agent/backlog/archive/2026-06.yaml`, `agent/inner-loop-checkpoint.md`, `agent/inner-loop-worklog.md`
- Delta Verification: cargo fmt --check ✅, cargo test -p novadraw-scene 154/154 + 3 doctests ✅, cargo clippy -p novadraw-scene -- -D warnings ✅, ruby agent/workflow-doctor.rb ✅, bash agent/workflow-verify.sh ✅
- Decision: AD-027 状态 `verified`；M2 保持 `in_progress`。
- Split Decision: 本轮不处理 z-order API、不做 remove dispose 语义、不推进 M3 绘制裁剪闭环。
- Post-Execution Reflection: M2 的树状态不能只作为单节点字段存在；图级入口必须统一使用父链有效状态，才能让 Figure 树真正成为更新、命中与交互的运行时骨架。
- Next Step: M2 child order and z-order contract audit。

## 2026-06-11 / Context anchor cleanup

- Goal: 修正当前工作流热路径，避免已剥离历史主题在 M2-M7 核心主线中被反复作为阻塞项、残余风险或下一步带出。
- Root Cause: `project_memory.md`、`goal-roadmap.md`、`inner-loop-checkpoint.md`、产品清单和 demo matrix 中都记录了具体历史暂停项；Agent 恢复工作流时会把这些热路径信息误读为当前每轮都需要提醒的阻塞。
- Decision: 热路径不保留已剥离主题的具体名称、恢复条件或 保留或延期 标记；当前核心工作流只维护通用 Figure 树与父链坐标协议。
- Minimal Fix: 更新项目记忆、milestone 描述、goal roadmap、产品清单、demo matrix 和 checkpoint context boundary，改为无锚点规则表达。
- Verification: ruby agent/workflow-doctor.rb 待跑；本轮只改治理文档，不改业务代码。
- Next Step: 回到 M2 effective visible/enabled propagation。

## 2026-06-11 / AD-026

- Goal: 启动 M2 Figure 树与盒模型，先收口 FigureGraph 公开 add 入口的拓扑不变量。
- Root Cause: M2 要求 Figure tree 是 ownership 与 topology 的运行时骨架；延迟 add/reparent 路径已有 invalid-parent 防御，但公开直接 add 入口仍通过 `self.blocks[parent_id]` 访问 parent，parent 无效时会 panic，且与 pending mutation 路径的无副作用契约不一致。
- Minimal Fix:
  - `add_child_to` 改为复用 `try_add_child_to`，无效 parent 时返回 `BlockId::null()`，不分配 block、不写 uuid_map、不修改 validation path。
  - 新增显式可失败入口 `try_add_child_to(parent, figure) -> Option<BlockId>`。
  - `add_child_with_bounds` 与交互式 `add_child` 同步接入无效 parent 防御。
  - 交互式 `add_child` 无效 parent 时不触发 layout/repaint queue。
  - 新增 invalid-parent 定向测试，覆盖 pending add、直接 add、try add 与 update-manager add。
- Files: `novadraw-scene/src/graph/mod.rs`, `novadraw-scene/src/graph/update_integration_test.rs`, `agent/draw2d-core-milestones.yaml`, `agent/goal-roadmap.md`, `agent/backlog/index.yaml`, `agent/backlog/recent.yaml`, `agent/backlog/archive/2026-06.yaml`, `agent/inner-loop-checkpoint.md`, `agent/inner-loop-worklog.md`
- Delta Verification: cargo fmt ✅, cargo test -p novadraw-scene invalid_parent 5/5 ✅, cargo test -p novadraw-scene 149/149 + 3 doctests ✅, cargo clippy -p novadraw-scene -- -D warnings ✅, bash agent/workflow-verify.sh ✅
- Decision: AD-026 状态 `verified`；M2 状态从 `not_started` 推进为 `in_progress`。
- Split Decision: 本轮不处理 remove dispose 语义、不做 z-order API、不做 visible/enabled effective propagation、不接入 border insets；这些留作后续 M2 delta。
- Post-Execution Reflection: M2 第一轮应先保证树拓扑入口不产生部分写入或 panic，后续才能安全推进 effective state、z-order 与盒模型闭环。
- Next Step: M2 effective visible/enabled propagation。

## 2026-06-11 / BASELINE-002 cleanup

- Goal: 清理已登记的 `BASELINE-002`，让完整 `workflow-verify.sh` 恢复全绿。
- Root Cause: `workflow-verify.sh` 在 `cargo clippy -- -D warnings` 阶段失败；直接登记点是 `apps/vello-app/src/main.rs` 两处 needless borrow，修复后继续暴露 scene 与 demo apps 的旧 clippy style debt。
- Minimal Fix:
  - `apps/vello-app/src/main.rs` 移除 `scene.fill()` 颜色参数的不必要引用。
  - `novadraw-scene` 清理 Copy 类型 clone、derive Default、collapsible-if、needless-borrow 与 Copy event clone。
  - demo apps 清理冗余零参闭包、复杂 scene entry 类型别名、无效 cast、无效 return、range loop；示例函数参数过多处保留局部 allow。
- Files: `apps/vello-app/src/main.rs`, `novadraw-scene/src/figure/triangle.rs`, `novadraw-scene/src/figure/border/mod.rs`, `novadraw-scene/src/graph/render_iterative.rs`, `novadraw-scene/src/layout/border_layout.rs`, `novadraw-scene/src/layout/flow_layout.rs`, `novadraw-scene/src/runtime/update/deferred.rs`, `novadraw-apps/src/app.rs`, `apps/editor/src/scene_manager/interactive_figure.rs`, `apps/*-app/src/main.rs`, `agent/backlog/baseline-debts.yaml`, `agent/inner-loop-checkpoint.md`, `agent/inner-loop-worklog.md`
- Delta Verification: cargo fmt ✅, cargo fmt --check ✅, cargo clippy -p vello-app -- -D warnings ✅, cargo test -p novadraw-scene 146/146 + 3 doctests ✅, cargo clippy -- -D warnings ✅, bash agent/workflow-verify.sh ✅
- Decision: `BASELINE-002` 状态从 `open` 改为 `resolved`；完整仓库门禁恢复全绿。
- Split Decision: 本轮不做业务语义重构，不改变 M1/M2 状态，不处理 已剥离主题 视觉验证。
- Post-Execution Reflection: 基线债务不能只修登记点；`cargo clippy -- -D warnings` 会逐层暴露后续 crate 的旧风格债务，必须以完整 `workflow-verify.sh` 作为最终判断。
- Next Step: 启动 M2 Figure 树与盒模型。

## 2026-06-11 / AD-025

- Goal: 补齐 M1 产品层存在性检查，判断 M1 是否可以从 `contract_aligned` 推进到 `behavior_verified`。
- Root Cause: M1 已通过 YAML probes 汇总，但 `doc/06-roadmap/product-deliverables.md` 的 M1 几何类型清单与 Graphics API 清单缺少独立自动化检查，不能只凭契约层证据推进状态。
- Minimal Fix:
  - `novadraw-geometry` 新增 `PrecisionPoint`、`PrecisionRectangle`、`PrecisionDimension`、`Vector`、`AffineTransform` 兼容别名。
  - 新增 `novadraw-geometry/tests/m1_product_existence.rs`，覆盖 M1 几何产品层类型导入、PointList、Insets 与 ApproxEq 兼容语义。
  - `NdCanvas` 新增 draw2d 风格 snake_case 产品入口：rectangle、oval、polygon、text/string、clip、alpha 与 style setters。
  - 新增 `LineStyle`，并将其记录进 stroke command snapshot，保持命令流可回放。
  - 新增 `novadraw-render/tests/m1_product_existence.rs`，覆盖 shape/style、text/image/clip/alpha 命令输出。
  - 新增 `agent/m1-product-existence-checks.md`，将 M1 推进到 `behavior_verified`。
- Files: `novadraw-geometry/src/lib.rs`, `novadraw-geometry/tests/m1_product_existence.rs`, `novadraw-render/src/context.rs`, `novadraw-render/src/command.rs`, `novadraw-render/src/lib.rs`, `novadraw-render/src/backend/vello/mod.rs`, `novadraw-render/tests/m1_product_existence.rs`, `agent/m1-product-existence-checks.md`, `agent/draw2d-core-milestones.yaml`, `agent/goal-roadmap.md`, `agent/backlog/index.yaml`, `agent/backlog/recent.yaml`, `agent/backlog/archive/2026-06.yaml`, `agent/inner-loop-checkpoint.md`, `agent/inner-loop-worklog.md`
- Delta Verification: cargo fmt --check ✅, cargo test -p novadraw-geometry 44/44 ✅, cargo test -p novadraw-render 9/9 ✅, cargo check --workspace ✅, cargo clippy -p novadraw-geometry -p novadraw-render -- -D warnings ✅, cargo check -p novadraw-render --features vello ✅, ruby agent/workflow-doctor.rb ✅, git diff --check ✅
- Baseline Verification: `bash agent/workflow-verify.sh` 仍失败于已登记的 `BASELINE-002`（`apps/vello-app/src/main.rs` 两处 needless borrow），不混入 AD-025 修复。
- Decision: AD-025 状态 `verified`；M1 状态从 `contract_aligned` 推进为 `behavior_verified`，但不推进到 `complete`。
- Split Decision: 本轮不实现真实 text shaping/image resource manager，不修复 已剥离主题 visual clipping，不启动 M2；只关闭 M1 产品层存在性门禁。
- Post-Execution Reflection: 本轮补上了 contract probes 与产品清单之间的缺口，M1 现在具备可重复的契约层和产品层证据，可作为 M2 的稳定基础。
- Next Step: 启动 M2 Figure 树与盒模型，或先独立处理 `BASELINE-002` 使完整 `workflow-verify.sh` 恢复全绿。

## 2026-06-11 / WF-003

- Goal: 收敛 `active.yaml` 生命周期，避免 backlog 拆分后热路径文件继续因终态条目增长而污染 Agent 上下文。
- Root Cause: WF-002 只完成物理拆分，未定义 active 的生命周期边界；`verified` 终态 delta 仍滞留在 `agent/backlog/active.yaml`，导致恢复工作时默认读取冷历史。
- Minimal Fix:
  - 将 `active.yaml` 压缩为只保存非终态工作，当前 `items: []`。
  - 将原 active 终态条目迁入 `agent/backlog/archive/2026-06.yaml`。
  - 新增 `agent/backlog/recent.yaml`，保留最近 5 个终态 delta 摘要。
  - 更新 `workflow-doctor.rb`，禁止终态条目滞留 active，限制 active 最大 10 项，并校验 recent 只包含终态摘要。
  - 更新 workflow 文档与启动脚本，使默认热路径读取 `index.yaml`、`active.yaml`、`recent.yaml`，archive 仅用于审计追溯。
- Files: `agent/outer-loop-delta-backlog.yaml`, `agent/backlog/index.yaml`, `agent/backlog/active.yaml`, `agent/backlog/recent.yaml`, `agent/backlog/archive/2026-06.yaml`, `agent/workflow-doctor.rb`, `agent/workflow-continuous.md`, `agent/README.md`, `agent/workflow-run-continuous.sh`, `agent/workflow-run-once.sh`, `agent/inner-loop-checkpoint.md`, `agent/inner-loop-worklog.md`
- Delta Verification: ruby -c agent/workflow-doctor.rb ✅, ruby agent/workflow-doctor.rb ✅, backlog YAML parse ✅, git diff --check ✅
- Decision: WF-003 状态 `verified`；active backlog 的语义从“当前和近期”收窄为“仅非终态工作”，recent 是短摘要，不是完整历史 SSOT。
- Split Decision: 本轮不重写 archive 中的历史条目结构，不按月重分旧 archive；只修正热路径生命周期和机器门禁。
- Post-Execution Reflection: 本轮从根因上解决 active 文件继续膨胀的问题，后续 Agent 恢复只需要读取热路径和 recent 摘要，冷历史不会默认进入上下文。
- Next Step: 继续 M1 product-layer existence checks 以准备 `behavior_verified`，或在依赖允许的前提下启动 M2。

## 2026-06-11 / AD-024

- Goal: 生成 M1 contract probes summary，判断 M1 是否可以从 `in_progress` 推进到 `contract_aligned`。
- Root Cause: M1 已完成 AD-020/021/022/023 多个实现 delta，但 milestone 状态不能按主观进度推进，必须按 `agent/draw2d-core-milestones.yaml` 的 probes 逐项映射自动化证据。
- Minimal Fix:
  - 新增 `agent/m1-contract-probes-summary.md`。
  - 汇总 M1 scope、contracts、probes、delta evidence、verification 与 residual risks。
  - 将 geometry operation、Graphics state stack nesting、clip/transform command snapshots、text/image/alpha command snapshots 映射到具体测试。
  - 将 M1 从 `in_progress` 推进到 `contract_aligned`。
- Tests: `cargo test -p novadraw-geometry` 42/42 ✅；`cargo test -p novadraw-render` 7/7 ✅。
- Files: `agent/m1-contract-probes-summary.md`, `agent/draw2d-core-milestones.yaml`, `agent/goal-roadmap.md`, `agent/backlog/active.yaml`, `agent/backlog/index.yaml`, `agent/inner-loop-checkpoint.md`, `agent/inner-loop-worklog.md`
- Decision: M1 可以推进到 `contract_aligned`；不能推进到 `behavior_verified` 或 `complete`，因为产品层检查尚未建立，demo matrix 也只允许 M1 无独立 demo。
- Verification: workflow doctor、backlog YAML parse、git diff --check 待本轮最终校验。
- Next Step: 选择 `M1 product-layer existence checks`，或在依赖允许的前提下启动 M2。

## 2026-06-11 / AD-023

- Goal: 继续 M1，补齐 Graphics text / image / alpha command support。
- Root Cause: M1 scope 要求 Graphics draw/fill/text/image/clip/transform/state stack 与 alpha 状态；`NdCanvas` 已有 `font`、`fill_text`、`stroke_text`、`draw_image`、`global_alpha` API，但这些入口仍是 no-op 或不进入命令流，导致录制回放与后端替换无法验证 text/image/alpha 语义。
- Minimal Fix:
  - `GraphicsState` 新增 `font`、`font_size`、`global_alpha`，随 push/restore/pop 一起作用域化。
  - `RenderCommandKind` 新增 `SetGlobalAlpha`；`FillText` / `StrokeText` 携带 font、font_size、color；`Image` 携带 alpha。
  - `NdCanvas::fill_text`、`stroke_text`、`draw_image`、`draw_image_with_size`、`global_alpha` 生成可回放命令。
  - 形状、路径、文字命令在录制时应用当前 global alpha。
- Tests: 新增 alpha 作用域、text snapshot、image destination/alpha snapshot、measure_text 字体大小语义测试。
- Files: `novadraw-render/src/context.rs`, `novadraw-render/src/command.rs`, `agent/backlog/active.yaml`, `agent/backlog/index.yaml`, `agent/goal-roadmap.md`, `agent/inner-loop-checkpoint.md`, `agent/inner-loop-worklog.md`
- Delta Verification: cargo fmt --check ✅, cargo test -p novadraw-render 7/7 ✅, cargo check --workspace ✅, cargo check -p novadraw-render --features vello ✅
- Decision: AD-023 挂 M1，状态 `verified`；M1 仍保持 `in_progress`，下一步先做 M1 probes 汇总判断是否可推进到 `contract_aligned`。
- Split Decision: 本轮不实现真实 Vello text/image rasterization，不引入 font/image resource manager；只收口 command snapshot 与 Graphics state 语义。
- Post-Execution Reflection: M1 的 geometry、state stack、clip/transform、text/image/alpha 命令层 probes 已基本具备，可进入 contract probes summary。
- Next Step: 执行 `M1 contract probes summary`，检查 YAML M1 probes 是否都已有对应测试与证据，再决定是否把 M1 推进到 `contract_aligned`。

## 2026-06-11 / WF-002

- Goal: 拆分过长的 `agent/outer-loop-delta-backlog.yaml`，降低 Agent 每轮读取 backlog 时的上下文污染。
- Root Cause: 原 backlog 文件约 1000 行，混合规则、candidate、baseline debt、当前 delta 和历史 archive；Agent 默认全量读取会把冷历史、旧路径证据和已完成决策带入热路径上下文。
- Minimal Fix:
  - 将 `outer-loop-delta-backlog.yaml` 降级为 manifest。
  - 新增 `agent/backlog/schema.yaml`、`index.yaml`、`active.yaml`、`candidates.yaml`、`baseline-debts.yaml`、`archive/2026-06.yaml`。
  - 更新 `workflow-doctor.rb`，通过 manifest 聚合 active/candidates/archive/debts 并校验 ID、状态、milestone_id、checkpoint current delta。
  - 更新 workflow 启动脚本和说明文档，让 Agent 默认读取 index/active，不默认读取 archive。
- Files: `agent/outer-loop-delta-backlog.yaml`, `agent/backlog/**`, `agent/workflow-doctor.rb`, `agent/workflow-continuous.md`, `agent/workflow-run-continuous.sh`, `agent/workflow-run-once.sh`, `agent/README.md`, `agent/workflow-map.md`, `agent/architecture-review-agent.md`, `agent/inner-loop-checkpoint.md`, `agent/inner-loop-worklog.md`
- Delta Verification: ruby -c agent/workflow-doctor.rb ✅, ruby agent/workflow-doctor.rb ✅, backlog YAML parse ✅, git diff --check ✅
- Decision: WF-002 状态 `verified`；`outer-loop-delta-backlog.yaml` 只保留 manifest，历史条目进入 archive。
- Split Decision: 本轮不重写全部历史 worklog 中的旧路径引用；这些属于历史记录，不在热路径读取。
- Post-Execution Reflection: Agent 热路径上下文从 992 行 backlog 降到 manifest 13 行 + active 162 行；doctor 仍可机器全量校验 archive。
- Next Step: 回到 M1，继续 `Graphics text/image/alpha command support`，或先做 M1 probes 汇总判断是否可推进到 `contract_aligned`。

## 2026-06-11 / AD-022

- Goal: 推进 M1 geometry missing types，补齐 `Dimension`、`PointList` 和 precision geometry 的具体实现与测试。
- Root Cause: M1 SSOT 要求 `Point, Dimension, Rectangle, Insets, PointList, precision geometry, Transform`；当前 geometry 已有 `Point/Rectangle/Insets/Transform` 和兼容命名 `Size`，但缺少正式 `Dimension` 命名、点序列封装和统一浮点精度比较 primitive。
- Minimal Fix:
  - 将正式尺寸类型提升为 `Dimension`，保留 `Size` 作为兼容别名。
  - 新增 `PointList`，支持 `bounds()`、`transformed()`、`Translatable`、迭代和 serde。
  - 新增 `Precision`、`DEFAULT_EPSILON`、`ApproxEq`，覆盖 `f64`、`Point`、`Rectangle`、`Transform`。
- Tests: 新增 `PointList` bounds / empty / translate-scale / transform 测试，新增 Precision eq / snap_zero / ApproxEq 测试。
- Files: `novadraw-geometry/src/rect.rs`, `novadraw-geometry/src/point_list.rs`, `novadraw-geometry/src/precision.rs`, `novadraw-geometry/src/lib.rs`, `agent/goal-roadmap.md`, `agent/outer-loop-delta-backlog.yaml`, `agent/inner-loop-checkpoint.md`, `agent/inner-loop-worklog.md`
- Delta Verification: cargo fmt --check ✅, cargo test -p novadraw-geometry 42/42 ✅, cargo check --workspace ✅
- Decision: AD-022 挂 M1，状态 `verified`；M1 仍保持 `in_progress`，不升级为 `contract_aligned`。
- Split Decision: 本轮不处理 Graphics text/image/alpha，也不重命名全仓 `Size` 调用点，避免无价值 churn。
- Post-Execution Reflection: M1 geometry substrate 更接近 draw2d 命名与基础能力，后续 M2/M3 可以依赖稳定的 Dimension/PointList/Precision 语义。
- Next Step: 继续 M1，优先补 Graphics text/image/alpha command support，或做 M1 contract probe 汇总后判断是否可推进到 `contract_aligned`。

## 2026-06-11 / AD-021

- Goal: 收窄 `NdCanvas::commands_mut()` 访问权限，防止外部直接改写命令流破坏 AD-020 引入的 GraphicsState 一致性。
- Root Cause: AD-020 后 `NdCanvas` 开始维护内部 Graphics 状态；公开 `commands_mut()` 会允许外部直接修改 `Vec<RenderCommand>`，却不同步 state/clip_depth/transform，破坏后端替换和录制回放所依赖的命令流确定性。
- Audit: `commands_mut()` 只有定义自身，没有任何跨 crate 或 crate 内调用。
- Minimal Fix: 先收窄为 `pub(crate)`，验证出现 dead_code warning；最终直接移除该接口，只保留 `commands()` 只读快照和 `to_submission()`。
- Files: `novadraw-render/src/context.rs`, `agent/outer-loop-delta-backlog.yaml`, `agent/inner-loop-checkpoint.md`, `agent/inner-loop-worklog.md`
- Delta Verification: cargo fmt --check ✅, cargo test -p novadraw-render ✅, cargo check --workspace ✅
- Decision: AD-021 作为 AD-020 的边界补强，挂 M1，状态 `verified`。
- Split Decision: 本轮不改命令存储结构、不新增 interpreter conformance tests、不处理 ResetClip 深层语义；只关闭外部可变命令 Vec 入口。
- Post-Execution Reflection: `NdCanvas` 继续作为 Graphics facade / command recorder，外部无法绕过 Graphics API 篡改命令流，更符合未来后端替换与录制回放设计。
- Next Step: 继续 M1，优先选择 geometry missing types 或 Graphics text/image/alpha command support 中一个最小 delta。

## 2026-06-11 / AD-020

- Goal: 启动 M1，补齐 Graphics state stack / clip-transform command snapshot 的最小契约实现与测试。
- Root Cause: `NdCanvas` 只生成 `PushState/RestoreState/PopState` 命令但不维护 Graphics 状态，`clip_depth()` 恒为 0，`set_transform/reset_transform` 被编码为 concat/no-op；因此 M1 要求的 Graphics state stack nesting 与 clip/transform command snapshot 无法在命令层稳定验证。
- Minimal Fix: 在 `NdCanvas` 中新增 `GraphicsState` 与 `state_stack`，维护 fill/stroke/line/transform/clip_depth；新增 `SetTransform`、`ResetTransform`、`ResetClip` 命令；Vello backend 改为按 clip depth 恢复 scene layer，PushState 不再重复推 clip layer。
- Tests: 新增 3 个 `novadraw-render` 单元测试，覆盖嵌套状态恢复、`set_transform/reset_transform` 快照命令、clip reset/restore 命令序列。
- Files: `novadraw-render/src/context.rs`, `novadraw-render/src/command.rs`, `novadraw-render/src/backend/vello/mod.rs`, `agent/draw2d-core-milestones.yaml`, `agent/goal-roadmap.md`, `agent/outer-loop-delta-backlog.yaml`, `agent/inner-loop-checkpoint.md`, `agent/inner-loop-worklog.md`
- Delta Verification: cargo fmt --check ✅, cargo test -p novadraw-render ✅, cargo check --workspace ✅
- Decision: M1 从 `not_started` 推进为 `in_progress`；本轮只收口 Graphics 状态栈和 clip/transform 快照，不把 M1 标记为 `contract_aligned`。
- Split Decision: 不处理 Dimension/PointList/precision geometry、文本/图像/alpha，也不修复 已剥离主题 visual clipping 或 M3 render traversal probes。
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
- Split Decision: 本轮只做 workflow doctor 与状态模型补齐，不执行 M1 Graphics、M3 裁剪、M6 输入状态机或 M8 已剥离主题 修复，避免把 workflow gate 与业务能力混在一个 delta。
- Post-Execution Reflection: 本轮更接近最新工作流目标，因为状态文件不再只靠人工自律，baseline verification 会先检查 milestone/backlog/checkpoint 的机器可检出漂移，为后续 M1-M10 推进提供控制器基础。
- New Candidate Deltas: 建议新增 M1 Graphics state stack and clip/transform snapshot parity delta（下一轮 REVIEW 后决定正式编号）。
- Next Step: 回到 REVIEW，选择首个挂 M1-M10 的最小 delta；建议从 M1 Graphics 状态栈与 clip/transform 命令快照开始。

## 2026-06-10 / AD-019B

- Goal: 按用户要求一次性完成当前可安全调整的 `novadraw-scene` 目录边界，并判断是否创建新 crate。
- Root Cause: `novadraw-scene/src` 根层仍平铺 `scene/update/context/event/mutation/system/redacted_topic/border` 等不同职责域，物理目录没有完整表达 `figure / graph / runtime / host / container` 子域边界。
- Minimal Fix: 完成 `scene -> graph`、`context/event/mutation/system/update -> runtime`、`redacted_topic.rs -> container/redacted_topic.rs`、`border -> figure/border`；保留 root facade alias 兼容旧外部入口。
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
- Split Decision: 不迁移 runtime，不执行 `scene -> graph` 大迁移，不移动暂停中的 `redacted_topic.rs`，不修改 `render_recursive.rs` / `render_iterative.rs` 主循环。
- Post-Execution Reflection: 本轮更接近理想架构，因为 `SceneHost` 的物理目录位置与“极薄平台宿主”职责一致，后续 `WinitSceneHost / WebSceneHost / HeadlessSceneHost` 有自然归属。
- New Candidate Deltas: AD-019B runtime boundary directory split；AD-019C graph boundary directory split；AD-019D container boundary directory split（待 已剥离主题 恢复后）。
- Next Step: 如继续模块拆分，优先评估 AD-019B runtime 边界；保持 facade crate `novadraw` 不变。

## 2026-06-10 / 已剥离主题 Visual Follow-up Paused

- Goal: 冻结 已剥离主题 可视化验证现场，按用户要求暂停 已剥离主题 后续开发。
- What Was Finished: 已新增 `apps/redacted_topic-app`，包含 `redacted_visual_case`、`origin_scroll`、`zoomed_content`、`nested_redacted_topics` 四个可视化场景；已新增 `--screenshot-clip` 自动截图入口。
- Visual Verification Result: `cargo run -p redacted_topic-app -- --screenshot-clip` 成功生成 `apps/redacted_topic-app/screenshot/redacted_topic-app_redacted_visual_case_1781071623.png`，但截图未通过，黄色原点块和蓝/绿 content 网格绘制到黑色 已剥离主题 边框外。
- Current Hypothesis: 问题不应先归因到 `render_recursive.rs` / `render_iterative.rs` 主循环；恢复时优先审查 `NdCanvas::clip_rect` 到 `VelloRenderer::push_clip_layer()` 的 clip layer / transform 映射，以及 `perform_update -> repair -> render_to_iterative` 截图路径。
- Guardrail: `render_recursive.rs` 与 `render_iterative.rs` 主循环是保护区，除非已对标 draw2d 证明主流程不符，否则不要改主循环逻辑。
- Exact Restart Point: 重新运行 `cargo run -p redacted_topic-app -- --screenshot-clip`，打开最新截图，对照黑色 已剥离主题 边框检查 content 裁剪；然后从 Vello clip 映射和 UpdateManager repair 调用路径开始排查。
- Decision: 用户要求 已剥离主题 后续开发暂时搁置；当前不继续修复 `redacted_visual_case`，不推进 已剥离主题容器 / mouse wheel / auto-expose。
- Next Step: 若继续架构主线，回到 discovery/review，选择非 已剥离主题 的最小 delta。

## 2026-06-10 / 历史剥离条目

- Goal: 按 draw2d 对标，把 已剥离主题 从 standalone math helper 推进为 Figure 树中的坐标根和裁剪容器。
- Root Cause: g2 中 `已剥离主题` / `已剥离主题容器` 位于 `org.eclipse.draw2d`，GEF 只提供 viewer/helper/policy 集成；Novadraw 当前 `已剥离主题` 尚未作为 Figure 节点参与 render / hit-test / damage repair，若在 apps/editor 或 SceneHost 保存滚动状态，会重新引入 Figure 树外全局坐标特判。
- Minimal Fix: 新增 `ChildTransform` 作为 Figure 子树坐标协议；新增 `已剥离主题Figure`，用 `origin` / `zoom` 计算 content -> parent 变换，并用 content-domain `client_area()` 驱动裁剪；render recursive / iterative、hit-test、damage repair 均消费同一协议。
- Files: `novadraw-scene/src/figure/mod.rs`, `novadraw-scene/src/redacted_topic.rs`, `novadraw-scene/src/scene/mod.rs`, `novadraw-scene/src/scene/render_recursive.rs`, `novadraw-scene/src/scene/render_iterative.rs`, `novadraw-scene/src/update/repair.rs`, `novadraw-scene/src/scene/bounds_test.rs`, `novadraw-scene/src/scene/update_integration_test.rs`, `novadraw-scene/src/lib.rs`, `novadraw/src/lib.rs`, `agent/outer-loop-delta-backlog.yaml`, `agent/governance-contract-coverage.md`, `agent/inner-loop-checkpoint.md`, `agent/inner-loop-worklog.md`
- Delta Verification: cargo fmt ✅, cargo check ✅, cargo test -p novadraw-scene 146/146 + 3 doctests ✅, backlog YAML parse ✅, git diff --check ✅
- Baseline Verification: `cargo clippy -- -D warnings` 发现非本轮 clippy debt（apps/vello-app needless borrow、旧 novadraw-scene derivable impl / clone-on-copy 等），未混入 历史剥离条目 修复。
- Decision: C-03 / C-09 保持 aligned；已剥离主题 核心语义归属 `novadraw-scene`，apps/editor 和 SceneHost 不拥有滚动/视口图语义状态。
- Split Decision: 不实现 已剥离主题控件 UI、mouse wheel、auto-expose、selection feedback 或完整 已剥离主题容器 layout；这些属于后续独立 delta。
- Post-Execution Reflection: 本轮更接近理想架构，因为 已剥离主题 现在沿 Figure 树父链坐标协议闭合，render / hit-test / damage repair 不需要各自维护特殊分支，也没有把 draw2d Figure 级语义上移到 GEF-like/editor 层。
- New Candidate Deltas: 已剥离主题容器 layout and range model integration；editor mouse-wheel redacted_topic policy；已剥离主题 auto-expose helper。
- Next Step: 提交 历史剥离条目；如继续迭代，回到 discovery/review 选择 已剥离主题容器 或 editor policy 中的一个最小项。

## 2026-06-10 / AD-017

- Goal: 收敛 PendingMutation reparent apply 阶段的树不变量，防止 Figure 回调申请形成 FigureGraph 环。
- Root Cause: `apply_reparent_mutation()` 已校验 child、old_parent、new_parent 存在，但没有拒绝 `child == new_parent` 或 `new_parent` 位于 child 子树；一旦 detach/attach 执行，会破坏 render/event/layout/update 遍历依赖的树结构前提。
- Minimal Fix: 在 `FigureGraph` 内新增迭代式祖先链校验，并在 detach/attach 前拒绝 self reparent 与 descendant reparent；使用 blocks 长度作为遍历上界，避免既有损坏图导致无限循环。
- Files: `novadraw-scene/src/scene/mod.rs`, `novadraw-scene/src/scene/update_integration_test.rs`, `agent/outer-loop-delta-backlog.yaml`, `agent/governance-contract-coverage.md`, `agent/inner-loop-checkpoint.md`, `agent/inner-loop-worklog.md`
- Delta Verification: cargo fmt --check ✅, cargo check ✅, cargo test -p novadraw-scene 143/143 + 3 doctests ✅
- Decision: C-08 恢复为 aligned；PendingMutation apply 阶段现在在产生结构副作用前维护 FigureGraph 树不变量。
- Split Decision: 不处理 `EditorInteractionCore::scene_manager_mut()`、`FigureGraph::get_block()` 或 已剥离主题/已剥离主题容器 集成；这些是相邻候选，不属于本轮 reparent 防环根因。
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
- Split Decision: 不处理 已剥离主题/已剥离主题容器 真实 Figure-tree 集成，不重新打开 AD-014/AD-015。
- Post-Execution Reflection: 本轮更接近理想架构，因为结构性变更现在由 `NovadrawContext`/`SceneDispatchContext` 产生，`FigureGraph` 只在稳定事务边界消费不可外部伪造的 batch，类型系统更直接地表达了 PendingMutation 契约。
- New Candidate Deltas: 无。
- Next Step: 进入 completion baseline verification；若 `cargo test` 全量通过，可将当前架构循环标记为 complete-ready。

## 2026-06-09 / AD-015

- Goal: 清理理想架构文档中的组合根旧表述，使文档与 AD-010 / AD-014 后的公开接口边界一致。
- Root Cause: `doc/理想架构设计.md` 仍把 `NovadrawSystem (trait)` 描述为持有 `scene/update_manager/dispatcher/scene_host`，并保留 `NovadrawSystem.update_manager` / `NovadrawSystem.dispatcher` 与 `WinitEventDispatcher` 旧平台入口表述；这会把已移除的公开逃生口重新写成理想架构。
- Minimal Fix: 将相关段落统一改为“NovadrawSystem 平台实现内部装配 FigureGraph / UpdateManager / EventDispatcher / SceneHost；公开 trait 只暴露 render / redacted_topic_size / request_update”；将组合根/事件流中的旧 `WinitEventDispatcher` 入口改为 `app_window` 平台输入适配 + `BasicEventDispatcher` 引擎无状态分发。
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
- Minimal Fix: 从 `NovadrawSystem` trait 和 `WinitNovadrawSystem` impl 中移除三个公开逃生口，只保留 `render()` / `redacted_topic_size()` / `request_update()`；`NovadrawContext` 的 `set_selected()` / `add_child_later()` / `remove_child_later()` / `reparent_later()` 改为必须显式实现的方法
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
- Split Decision: 不拆分；本轮只清理 event / hit-test 热路径日志，不处理 focus state machine、render strategy wiring 或 已剥离主题/已剥离主题容器 集成
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


## 2026-06-12 / AD-034

- Goal: M3 border insets client-area clipping
- Root Cause: `RectangleFigure::with_border(...)` 已支持 Border 装饰器，但 `Bounded::insets()` 未从 border 读取 `get_insets()`，导致产品图元的 border insets 不会收窄 clientArea，children 可进入应由 border/insets 隔离的绘制区域
- Files: `novadraw-scene/src/figure/rectangle.rs`, `novadraw-scene/src/graph/mod.rs`, `agent/backlog/index.yaml`, `agent/backlog/recent.yaml`, `agent/backlog/archive/2026-06.yaml`, `agent/goal-roadmap.md`, `agent/inner-loop-checkpoint.md`, `agent/inner-loop-worklog.md`
- Delta Verification: cargo fmt --check ✅, cargo test -p novadraw-scene ✅, cargo clippy -p novadraw-scene -- -D warnings ✅
- Baseline Verification: ruby agent/workflow-doctor.rb ✅, bash agent/workflow-verify.sh --fast ✅, git diff --check ✅
- Decision: `RectangleFigure` 的 `Bounded::insets()` 以 Border insets 为 SSOT；渲染链路继续通过 `Bounded::client_area()` 统一使用 inset-adjusted child painting area
- Split Decision: 不推进 M3 到 `contract_aligned`；下一轮单独闭合 paint versus hit-test consistency
- Post-Execution Reflection: 本轮是代码-bearing delta，符合 WF-004 execution momentum gate；递归/迭代等价检查继续以状态机行为签名为准
- New Candidate Deltas: 无
- Next Step: M3 paint versus hit-test consistency tests

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


## 2026-05-25 / AD-007 已剥离主题 coordinate-domain audit

- Root Cause: `已剥离主题` 仍使用 `screen_to_world` / `world_to_screen` 命名，虽然当前只是数学 helper，但 API 会暗示存在 Figure 树外的全局 world 坐标；同时 transform 组合只在 origin=0 用例下被覆盖，非零 origin 时公式风险未被测试锁住
- Files: `novadraw-scene/src/redacted_topic.rs`, `apps/transform-app/src/main.rs`, `doc/04-coordinates/coordinates.md`, `doc/理想架构设计.md`, `agent/outer-loop-delta-backlog.yaml`, `agent/inner-loop-checkpoint.md`, `agent/inner-loop-worklog.md`
- Delta Verification: cargo fmt ✅, cargo test -p novadraw-scene 138/138 + 3 doctests ✅
- Baseline Verification: 未运行全仓 `workflow-verify.sh`；本轮只执行 delta scope 验证
- Decision: `已剥离主题` 统一使用 redacted_topic/content 坐标域；`origin` 表示 redacted_topic 左上角对应的 content 坐标；转换公式为 `content = redacted_topic_point / zoom + origin` 与 `redacted_topic_point = (content - origin) * zoom`
- Split Decision: 不将真实 已剥离主题/已剥离主题容器 Figure-tree 集成混入本轮；当前只收口 standalone helper 的语义、公式与文档
- Post-Execution Reflection: 已剥离主题 应作为父链坐标协议的未来扩展点，而不是事件或渲染入口的特殊全局通道；`translate_to_parent` / `translate_from_parent` 已作为协议方向锚点保留
- New Candidate Deltas: 已剥离主题/已剥离主题容器 Figure-tree integration（通过 Figure 节点、client area 裁剪、hit-test、damage repair 接入父链坐标协议）
- Next Step: AD-007 主干与 已剥离主题 audit 均已 verified，建议切回 AD-001C scheduling boundary audit


## 2026-05-20 / AD-007

- Root Cause: `Bounded::client_area()` 对 `use_local_coordinates=true` 仍返回 `bounds.x/y + insets`，没有像 draw2d `getClientArea()` 一样把坐标根 client area 原点重置到 `(0,0)`；同时 recursive / iterative 渲染的非坐标根分支直接 clip 完整 bounds，忽略 insets，导致 children 可绘制进 border 区域
- Files: `novadraw-scene/src/figure/mod.rs`, `novadraw-scene/src/scene/mod.rs`, `novadraw-scene/src/scene/render_recursive.rs`, `novadraw-scene/src/scene/render_iterative.rs`, `novadraw-scene/src/scene/bounds_test.rs`, `agent/outer-loop-delta-backlog.yaml`, `agent/inner-loop-checkpoint.md`, `agent/inner-loop-worklog.md`
- Delta Verification: cargo fmt ✅, cargo test -p novadraw-scene 136/136 + 3 doctests ✅
- Baseline Verification: 未运行全仓 `workflow-verify.sh`；本轮只执行 delta scope 验证
- Decision: `Bounded::client_area()` 成为 client area SSOT；坐标根返回 `(0,0,width,height)`，非坐标根返回 `bounds + insets`；布局和渲染统一复用该语义
- Split Decision: 不拆分；本轮只处理 render/client-area 闭环，已剥离主题/scroll/scale 留作下一轮独立 delta
- Post-Execution Reflection: 早期 AD-007 backlog 中“全绝对坐标”契约已经过期，当前正确契约是 `bounds` 表示相对最近坐标根的绝对值；render 修复后主干链路已基本闭合
- New Candidate Deltas: 已剥离主题 coordinate-domain audit（确认 screen/world API 是否为独立视口抽象，以及如何接入 draw2d translateToParent/fromParent 协议）
- Next Step: 审计 `novadraw-scene/src/redacted_topic.rs` 与 redacted_topic 文档，避免 redacted_topic 概念重新引入全局/世界坐标假设



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
- Post-Execution Reflection: 通知体系比坐标模型更上层；若 bounds / dirty / transform 语义继续混用，后续 repair、hit-test、redacted_topic、通知都会持续漂移
- New Candidate Deltas: 无
- Next Step: 以 `AD-007` 为主线，先统一 bounds 正式定义，再逐步收口 translate API 与 repair/damage 主链

## 2026-04-28 / Backlog Review

- Goal: 评估 `AD-001` 是否应继续作为单一主线，以及是否需要先切换到通知体系基础设施
- Root Cause: `AD-001` 已同时承载 validation、repair、scheduling 三类边界问题，继续在同一 delta 下推进会失焦；而通知体系又是 repair / redacted_topic / scroll 等后续机制的公共底座
- Files: `agent/outer-loop-delta-backlog.yaml`, `agent/inner-loop-checkpoint.md`, `agent/inner-loop-worklog.md`, `novadraw-scene/src/update/listener.rs`, `novadraw-scene/src/scene/mod.rs`
- Verification: backlog review 完成；checkpoint schema 仍满足 v1
- Decision: 将 `AD-001` 正式拆分为 `AD-001A validation boundary`、`AD-001B repair boundary`、`AD-001C scheduling boundary`；新增 `AD-006 notification foundation audit` 作为新的主线候选
- Split Decision: `AD-001` 转为 `split`，其中 validation 子项标记为 `verified`，repair / scheduling 保持 `pending`
- Post-Execution Reflection: 通知体系不应继续作为 `CAD-001` 这种窄候选保留，而应提升为正式架构条目；否则后续 redacted_topic / scroll / router 仍缺少稳定基础设施
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
