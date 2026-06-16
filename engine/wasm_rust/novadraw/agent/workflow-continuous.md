# Continuous Architecture Workflow

本文件定义基于现有 `agent/README.md` 的持续运行控制层。它不替代单轮工作流，而是在单轮 `discover -> review -> resume -> execute -> verify -> record` 之上增加循环调度、终局判定和防失控门禁。

## Goal

持续推进 Novadraw，直到项目达到 `doc/理想架构设计.md` 与 `agent/governance-architecture-contracts.md` 定义的理想目标。

持续运行的目标不是“不断改代码”，而是让每一轮都可证明地减少理想架构与当前实现之间的差距。

## Ideal Completion Definition

项目进入理想目标完成态必须同时满足：

1. `agent/governance-contract-coverage.md` 中所有契约状态均为 `aligned`。
2. `agent/backlog/active.yaml` 中不存在 `architecture` 或 `parity` 类型的 `pending`、`proposed`、`in_progress`、`split`、`blocked` 条目。
3. 最近一次 `discover-architecture-deltas` 对 `unassessed`、`drifting`、`partially_aligned` 契约重新审计后，没有发现新的可执行 candidate。
4. `./agent/workflow-verify.sh --gate=ready` 通过。
5. 当前基线债务已关闭，或已被显式接受且不影响理想架构语义。
6. `cargo check`、相关 crate 测试与当前定义的 baseline verification 通过，或失败项均有基线债务记录。
7. `doc/理想架构设计.md`、`agent/governance-architecture-contracts.md`、`agent/governance-contract-coverage.md` 与代码现状互相一致。
8. 最近一次代码类 delta 已通过 `agent/architecture-review-agent.md` 定义的架构 Review，且无未处理 Blocker / High finding。

## Controller States

持续运行控制器只允许处于以下状态之一：

| State | Meaning | Next |
|---|---|---|
| `BOOTSTRAP` | 读取规则、理想架构、契约、checkpoint 和 backlog | `ASSESS` |
| `ASSESS` | 判断 backlog、checkpoint、coverage 是否可信 | `DISCOVER` 或 `REVIEW` 或 `RESUME` |
| `DISCOVER` | 从理想架构与现状偏差发现 candidate | `REVIEW` |
| `REVIEW` | 去重、拆分、重排 backlog，选出一个最小 delta | `RESUME` |
| `RESUME` | 恢复当前主线与下一步最小动作 | `EXECUTE` 或 `REVIEW` |
| `EXECUTE` | 只执行一个最小 delta | `VERIFY` |
| `VERIFY` | 运行 delta verification 和必要 baseline verification | `RECORD` 或 `REPAIR_CURRENT_DELTA` |
| `RECORD` | 更新 backlog、checkpoint、worklog、coverage、必要记忆 | `REFLECT` |
| `REFLECT` | 判断是否生成新 candidate、是否回外循环、是否完成 | `ASSESS` 或 `COMPLETE` |
| `INTERRUPTED` | 捕获突发任务并冻结现场 | `BOOTSTRAP` |
| `COMPLETE` | 达到理想完成态 | stop |

## Continuous Loop

每轮必须按以下顺序执行：

1. `BOOTSTRAP`
   - 读取 `AGENTS.md`、`CLAUDE.md`、`doc/理想架构设计.md`。
   - 读取 `agent/README.md`、`agent/workflow-continuous.md`、`agent/governance-architecture-contracts.md`。
   - 读取 `agent/outer-loop-delta-backlog.yaml` manifest、`agent/backlog/index.yaml`、`agent/backlog/active.yaml`、`agent/backlog/recent.yaml`、checkpoint、worklog、coverage；仅在审计或冲突排查时读取 `agent/backlog/archive/*.yaml`。
   - 读取 `agent/draw2d-core-milestones.yaml`（milestone 编号 SSOT）和 `agent/goal-roadmap.md`（当前进度快照）。
   - 运行 `ruby agent/workflow-doctor.rb`，先确认 milestone / roadmap / backlog / checkpoint 状态没有机器可检出的漂移。

2. `ASSESS`
   - 校验 checkpoint 是否满足 `quality-checkpoint-schema.md`。
   - 检查 backlog 是否存在失焦、重复、过大或状态冲突。
   - 检查 coverage 中是否还有 `unassessed`、`drifting`、`partially_aligned`。
   - 判断当前是否应该先 `discover`、先 `review`，还是可以直接 `resume/execute`。

3. `DISCOVER`（按需）
   - 优先审计 coverage 中未对齐契约。
   - 每轮至少检查一个具体代码入口。
   - 新问题先进入 candidate，不直接插队执行。

4. `REVIEW`（按需或每 1 到 3 个 delta 后强制执行）
   - 将 candidate 提升、拒绝或拆分。
   - 选择一个最值得执行的 `architecture` 或 `parity` delta。
   - 若条目属于 `new_feature`，默认不进入持续主线。

