# Demo 与验证矩阵

> 本文档承载每个 draw2d 核心 milestone **配套的 demo + 验证策略**。
>
> **编号唯一来源**：`agent/draw2d-core-milestones.yaml`，本文不发明独立编号。
> **与 product-deliverables.md 的关系**：产品清单回答 *what to ship*，本文回答 *how to verify*。

## 验证分层

每个 milestone 完成（YAML `complete` 状态）必须三层验证齐过：

| 层 | 来源 | 度量 |
|----|------|------|
| **契约层** | YAML `probes` | 单元测试、契约属性测试 |
| **产品层** | `product-deliverables.md` 清单 | 类型/能力存在性测试 |
| **端到端层** | 本文 demo + 验证策略 | demo 截图断言 / 帧率断言 / 集成测试 |

YAML 状态 `behavior_verified` 需要前两层过；`complete` 需要三层全过。

## Demo 矩阵

| YAML | Demo 名称 | 路径 | 验证策略 | 测试增量预期 |
|------|-----------|------|----------|--------------|
| M1 | 无独立 demo | — | 仅类型单测 + Graphics 状态栈嵌套测试 | +30 |
| M2 | `shapes-demo` | `apps/shapes-demo` | 5 图元 + 三段式 paint 分层可视化 + 截图断言 | +60 |
| M3 | 集成入 `shapes-demo` | `apps/shapes-demo` | 嵌套裁剪测试 + 递归/迭代等价测试（同输入→同 RenderCommand 序列） | +40 |
| M4 | `coordinates-demo` | `apps/coordinates-demo` | 深层嵌套坐标转换 + 坐标根移动 + 入口域降域可视化 | +50 |
| M5 | `layouts-demo` + `update-stress-demo` | `apps/layouts-demo`、`apps/update-stress-demo` | 6 布局并排对比 + draw2d 反向等价测试；三种失效粒度 + 1k+ Figure 帧率断言 | +250 |
| M6 | `events-demo` | `apps/events-demo` | 4 类监听 + hit-test 全图元 + capture/focus 状态机断言 | +100 |
| M7 | 集成入 `events-demo` + `update-stress-demo` | 同上 | bounds 变化触发 `figureMoved`；坐标根移动触发 `coordinateSystemChanged`；UpdateManager 触发 validating/painting 通知 | +80 |
| M8 | `scroll-pane-demo` + 收口 `viewport-app` | `apps/scroll-pane-demo`、`apps/viewport-app` | ScrollBar + 滚轮 + autoexpose；AD-018 暂停的 4 场景视觉验证全过 | +120 |
| M9 | `connections-demo` | `apps/connections-demo` | 5 anchor × 3 router 组合矩阵 + 节点移动连线跟随测试 | +150 |
| M10 | `borders-demo` + `text-image-demo` + `tooltip-demo` | `apps/borders-demo`、`apps/text-image-demo`、`apps/tooltip-demo` | 6 边框 + 文本布局 + Tooltip 悬停延迟 + Accessible 键盘可达性 | +220 |

**测试增量合计**：+1,100（基线 146，目标 ~1,250）

## 通用验证规范

### 截图断言

- 工具：`--screenshot` 参数（见 CLAUDE.md）
- 比对：`mcp__MiniMax__understand_image` 分析
- 背景色 RGB(238, 238, 238)，图形颜色禁止与此重复

### 帧率断言

仅 M5 `update-stress-demo` 与 M8 滚动 demo 需要：

- 1k+ Figure 全量更新 ≥ 30fps
- 局部失效 ≥ 60fps（部分场景）

> 注：Year 1 不强求所有 demo 都达 60fps，符合 CLAUDE.md "扩展性 > 稳定性 > 性能" 原则。

### 等价测试

- M3 递归/迭代等价：同输入下两条渲染主循环输出**相同** `RenderCommand` 序列
- M5 draw2d 反向等价：本项目 6 布局的输出与 g2 同输入下的 `bounds` 结果**位级一致**或在 ±1px 容差内
- 路径：`novadraw-scene/tests/` + `novadraw-scene/benches/`

### 阻塞收口规则

每个 demo 启动前必须确认依赖 milestone 已 `behavior_verified`。已知阻塞：

- **AD-018**：`apps/viewport-app` 4 场景视觉验证暂停 → 必须随 M8 `scroll-pane-demo` 一起关闭

## 状态同步规则

每个 demo 完成时：

1. 在本文对应行追加 ✅ 标记
2. 同步更新 `agent/goal-roadmap.md` 状态快照
3. 同步更新 YAML 对应 milestone 的 `status`（按 5 状态机推进）

## Demo 完成清单（勾选区）

- [ ] M2 `apps/shapes-demo`
- [ ] M4 `apps/coordinates-demo`
- [ ] M5 `apps/layouts-demo`
- [ ] M5 `apps/update-stress-demo`
- [ ] M6 `apps/events-demo`
- [ ] M8 `apps/scroll-pane-demo`
- [ ] M8 `apps/viewport-app` 4 场景视觉验证（AD-018 收口）
- [ ] M9 `apps/connections-demo`
- [ ] M10 `apps/borders-demo`
- [ ] M10 `apps/text-image-demo`
- [ ] M10 `apps/tooltip-demo`

---

## 附录 A：GEF 层早期探索（非 draw2d 核心）

> ⚠️ 以下 demo **不计入 draw2d 核心 milestone 完成判据**，不挂在 M1-M10 任何项下。
> 它们用于验证 draw2d 协议层的承载能力，但其能力本身（创建/拖拽/连接/删除节点等）属于 GEF 层。

### 节点编辑器探索 demo

- **路径**：`apps/node-editor-demo`（暂定）
- **触发时机**：M1-M10 全部 `behavior_verified` 之后
- **能力范围**：创建节点 / 拖拽 / 连接 / 删除 / 滚动+缩放 / Tooltip
- **验证目的**：
  - draw2d 核心协议在端到端编辑场景下是否仍自洽
  - 暴露未来 GEF 层的需求点（Tool / Command / Request 在何处自然涌现）
- **不验证目的**：
  - ❌ 不作为 draw2d 核心毕业判据
  - ❌ 不强求 60fps / 100 节点性能基线
  - ❌ 不允许为通过本 demo 而修改 draw2d 核心协议

### 边界守门

如果探索过程中发现协议层缺口，正确做法：

1. 在本文档附录记录缺口
2. 起一个新的 delta（在 YAML 内）
3. 通过 contract probe 把缺口收口
4. **禁止**为单独让 demo 跑通而在 apps 层堆便利方法

如果未来确认要做 GEF 层，新开 `doc/07-gef-roadmap/` 目录承载，本附录迁移过去。
