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
- 将工作流迭代历史保存在 `agent/workflow-history.md`
- 将流程状态文件保存在 `agent/`，而不是 `.trae/`
- 将技能入口保存在 `.trae/skills/`
- 采用“单轮可恢复循环”，而不是无限自动 ReAct 循环

### 当前工作流形态

- `agent/README.md`: 当前正式流程定义
- `agent/governance-architecture-contracts.md`: 理想架构硬约束
- `agent/outer-loop-delta-backlog.yaml`: 架构差距列表
- `agent/inner-loop-checkpoint.md`: 当前恢复点
- `agent/interruptions-inbox.md`: 突发任务箱
- `agent/inner-loop-worklog.md`: 每轮工作记录
- `agent/workflow-verify.sh`: 固定验证脚本
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
- 目前还没有 `workflow-run-once.sh` 之类的执行器脚本
- 当前 backlog 还是初始化版本，需要在实际使用中继续打磨粒度

### 后续迭代方向

1. 增加 `agent/workflow-run-once.sh`
2. 增加 `weekly-architecture-review` Skill
3. 在 `outer-loop-delta-backlog.yaml` 中补充更细的 delta 拆分规则
4. 将 `inner-loop-worklog.md` 演进为按日期归档
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
- 在 `outer-loop-delta-backlog.yaml` 中显式支持 `candidate` 和 `rejected` 状态
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

1. 在 `inner-loop-worklog.md` 中增加固定的“Post-Execution Reflection”模板
2. 增加 `review-delta-backlog` 或 `weekly-architecture-review` Skill
3. 增加 `agent/workflow-run-once.sh`，把发现或执行的单轮流程脚本化
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
- 新增 `agent/workflow-run-once.sh`
- 将“各种情况使用什么 prompt”完整写入 `agent/README.md`

### 当前收益

- backlog 不再只能依赖发现型 Skill 间接整理
- 日常使用不必记忆 prompt，可直接查 README 或运行脚本
- 工作流从“设计完备”进一步升级为“更易实际使用”

## 2026-04-23 / v1.2

### 触发原因

- 真实执行 `AD-001` 后，暴露出“单个 delta 膨胀失焦”的风险
- 全仓 `cargo fmt --check` 失败来自仓库既有漂移，说明“全量验证”和“当前改动验证”需要分层
- 仅靠 delta backlog 无法直接回答“哪些理想契约正在收敛”

### 本轮升级目标

- 把“建议拆分”升级为“强制拆分门禁”
- 把验证升级为“delta_verification + baseline_verification”双层模型
- 把“任务推进”补齐为“契约收敛”视角

### 本轮决策

- 在 `agent/README.md` 中加入强制拆分、强制回外循环和验证门禁
- 在 `agent/outer-loop-delta-backlog.yaml` 中加入 `hard_gates`、`verification_definitions` 和 `baseline_debts`
- 新增 `agent/governance-contract-coverage.md`
- 更新 `inner-loop-checkpoint.md`，让下一轮先过 review 门禁

### 当前收益

- 单个 delta 不再能无限续命
- 仓库基线债务被显式记录，不再模糊混入当前 delta 失败
- 可以从契约状态而不是任务数量，观察代码是否持续逼近理想架构

## 2026-04-23 / v1.3

### 触发原因

- 现有 discover skill 仍然偏高层，缺少契约级审计清单
- 工作流虽然可以执行已知问题，但还不能自证“它真的会发现问题”
- 缺少从零开始验证发现能力的冒烟测试

### 本轮升级目标

- 把 discover 从“高层原则”升级为“可重复的契约级审计”
- 增加工作流自测，验证 discover 能否重新发现已知架构偏差
- 让 README 明确说明如何发现未知问题，以及如何验证工作流本身有效

### 本轮决策

- 重写 `discover-architecture-deltas`，加入契约级审计清单和输出覆盖要求
- 新增 `agent/quality-discover-smoke-test.md`
- 在 `agent/README.md` 中加入 discover smoke test、状态迁移表和“如何发现新的未知问题”
- 为 `agent/workflow-run-once.sh` 增加 `smoke` 模式

### 当前收益

- discover 不再只是“读文档然后想一想”，而是带着检查表审计代码
- 当 discover 输出 0 个 candidate 时，必须解释覆盖范围，降低假阳性乐观
- 工作流本身开始具备最小自证能力

## 2026-04-23 / v1.4

### 触发原因

- 当前工作流已经能跑，但用户希望先把 workflow 打磨稳定，再正式投入长期使用
- checkpoint 缺少 schema，resume/review 对格式漂移的防御不足
- 还缺少一个明确的“现在能不能开始依赖这套 workflow 做真实工作”的 go/no-go 判断

### 本轮升级目标

- 给 checkpoint 定义稳定结构
- 引入 workflow readiness level
- 让 resume/review 在使用前先做稳定性检查

### 本轮决策

- 新增 `agent/quality-checkpoint-schema.md`
- 新增 `agent/quality-workflow-readiness.md`
- 为 `inner-loop-checkpoint.md` 增加 metadata
- 增强 `resume-architecture-work` 和 `review-delta-backlog` 的 schema/gate 检查要求
- 在 `agent/README.md` 中加入稳定化模式和 go/no-go 规则
- 为 `agent/workflow-run-once.sh` 增加 `stabilize` 模式

### 当前收益

- checkpoint 格式开始稳定，不再完全依赖隐式约定
- 工作流进入“可评估稳定性”的阶段，而不是只能靠主观感觉
- 你可以先继续优化 workflow，本身也有明确的验收标准

## 2026-04-23 / v1.5

### 触发原因

- 当前工作流已经有验证门禁，但还缺“如何生成符合架构的自动测试”的明确规则
- 理想架构文档与工作流文档都需要补测试原则，避免测试冻结错误实现

### 本轮升级目标

- 定义自动测试生成原则
- 建立“契约 -> failure mode -> 验证层级 -> 文件位置”的映射
- 让工作流能明确决定何时应补测试、何时可以不补

### 本轮决策

- 新增 `agent/quality-testing-strategy.md`
- 在 `agent/README.md` 中新增“架构测试策略”部分
- 为 `agent/workflow-run-once.sh` 增加 `test` 模式
- 在理想架构文档中补高层测试原则

### 当前收益

- 自动测试生成不再只靠临场判断
- 测试开始与架构契约显式对齐
- 工作流能够把“是否补测试”当作正式决策，而不是附带动作
