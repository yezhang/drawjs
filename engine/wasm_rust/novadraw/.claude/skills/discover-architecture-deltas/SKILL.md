---
name: discover-architecture-deltas
description: 从理想架构与当前实现偏差中发现候选问题。适合开始新阶段、backlog 失真或执行后需要继续发现问题时调用。
---

# Discover Architecture Deltas

从理想架构文档与当前实现的偏差中识别候选问题，并决定哪些问题值得进入正式 backlog。

## When To Use

- 新一轮架构改进开始前
- 当前 backlog 价值下降或过期时
- 执行完一个 delta 后，需要判断是否还存在残余问题时
- 发现测试失败、代码异味、职责回流，但还没被结构化时

## Required Inputs

- `CLAUDE.md`
- `AGENTS.md`
- `doc/理想架构设计.md`
- `agent/architecture_contracts.md`
- `agent/delta_backlog.yaml`
- 与候选问题相关的代码文件

## Procedure

1. 总结当前理想架构的关键契约
2. 对照当前实现，识别契约偏差
3. 将偏差写为候选 delta
4. 判断候选项是进入 backlog、拒绝还是拆分
5. 为建议进入 backlog 的项补齐优先级、证据和完成标准

## Output Format

- Candidate Deltas
- Root Cause Summary
- Promote Or Reject Decision
- Suggested Priority
- Suggested Done When
- Recommended Next Delta

## Guardrails

- 不要把模糊观察直接写成正式 backlog
- 不要一次生成过多难以执行的大问题
- 如果问题只是现象而非职责边界偏差，继续下钻根因
- 如果与已有 delta 重复，优先合并或标记重复
