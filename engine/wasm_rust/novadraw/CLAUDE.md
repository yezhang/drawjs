# Novadraw 图形工具包

## 项目概述

使用 Rust + WebGPU 技术栈实现的高性能绘图引擎工具包，参考 eclipse draw2d/GEF 架构设计，目标是扩展为通用图形框架。
为了简便，我会使用 d2 指代 draw2d。

## 技术栈

- **渲染后端**: vello (WebGPU)，考虑扩展性，未来支持替换为其他后端
- **窗口/事件**: winit，仅作为技术验证使用
- **文本渲染**: cosmic-text
- **构建工具**: cargo

## 架构设计

### 模块划分

| crate             | 职责                                          |
| ----------------- | --------------------------------------------- |
| `novadraw-core`   | 核心数据类型（Color, Point, Rect, Transform） |
| `novadraw-scene`  | 场景图、Figure 接口、布局管理                 |
| `novadraw-render` | 渲染上下文、Backend 抽象                      |
| `novadraw-math`   | 数学运算（向量、矩阵）                        |
| `apps/editor`     | 示例编辑器应用                                |

### 核心原则

- Figure 只负责渲染接口和几何定义，不包含状态
- 运行时状态（可见性、选中状态、层次关系）由 RuntimeBlock 管理
- 渲染与逻辑分离，支持海量图形高效渲染

## 代码约定

### 命名规范

- 类型/ trait: `PascalCase`
- 函数/变量: `snake_case`
- 常量: `SCREAMING_SNAKE_CASE`
- 泛型参数: `T`, `U` 等单大写字母

### 文档规范

- Markdown 标题（`#`、`##`、`###`、`####`）前后各留一行空行
- 列表项之间不添加额外空行
- 代码块使用 `rust` 标注语言

### 可见性

- 公开 API 需添加文档注释 `///`
- 内部实现细节保持私有
- 使用 `pub(crate)` 限制 crate 内部共享

### 错误处理

- 关键错误使用 `thiserror` 定义自定义错误类型
- 预期内的"空状态"返回 `Option`
- 避免使用 `unwrap()` / `panic!`

### 测试

- 核心算法必须添加单元测试
- 使用 `cargo test` 验证
- 复杂逻辑添加集成测试

## 算法要求

### 树遍历

- **禁止使用函数递归** 实现树遍历
- 必须使用**堆栈迭代** 替代递归逻辑
- 目的：避免深度层次结构导致堆栈溢出

```rust
// 正确示例
let mut stack = vec![root_id];
while let Some(id) = stack.pop() {
    // 处理节点
    for &child in &block.children {
        stack.push(child);
    }
}

// 禁止示例
fn visit(node: &Node) {
    // ...
    for child in &node.children {
        visit(child);  // 禁止
    }
}
```

### 性能敏感路径

- 渲染循环中避免内存分配
- 使用 `SlotMap` / `Foa` 等高效数据结构
- 热点代码考虑 `unsafe` 优化（需注释说明）

## 代码质量

### 提交前检查

```bash
cargo fmt      # 代码格式
cargo check    # 编译检查
cargo clippy   # 代码质量建议
cargo test     # 运行测试
```

### 文档要求

- 所有 `pub` 类型和函数必须添加文档注释
- 复杂算法添加示例代码
- 使用 `cargo doc --open` 生成文档

## 禁止事项

- 不创建临时方案绕过问题
- 不引入未经验证的第三方 crate
- 不在渲染/热路径中打印日志
- 不使用全局状态或 Singleton
- 不在业务代码中硬编码magic numbers

## 交互方式

- **重大架构变更**: 先提出方案讨论，获得确认后再实现
- **Bug 修复**: 先理解根因，解释后再修复
- **代码改动**: 超过 50 行分步提交，每次提交聚焦单一变更
- **新增功能**: 先定义接口，再逐步实现
- **性能优化**: 提供基准测试数据佐证效果

### 设计决策原则

- **渐进式设计**: 遇到需要决策的问题时，评估该选择在后期是否容易渐进式添加
  - 若后期添加成本高或难以渐进式扩展：提前设计好
  - 若可平滑增量添加：留待后续迭代
- **接口先行**: 对于不确定的实现细节，先定义接口契约，延迟具体实现

## 角色定位

你是 Rust 图形编程助手，专注于：

- 正确性优先于性能
- 遵循 Rust 最佳实践
- 代码长期可维护性
- 清晰的架构设计

## Claude Code 指令

- 当前日期：2026-01-26
- 如果我的请求涉及最新的库、安全补丁或 API，务必先进行网络搜索
- 不要依赖 2024 年之后的内部知识

## 参考代码

- draw2d/gef 原始实现: `/Users/bytedance/Documents/code/GitHub/gef-classic`
- vello 渲染库: `/Users/bytedance/Documents/code/GitHub/vello`
