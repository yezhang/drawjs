---
title: Solo Coder Architecture Workflow
---

# Solo Coder Architecture Workflow

这套工作流用于让 solo coder 在多轮会话、中断恢复、突发任务插入的情况下，仍然能够持续按照 `doc/理想架构设计.md` 渐进式改善当前代码，并形成“发现问题 -> 建模问题 -> 解决问题 -> 反思收敛”的完整闭环。

## 目标

- 让 Agent 既能发现架构差距，也能小步推进架构差距
- 让长期任务具备可中断、可恢复、可审计能力
- 降低每次重新开始时重新梳理上下文的成本
- 让 backlog 持续反映“当前最值得解决的问题”

## 核心原则

- 理想架构是目标，当前代码只是输入，不是约束
- 一次只处理一个 `Architecture Delta`
- 问题进入正式 backlog 之前，先作为候选项进行建模与分拣
- 先解释根因，再做最小改动
- 每轮结束必须写回状态文件
- 没有验证结果，不能标记完成
- 执行完成后必须反思是否仍有残余架构问题

## 演进范围

当前工作流服务的不是“任意新功能开发”，而是“为实现理想架构和 g2 等价语义而进行的演进”。

每个 backlog 条目都应属于以下三类之一：

- `architecture`
  - 纯架构收敛项
  - 目标是职责边界、状态归属、协议时序和依赖方向收敛
- `parity`
  - 为实现 g2 等价语义必须补齐的能力缺口
  - 从产品视角可能像新功能，但仍属于目标范围
- `new_feature`
  - 超出当前理想架构 / g2 等价目标之外的新能力

默认规则：

- 工作流默认主动推进 `architecture` 和 `parity`
- 工作流默认不主动推进 `new_feature`
- 若某项属于 `new_feature`，必须在 backlog review 中被显式标记，避免混入主线

### 如何区分

- 如果问题本质是“当前实现偏离理想职责边界或协议”，归为 `architecture`
- 如果问题本质是“g2 已有明确语义，而当前 Novadraw 尚未补齐”，归为 `parity`
- 如果问题不属于理想架构必需能力，也不属于 g2 等价范围，归为 `new_feature`

## 文件职责

### 固定入口

- `agent/README.md`: 工作流总入口，解释整体规则、日常路径和常用 prompt

### Governance

- `agent/governance-architecture-contracts.md`: 机器可执行的架构硬约束
- `agent/governance-contract-coverage.md`: 理想架构契约的收敛状态总览

### Outer Loop

- `agent/outer-loop-delta-backlog.yaml`: 外循环 backlog，记录 candidate、正式 delta、门禁和基线债务

### Inner Loop

- `agent/inner-loop-checkpoint.md`: 当前主线、下一步、阻塞点与恢复入口
- `agent/inner-loop-worklog.md`: 每轮执行记录，包括验证、反思和拆分决策

### Interruptions

- `agent/interruptions-inbox.md`: 突发任务收纳箱，避免打乱主线 backlog

### Quality

- `agent/quality-checkpoint-schema.md`: checkpoint 的稳定结构定义与兼容规则
- `agent/quality-workflow-readiness.md`: 当前工作流稳定性等级与 go/no-go 检查
- `agent/quality-discover-smoke-test.md`: discover 能力自测用例
- `agent/quality-testing-strategy.md`: 自动测试生成原则、验证层级与契约映射

### Workflow

- `agent/workflow-history.md`: 工作流为什么演进成当前形态
- `agent/workflow-map.md`: 工作流总览图、状态机图和中断决策图
- `agent/workflow-verify.sh`: 固定验证脚本
- `agent/workflow-run-once.sh`: 单轮启动器，输出本轮推荐 prompt

## 命名约定

