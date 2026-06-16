# Integration Test Plan

本文件定义 Novadraw 面向 Draw2D 核心能力的集成测试方案。

目标不是补更多截图 demo，也不是为通过而设计伪测试，而是建立一组能够持续验证以下闭包的可交互、可渲染、可脚本回放的场景：

- 几何闭包：`bounds` / `insets` / `clientArea` / `containsPoint` / `intersects`
- 坐标闭包：`useLocalCoordinates` / `translate*` / `coordinateSystemChanged`
- 更新闭包：`invalidate` / `revalidate` / `repairDamage` / notification flush
- 交互闭包：hit-test / hover / capture / focus / keyboard dispatch

## 1. 定位

- 文件类型：阶段性执行计划
- 所属层级：`agent/`
- 当前状态：planned
- 演化方向：后续逐步落地为 `apps/integration-app/`

之所以先放在 `agent/`，是因为这份内容当前仍是执行蓝图，而不是已经稳定收敛的正式文档。后续随着 `integration-app`、脚本回放和截图基线落地，再将摘要同步到 `apps/README.md`。

## 2. 设计原则

### 2.1 测试契约，不测试实现镜像

- 不断言私有 helper 调用顺序
- 不用 `commands.len() > 0` 之类的弱断言冒充验证
- 不把“收到过日志”当成行为正确

### 2.2 以公理为入口，而不是以模块名为入口

场景设计必须回到 `doc/01-architecture/draw2d_design_axioms.md` 中的系统公理，重点覆盖：

- A2 / C2：树即运行时骨架
- A3 / C3：bounds 统一几何真相
- A4 / C4：坐标根与父链坐标转换
- A5 / C5：client area 盒模型
- A6 / C6：几何变更协议
- A7 / C7：两阶段更新事务
- A8 / C8：damage 父链传播
- A9 / C9：统一事件状态机

同时，通知相关验证需遵守 `doc/adr/adr-002-notification-effect-queue.md`：

- 通知语义必须分层
- `CoordinateSystemChanged` 是一等事件
- 状态修改期间不执行外部 listener
- flush 必须位于稳定事务边界

### 2.3 每个场景同时输出三类结果

- 视觉结果：用户可见的渲染效果或截图
- 行为结果：interaction / update / notification trace
- 状态结果：target/focus/capture/selection/damage 等最终快照

## 3. 放置策略

### 3.1 最终代码位置

- 目标目录：`apps/integration-app/`

### 3.2 当前计划位置

- 计划文件：`agent/quality-integration-test-plan.md`

### 3.3 不直接复用现有 app 作为最终载体的原因

- `apps/editor/`：更偏诊断型集成示例，职责较重
- `apps/update-app/`：适合观察更新链路，不是完整交互渲染测试入口
- `apps/event-app/`：当前更像静态主题样例，尚未形成契约级验证能力
- `apps/transform-app/` / `apps/clip-app/`：目前更偏展示，不足以承载严格集成测试

## 4. 集成测试载体形态

`apps/integration-app/` 规划为一个统一的场景测试宿主，而不是多个分散 demo。

每个场景都应支持三种模式：

- 交互模式：人工操作，看实时效果
- 脚本模式：固定输入序列自动回放
- 截图模式：导出截图用于视觉回归

每个场景都应具备四类基础能力：

- 场景注册与切换
- 原始输入脚本回放
- trace 收集与显示
- 最终状态断言与截图导出

## 5. 首批核心场景

以下场景不是“可选示例”，而是对 Draw2D 核心能力的第一批契约验证入口。

### 5.1 场景 S1：Z-Order 与命中一致性

- 对应公理：A2 / C2
- 要验证的能力：
  - 子顺序决定渲染层级
  - hit-test 逆序遍历子节点
  - 视觉上层与交互命中目标一致
- 交互形式：
  - 两个重叠 figure
  - 点击重叠区域
- 视觉断言：
  - 上层 figure 高亮
  - 下层不应被选中
- 行为断言：
  - target id 必须是后添加者

### 5.2 场景 S2：Bounds 统一几何真相

- 对应公理：A3 / C3
- 要验证的能力：
  - `bounds` 同时参与绘制、命中、repaint、damage
  - 边界点击与可见区域一致
- 交互形式：
  - 点击 figure 边缘
  - 触发移动与 repaint
- 视觉断言：
  - 旧区域被正确擦除
  - 新区域与命中区域重合
- 行为断言：
  - dirty rect 与 bounds 同域

### 5.3 场景 S3：坐标根切换与局部坐标分段

- 对应公理：A4 / C4
- 要验证的能力：
  - `useLocalCoordinates` 创建子树坐标根
  - 父节点移动后，子节点不应被错误逐个平移
  - 事件点必须在 target/source figure 坐标域中解释
- 交互形式：
  - 移动坐标根父节点
  - 点击子节点内部热点
- 视觉断言：
  - 父移动后子节点显示位置正确
- 行为断言：
  - 触发 `CoordinateSystemChanged`
  - 不错误产生整棵子树 `FigureMoved`

### 5.4 场景 S4：ClientArea / Insets / Children Clip

- 对应公理：A5 / C5
- 要验证的能力：
  - `clientArea` 是子布局区和 children clip 区的共同来源
  - 本地坐标模式下 client area 原点重置为 `(0, 0)`
