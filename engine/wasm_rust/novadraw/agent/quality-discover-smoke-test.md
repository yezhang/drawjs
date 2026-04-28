# Workflow Smoke Test

本文件用于验证工作流本身，尤其是 `discover-architecture-deltas` 的发现能力是否有效。

目标不是验证业务功能，而是验证：

- 从零开始运行 discover，是否能稳定发现已知架构偏差
- 不同 Agent 是否能基于同一审计清单得到大致一致的候选问题
- 当 discover 输出 `0 个 candidate` 时，是否给出了足够的审计覆盖说明

## 使用时机

- 新增或重写 discover skill 之后
- 明显感觉外循环过于乐观时
- 工作流升级后，想验证“它真的会发现问题”时

## 执行方式

让 Agent 读取：

- `agent/governance-architecture-contracts.md`
- `agent/governance-contract-coverage.md`
- `agent/outer-loop-delta-backlog.yaml`
- 本文件
- `doc/理想架构设计.md`

然后执行：

```text
请执行 discover-architecture-deltas 的 smoke test。不要直接沿用 backlog 结论，而是按审计清单重新检查代码，并告诉我这次 discover 是否能重新发现下面这些已知偏差中的至少一部分。
```

## 已知问题样本

### Sample A / UpdateManager boundary

- 目标契约：
  - `C-03 FigureGraph 持有图级信息`
  - `C-04 UpdateManager 只负责两阶段更新编排`
- 期望 discover 至少能发现：
  - UpdateManager 可能越界持有图级语义
  - 或者指出当前只局部收敛，repair / scheduling 仍待继续审计
- 相关入口：
  - `novadraw-scene/src/update/mod.rs`
  - `novadraw-scene/src/update/deferred.rs`
  - `apps/editor/src/system.rs`

### Sample B / SceneHost thin boundary

- 目标契约：
  - `C-06 SceneHost 是极薄平台调度层`
- 期望 discover 至少能发现：
  - SceneHost 边界尚未被证明足够薄
  - 或现有代码尚未充分审计，应保留 candidate / pending
- 相关入口：
  - `apps/editor/src/scene_manager/scene_host.rs`
  - `apps/editor/src/system.rs`

### Sample C / PendingMutation timing

- 目标契约：
  - `C-08 结构性变更必须通过 PendingMutation 延迟应用`
- 期望 discover 至少能发现：
  - 该契约当前仍处于 `drifting` 或 `unassessed`
  - 或指出 mutation timing 仍需审计
- 相关入口：
  - `novadraw-scene/src/mutation/mod.rs`
  - `apps/editor/src/system.rs`

## 通过标准

一次 smoke test 通过，至少满足：

- 明确列出本轮审计了哪些契约
- 明确列出本轮检查了哪些代码入口
- 对以上 3 个样本，至少能重新发现 2 个问题或明确指出为何暂不能判断
- 如果输出 `0 个 candidate`，必须同时给出：
  - 已覆盖契约范围
  - 未覆盖契约范围
  - 为什么当前没有足够证据形成 candidate

## 失败信号

出现以下任一情况，说明 discover 能力仍然不足：

- 只重复 backlog 现有结论，没有独立审计痕迹
- 没列出审计了哪些代码入口
- 样本问题几乎一个都没提到
- 直接得出“当前高度对齐”但没有覆盖说明

## 记录建议

每次 smoke test 后，建议把结果记入 `agent/inner-loop-worklog.md` 或单独追加到本文件，至少包含：

- Date
- Agent
- Audited Contracts
- Checked Entrypoints
- Rediscovered Samples
- Missed Samples
- Follow-up Fixes
