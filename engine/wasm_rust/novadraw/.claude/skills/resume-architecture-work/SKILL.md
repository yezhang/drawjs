---
name: resume-architecture-work
description: 恢复当前架构改进上下文并给出下一步。适合在中断恢复、重新开工或下一步不明确时调用。
---

# Resume Architecture Work

恢复当前架构改进主线，减少每次重新开始时的上下文重建成本。

## When To Use

- 开始新一轮工作前
- 上次工作被中断后
- backlog 很长，当前优先级不清楚时
- 刚处理完突发任务，需要回到主线时

## Required Inputs

优先读取以下文件：

- `CLAUDE.md`
- `AGENTS.md`
- `doc/理想架构设计.md`
- `agent/architecture_contracts.md`
- `agent/delta_backlog.yaml`
- `agent/session_checkpoint.md`
- `agent/inbox.md`
- `agent/worklog.md`

## Procedure

1. 总结当前理想架构的关键约束
2. 读取 checkpoint，确认当前主线 delta
3. 检查 backlog 中该 delta 的状态和完成标准
4. 检查 inbox 中是否存在阻塞主线的突发任务
5. 输出当前阶段、已完成内容、当前假设、最小下一步

## Output Format

- Current Delta
- Current Status
- What Is Already Done
- Open Questions
- Recommended Next Small Step
- Whether To Continue Or Replan

## Guardrails

- 不要直接开始大改代码
- 如果 checkpoint 与 backlog 冲突，先指出冲突
- 如果理想架构文档与当前契约不一致，先要求澄清或补契约
