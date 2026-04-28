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

## 2026-04-29 / Coordinate Delta Switch

- Goal: 决定是否把坐标模型统一提升为当前主线，并暂停通知体系基础设施
- Root Cause: 代码、文档、测试同时存在“局部坐标”与“相对最近坐标根的绝对坐标”两套 bounds 语义，继续推进 `AD-006` 会让通知体系建立在不稳定坐标契约上
- Files: `agent/outer-loop-delta-backlog.yaml`, `agent/inner-loop-checkpoint.md`, `novadraw-scene/src/scene/mod.rs`, `novadraw-scene/src/update/repair.rs`, `doc/02-figure/figure_bounds.md`, `doc/04-coordinates/coordinates.md`
- Verification: g2/draw2d 坐标源码与仓库文档再次核对完成；当前代码验证进行中
- Decision: 新增 `AD-007 Coordinate model alignment with g2` 作为新的当前主线；`AD-006` 保留为后续 pending 条目
- Split Decision: 无新增拆分；属于主线切换而非继续拆分
- Post-Execution Reflection: 通知体系比坐标模型更上层；若 bounds / dirty / transform 语义继续混用，后续 repair、hit-test、viewport、通知都会持续漂移
- New Candidate Deltas: 无
- Next Step: 以 `AD-007` 为主线，先统一 bounds 正式定义，再逐步收口 translate API 与 repair/damage 主链

## 2026-04-28 / Backlog Review

- Goal: 评估 `AD-001` 是否应继续作为单一主线，以及是否需要先切换到通知体系基础设施
- Root Cause: `AD-001` 已同时承载 validation、repair、scheduling 三类边界问题，继续在同一 delta 下推进会失焦；而通知体系又是 repair / viewport / scroll 等后续机制的公共底座
- Files: `agent/outer-loop-delta-backlog.yaml`, `agent/inner-loop-checkpoint.md`, `agent/inner-loop-worklog.md`, `novadraw-scene/src/update/listener.rs`, `novadraw-scene/src/scene/mod.rs`
- Verification: backlog review 完成；checkpoint schema 仍满足 v1
- Decision: 将 `AD-001` 正式拆分为 `AD-001A validation boundary`、`AD-001B repair boundary`、`AD-001C scheduling boundary`；新增 `AD-006 notification foundation audit` 作为新的主线候选
- Split Decision: `AD-001` 转为 `split`，其中 validation 子项标记为 `verified`，repair / scheduling 保持 `pending`
- Post-Execution Reflection: 通知体系不应继续作为 `CAD-001` 这种窄候选保留，而应提升为正式架构条目；否则后续 viewport / scroll / router 仍缺少稳定基础设施
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
