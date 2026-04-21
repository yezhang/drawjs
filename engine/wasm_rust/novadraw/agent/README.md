---
title: Solo Coder Architecture Workflow
---

# Solo Coder Architecture Workflow

这套工作流用于让 solo coder 在多轮会话、中断恢复、突发任务插入的情况下，仍然能够持续按照 `doc/理想架构设计.md` 渐进式改善当前代码。

## 目标

- 让 Agent 每轮只推进一个最小的架构差距
- 让长期任务具备可中断、可恢复、可审计能力
- 降低每次重新开始时重新梳理上下文的成本

## 核心原则

- 理想架构是目标，当前代码只是输入，不是约束
- 一次只处理一个 `Architecture Delta`
- 先解释根因，再做最小改动
- 每轮结束必须写回状态文件
- 没有验证结果，不能标记完成

## 文件职责

- `agent/architecture_contracts.md`: 机器可执行的架构硬约束
- `agent/delta_backlog.yaml`: 架构差距队列
- `agent/session_checkpoint.md`: 当前进度、下一步、阻塞点
- `agent/inbox.md`: 突发任务收纳箱
- `agent/worklog.md`: 每轮工作的结构化记录
- `agent/workflow_evolution.md`: 工作流为何这样设计，以及后续如何迭代
- `agent/verify.sh`: 固定验证脚本

## 三个 Skill

- `resume-architecture-work`: 恢复当前主线，决定下一步
- `execute-architecture-delta`: 执行一个最小架构差距
- `capture-interruption`: 处理中断，冻结现场并登记突发任务

## 推荐循环

1. 恢复现场
2. 选择一个待处理 delta
3. 解释根因与最小方案
4. 修改代码
5. 运行验证
6. 更新 backlog、checkpoint、worklog
7. 决定下一轮是否继续

## 推荐状态机

- `pending`: 尚未处理
- `proposed`: 已完成根因分析，待实施
- `in_progress`: 正在修改代码
- `blocked`: 发现依赖或文档缺口，暂停
- `verified`: 已通过验证
- `done`: 已完成并回写状态

## 每日使用方式

### 开始工作

让 Agent 先执行 `resume-architecture-work`，读取：

- `CLAUDE.md`
- `AGENTS.md`
- `doc/理想架构设计.md`
- `agent/architecture_contracts.md`
- `agent/delta_backlog.yaml`
- `agent/session_checkpoint.md`

然后输出：

- 当前主线 delta
- 当前阶段
- 已完成内容
- 阻塞点
- 推荐下一步

### 推进一轮

让 Agent 执行 `execute-architecture-delta`，要求：

- 只处理一个 delta
- 先说明根因
- 给最小修改方案
- 修改代码并验证
- 更新 `delta_backlog.yaml`
- 更新 `session_checkpoint.md`
- 追加 `worklog.md`

### 被打断时

让 Agent 执行 `capture-interruption`，要求：

- 把突发任务写入 `agent/inbox.md`
- 把当前主线状态写入 `agent/session_checkpoint.md`
- 明确下一步最小动作

## 推荐提问模板

### 恢复

```text
请执行 resume-architecture-work，告诉我当前主线、最近停在哪、推荐下一步。
```

### 执行一轮

```text
请执行 execute-architecture-delta，本轮只处理一个 delta，不要跨层级大改。
```

### 中断

```text
请执行 capture-interruption，把当前工作冻结，并记录这个突发任务。
```

## Delta 设计规则

- 每个 delta 只描述一个职责边界问题或一个状态归属问题
- 一个 delta 的修改范围应尽量限制在一个接口簇或一个调用链
- 如果改动超过 50 行，优先拆分为多个 delta

## 完成标准

一个 delta 只有同时满足以下条件才能进入 `done`：

- 根因已解释清楚
- 代码已修改
- 固定验证已通过
- backlog 已更新
- checkpoint 已更新

## 注意事项

- 如果发现理想架构文档本身不完整，优先补契约，不直接写代码
- 如果当前 delta 需要跨多个子系统，先拆解再执行
- 如果突发任务很多，优先保证 `session_checkpoint.md` 始终可信