- 交互形式：
  - 容器带 border/insets
  - 子节点部分超出内容区
- 视觉断言：
  - 子节点不能侵入 border/insets 区域
  - 被裁掉区域不可见
- 行为断言：
  - 命中与 children clip 语义一致

### 5.5 场景 S5：几何变更协议

- 对应公理：A6 / C6
- 要验证的能力：
  - `set_bounds()`、`prim_translate()`、resize 触发正确更新链
  - 区分 `FigureMoved` 与 `CoordinateSystemChanged`
- 交互形式：
  - 脚本触发 move / resize / add child / remove child
- 视觉断言：
  - 旧影不残留
  - 新位置渲染正确
- 行为断言：
  - trace 中出现正确的图形事件类型

### 5.6 场景 S6：两阶段更新事务与通知 flush

- 对应公理：A7 / C7
- 对应 ADR：ADR-002 公理 3 / 4 / 5
- 要验证的能力：
  - `Validation -> Repair -> Notification Flush`
  - listener 不读取半稳定状态
- 交互形式：
  - 一次操作同时触发 invalid + dirty
- 视觉断言：
  - 结果稳定，无中间态残影
- 行为断言：
  - update trace 顺序为：
    - `Validating`
    - `Validated`
    - `Painting`
    - `Painted`
  - listener 事件在 repair 之后统一出现

### 5.7 场景 S7：Damage 父链传播

- 对应公理：A8 / C8
- 要验证的能力：
  - dirty rect 先与自身 bounds 相交
  - 沿父链逐层 `translateToParent` 并做 ancestor clip
- 交互形式：
  - 深层嵌套子节点局部 repaint
- 视觉断言：
  - 只更新祖先裁剪链允许的区域
- 行为断言：
  - damage trace 可追溯到完整父链传播结果

### 5.8 场景 S8：统一事件状态机

- 对应公理：A9 / C9
- 要验证的能力：
  - hover / press / drag / release / capture / focus / keyboard
  - capture 建立后绕过普通 hit-test
  - keyboard 事件只投递给 focus owner
- 交互形式：
  - 脚本序列：`move -> press -> drag out -> release -> key`
- 视觉断言：
  - hover 高亮、drag 反馈、focus ring 可见
- 行为断言：
  - capture 生效期间 target 不应被普通命中替换
  - 键盘事件目标与 focus owner 一致

## 6. 验证形式

### 6.1 视觉验证

- 人工观察实时渲染结果
- 截图输出用于回归比对
- 必要时增加 overlay：
  - bounds overlay
  - client area overlay
  - damage overlay
  - hit target overlay

### 6.2 行为验证

- interaction trace
- update trace
- notification trace
- damage propagation trace

### 6.3 状态验证

每个场景结束后应能导出最小状态快照，例如：

- current target
- focus owner
- capture owner
- selected ids
- last damage union
- last update phase sequence

## 7. 首批实现范围

为了避免一次做成大而散的 demo，首批只建议落 3 个高收益场景：

1. S1：Z-Order 与命中一致性
2. S3：坐标根切换与局部坐标分段
3. S6：两阶段更新事务与通知 flush

原因：

- 三者分别锁定树骨架、坐标闭包、更新事务三条主线
- 能最快暴露 draw2d 等价能力是否已经形成闭环
- 便于先把 trace、脚本、截图三套基础设施搭起来

## 8. 分阶段落地计划

### Phase 1：测试宿主骨架

- 新建 `apps/integration-app/`
- 抽象场景注册表
- 接入脚本回放
- 接入 trace 收集
- 接入截图输出

### Phase 2：首批 3 个核心场景

- S1：Z-Order 与命中一致性
- S3：坐标根切换与局部坐标分段
- S6：两阶段更新事务与通知 flush

### Phase 3：补齐几何与交互闭包

- S2：Bounds 统一几何真相
- S4：ClientArea / Insets / Children Clip
- S5：几何变更协议
- S8：统一事件状态机

### Phase 4：补齐更新深水区

- S7：Damage 父链传播
- 后续视项目进度再考虑保留扩展、router / connection 的专项场景

## 9. 验收标准

该方案后续转为真实 `integration-app` 时，至少应满足：

- 能通过命令切换场景
- 能脚本回放真实交互
- 能导出截图
- 能输出 trace
- 至少 3 个核心场景具有明确的视觉、行为、状态断言
- 不依赖热路径日志作为唯一验证手段

## 10. 反模式

以下做法不应进入集成测试方案：

- 把静态展示 app 改名后当作集成测试
- 只验证“程序没崩”
- 只验证“有日志输出”
- 只验证 `commands.len() >= N`
- 只做截图，没有交互和状态 trace
- 把 editor 的诊断逻辑直接复制一份堆到测试入口

## 11. 与现有文件的关系

- 契约来源：`doc/01-architecture/draw2d_design_axioms.md`
- 通知边界：`doc/adr/adr-002-notification-effect-queue.md`
- 测试方法论：`agent/quality-testing-strategy.md`
- 目录职责摘要：`apps/README.md`

本文件回答“要测什么、为什么这样测、先做哪些”；真正的代码落地与运行入口以后续 `apps/integration-app/` 为准。