- `governance-*`: 约束、覆盖率、治理视角的文件
- `outer-loop-*`: 外循环，负责发现问题、整理 backlog
- `inner-loop-*`: 内循环，负责恢复、执行、记录当前主线
- `interruptions-*`: 中断处理与突发任务入口
- `quality-*`: 质量门禁、自测、schema、readiness 检查
- `workflow-*`: 工作流本身的总览、历史、地图和辅助脚本
- `README.md`: 保持固定入口，不参与前缀化，避免破坏默认导航习惯

## 五个 Skill

- `discover-architecture-deltas`: 从理想架构与现状偏差中发现候选 delta
- `review-delta-backlog`: 对 backlog 做去重、拆分、重排优先级
- `resume-architecture-work`: 恢复当前主线，决定下一步
- `execute-architecture-delta`: 执行一个最小架构差距
- `capture-interruption`: 处理中断，冻结现场并登记突发任务

## 总体闭环

这套工作流分为两个循环：

- 外循环：问题发现与 backlog 整理
- 内循环：问题执行与验证收敛

只有两个循环都存在，工作流才是完整闭环。

如果你想先看图，再读规则，直接查看 `agent/workflow-map.md`。

## 稳定性阶段

在正式长期依赖这套工作流之前，先看它处于哪个稳定性阶段。

- `Level 0 / Draft`: 只有概念，没有可执行闭环
- `Level 1 / Runnable`: 可以跑，但缺少自测或门禁
- `Level 2 / Stable Enough`: 可以开始承载真实工作，但仍需持续观察
- `Level 3 / Trusted`: 多轮真实使用后已足够稳定

当前等级以 `agent/quality-workflow-readiness.md` 为准。

## 外循环：发现与整理

外循环负责回答“现在最值得解决的问题是什么”。

### 步骤

1. 读取理想架构契约和当前代码现状
2. 识别理想架构与当前实现之间的偏差
3. 将偏差写为 `candidate delta`
4. 判断候选项是否应该进入正式 backlog
5. 为正式 backlog 排定优先级、范围和完成标准

### 何时触发

- 新阶段开始时
- 当前 backlog 做完或价值下降时
- 做完一轮执行后发现新的残余问题时
- 人工 review、测试失败、设计讨论后出现新问题时

### 产出

- `outer-loop-delta-backlog.yaml` 中新增或更新候选项
- 当前最值得处理的正式 delta
- 不值得进入 backlog 的 rejected 项
- 每个候选项的 `evolution_kind`

## 内循环：执行与收敛

内循环负责回答“如何安全地解决当前这个问题”。

### 步骤

1. 恢复现场
2. 选择一个待处理 delta
3. 解释根因与最小方案
4. 修改代码
5. 运行验证
6. 更新 backlog、checkpoint、worklog
7. 反思是否仍有残余问题，必要时回流到外循环
8. 决定下一轮是否继续

## 强制门禁

以下条件不是建议，而是必须执行的门禁。

### 强制拆分门禁

当满足任一条件时，当前 `in_progress` delta 必须转为 `split`，不得继续直接执行：

- `Post-Execution Reflection` 中出现 2 个或以上独立子问题
- `Next Step` 已分叉为多个不共享同一职责边界的问题
- 同一 delta 连续 2 轮后仍未明显收敛
- 为了继续推进，必须同时修改两个以上不共享同一根因的子系统

### 强制回外循环门禁

当满足任一条件时，不得继续直接执行旧 delta，必须先运行 `review-delta-backlog`：

- 当前 delta 被标记为 `split`
- backlog 里的当前 delta 优先级已不再明显最高
- 当前验证失败原因主要来自仓库基线而非本轮改动
- `worklog` 或 `checkpoint` 已写出“先整理 backlog”“先判断是否拆分”等信号

### 验证门禁

验证分为两层：

- `delta_verification`: 当前 delta 相关的最小验证，必须通过
- `baseline_verification`: 全仓校验，若失败但属于既有问题，必须登记为基线债务

规则：

