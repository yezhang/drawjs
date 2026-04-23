---
name: review-delta-backlog
description: 审查并整理架构 backlog，执行去重、拆分和重排优先级。适合 backlog 变旧、变乱或一轮执行完成后调用。
---

# Review Delta Backlog

审查 `delta_backlog.yaml` 的质量，确保 backlog 始终反映“当前最值得解决的问题”。

## When To Use

- backlog 累积较多条目时
- 做完 1 到 3 个 delta 后
- 发现多个 delta 重复或边界模糊时
- backlog 很久未整理、优先级明显失真时

## Required Inputs

- `agent/architecture_contracts.md`
- `agent/delta_backlog.yaml`
- `agent/session_checkpoint.md`
- `agent/worklog.md`
- 与可疑条目相关的代码文件

## Procedure

1. 检查 backlog 中是否存在重复项、过大项、过时项
2. 检查 candidate 是否应提升、拒绝或继续保留
3. 按“职责边界优先于实现细节”的原则重排优先级
4. 对过大问题给出拆分建议
5. 明确当前最值得执行的一个 delta

## Output Format

- Duplicate Items
- Overgrown Items
- Candidate Promote Or Reject Decisions
- Priority Reorder Suggestions
- Recommended Current Delta
- Suggested Backlog Edits

## Guardrails

- 不要为了整理 backlog 而直接改代码
- 不要保留语义重复的 delta
- 不要把候选项直接提升为正式 backlog，除非证据和完成标准都充分
- 如果发现工作流本身需要调整，写入 `agent/workflow_evolution.md`
