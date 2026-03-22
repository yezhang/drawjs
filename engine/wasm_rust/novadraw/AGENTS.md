# AGENTS.md

## 跨 Agent 兼容性说明

本项目主要使用 **CLAUDE.md** 作为项目上下文配置（针对 Claude Code 优化）。

如使用其他 Agent 工具（Cursor、OpenCode、Zed 等），关键约定如下：

### 核心规则

| 规则 | 说明 |
|------|------|
| 树遍历 | 递归深度限制 10,000 层，性能瓶颈时切换迭代 |
| 禁止临时方案 | 问题必须从根因解决 |
| 禁止全局状态 | 不使用 Singleton |
| 渲染热路径 | 不打印日志 |
| 硬编码 | 业务代码中不使用 magic numbers |

### 详细文档

完整文档见 [doc/00-index.md](doc/00-index.md)。

### 项目特性

- **语言**: Rust (Edition 2024)
- **渲染**: Vello (WebGPU)
- **构建**: `cargo build && cargo test`
- **模块**: novadraw-core, novadraw-scene, novadraw-render, novadraw-math

### 详细约定

详细代码规范、提交流程、开发流程见 **CLAUDE.md**。