- `delta_verification` 未通过，delta 不得进入 `verified`
- `baseline_verification` 未通过且属于仓库既有问题时，允许继续推进，但必须写入 `outer-loop-delta-backlog.yaml`、`inner-loop-checkpoint.md` 和 `inner-loop-worklog.md`
- 任何口头说明都不能替代基线债务记录

## 推荐状态机

- `candidate`: 已被发现，但还没进入正式 backlog
- `rejected`: 已评估，但当前不值得进入 backlog
- `pending`: 已进入正式 backlog，尚未处理
- `proposed`: 已完成根因分析，待实施
- `in_progress`: 正在修改代码
- `split`: 原问题过大，已拆成多个 delta
- `blocked`: 发现依赖或文档缺口，暂停
- `verified`: 已通过验证
- `done`: 已完成并回写状态

## 契约覆盖视图

为了确认代码在整体上持续逼近理想架构，不只跟踪 delta，还要跟踪契约收敛状态。

维护文件：`agent/governance-contract-coverage.md`

每条契约的状态只能是：

- `unassessed`: 尚未评估
- `drifting`: 明显偏离理想架构
- `partially_aligned`: 已局部收敛，但仍有残余问题
- `aligned`: 当前已与理想架构对齐

每轮执行后，必须更新受影响契约的覆盖状态。

## 每日使用方式

### 发现问题

让 Agent 先执行 `discover-architecture-deltas`，读取：

- `CLAUDE.md`
- `AGENTS.md`
- `doc/理想架构设计.md`
- `agent/governance-architecture-contracts.md`
- `agent/outer-loop-delta-backlog.yaml`
- `agent/governance-contract-coverage.md`
- `agent/quality-discover-smoke-test.md`
- 相关代码文件

然后输出：

- 新发现的 candidate delta
- 本轮审计了哪些契约
- 本轮检查了哪些代码入口
- 每个 candidate 的 `evolution_kind`
- 每个候选项的根因摘要
- 是否建议进入正式 backlog
- 若进入 backlog，建议优先级和完成标准

### 验证 discover 能力

让 Agent 执行 `discover-architecture-deltas` 的 smoke test，要求：

- 不直接重复 backlog 结论
- 按契约级审计清单重新检查代码
- 说明这次发现重新识别出了哪些已知偏差
- 如果发现结果为 0，必须说明覆盖范围和遗漏范围

### 整理 backlog

让 Agent 执行 `review-delta-backlog`，要求：

- 检查 backlog 是否重复、过大、过时
- 判断 candidate 是否应提升或拒绝
- 明确当前最值得执行的一个 delta
- 如有必要，建议拆分或重排优先级
- 如果当前 delta 命中强制拆分或强制回外循环门禁，必须明确指出，不得继续直接执行
- 检查条目是否被错误归类为 `architecture / parity / new_feature`

### 开始工作

让 Agent 先执行 `resume-architecture-work`，读取：

- `CLAUDE.md`
- `AGENTS.md`
- `doc/理想架构设计.md`
- `agent/governance-architecture-contracts.md`
- `agent/outer-loop-delta-backlog.yaml`
- `agent/inner-loop-checkpoint.md`

然后输出：

- 当前主线 delta
- 当前阶段
- 已完成内容
- 阻塞点
- 推荐下一步

### 使用前稳定性检查

如果你准备开始真正依赖这套工作流推进代码，而不是继续调 workflow 本身，先执行一次稳定性检查：

- 查看 `agent/quality-workflow-readiness.md`
- 校验 `agent/inner-loop-checkpoint.md` 是否满足 `agent/quality-checkpoint-schema.md`
- 跑一次 smoke test

### 推进一轮

让 Agent 执行 `execute-architecture-delta`，要求：

- 只处理一个 delta
- 先说明根因
- 给最小修改方案
- 修改代码并验证
- 更新 `outer-loop-delta-backlog.yaml`
- 更新 `inner-loop-checkpoint.md`
- 追加 `inner-loop-worklog.md`
- 更新 `governance-contract-coverage.md`
- 若发现残余问题，输出新的 candidate delta

