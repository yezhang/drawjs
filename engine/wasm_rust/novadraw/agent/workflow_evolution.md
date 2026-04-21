# Workflow Evolution

本文件记录 solo coder 架构改进工作流的设计背景、关键决策、已知边界和后续迭代方向。

## 2026-04-10 / v0

### 背景

- 长期架构改进任务容易被中断
- 每次恢复时都需要重新检查进度、重新梳理主线
- 突发任务会打断原有节奏，导致上下文丢失
- 单靠会话记忆不足以支撑多轮、跨天的架构收敛工作

### 核心问题

- 缺少外化的项目级状态
- 缺少“当前做到哪、下一步做什么”的统一记录点
- 缺少专门处理中断和恢复的流程
- 缺少把理想架构文档转化为可执行约束的中间层

### 本轮决策

- 将当前生效工作流保存在 `agent/README.md`
- 将工作流迭代历史保存在 `agent/workflow_evolution.md`
- 将流程状态文件保存在 `agent/`，而不是 `.trae/`
- 将技能入口保存在 `.trae/skills/`
- 采用“单轮可恢复循环”，而不是无限自动 ReAct 循环

### 当前工作流形态

- `agent/README.md`: 当前正式流程定义
- `agent/architecture_contracts.md`: 理想架构硬约束
- `agent/delta_backlog.yaml`: 架构差距列表
- `agent/session_checkpoint.md`: 当前恢复点
- `agent/inbox.md`: 突发任务箱
- `agent/worklog.md`: 每轮工作记录
- `agent/verify.sh`: 固定验证脚本
- `.trae/skills/*`: 恢复、执行、中断三个 Skill

### 为什么状态文件放在 `agent/`

- 这些文件是项目级工作流资产，不是 Trae 专属元数据
- 未来可以被 Trae、Claude Code、Cursor、手工脚本等多种工具复用
- 使用语义化目录名，便于长期维护和迁移

### 为什么暂时不拆子目录

- 当前文件数量少，平铺结构更容易浏览和恢复
- 过早拆目录会增加层级，但不会明显降低恢复成本
- 等文件数量增长后，再按职责自然拆分为 `contracts/`、`state/`、`logs/`、`scripts/`

### 当前边界

- 这是半自动工作流，不是全自动自治 Agent
- 目前依赖人工触发 Skill
- 目前还没有 `run_once.sh` 之类的执行器脚本
- 当前 backlog 还是初始化版本，需要在实际使用中继续打磨粒度

### 后续迭代方向

1. 增加 `agent/run_once.sh`
2. 增加 `weekly-architecture-review` Skill
3. 在 `delta_backlog.yaml` 中补充更细的 delta 拆分规则
4. 将 `worklog.md` 演进为按日期归档
5. 视文件数量决定是否拆分 `agent/` 子目录

### 迭代规则

- 先在本文件记录“为什么要改工作流”
- 再修改 `agent/README.md` 中的当前生效流程
- 如果只是一次性实验，不要直接写入正式流程
- 只有在连续多轮证明有效后，才将新机制提升为默认流程
