# Architecture Testing Strategy

本文件定义“如何生成自动测试文件，才能与理想架构保持一致”。

目标不是追求测试数量，而是保证测试验证的是**契约**而不是**当前实现细节**。

## 核心原则

1. **测试契约，不测试实现镜像**
   - 不要断言私有调用顺序、临时字段或 helper 细节
   - 要验证职责边界、时序语义、输入输出行为和状态归属

2. **测试跟着 delta 走，不跟着函数走**
   - 每个 delta 只补本轮最小必要测试
   - 不因为顺手改到某个函数，就顺手为它补低价值测试

3. **优先验证最容易回归的架构语义**
   - 职责边界
   - 状态归属
   - 时序/协议
   - 坐标空间一致性

4. **验证层级必须与风险匹配**
   - 局部职责边界：模块级集成测试
   - 协议时序：场景级集成测试
   - DPI/输入链路：系统级测试或最小复现测试

5. **允许不加测试，但必须说清理由**
   - 如果某个 delta 只涉及文档、命名或纯低风险重排，可不补测试
   - 但必须在 worklog/checkpoint 中说明为何本轮不需要新增测试

## 测试层级

### L1 / Contract Unit Test

适用于：

- 局部对象的纯行为契约
- 小范围可稳定隔离的规则

关注点：

- 输入到输出的规则
- 不依赖复杂外部调度

### L2 / Module Integration Test

适用于：

- 一个模块内多个角色之间的职责边界
- UpdateManager / FigureGraph / EventDispatcher 等协作关系

关注点：

- 边界是否回流
- 时序是否符合契约

### L3 / Scenario Integration Test

适用于：

- 结构性变更时序
- 输入到场景图更新的闭环
- 多组件协作行为

关注点：

- 顶层场景是否收敛
- 关键阶段是否按约定执行

### L4 / System Test

适用于：

- DPI/坐标转换
- 原生鼠标/键盘输入
- 平台相关交互

关注点：

- 真实输入链路
- 物理坐标到逻辑坐标再到场景坐标的一致性

## 测试生成规则

### 何时必须补测试

- 当前 delta 改变了职责边界
- 当前 delta 改变了结构性变更时机
- 当前 delta 修复了已知回归风险
- 当前 delta 所在模块已有相邻测试模式可复用

### 何时可不补测试

- 纯文档修改
- 纯命名收口
- 低风险重排，且已有测试已覆盖相关契约

### 何时优先补系统级测试

- DPI/缩放/命中测试
- 原生输入模拟
- 平台调度时机

## 契约映射

| Contract | Failure Mode | Recommended Verification | Suggested Location |
|---|---|---|---|
| `C-03 FigureGraph 持有图级信息` | 图级状态回流到 UpdateManager/EventDispatcher | L2 模块集成测试 | `novadraw-scene/src/scene/*_integration_test.rs` |
| `C-04 UpdateManager 只负责两阶段更新编排` | validation/repair/scheduling 持有图语义 | L2/L3 集成测试 | `novadraw-scene/src/scene/update_integration_test.rs` |
| `C-05 EventDispatcher 只负责分发` | 交互状态归属回流到 dispatcher | L2 集成测试 | `novadraw-scene/src/event/*_test.rs` |
| `C-06 SceneHost 是极薄平台调度层` | SceneHost 吞入业务规则/图状态 | 结构审计 + L3 场景测试 | `apps/editor/src/scene_manager/*_test.rs` |
| `C-07 NovadrawSystem 是组合根` | 装配分散、全局状态回流 | 结构审计 + 少量集成测试 | `apps/editor/src/system.rs` 邻近测试 |
| `C-08 PendingMutation 延迟应用` | 分发期间直接改树、时机错误 | L3 场景集成测试 | `novadraw-scene/src/scene/*mutation*_test.rs` |
| 坐标空间一致性 | physical/logical/scene 坐标错位 | L4 系统测试 | `apps/editor` 系统测试或最小复现场景 |
| Damage / Repair 协定 | 局部重绘与语义更新不一致 | L2/L3 集成测试 | `novadraw-render` / `novadraw-scene` 邻近测试 |

## 文件放置规则

- `novadraw-scene`
  - 放图结构、更新、mutation、事件分发的模块级和场景级测试
- `novadraw-render`
  - 放提交协定、damage、渲染边界相关测试
- `apps/editor`
  - 放系统级交互、平台调度、DPI 与最小复现场景

## 反模式

- 为了提升覆盖率而写“实现复述型”测试
- 断言私有字段或临时 helper 的调用细节
- 把一个大 delta 的所有潜在风险都塞进单轮测试补丁
- 用 mock 完全替代关键协议时序，导致测试失去约束力

## 建议 Prompt

### 为当前 delta 制定测试策略

```text
请基于 agent/quality-testing-strategy.md，为当前 delta 制定最小测试策略。回答这 4 件事：
1. 当前契约是什么
2. 最可能的 failure mode 是什么
3. 应该用哪一层验证（L1/L2/L3/L4）
4. 建议把测试放到哪个文件
```

### 为当前 delta 补最小测试

```text
请基于 agent/quality-testing-strategy.md，为当前 delta 补最小必要测试。测试必须验证契约，不要镜像当前实现；如果你判断本轮不应新增测试，请明确说明原因。
```

## 与工作流的关系

- `governance-architecture-contracts.md` 定义“必须守住的契约”
- 本文件定义“这些契约如何被验证”
- `outer-loop-delta-backlog.yaml` 定义“当前最值得处理的问题”
- `inner-loop-worklog.md` 负责记录“本轮为什么补或不补测试”
