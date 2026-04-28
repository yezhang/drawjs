# Workflow Visualization

本文件用 `Mermaid` 将 `agent/README.md` 中的工作流规则可视化，帮助快速理解外循环、内循环、状态迁移与中断恢复。

## 1. 总览闭环

```mermaid
flowchart TD
    A[开始一轮工作] --> B[resume-architecture-work]
    B --> C{backlog 是否可信?}
    C -- 否 --> D[review-delta-backlog]
    C -- 是 --> E[选择当前最值得执行的 delta]
    D --> E
    E --> F[execute-architecture-delta]
    F --> G[根因分析与最小修改]
    G --> H[delta_verification]
    H --> I{是否通过?}
    I -- 否 --> J[继续收敛当前 delta]
    J --> K{是否命中强制拆分或回外循环门禁?}
    K -- 是 --> D
    K -- 否 --> F
    I -- 是 --> L[baseline_verification]
    L --> M{失败是否属于既有基线债务?}
    M -- 是 --> N[登记 baseline debt]
    M -- 否 --> O[修复本轮问题]
    O --> F
    M -- 否，且验证通过 --> P[更新 backlog checkpoint worklog contract coverage]
    N --> P
    P --> Q[Post-Execution Reflection]
    Q --> R{发现残余问题或新候选项?}
    R -- 是 --> S[discover-architecture-deltas]
    S --> D
    R -- 否 --> T[结束本轮]
```

## 2. Delta 状态机

```mermaid
stateDiagram-v2
    [*] --> candidate
    candidate --> pending: 证据充分且值得进入 backlog
    candidate --> rejected: 证据不足 / 重复 / 价值低
    pending --> proposed: 完成根因分析
    proposed --> in_progress: 决定本轮执行
    in_progress --> split: 命中强制拆分门禁
    in_progress --> blocked: 缺依赖 / 缺契约 / 缺验证前置
    in_progress --> verified: delta_verification 通过
    verified --> done: 状态文件与覆盖视图已更新
    blocked --> pending: 前置条件补齐
    split --> pending: 子 delta 回到 backlog
```

## 3. 中断与回外循环决策

```mermaid
flowchart TD
    A[当前正在推进 delta] --> B{是否被临时任务打断?}
    B -- 是 --> C[capture-interruption]
    C --> D[写入 inbox]
    C --> E[冻结 checkpoint]
    E --> F[记录下次恢复的最小动作]
    F --> G[等待下次 resume]
    B -- 否 --> H{是否命中回外循环门禁?}
    H -- 是 --> I[review-delta-backlog]
    I --> J{当前 delta 是否需要 split?}
    J -- 是 --> K[生成子 delta 并重排优先级]
    J -- 否 --> L[确认仍由当前 delta 继续]
    K --> M[resume-architecture-work]
    L --> M
    H -- 否 --> N[继续 execute-architecture-delta]
```

## 4. 图与真源的对应关系

- 总体闭环、门禁与日常路径以 `agent/README.md` 为准。
- 当前进行中的 delta、阻塞点和推荐下一步以 `agent/inner-loop-checkpoint.md` 为准。
- 状态定义、强制门禁和基线债务以 `agent/outer-loop-delta-backlog.yaml` 为准。
- 工作流可用等级以 `agent/quality-workflow-readiness.md` 为准。

## 5. 维护建议

- 如果工作流规则变更，优先改真源文件，再同步本图。
- 这份图更适合作为导航图，不替代 `README` 中的完整规则文本。
- 若后续新增新的硬门禁，优先更新“总览闭环”和“中断与回外循环决策”两张图。