### 被打断时

让 Agent 执行 `capture-interruption`，要求：

- 把突发任务写入 `agent/interruptions-inbox.md`
- 把当前主线状态写入 `agent/inner-loop-checkpoint.md`
- 明确下一步最小动作

### 单轮启动器

可以先运行：

```bash
./agent/workflow-run-once.sh discover
./agent/workflow-run-once.sh review
./agent/workflow-run-once.sh resume
./agent/workflow-run-once.sh execute
./agent/workflow-run-once.sh smoke
./agent/workflow-run-once.sh stabilize
```

它不会直接调用模型，而是输出本轮推荐 prompt 和所需上下文文件，方便你复制给 Agent。

## 推荐提问模板

### 恢复

```text
请执行 resume-architecture-work，告诉我当前主线、最近停在哪、推荐下一步。
```

### 发现

```text
请执行 discover-architecture-deltas，对照理想架构找出当前最值得进入 backlog 的候选问题。
```

要求补充：

- 为每个 candidate 标注 `evolution_kind`
- 明确说明它为什么属于 `architecture`、`parity` 或 `new_feature`

### 发现能力自测

```text
请执行 discover-architecture-deltas 的 smoke test。不要直接沿用 backlog 结论，而是按审计清单重新检查代码，并告诉我这次 discover 是否能重新发现 quality-discover-smoke-test.md 中列出的已知问题样本。
```

### 稳定性检查

```text
请执行 resume-architecture-work 和 review-delta-backlog 的稳定性检查。先验证 inner-loop-checkpoint.md 是否满足 quality-checkpoint-schema.md，再结合 quality-workflow-readiness.md 判断当前工作流是否已经达到可用于真实架构推进的等级。
```

### 整理 backlog

```text
请执行 review-delta-backlog，整理当前 backlog，给出去重、拆分、优先级重排建议，并指出当前最值得执行的一个 delta。
```

要求补充：

- 检查每个条目的 `evolution_kind` 是否正确
- 若某条目属于 `new_feature`，明确指出它不应默认进入当前主线

### 强制拆分判断

```text
请执行 review-delta-backlog，重点判断当前 in_progress delta 是否已经触发强制拆分门禁；如果触发，请直接给出拆分后的子 delta 建议，而不是继续执行旧 delta。
```

### 执行一轮

```text
请执行 execute-architecture-delta，本轮只处理一个 delta，不要跨层级大改。
```

### 中断

```text
请执行 capture-interruption，把当前工作冻结，并记录这个突发任务。
```

## 场景速查

### 场景 1：刚开始一天工作，不确定从哪开始

- 先用：`resume-architecture-work`
- 如果它判断 backlog 可能失真，再用：`discover-architecture-deltas`

推荐提示词：

```text
请执行 resume-architecture-work，告诉我当前主线、最近停在哪、推荐下一步；如果 backlog 可能失真，请明确建议我先跑 discover-architecture-deltas 还是 review-delta-backlog。
```

### 场景 1a：暂时不想继续改代码，只想继续打磨 workflow

- 先用：`resume-architecture-work`
- 再用：`review-delta-backlog`
- 结合 `quality-workflow-readiness.md` 判断还缺什么

推荐提示词：

```text
请先检查当前工作流稳定性：验证 checkpoint schema 是否健康、当前 backlog 门禁是否足够、discover smoke test 是否已通过；然后告诉我还差哪些条件，工作流才能进入更稳定的使用阶段。
```

### 场景 2：感觉 backlog 已经过时，或者做了一轮后优先级不清楚

- 先用：`review-delta-backlog`

推荐提示词：

```text
请执行 review-delta-backlog，检查当前 backlog 是否存在重复项、过大项、过时项，并给出当前最值得执行的一个 delta。
```

