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

## 2026-04-23 / AD-001

- Goal: 收口 UpdateManager 在 validation phase 的职责边界，避免图级语义继续留在更新服务里
- Root Cause: `SceneUpdateManager::perform_validation()` 直接读取 `FigureGraph.blocks` 并决定可见性/启用语义，导致 UpdateManager 从“两阶段编排器”回流为“图语义执行者”
- Files: `novadraw-scene/src/update/mod.rs`, `novadraw-scene/src/update/deferred.rs`, `novadraw-scene/src/scene/mod.rs`, `novadraw-scene/src/scene/update_integration_test.rs`, `agent/outer-loop-delta-backlog.yaml`, `agent/inner-loop-checkpoint.md`
- Verification: `cargo test -p novadraw-scene` 通过；`cargo check` 通过；`./agent/workflow-verify.sh` 在 `cargo fmt --check` 阶段因仓库既有格式化漂移失败
- Decision: 本轮只把 validation 图语义回收到 `FigureGraph`，不同时触碰 repair phase 和 system 调度，保持单轮单 delta 的最小切口
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
