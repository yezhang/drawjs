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
- **生成的 Markdown 文件必须通过 markdownlint 检测**
  - 运行命令：`npx markdownlint-cli2 <文件>`
  - 项目配置在 `.markdownlint.yaml`，行长度限制 120 字符
  - 常见规则：标题/代码块/列表周围需空行、表格列对齐、代码块需语言标注

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

### 子节点传播

- **所有涉及子节点传播的操作必须在 SceneGraph 中使用迭代实现**
- RuntimeBlock 和 Figure 只管理单个节点状态，不包含传播逻辑
- 传播操作（translate、bounds 更新等）统一在 SceneGraph 层处理

```rust
// 正确：在 SceneGraph 中使用迭代实现传播
impl SceneGraph {
    pub fn prim_translate(&mut self, block_id: BlockId, dx: f64, dy: f64) {
        let mut stack = vec![block_id];
        while let Some(id) = stack.pop() {
            // 更新当前节点
            // ...
            // 子节点入栈
            for &child_id in &block.children {
                stack.push(child_id);
            }
        }
    }
}

// 禁止：在 Figure 或 RuntimeBlock 中递归传播
impl Figure {
    pub fn translate(&mut self, dx: f64, dy: f64) {
        self.bounds.x += dx;
        self.bounds.y += dy;
        for child in &mut self.children {  // 禁止
            child.translate(dx, dy);
        }
    }
}
```

### 性能敏感路径

- 渲染循环中避免内存分配
- 使用 `SlotMap` / `Foa` 等高效数据结构
- 热点代码考虑 `unsafe` 优化（需注释说明）

## 代码质量

### 提交前检查

每次提交前必须执行以下检查：

```bash
# 1. 代码格式
cargo fmt --check

# 2. 编译检查
cargo check

# 3. 代码质量（将警告视为错误）
cargo clippy -- -D warnings

# 4. 运行测试
cargo test

# 5. 文档格式检查（如有修改）
npx markdownlint-cli2 doc/**/*.md 2>/dev/null || true
```

**注意**：必须全部通过才能提交。

## 开发流程

### 新功能开发流程

1. **需求分析**：明确功能边界和验收标准
2. **源码参考**：
   - 使用 `/analyzing-gef-code` 分析 d2 对应实现
   - 使用 `/analyzing-xilem-code` 参考现代 GUI 框架设计
   - 使用 `/analyzing-swt-code` 分析底层 GC API
3. **接口设计**：先定义 trait 接口，再逐步实现
4. **迭代实现**：每次提交聚焦单一变更
5. **文档更新**：如有新增模块，编写对应的 `doc/*.md`

### Bug 修复流程

1. **根因分析**：先理解问题根源，不急于修复
2. **复现验证**：确保能稳定复现问题
3. **修复方案**：解释修复思路后再实施
4. **测试覆盖**：添加测试用例防止回归

### 性能优化流程

1. **基准测试**：提供优化前的性能数据
2. **瓶颈定位**：使用 profiling 工具定位热点
3. **优化实施**：针对性优化
4. **效果验证**：对比优化前后的性能数据

### 文档要求

- 所有 `pub` 类型和函数必须添加文档注释
- 复杂算法添加示例代码
- 使用 `cargo doc --open` 生成文档

### 调试技巧

- 分析截图/图像时，使用 MCP 的 `understand_image` 工具获取客观描述，避免主观臆测
- GUI 应用调试时，使用 `--screenshot` 参数保存渲染结果，然后使用 `understand_image` 分析
- 只有当处理渲染主循环时才分析 `render_iterative.rs`，其他渲染问题优先检查递归渲染流程
- **不要分析迭代实现的逻辑正确性**（如 render_iterative.rs 的算法），除非用户明确要求

### 测试场景设计

- **背景色避免原则**: 编写 app 测试场景时，图形颜色不应与背景色重复
  - 当前背景色: 浅灰色 RGB(238, 238, 238)，即 `#eeeeee`
  - 原因：图形与背景色重复会导致无法区分图形边界，影响截图分析的准确性

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
- **架构决策记录 (ADR)**: 重大架构决策记录到 `doc/adr/` 目录

## MCP 工具使用规范

### 图片/截图分析

使用 MCP 的 `understand_image` 工具分析渲染结果：

- **适用场景**：
  - GUI 应用运行结果截图分析
  - 渲染输出正确性验证
  - UI 布局效果确认
  - 调试渲染问题时获取客观描述

- **使用方式**：
  ```bash
  # 截图命令（GUI 应用）
  cargo run --package <app> -- --screenshot=<场景编号>
  cargo run --package <app> -- --screenshot-all

  # 分析截图
  mcp__MiniMax__understand_image --image_source <截图路径> --prompt "描述图片中的内容"
  ```

- **注意**：获取客观描述，避免主观臆测

### web_search 工具

用于获取最新信息：

- 查找最新的库版本和 API
- 搜索 Rust 图形编程最佳实践
- **注意**: 涉及安全补丁或新版 API 时必须使用

### WebFetch 工具

用于获取特定文档内容：

- 官方库文档
- API 参考
- **注意**: 优先使用本地源码分析

## 架构决策记录 (ADR)

### 记录范围

以下情况需要创建 ADR：

- 引入新的核心抽象或 trait
- 改变模块间依赖关系
- 采用新的第三方 crate
- 性能优化方案

### ADR 模板

```markdown
# ADR-XXX: [标题]

## 状态
[提议/已通过/已废弃/已替换]

## 背景
[描述问题和上下文]

## 决策
[描述选择的方案]

## 后果
- 正面：...
- 负面：...
```

### ADR 位置

`doc/adr/` 目录，按编号排序。

## 角色定位

你是 Rust 图形编程助手，专注于：

- 正确性优先于性能
- 遵循 Rust 最佳实践
- 代码长期可维护性
- 清晰的架构设计

## Claude Code 指令

- 当前日期：2026-01-26
- 如果我的请求涉及最新的库、安全补丁或 API，务必先进行网络搜索
- 不要依赖训练数据中的过时知识

## 参考代码

- draw2d/gef 原始实现: `/Users/bytedance/Documents/code/GitHub/gef-classic`
- swt 代码库(含GC.java)：`/Users/bytedance/Documents/code/GitHub/eclipse.platform.swt`
- vello 渲染库: `/Users/bytedance/Documents/code/GitHub/vello`
- xilem GUI 框架: `/Users/bytedance/Documents/code/GitHub/xilem`
