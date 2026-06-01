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
| C-01 | Figure 只负责内在能力，不持有外部关系 | aligned | `AD-009` verified | Figure trait 与具体 figure 未持有 parent/children、FigureGraph、UpdateManager、SceneHost 或平台对象；事件响应通过 NovadrawContext 请求外部操作 |
| C-02 | FigureBlock 是节点运行时状态容器 | partially_aligned | `AD-009` verified; `AD-011` proposed | FigureBlock 已移除 selection overlay 绘制行为；CAD-002 REVIEW 已拆分，FigureBlock 字段/方法公开面留待 AD-011 后重新评估 |
| C-03 | FigureGraph 持有树关系、交互状态和命中测试图级信息 | partially_aligned | `AD-005` verified; `AD-011` proposed | 交互状态归属已收敛；AD-011 将处理 `FigureGraph.blocks` / `uuid_map` 公开存储面可能破坏图级单一所有权的问题 |
| C-04 | UpdateManager 只负责两阶段更新编排 | aligned | `AD-001` done | validation / repair / scheduling 三个子边界均已收敛；UpdateManager 管理队列与阶段，FigureGraph/组合根/SceneHost 分别承载图语义与平台调度 |
| C-05 | EventDispatcher 只负责事件分发 | partially_aligned | `AD-005` verified; `CAD-003` candidate | BasicEventDispatcher 本身仍只分发；completion audit 发现 editor interaction 路径仍有热路径日志残留，需确认是否违反事件高频路径边界 |
| C-06 | SceneHost 是极薄平台调度层 | aligned | `AD-002` verified | `WinitSceneHost` 仅保留 window proxy 与 redraw pending；editor/render 策略状态位于组合根 |
| C-07 | NovadrawSystem 是组合根 | partially_aligned | `AD-004` verified; `CAD-004` candidate | 可变 escape hatch 已移除；completion audit 发现 app_window 仍通过只读 scene_manager/query 和 EditorInteractionCore 静态方法触达内部结构，需审计是否职责泄漏 |
| C-08 | 结构性变更必须通过 PendingMutation 延迟应用 | partially_aligned | `AD-003` verified; `CAD-006` candidate | 主路径已在顶层分发后 apply；completion audit 发现 PendingMutations 生产阶段和 AddChild 既有节点能力仍需类型层面审计 |
| C-09 | 新接口优先表达职责边界，不为兼容现状引入职责回流 | partially_aligned | `AD-010` verified; `AD-011` proposed; `CAD-004` / `CAD-005` candidates | 公开可变系统逃生口已移除；completion audit 发现 FigureGraph/FigureBlock 公开面、组合根只读面和理想文档旧表述仍可能引入职责回流 |
| C-10 | 任何架构改动都要说明为何更接近理想架构 | aligned | `agent/inner-loop-worklog.md`, `agent/workflow-continuous.md` | 近几轮 delta 已稳定记录 Root Cause / Minimal Fix / Decision / Split Decision / Reflection / Verification；持续工作流强制每轮判断是否减少架构差距 |

## 使用规则

- 每轮执行后，只更新受影响契约
- 如果一个 delta 被拆分，父 delta 影响的契约状态保持不变，直到子问题分别收敛
- 不允许因为完成了一个 delta，就自动把契约状态提升到 `aligned`
- 若契约状态无法判断，保持 `unassessed`，不要主观乐观升级