### 场景 3：进入一个新阶段，需要重新找问题

- 先用：`discover-architecture-deltas`

推荐提示词：

```text
请执行 discover-architecture-deltas，从理想架构与当前实现偏差中找出新的 candidate delta，并说明哪些值得进入正式 backlog。
```

### 场景 3b：怀疑某项其实是目标外新功能

- 先用：`review-delta-backlog`

推荐提示词：

```text
请执行 review-delta-backlog，重点判断当前候选项和正式 delta 中，哪些属于 architecture，哪些属于 parity，哪些其实是目标外 new_feature；如果是 new_feature，请明确指出为什么它不应默认进入当前主线。
```

### 场景 3a：怀疑 discover 太乐观，想确认它真的会找问题

- 先用：`discover-architecture-deltas`
- 以 smoke test 模式运行

推荐提示词：

```text
请执行 discover-architecture-deltas 的 smoke test。重点回答：这次 discover 重新审计了哪些契约、看了哪些代码入口、重新发现了哪些已知偏差、漏掉了哪些样本。
```

### 场景 4：已经选定一个 delta，准备开始改代码

- 先用：`execute-architecture-delta`

推荐提示词：

```text
请执行 execute-architecture-delta，本轮只处理一个 delta，先解释根因，再给最小修改方案，最后运行验证并更新工作流文件。
```

### 场景 5：做完一个 delta 后，不确定是否真的收敛

- 先用：`discover-architecture-deltas`
- 再用：`review-delta-backlog`

推荐提示词：

```text
请先执行 discover-architecture-deltas，判断本轮执行后是否暴露了新的候选问题；然后执行 review-delta-backlog，整理 backlog 并给出下一步建议。
```

### 场景 6：中途被 Bug、Review 或临时任务打断

- 先用：`capture-interruption`

推荐提示词：

```text
请执行 capture-interruption，把当前主线冻结到 checkpoint，并把这个突发任务登记到 inbox，明确下次恢复时的第一步。
```

### 场景 7：当前问题太大，不适合一轮做完

- 先用：`review-delta-backlog`
- 再用：`execute-architecture-delta`

推荐提示词：

```text
请先执行 review-delta-backlog，判断当前 delta 是否需要拆分；如果需要，请给出拆分建议和新的执行顺序。
```

### 场景 8：文档和代码矛盾，不知道应该信谁

- 先用：`discover-architecture-deltas`
- 结果若指向契约缺口，则先更新契约，不直接写代码

推荐提示词：

```text
请执行 discover-architecture-deltas，判断这是代码偏差还是契约缺口；如果是契约缺口，请明确指出应先补哪条契约，而不是直接改代码。
```

## 推荐日常路径

### 轻量模式

适合日常持续推进：

1. `resume-architecture-work`
2. `execute-architecture-delta`
3. `capture-interruption`（如果被打断）

### 标准模式

适合 backlog 可能已经失真时：

1. `resume-architecture-work`
2. `review-delta-backlog`
3. `execute-architecture-delta`
4. `discover-architecture-deltas`

### 新阶段模式

适合开始一个新的架构收敛阶段：

1. `discover-architecture-deltas`
2. `review-delta-backlog`
3. `resume-architecture-work`
4. `execute-architecture-delta`

### 稳定化模式

适合你当前这种“先把 workflow 打磨稳，再正式使用”的阶段：

1. `resume-architecture-work`
2. `discover-architecture-deltas` smoke test
3. `review-delta-backlog`
4. 检查 `quality-workflow-readiness.md`
5. 只在满足 go/no-go 条件后再进入 `execute-architecture-delta`

## 脚本化辅助

如果你不想每次手写 prompt，可以先运行脚本再复制输出：

```bash
./agent/workflow-run-once.sh discover
./agent/workflow-run-once.sh review
./agent/workflow-run-once.sh resume
./agent/workflow-run-once.sh execute
./agent/workflow-run-once.sh smoke
./agent/workflow-run-once.sh stabilize
```

