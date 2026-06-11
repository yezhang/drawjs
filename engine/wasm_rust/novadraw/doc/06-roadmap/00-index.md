# 06-Roadmap 路线图

本目录承载 Novadraw 朝 draw2d 核心演进的**产品视图**与 **demo 视图**，与 `agent/draw2d-core-milestones.yaml`（协议视图）和 `agent/goal-roadmap.md`（状态快照）三方协同。

## 三方职能边界

| 文件 | 职能 | 性质 | 更新频率 |
|------|------|------|----------|
| `agent/draw2d-core-milestones.yaml` | **协议视图 SSOT**：milestone 编号（M1-M10）、契约（contracts）、验证项（probes）、依赖（dependencies） | 机器可读，对齐 draw2d 公理 A1-A9 | 极少（契约稳定） |
| `doc/06-roadmap/product-deliverables.md` | **产品视图**：每个 milestone 下要交付的图元数量、布局种类、边框种类等策略层清单 | 人读，启动期定稿 | 启动期一次，后续微调 |
| `doc/06-roadmap/demo-matrix.md` | **验证视图**：每个 milestone 配套的 demo 名称、覆盖范围、截图/帧率断言策略 | 人读，启动期定稿 | 启动期一次，后续微调 |
| `agent/goal-roadmap.md` | **状态快照 + 同步规则**：当前进度、阻塞项、测试增量统计、三方文件同步规则 | 人读，工作流入口 | 每完成一个 milestone 更新 |

## 编号唯一来源

**所有引用 milestone 编号（M1-M10）的地方都以 YAML 为准。**

本目录内任何 milestone 引用必须写明 "M{n} (YAML)"，不允许在 doc 内发明编号。
旧 `agent/goal-roadmap.md` 中存在的独立 M0-M11 编号已废弃。

## GEF 边界

YAML 第 391-400 行的 `excluded_gef_capabilities` 是本项目核心目标的边界：

- EditPart / EditPolicy / Tool / Command / Request / Viewer / Palette / Selection provider / Undo-redo command stack

这些不在 draw2d 核心里程碑内。带"节点编辑器"性质的 demo 视为 GEF 层早期探索，详见 `demo-matrix.md` 附录。

## 文档列表

| 文档 | 主题 |
|------|------|
| `product-deliverables.md` | 每个 milestone 下要交付的产品策略层清单 |
| `demo-matrix.md` | 每个 milestone 对应的 demo + 验证矩阵 |
