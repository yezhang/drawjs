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
| C-02 | FigureBlock 是节点运行时状态容器 | aligned | `AD-009` verified; `AD-011` verified; `AD-012` verified | FigureBlock 已移除 selection overlay 绘制行为；AD-012 已收窄字段、删除 public mutator，并移除 facade re-export；crate 外不能通过 FigureBlock 绕过 FigureGraph 图级不变量 |
| C-03 | FigureGraph 持有树关系、交互状态和命中测试图级信息 | aligned | `AD-005` verified; `AD-011` verified; `AD-018` verified; `AD-019B` verified | 交互状态归属已收敛；`FigureGraph.blocks` / `uuid_map` 已收窄为私有存储；AD-018 将 Viewport 接入 Figure 树父链协议，FigureGraph 的 hit-test / render / damage repair 继续统一消费图级拓扑与坐标转换；AD-019B 已将图结构核心物理归入 `graph` 子域 |
| C-04 | UpdateManager 只负责两阶段更新编排 | aligned | `AD-001` done | validation / repair / scheduling 三个子边界均已收敛；UpdateManager 管理队列与阶段，FigureGraph/组合根/SceneHost 分别承载图语义与平台调度 |
| C-05 | EventDispatcher 只负责事件分发 | aligned | `AD-005` verified; `AD-013` verified | BasicEventDispatcher 本身仍只分发；editor interaction 默认热路径日志已清理，平台输入和示例 Figure 回调不再默认打印 mouse/raw pointer/entered/exited 日志 |
| C-06 | SceneHost 是极薄平台调度层 | aligned | `AD-002` verified; `AD-019A` verified | `WinitSceneHost` 仅保留 window proxy 与 redraw pending；editor/render 策略状态位于组合根；`SceneHost` trait 已归入 `host` 子域，目录边界同步表达平台宿主职责 |
| C-07 | NovadrawSystem 是组合根 | aligned | `AD-004` verified; `AD-010` verified; `AD-014` verified | 可变 escape hatch 与 editor 组合根残余只读面均已收敛；app_window 不再通过 scene_manager/query 触达组合根内部 SceneManager，平台输入层改用组合根命名 query/action |
| C-08 | 结构性变更必须通过 PendingMutation 延迟应用 | aligned | `AD-003` verified; `AD-016` verified; `AD-017` verified | 主路径仍在顶层分发后 apply；PendingMutation 具体构造、enqueue 与 MutationContext 已收窄为 crate 内部，apply 只接受 PendingMutations drain 出的 batch，既有 BlockId AddChild 能力已移除；AD-017 已在 reparent apply 阶段补齐防环校验，拒绝 self reparent 与 descendant reparent 且失败无副作用 |
| C-09 | 新接口优先表达职责边界，不为兼容现状引入职责回流 | aligned | `AD-010` verified; `AD-011` verified; `AD-012` verified; `AD-014` verified; `AD-015` verified; `AD-018` verified; `AD-019B` verified | 公开可变系统逃生口、FigureGraph 存储逃生口、FigureBlock 可变面与 editor 组合根只读面已移除；Viewport 核心语义位于 `novadraw-scene` Figure 协议，apps/editor 未引入平台滚动状态或全局坐标特判；目录重组保留 facade 兼容但内部路径改向真实子域，避免新代码继续依赖 root re-export |
| C-10 | 任何架构改动都要说明为何更接近理想架构 | aligned | `agent/inner-loop-worklog.md`, `agent/workflow-continuous.md`, `AD-019B` verified | 近几轮 delta 已稳定记录 Root Cause / Minimal Fix / Decision / Split Decision / Reflection / Verification；持续工作流强制每轮判断是否减少架构差距；AD-019B 明确记录不创建 crate 的依赖闭环原因 |

## 使用规则

- 每轮执行后，只更新受影响契约
- 如果一个 delta 被拆分，父 delta 影响的契约状态保持不变，直到子问题分别收敛
- 不允许因为完成了一个 delta，就自动把契约状态提升到 `aligned`
- 若契约状态无法判断，保持 `unassessed`，不要主观乐观升级