对应关系：

- `discover`: 发现候选问题
- `review`: 整理 backlog
- `resume`: 恢复主线
- `execute`: 推进一轮
- `smoke`: 验证 discover 是否真的能发现已知问题
- `stabilize`: 检查 workflow 是否已达到可用等级

## Delta 设计规则

- 每个 delta 只描述一个职责边界问题或一个状态归属问题
- 一个 delta 的修改范围应尽量限制在一个接口簇或一个调用链
- 如果改动超过 50 行，优先拆分为多个 delta
- 如果还不能确定是否值得执行，先作为 `candidate`
- 若一个 delta 的 `Next Step` 已经分叉成多个独立子问题，必须转为 `split`
- 判断是否拆分时，优先看“独立根因数量”和“职责边界数量”，代码行数只作辅助参考

## 候选项进入 backlog 的标准

- 能清楚描述理想契约与当前现状的偏差
- 影响范围可估计
- 可以写出明确的 `done_when`
- 有足够证据指向相关代码位置
- 不只是一次性观察或模糊抱怨
- discover 输出若为 `0 个 candidate`，也必须同时说明已审计范围和未审计范围
- 能明确判定其 `evolution_kind`

## 状态迁移表

| From | To | Trigger | Required Output |
|---|---|---|---|
| `candidate` | `pending` | 证据、done_when、入口文件齐全，且值得进入正式 backlog | backlog 更新 |
| `candidate` | `rejected` | 当前证据不足、重复或价值较低 | reject reason |
| `pending` | `proposed` | 已完成根因分析，形成最小实施方案 | root cause + minimal plan |
| `proposed` | `in_progress` | 当前轮决定正式执行该 delta | checkpoint 更新 |
| `in_progress` | `split` | 命中强制拆分门禁 | split 子项建议 |
| `in_progress` | `verified` | `delta_verification` 通过 | verification result |
| `verified` | `done` | backlog、checkpoint、worklog、contract coverage 均已更新 | done summary |
| `*` | `blocked` | 缺依赖、缺契约、缺验证前置条件 | blocker note |

## 双层验证定义

### Delta Verification

用于确认本轮改动本身是正确的，示例：

- 当前 crate 的 `cargo test`
- 当前链路的集成测试
- 本轮接口变更相关的 `cargo check`

### Baseline Verification

用于确认仓库整体健康度，示例：

- `./agent/workflow-verify.sh`
- 全仓 `cargo fmt --check`
- 全仓 `cargo clippy -- -D warnings`

### 基线债务处理规则

- 若 `baseline_verification` 因本轮未修改文件失败，登记为基线债务
- 基线债务必须有唯一标识、影响范围和后续处理建议
- 基线债务不能伪装成本轮 delta 的失败，也不能被静默忽略

## 架构测试策略

测试相关的规则分两层：

- `doc/理想架构设计.md`：定义高层测试原则与哪些契约必须可验证
- `agent/quality-testing-strategy.md`：定义自动测试生成规则、验证层级和契约映射

核心要求：

- 测试契约，不测试实现镜像
- 每个 delta 只补最小必要测试
- 测试层级必须与风险匹配
- 若本轮不补测试，必须在 `inner-loop-worklog.md` 中说明原因

### 何时应该补自动测试

- 当前 delta 改变了职责边界
- 当前 delta 改变了结构性变更时机
- 当前 delta 修复了已知回归风险
- 相邻模块已有可复用的测试模式

### 何时可以不补自动测试

- 纯文档修改
- 纯命名收口
- 低风险重排且已有测试足够覆盖

### 测试结果应写到哪里

- `outer-loop-delta-backlog.yaml`
  - 记录 `verification_scope`
- `inner-loop-worklog.md`
  - 记录本轮补了哪些测试，或为什么没补
