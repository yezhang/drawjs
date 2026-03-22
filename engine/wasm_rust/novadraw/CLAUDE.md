# Novadraw 图形工具包

## 项目概述

使用 Rust + WebGPU 技术栈实现的高性能绘图引擎工具包，参考 eclipse draw2d/GEF 架构设计，目标是扩展为通用图形框架。

### 技术栈

- **渲染后端**: vello (WebGPU)
- **窗口/事件**: winit
- **文本渲染**: cosmic-text
- **构建工具**: cargo

### 模块结构

```text
novadraw-core/     - 核心数据类型
novadraw-math/     - 数学运算
novadraw-geometry/ - 几何计算
novadraw-render/   - 渲染抽象
novadraw-scene/    - 场景图、Figure
apps/editor/       - 编辑器示例
```

## 核心禁止事项

- **递归深度限制**：树遍历允许使用递归，深度上限 10,000 层；当性能成为瓶颈时切换迭代方案
- **禁止临时方案**：问题必须从根因解决
- **禁止全局状态**：不使用 Singleton
- **禁止热路径日志**：渲染循环中不打印日志
- **禁止 magic numbers**：业务代码中不使用硬编码数字
- **保留渲染模式切换**：`use_iterative_render` 字段和 I 键切换必须保留

## 提交前检查

```bash
cargo fmt --check && cargo check && cargo clippy -- -D warnings && cargo test
```

## Skills 与文档

### 可用 Skills

- `/analyzing-gef-code` - 分析 draw2d/GEF 源码
- `/analyzing-xilem-code` - 参考 xilem GUI 框架
- `/analyzing-swt-code` - 分析 SWT GC 底层 API
- `/novadraw-impl` - 项目实现指导
- `/code-review` - 代码审查
- `/simplify` - 代码简化审查

### 详细文档

完整文档见 [doc/00-index.md](doc/00-index.md)。

| 场景 | 文档 |
|------|------|
| Figure 开发 | [doc/02-figure/](doc/02-figure/) |
| 渲染管线 | [doc/03-rendering/](doc/03-rendering/) |
| 架构设计 | [doc/01-architecture/](doc/01-architecture/) |
| 坐标系 | [doc/04-coordinates/](doc/04-coordinates/) |

## 交互方式

### 思维原则

运用第一性原理思考，拒绝经验主义和路径盲从。不要假设我完全清楚目标，保持审慎。若目标模糊请停下讨论，若目标清晰但路径非最优，请直接建议更短、更低成本的办法。

### 回答格式

所有回答分为两部分：

- **直接执行**：按要求直接给出任务结果
- **深度交互**：审慎挑战需求动机、XY问题、路径弊端，给出替代方案

### 行为准则

- **重大架构变更**：先讨论方案，确认后再实现
- **Bug 修复**：先理解根因，解释后再修复
- **代码改动**：超过 50 行分步提交
- **新增功能**：先定义接口，再迭代实现
- **性能优化**：提供基准数据佐证

## 调试技巧

- 使用 `--screenshot` 参数保存渲染结果
- 使用 `mcp__MiniMax__understand_image` 分析截图
- 只在处理渲染主循环时分析 `render_iterative.rs`
- 背景色 RGB(238, 238, 238)，图形颜色应避免与此重复

## 参考代码

- draw2d/GEF: `/Users/bytedance/Documents/code/GitHub/gef-classic`
- SWT GC: `/Users/bytedance/Documents/code/GitHub/eclipse.platform.swt`
- vello: `/Users/bytedance/Documents/code/GitHub/vello`
- xilem: `/Users/bytedance/Documents/code/GitHub/xilem`
