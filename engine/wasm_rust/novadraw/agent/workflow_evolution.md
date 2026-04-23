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

## 2026-04-22 / v1

### 触发原因

- v0 主要解决了“中断恢复”和“已知问题执行”的问题
- 但它默认 backlog 已经存在，无法覆盖“从识别问题到解决问题”的完整闭环
- 实际使用中会遇到两类缺口：
  - 不知道当前最值得解决的问题是什么
  - 执行完一个 delta 后，不知道是否还存在残余问题

### 本轮升级目标

- 将工作流从“执行流”升级为“发现 + 执行”的双循环闭环
- 让 Agent 能从理想架构与当前实现偏差中持续生成候选问题
- 让执行完成后能够回流生成新的候选 delta，而不是在已有问题执行完后停止

### 本轮决策

- 在 `agent/README.md` 中引入双循环模型：
  - 外循环：发现与整理
  - 内循环：执行与收敛
- 在 `delta_backlog.yaml` 中显式支持 `candidate` 和 `rejected` 状态
- 新增 `discover-architecture-deltas` Skill
- 将“执行后反思”提升为闭环必须步骤

### 对 v0 的修正

- v0 不是错误设计，而是第一阶段版本
- v0 优先解决执行连续性
- v1 补齐问题发现、候选建模、backlog 整理和执行后回流

### 当前 v1 的能力边界

- 已具备完整闭环设计
- 仍然是半自动工作流，不是无人监督的自动自治系统
- 仍然需要人工判断哪些候选项值得进入正式 backlog
- backlog 的粒度仍需通过真实使用继续打磨

### 下一步演进方向

1. 在 `worklog.md` 中增加固定的“Post-Execution Reflection”模板
2. 增加 `review-delta-backlog` 或 `weekly-architecture-review` Skill
3. 增加 `agent/run_once.sh`，把发现或执行的单轮流程脚本化
4. 视真实使用情况决定是否引入 `candidate_deltas` 单独文件

## 2026-04-22 / v1.1

### 触发原因

- v1 已经形成完整闭环，但日常使用时仍然有两个摩擦点：
  - backlog 需要一个专门的整理入口
  - 各种场景下该用什么提示词，仍然需要人工记忆

### 本轮升级目标

- 增加专门的 backlog review 能力
- 增加脚本化入口，降低每轮启动成本
- 把常见场景下的提示词沉淀到 `agent/README.md`

### 本轮决策

- 新增 `review-delta-backlog` Skill
- 新增 `agent/run_once.sh`
- 将“各种情况使用什么 prompt”完整写入 `agent/README.md`

### 当前收益

- backlog 不再只能依赖发现型 Skill 间接整理
- 日常使用不必记忆 prompt，可直接查 README 或运行脚本
- 工作流从“设计完备”进一步升级为“更易实际使用”
