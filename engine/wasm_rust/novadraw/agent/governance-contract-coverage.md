# Contract Coverage

本文件用于从“契约收敛”而不是“任务推进”视角，观察代码是否持续逼近理想架构。

## 状态定义

- `unassessed`: 尚未评估
- `drifting`: 明显偏离理想架构
- `partially_aligned`: 已局部收敛，但仍有残余问题
- `aligned`: 当前已与理想架构对齐

## Coverage

| Contract ID | Contract | Status | Evidence | Notes |
|---|---|---|---|---|
| C-01 | Figure 只负责内在能力，不持有外部关系 | unassessed | `agent/governance-architecture-contracts.md` | 待通过后续 delta 评估 |
| C-02 | FigureBlock 是节点运行时状态容器 | unassessed | `agent/governance-architecture-contracts.md` | 待评估 |
| C-03 | FigureGraph 持有树关系、交互状态和命中测试图级信息 | partially_aligned | `AD-001` validation 收口 | validation 图语义已部分回收，但需继续观察 repair/scheduling 边界 |
| C-04 | UpdateManager 只负责两阶段更新编排 | partially_aligned | `AD-001` | validation phase 已收口；repair 与 scheduling 仍待收敛 |
| C-05 | EventDispatcher 只负责事件分发 | unassessed | `agent/governance-architecture-contracts.md` | 待后续事件相关 delta |
| C-06 | SceneHost 是极薄平台调度层 | drifting | `AD-002` pending | 尚未进入执行 |
| C-07 | NovadrawSystem 是组合根 | drifting | `AD-004` pending | 尚未进入执行 |
| C-08 | 结构性变更必须通过 PendingMutation 延迟应用 | drifting | `AD-003` pending | 尚未进入执行 |
| C-09 | 新接口优先表达职责边界，不为兼容现状引入职责回流 | partially_aligned | `AD-001` | 当前方向正确，但仍需防止 repair/scheduling 回流 |
| C-10 | 任何架构改动都要说明为何更接近理想架构 | partially_aligned | `agent/inner-loop-worklog.md` | 已有结构化记录，但还需保持长期执行 |

## 使用规则

- 每轮执行后，只更新受影响契约
- 如果一个 delta 被拆分，父 delta 影响的契约状态保持不变，直到子问题分别收敛
- 不允许因为完成了一个 delta，就自动把契约状态提升到 `aligned`
- 若契约状态无法判断，保持 `unassessed`，不要主观乐观升级
