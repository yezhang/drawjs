# Goal Roadmap 状态快照

> 工作流入口：**当前进度 + 阻塞项 + 三方同步规则**。
> 不持有独立 milestone 编号、不持有产品策略、不持有 demo 策略。
>
> - 协议契约（编号 SSOT）：`agent/draw2d-core-milestones.yaml`
> - 产品清单：`doc/06-roadmap/product-deliverables.md`
> - Demo 矩阵：`doc/06-roadmap/demo-matrix.md`

## 原目标（来自 CLAUDE.md）

> "使用 Rust + WebGPU 技术栈实现的高性能绘图引擎工具包，参考 eclipse draw2d/GEF 架构设计，**目标是扩展为通用图形框架**。"

## Year 1 主题

- **核心收口**：YAML M1-M10 全部 `complete`
- **测试增量目标**：+1,100（基线 146 → ~1,250），按 `doc/06-roadmap/product-deliverables.md` 拆分
- **GEF 层探索**：`apps/node-editor-demo` 视为附录探索，**不在核心毕业判据**（见 `demo-matrix.md` 附录 A）

## 三方文件同步规则

每完成一个 YAML milestone 必须按顺序更新：

1. **YAML**（`agent/draw2d-core-milestones.yaml`）—— `status` 字段按 5 状态机推进（`not_started → in_progress → contract_aligned → behavior_verified → complete`）
2. **Demo 矩阵**（`doc/06-roadmap/demo-matrix.md`）—— 勾选 Demo Completion Checklist 对应行
3. **本文**（`agent/goal-roadmap.md`）—— 更新"状态快照"小节
4. **可选**：若涉及阻塞项，记录到本文"阻塞项"与 `agent/interruptions-inbox.md`

**编号规则**：本文及任何 delta、PR、commit 信息引用 milestone 编号一律写 `M{n}`，指 YAML M{n}。不允许出现"本文 M{n}"或"MD M{n}"的歧义说法。

## 状态快照（2026-06-11）

### 总览

| 维度 | 当前 | 目标 |
|------|------|------|
| 完成 milestone 数 | 0 / 10 | 10 / 10 |
| 总测试数 | 146 | ~1,250 |
| 已 verify demo 数 | 0 / 11 | 11 / 11 |

### Milestone 状态

| YAML | 标题 | 当前状态 | 备注 |
|------|------|----------|------|
| M1 | 几何与 Graphics 基础 | `contract_aligned` | M1 probes summary 已确认 geometry operation、Graphics state stack、clip/transform snapshot、text/image/alpha snapshot 均有自动化证据；尚未进入 `behavior_verified` |
| M2 | Figure 树与盒模型 | `not_started` | — |
| M3 | 绘制遍历与裁剪闭环 | `not_started` | — |
| M4 | 坐标域与变换闭环 | `not_started` | — |
| M5 | Layout + Validation + UpdateManager | `not_started` | — |
| M6 | 事件分发与交互状态机 | `not_started` | — |
| M7 | 通知语义分层 | `not_started` | — |
| M8 | Viewport / Scroll / Zoom | `not_started` | AD-018 视觉验证暂停，需随本 milestone 收口 |
| M9 | Connection / Anchor / Router | `not_started` | — |
| M10 | 常用 Figure 与文本/控件 | `not_started` | — |

### 阻塞项

- **INBOX-20260610-01**：`apps/viewport-app` 视觉验证暂停 → 收口入 M8
- 其他无

## 废弃说明（迁移记录）

本文先前持有独立 M0-M11 编号、产品清单、demo 清单、Parity Checklist 等内容，已于 2026-06-11 拆分迁移：

| 旧内容 | 新位置 |
|--------|--------|
| 独立 M0-M11 编号 | **废弃**，统一引用 YAML M1-M10 |
| 产品策略清单（5 图元/6 布局/6 边框等） | `doc/06-roadmap/product-deliverables.md` |
| Demo Checklist + 验证策略 | `doc/06-roadmap/demo-matrix.md` |
| Draw2d Parity Checklist | 合并入 `demo-matrix.md` Demo Completion Checklist |
| M11 节点编辑器毕业 demo | `demo-matrix.md` 附录 A "GEF 层早期探索"（非核心） |

旧 MD/YAML M10 含义冲突问题随本次迁移自然消解（MD 不再持有 M10）。
