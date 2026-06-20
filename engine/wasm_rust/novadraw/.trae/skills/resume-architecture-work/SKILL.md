---
name: "resume-architecture-work"
description: "Restores the current architecture-improvement context from workflow files. Invoke when resuming work after interruption or when the next step is unclear."
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
- `agent/governance-architecture-contracts.md`
- `agent/outer-loop-delta-backlog.yaml`
- `agent/quality-checkpoint-schema.md`
- `agent/inner-loop-checkpoint.md`
- `agent/inner-loop-worklog.md`
- `agent/backlog/index.yaml`
- `agent/backlog/active.yaml`
- `agent/backlog/recent.yaml`
- `agent/workflow-verify.sh`

## Procedure

1. 总结当前理想架构的关键约束
2. 先根据 `agent/quality-checkpoint-schema.md` 校验 `agent/inner-loop-checkpoint.md` 是否结构完整
3. 读取 checkpoint，确认当前主线 delta
4. 检查 backlog 中该 delta 的状态、门禁和完成标准
5. 检查 checkpoint 的 `Interruptions` 小节是否存在阻塞主线的突发任务
6. 需要稳定性判断时运行 `./agent/workflow-verify.sh --gate=ready`
7. 输出当前阶段、已完成内容、当前假设、最小下一步

## Output Format

- Current Delta
- Schema Health
- Current Status
- What Is Already Done
- Open Questions
- Recommended Next Small Step
- Whether To Continue Or Replan

## Guardrails

- 不要直接开始大改代码
- 如果 checkpoint 与 backlog 冲突，先指出冲突
- 如果理想架构文档与当前契约不一致，先要求澄清或补契约
- 如果 checkpoint 缺少关键 section，不要静默猜测，先输出 schema repair 建议
- 如果当前 delta 命中强制拆分或强制回外循环门禁，优先建议 `review-delta-backlog`
