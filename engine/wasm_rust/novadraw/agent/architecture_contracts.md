# Architecture Contracts

本文件将 `doc/理想架构设计.md` 压缩为 Agent 可稳定消费的硬约束。每轮架构改进必须优先满足这些契约。

## 角色边界

1. `Figure` 只负责内在能力，不持有 `parent/children/focus/capture/update manager/platform` 等外部关系。
2. `FigureBlock` 是节点运行时状态容器，承载与图节点生命周期绑定的状态。
3. `FigureGraph` 持有树关系、交互状态和命中测试所需的图级信息。
4. `UpdateManager` 是系统服务，只负责两阶段更新编排，不持有业务级图状态语义。
5. `EventDispatcher` 是系统服务，只负责事件分发，不吞并图状态归属。
6. `SceneHost` 是极薄的平台调度层，不承载业务规则和图状态。
7. `NovadrawSystem` 是组合根，负责装配 `FigureGraph`、`UpdateManager`、`EventDispatcher` 和 `SceneHost`。

## 变更规则

8. 结构性变更必须通过 `PendingMutation` 延迟应用，不直接在分发过程中破坏图结构。
9. 新接口优先表达职责边界，不为兼容现状引入职责回流。
10. 任何架构改动都要说明它比现状更接近理想架构的原因。

## 工程约束

11. 禁止临时方案，优先修根因。
12. 禁止全局状态和 Singleton。
13. 渲染热路径禁止日志。
14. 业务代码禁止 magic numbers。
15. 若树遍历可能触发性能问题，优先评估迭代替代方案。

## 执行约束

16. 一次只处理一个 `Architecture Delta`。
17. 改动超过 50 行时，优先拆分步骤并保持提交粒度清晰。
18. 先解释根因，再改代码。
19. 没有验证结果，不得将 delta 标记为完成。
20. 如发现文档契约不完整或相互冲突，停止编码，先补契约或澄清。
