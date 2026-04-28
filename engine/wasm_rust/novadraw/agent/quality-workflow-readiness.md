# Workflow Stabilization

本文件用于回答两个问题：

- 这套工作流现在是否足够稳定，可以承载真实架构改进工作？
- 当前还存在哪些必须先解决的流程风险？

## Readiness Levels

### Level 0 / Draft

- 工作流能表达目标，但缺少闭环或状态文件

### Level 1 / Runnable

- 可以跑 discover / review / resume / execute
- 但缺少稳定门禁或自测机制

### Level 2 / Stable Enough

- discover 有契约级审计清单
- backlog review 能识别重复、失焦和拆分信号
- checkpoint 有 schema，resume 不会静默误读
- 有 smoke test，可验证 discover 不是空转
- 基线债务与当前 delta 失败能分层记录

### Level 3 / Trusted

- 不同模型多次运行 smoke test 结果基本稳定
- 状态迁移和 checkpoint 合并策略经真实使用验证
- workflow 变更不再频繁，进入常态使用期

## Current Assessment

- Current Level: `2 / Stable Enough`

### Why

- 已有 discover / review / resume / execute / interruption 闭环
- 已有强制拆分、强制回外循环、双层验证与基线债务
- 已有契约覆盖视图和 smoke test
- 已有 checkpoint schema

### Remaining Risks

- discover 对某些“需要更广上下文才看得出的薄边界问题”仍然偏弱
- smoke test 目前仍是文档驱动，尚未形成脚本化回归
- checkpoint schema 仅完成 v1，没有自动校验工具
- 状态迁移规则虽写入 README，但还未经过大量真实轮次验证

## Go / No-Go Checklist

在“正式依赖工作流推进架构改进”之前，建议至少满足：

- [ ] 最近一次 smoke test 通过
- [ ] 当前 checkpoint 满足 `quality-checkpoint-schema.md`
- [ ] 当前 backlog 没有明显失焦的大 delta
- [ ] 当前 baseline debt 已登记，不会误伤本轮判断
- [ ] README 中的常见场景 prompt 能直接使用，无需大量临时解释

## Recommended Next Validation

1. 用当前模型再跑一次 smoke test，确认 discover 稳定性
2. 用另一个模型跑同一份 smoke test，对比结果
3. 对 `AD-001` 真正执行一次 `review -> split decision -> resume -> execute`，验证门禁是否顺手

## When To Move To Trusted

满足以下条件后，可将当前工作流视为 `Trusted`：

- 至少两种模型在 smoke test 中都能稳定重新发现 2 个以上样本
- 至少完成 3 轮真实 delta 推进，且没有因为 checkpoint / 状态迁移出错而返工
- `workflow-history.md` 在一段时间内没有再发生结构级大改