- `inner-loop-checkpoint.md`
  - 记录当前验证状态和基线债务

### 推荐测试提问模板

#### 制定测试策略

```text
请基于 agent/quality-testing-strategy.md，为当前 delta 制定最小测试策略。回答这 4 件事：
1. 当前契约是什么
2. 最可能的 failure mode 是什么
3. 应该用哪一层验证（L1/L2/L3/L4）
4. 建议把测试放到哪个文件
```

#### 补最小必要测试

```text
请基于 agent/quality-testing-strategy.md，为当前 delta 补最小必要测试。测试必须验证契约，不要镜像当前实现；如果你判断本轮不应新增测试，请明确说明原因。
```

#### 评审测试是否符合架构

```text
请审查本轮新增测试是否符合 agent/quality-testing-strategy.md：重点检查它是在验证契约，还是在冻结当前实现细节。
```

## 如何发现新的未知问题

外循环不是“重复 backlog”，而是系统化审计契约偏差。每次 discover 至少遵守以下策略：

1. 先按 `governance-contract-coverage.md` 选择 `drifting` 和 `unassessed` 契约
2. 每条契约至少检查一个代码入口
3. 不允许只看文档就宣布“已对齐”
4. 如果结果是 `0 个 candidate`，必须说明为什么不是“没仔细看”
5. 定期跑 `quality-discover-smoke-test.md`，验证 discover 仍然能发现已知偏差

## 如何判断工作流本身有效

工作流有效，不等于“执行了一些 delta”，而是至少满足：

- discover 能从审计清单中稳定发现已知偏差样本
- review 能识别重复项、过大项和失焦 delta
- execute 能把一个 delta 推进到更接近理想架构的状态
- contract coverage 能显示整体契约状态不是随机波动，而是在逐步收敛
- checkpoint 能在中断后恢复主线，不需要重新建模全部上下文

## Go / No-Go

在“正式把主要精力转到代码架构改进，而不是继续打磨 workflow”之前，建议至少满足：

- 最近一次 smoke test 通过
- 当前 checkpoint 满足 `quality-checkpoint-schema.md`
- 当前 backlog 没有明显失焦的大 delta
- 当前 baseline debt 已登记
- `quality-workflow-readiness.md` 的当前等级至少达到 `Stable Enough`

如果以上条件仍不满足，优先继续优化 workflow，而不是盲目推进新 delta。

## 完成标准

一个 delta 只有同时满足以下条件才能进入 `done`：

- 根因已解释清楚
- 代码已修改
- 固定验证已通过
- `delta_verification` 已通过
- backlog 已更新
- checkpoint 已更新
- `governance-contract-coverage.md` 已更新
- 对残余问题已做反思并决定是否生成新候选项

## 闭环完成的判定

一次闭环不等于“执行完一个 delta 就结束”，而是指：

1. 当前问题被识别并结构化
2. 该问题被纳入 backlog 或明确拒绝
3. 已选中的问题被执行并验证
4. 执行后对残余问题进行了反思
5. 新问题重新进入候选池，或确认当前局部已经收敛

## 推荐节奏

- 每天开始时先跑一次外循环，确认 backlog 是否仍然可信
- 每次编码时只跑一轮内循环
- 每完成 1 到 3 个 delta 后，重新跑一次外循环
- 如果遇到大范围漂移，暂停执行，优先整理 backlog
- 若某个 delta 连续 2 轮仍未收敛，下一轮必须先跑 `review-delta-backlog`

## 注意事项

- 如果发现理想架构文档本身不完整，优先补契约，不直接写代码
- 如果当前 delta 需要跨多个子系统，先拆解再执行
- 如果突发任务很多，优先保证 `inner-loop-checkpoint.md` 始终可信
- 如果 backlog 很久未整理，不要盲目继续执行旧 delta
- 不允许用“继续推进同一个大 delta”替代拆分决策