5. `RESUME`
   - 确认当前主线 delta、当前状态、阻塞点和下一步最小动作。
   - 如果 checkpoint 与 backlog 冲突，停止执行，先修状态文件。

6. `EXECUTE`
   - 一次只处理一个 delta。
   - 先解释根因，再给最小方案，再改代码。
   - 不允许用临时方案制造视觉或测试效果。
   - 重大架构改动必须先评审方案。

7. `VERIFY`
   - 运行 delta verification。
   - 对代码类 delta，按 `agent/architecture-review-agent.md` 直接比对理想架构、契约和代码实现；该 Review 不审查 checkpoint/backlog/worklog 状态。
   - 需要时运行 baseline verification。
   - 失败必须分类为本轮回归或既有基线债务。

8. `RECORD`
   - 更新 `agent/backlog/active.yaml` / `agent/backlog/recent.yaml` / `agent/backlog/candidates.yaml` / `agent/backlog/baseline-debts.yaml`，必要时更新 manifest 或 archive。
   - 已进入 `verified` / `done` / `rejected` / `promoted` 的终态 delta 必须立即从 `active.yaml` 迁入 `archive/YYYY-MM.yaml`，并刷新 `recent.yaml` 的最近 5 个终态摘要。
   - 更新 `inner-loop-checkpoint.md`。
   - 追加 `inner-loop-worklog.md`。
   - 更新 `governance-contract-coverage.md`。
   - 若结论会影响跨会话判断，更新项目记忆。

9. `REFLECT`
   - 判断本轮是否真正减少架构差距。
   - 如果暴露新问题，生成 candidate 并回到 `ASSESS`。
   - 如果满足理想完成态，进入 `COMPLETE`。

## Budget And Stop Rules

为了防止持续运行退化为无限循环，每次启动持续模式必须显式给出预算。

推荐默认预算：

- `max_cycles`: 3
- `max_delta_per_run`: 1
- `max_files_changed_per_delta`: 6
- `max_consecutive_execute_without_review`: 2
- `max_failed_verification_retry`: 1

达到任一条件必须停止并输出状态：

- 已达到 `max_cycles`。
- 当前 delta 连续两轮未明显收敛。
- 验证失败且无法快速归因为本轮改动。
- 需要跨两个以上不共享同一根因的子系统。
- 发现理想架构文档或契约缺口。
- backlog 与 checkpoint 冲突。
- 用户任务插入，需要 `capture-interruption`。
- 达到 `COMPLETE`。

## Anti-Drift Rules

持续模式必须遵守以下防漂移规则：

- 不以“让 demo 看起来对”为目标，必须修复职责边界或语义根因。
- 不允许把 app/editor 临时逻辑提升为引擎语义。
- 不允许绕过 `FigureGraph`、`UpdateManager`、`EventDispatcher`、`SceneHost` 的既定边界。
- 不允许为了推进一个 delta 同时修改多个独立根因。
- 不允许把 `new_feature` 混入默认主线。
- 不允许用未记录的口头判断替代 backlog、checkpoint、worklog 和 coverage 更新。

## Mode Selection

持续运行每轮启动时按以下规则选择模式：

| Condition | Mode |
|---|---|
| coverage 存在 `unassessed` 或 backlog 明显过时 | `discover` |
| candidate 很多或当前 delta 分叉 | `review` |
| 当前 delta 明确且未触发门禁 | `execute` |
| 验证策略不清楚 | `review` |
| 突发任务插入 | `capture-interruption` |
| 所有终局条件满足 | `complete` |

## Completion Audit

进入 `COMPLETE` 前必须执行一次完成审计：

1. 重新读取 `doc/理想架构设计.md`。
2. 逐条检查 `governance-architecture-contracts.md`。
3. 对 coverage 中每个 `aligned` 契约至少抽查一个代码入口。
4. 运行 discover smoke test。
5. 运行 baseline verification。
6. 确认 backlog 中没有未处理的 `architecture` / `parity` 主线项。
7. 生成最终完成报告，写入 `inner-loop-worklog.md`。

## Operator Prompt

持续运行时使用以下提示词：

```text
请按 agent/workflow-continuous.md 运行持续架构闭环。

预算：
- max_cycles: 3
- max_delta_per_run: 1
- max_consecutive_execute_without_review: 2

要求：
1. 先执行 BOOTSTRAP 和 ASSESS，不要直接改代码。
2. 根据 gate 自动选择 discover / review / resume / execute。
3. 每轮只执行一个最小 delta。
4. 每轮结束必须更新 backlog、checkpoint、worklog、contract coverage。
5. 如果达到停止条件，立即停止并输出 Current State、Stop Reason、Next Restart Prompt。
6. 如果满足 Ideal Completion Definition，执行 Completion Audit 并输出最终完成报告。
```

## Relationship To Existing Files

- `agent/README.md`: 单轮工作流与日常操作 SSOT。
- `agent/workflow-continuous.md`: 持续运行控制层 SSOT。
- `agent/workflow-run-continuous.sh`: 持续运行 prompt 启动器。
