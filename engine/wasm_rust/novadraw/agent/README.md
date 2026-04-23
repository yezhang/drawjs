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

## 文件职责

- `agent/architecture_contracts.md`: 机器可执行的架构硬约束
- `agent/delta_backlog.yaml`: 架构差距队列
- `agent/session_checkpoint.md`: 当前进度、下一步、阻塞点
- `agent/inbox.md`: 突发任务收纳箱
- `agent/worklog.md`: 每轮工作的结构化记录
- `agent/workflow_evolution.md`: 工作流为何这样设计，以及后续如何迭代
- `agent/verify.sh`: 固定验证脚本
- `agent/run_once.sh`: 单轮工作流启动器，输出本轮建议 prompt

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

- `delta_backlog.yaml` 中新增或更新候选项
- 当前最值得处理的正式 delta
- 不值得进入 backlog 的 rejected 项

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

## 每日使用方式

### 发现问题

让 Agent 先执行 `discover-architecture-deltas`，读取：

- `CLAUDE.md`
- `AGENTS.md`
- `doc/理想架构设计.md`
- `agent/architecture_contracts.md`
- `agent/delta_backlog.yaml`
- 相关代码文件

然后输出：

- 新发现的 candidate delta
- 每个候选项的根因摘要
- 是否建议进入正式 backlog
- 若进入 backlog，建议优先级和完成标准

### 整理 backlog

让 Agent 执行 `review-delta-backlog`，要求：

- 检查 backlog 是否重复、过大、过时
- 判断 candidate 是否应提升或拒绝
- 明确当前最值得执行的一个 delta
- 如有必要，建议拆分或重排优先级

### 开始工作

让 Agent 先执行 `resume-architecture-work`，读取：

- `CLAUDE.md`
- `AGENTS.md`
- `doc/理想架构设计.md`
- `agent/architecture_contracts.md`
- `agent/delta_backlog.yaml`
- `agent/session_checkpoint.md`

然后输出：

- 当前主线 delta
- 当前阶段
- 已完成内容
- 阻塞点
- 推荐下一步

### 推进一轮

让 Agent 执行 `execute-architecture-delta`，要求：

- 只处理一个 delta
- 先说明根因
- 给最小修改方案
- 修改代码并验证
- 更新 `delta_backlog.yaml`
- 更新 `session_checkpoint.md`
- 追加 `worklog.md`
- 若发现残余问题，输出新的 candidate delta

### 被打断时

让 Agent 执行 `capture-interruption`，要求：

- 把突发任务写入 `agent/inbox.md`
- 把当前主线状态写入 `agent/session_checkpoint.md`
- 明确下一步最小动作

### 单轮启动器

可以先运行：

```bash
./agent/run_once.sh discover
./agent/run_once.sh review
./agent/run_once.sh resume
./agent/run_once.sh execute
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

### 整理 backlog

```text
请执行 review-delta-backlog，整理当前 backlog，给出去重、拆分、优先级重排建议，并指出当前最值得执行的一个 delta。
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

## 脚本化辅助

如果你不想每次手写 prompt，可以先运行脚本再复制输出：

```bash
./agent/run_once.sh discover
./agent/run_once.sh review
./agent/run_once.sh resume
./agent/run_once.sh execute
```

对应关系：

- `discover`: 发现候选问题
- `review`: 整理 backlog
- `resume`: 恢复主线
- `execute`: 推进一轮

## Delta 设计规则

- 每个 delta 只描述一个职责边界问题或一个状态归属问题
- 一个 delta 的修改范围应尽量限制在一个接口簇或一个调用链
- 如果改动超过 50 行，优先拆分为多个 delta
- 如果还不能确定是否值得执行，先作为 `candidate`

## 候选项进入 backlog 的标准

- 能清楚描述理想契约与当前现状的偏差
- 影响范围可估计
- 可以写出明确的 `done_when`
- 有足够证据指向相关代码位置
- 不只是一次性观察或模糊抱怨

## 完成标准

一个 delta 只有同时满足以下条件才能进入 `done`：

- 根因已解释清楚
- 代码已修改
- 固定验证已通过
- backlog 已更新
- checkpoint 已更新
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

## 注意事项

- 如果发现理想架构文档本身不完整，优先补契约，不直接写代码
- 如果当前 delta 需要跨多个子系统，先拆解再执行
- 如果突发任务很多，优先保证 `session_checkpoint.md` 始终可信
- 如果 backlog 很久未整理，不要盲目继续执行旧 delta
