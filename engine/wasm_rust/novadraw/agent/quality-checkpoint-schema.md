# Checkpoint Schema

本文件定义 `agent/inner-loop-checkpoint.md` 的稳定结构，降低合并冲突、字段漂移和恢复静默失败的风险。

## Schema Version

- Current Version: `1`

## Required Sections

以下一级标题必须存在，且顺序保持稳定：

1. `# Session Checkpoint`
2. `## Metadata`
3. `## Current Delta`
4. `## Current Status`
5. `## What Was Done`
6. `## Current Hypothesis`
7. `## Next Small Step`
8. `## Blockers`
9. `## Verification State`
10. `## Resume Prompt`

## Metadata Fields

`## Metadata` 下至少包含：

- `schema_version`
- `updated_at`
- `checkpoint_kind`

推荐格式：

```text
## Metadata

- schema_version: 1
- updated_at: 2026-04-23
- checkpoint_kind: architecture-loop
```

## Section Semantics

### Current Delta

- 当前主线 delta
- 只能有一个主线项

### Current Status

- 当前主线 delta 的状态
- 应与 `outer-loop-delta-backlog.yaml` 中的状态一致或可解释

### What Was Done

- 只记录已经完成的动作
- 不要混入计划或猜测

### Current Hypothesis

- 解释当前对问题状态的判断
- 如假设已改变，必须显式说明

### Next Small Step

- 必须是可立即执行的最小动作
- 如果存在分叉，应说明下一步需要先 `review` 还是先 `execute`

### Blockers

- 记录真正阻止推进的因素
- 包括基线债务、门禁触发、依赖缺失

### Verification State

- 区分 `delta_verification` 与 `baseline_verification`
- 如果失败，必须说明是否属于本轮引入

### Resume Prompt

- 提供下次恢复时可直接复制的提示词
- 不能只写“继续工作”，必须点明当前门禁和下一步

## Compatibility Rules

- 若旧 checkpoint 缺少 `Metadata`，resume 时应视为 legacy 格式，不直接失败
- 若缺少必需 section，resume 时必须输出 `Schema Health: invalid`
- 若字段语义不明确，resume 时应优先提示用户修复 checkpoint，而不是静默猜测

## Repair Guidance

当 checkpoint 不满足 schema 时：

1. 保留已有内容，不做覆盖式重写
2. 先补缺失 section
3. 再补 metadata
4. 最后更新 resume prompt
