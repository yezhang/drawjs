# UpdateManager 设计文档

## 概述

本文档描述 Novadraw 的 UpdateManager 实现，参考 Eclipse Draw2D (g2) 的设计，并说明从 Java 到 Rust 的迁移决策。

## g2 UpdateManager 机制

### 核心组件

| 组件 | g2 类 | 职责 |
|------|-------|------|
| 更新管理器 | `UpdateManager` | 抽象基类 |
| 延迟更新管理器 | `DeferredUpdateManager` | 具体实现，批量处理更新 |
| 脏区域 | `dirtyRegions: Map<IFigure, Rectangle>` | 需要重绘的区域 |
| 失效队列 | `invalidFigures: List<IFigure>` | 需要重新布局的图形 |
| 更新标志 | `updateQueued: boolean` | 是否有待处理更新 |

### 两阶段更新流程

```text
┌─────────────────────────────────────────────────────────────────────────────┐
│                         g2 UpdateManager 流程                                │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  Figure.repaint() ──────────► UpdateManager.addDirtyRegion()                │
│       │                                      │                              │
│       │                                      ▼                              │
│       │                           ┌─────────────────────┐                  │
│       │                           │  dirtyRegions Map   │                  │
│       │                           │  (figure -> rect)  │                  │
│       │                           └─────────────────────┘                  │
│       │                                      │                              │
│       │                               queueWork()                          │
│       │                                      │                              │
│  Figure.revalidate() ───────► UpdateManager.addInvalidFigure()              │
│       │                                      │                              │
│       │                                      ▼                              │
│       │                           ┌─────────────────────┐                  │
│       │                           │  invalidFigures     │                  │
│       │                           │  (List<IFigure>)    │                  │
│       │                           └─────────────────────┘                  │
│       │                                      │                              │
│       │                               queueWork()                          │
│       │                                      │                              │
│       ▼                                                                    │
│  performUpdate() ────────────────► Phase 1: performValidation()             │
│                                              │                              │
│                                              ▼                              │
│                                    ┌─────────────────────┐                │
│                                    │  Phase 2: repair   │                │
│                                    │  Damage()           │                │
│                                    │  - 合并脏区域       │                │
│                                    │  - 坐标变换到父节点 │                │
│                                    │  - root.paint()    │                │
│                                    └─────────────────────┘                │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

### repairDamage 核心算法

g2 的 `repairDamage` 有一个关键逻辑：**脏区域坐标变换到父节点**。

```java
// g2 DeferredUpdateManager.java
protected void repairDamage() {
    oldRegions.forEach((figure, contribution) -> {
        IFigure walker = figure.getParent();
        // 脏区域与当前 figure bounds 取交集
        contribution.intersect(figure.getBounds());

        // 向上遍历父节点，变换坐标并取交集
        while (!contribution.isEmpty() && walker != null) {
            walker.translateToParent(contribution);  // 坐标变换到父节点
            contribution.intersect(walker.getBounds());  // 与父节点 bounds 取交集
            walker = walker.getParent();
        }

        // 累加到总 damage
        damage.union(contribution);
    });
}
```

这个设计的目的是：脏区域需要逐级向上传播，只有在每个祖先节点的 bounds 范围内的部分才需要重绘。

## 本项目实现

### 核心概念映射

| g2 概念 | 本项目实现 | 说明 |
|----------|-----------|------|
| `UpdateManager` | `SceneUpdateManager` | 更新管理器 |
| `dirtyRegions` (Map) | `dirty_regions` (HashMap) | 脏区域映射 |
| `invalidFigures` (List) | `invalid_blocks` (Vec) | 失效块队列 |
| `updateQueued` | `update_queued` | 是否有待处理更新 |
| `addDirtyRegion()` | `add_dirty_region()` | 添加脏区域 |
| `addInvalidFigure()` | `add_invalid_figure()` | 添加失效块 |
| `performUpdate()` | `perform_update()` | 执行两阶段更新 |

### 数据结构

```rust
// novadraw-scene/src/update/deferred.rs

pub struct SceneUpdateManager {
    /// 脏区域映射：block_id -> 脏区域
    pub(crate) dirty_regions: std::collections::HashMap<BlockId, Rectangle>,

    /// 失效块队列
    pub(crate) invalid_blocks: Vec<BlockId>,

    /// 是否有更新待处理
    pub(crate) update_queued: bool,

    /// 合并脏区域时的扩展边距
    expand_margin: f64,
}
```

### SceneGraph 集成

```rust
// novadraw-scene/src/scene/mod.rs

pub struct SceneGraph {
    // ... 其他字段
    pub update_manager: super::update::SceneUpdateManager,
}
```

### 公开 API

```rust
impl SceneGraph {
    /// 标记块需要重新布局
    pub fn mark_invalid(&mut self, block_id: BlockId);

    /// 请求重绘指定块
    pub fn repaint(&mut self, block_id: BlockId, rect: Option<Rectangle>);

    /// 请求重绘整个场景
    pub fn repaint_all(&mut self);

    /// 检查是否有待处理的更新
    pub fn has_pending_updates(&self) -> bool;

    /// 执行更新（两阶段：布局 + 重绘）
    pub fn perform_update(&mut self);

    /// 获取合并后的脏区域
    pub fn get_damage_region(&self) -> Rectangle;

    /// 清空更新队列
    pub fn clear_updates(&mut self);
}
```

## 关键设计决策

### 决策 1: 脏区域不自动传播到父节点

**g2 方式**：在 `repairDamage` 中自动将脏区域变换到父坐标并与父 bounds 取交集。

**本项目方式**：脏区域存储在块的局部坐标，不自动传播。

**原因**：

- 本项目的渲染流程使用 clip 机制，裁剪由渲染器处理
- 脏区域主要用于视口裁剪（决定重绘范围）
- 简化实现，保持单一职责

```rust
// 本项目：直接存储局部坐标的脏区域
pub fn add_dirty_region(&mut self, block_id: BlockId, rect: Rectangle) {
    // 直接存储，不做坐标变换
    self.dirty_regions.insert(block_id, rect);
}
```

### 决策 2: SceneUpdateManager 作为 SceneGraph 的内部状态

**g2 方式**：`UpdateManager` 是独立对象，通过 `setRoot(IFigure)` 关联根 Figure。

**本项目方式**：`SceneUpdateManager` 直接作为 `SceneGraph` 的字段。

**原因**：

- Rust 的所有权模型更适合内部状态
- 避免复杂的生命周期管理
- 直接访问 SceneGraph 的 blocks

### 决策 3: 不实现异步更新机制

**g2 方式**：使用 `Display.asyncExec()` 异步执行更新。

**本项目方式**：同步执行 `perform_update()`。

**原因**：

- Vello 渲染需要同步调用
- WebGPU/Web 环境的异步模型不同
- 简化实现，按需手动调用

### 决策 4: 合并脏区域的方式

**g2 方式**：在 `repairDamage` 中合并所有脏区域为一个 `damage` 区域。

**本项目方式**：使用 `HashMap<BlockId, Rectangle>`，同一块的脏区域自动合并。

```rust
// 本项目：同一块的脏区域自动合并
if let Some(existing) = self.dirty_regions.get_mut(&block_id) {
    // 扩展区域
    existing.x = existing.x.min(rect.x);
    // ...
} else {
    self.dirty_regions.insert(block_id, rect);
}
```

**原因**：

- g2 需要支持任意 Figure 的脏区域
- 本项目以 Block 为单位，更简单

### 决策 5: 手动触发更新

**g2 方式**：`figure.repaint()` 自动触发 UpdateManager。

**本项目方式**：用户需要手动调用 `perform_update()`。

```rust
// 本项目使用模式
scene.repaint(block_id, None);

if scene.has_pending_updates() {
    scene.perform_update();
    let canvas = scene.render_iterative();
}
```

**原因**：

- Vello 渲染是同步的，需要在确定的时间点执行渲染
- 更明确的控制流程
- 避免隐式的异步行为

## API 对比

| g2 API | 本项目 API | 差异 |
|--------|-----------|------|
| `figure.repaint()` | `scene.repaint(block_id, rect)` | 参数不同 |
| `figure.revalidate()` | `scene.mark_invalid(block_id)` | 分离为两个操作 |
| `updateManager.performUpdate()` | `scene.perform_update()` | 基本一致 |
| `updateManager.addDirtyRegion(figure, rect)` | `scene.update_manager.add_dirty_region(block_id, rect)` | 内部方法 |
| `figure.invalidate()` | `scene.invalidate()` | 基本一致 |

## 使用示例

### 基本使用

```rust
use novadraw_scene::{SceneGraph, RectangleFigure};

// 创建场景
let mut scene = SceneGraph::new();
let container = RectangleFigure::new(0.0, 0.0, 200.0, 200.0);
let container_id = scene.set_contents(Box::new(container));

// 添加子块
let child = RectangleFigure::new(10.0, 10.0, 50.0, 50.0);
scene.add_child_to(container_id, Box::new(child));

// 修改块后，触发布局失效
scene.mark_invalid(container_id);

// 请求重绘
scene.repaint(container_id, None);

// 执行更新并渲染
if scene.has_pending_updates() {
    scene.perform_update();
    let canvas = scene.render_iterative();
    // ... 渲染到屏幕
}
```

### 批量修改

```rust
// 批量修改多个块
for child_id in children {
    scene.mark_invalid(child_id);
    scene.repaint(child_id, None);
}

// 一次更新和渲染
scene.perform_update();
let canvas = scene.render_iterative();
```

### 部分重绘

```rust
// 只重绘块的部分区域（用于小范围更新）
let dirty_rect = Rectangle::new(10.0, 10.0, 50.0, 50.0);
scene.repaint(block_id, Some(dirty_rect));
```

## 测试验证

### 单元测试

| 测试用例 | 验证内容 |
|---------|---------|
| `test_dirty_region_tracking` | 添加脏区域后 has_pending_repaint() 返回 true |
| `test_dirty_region_merge` | 同一块的多个脏区域自动合并 |
| `test_invalid_block_queue` | 添加失效块后 has_pending_layout() 返回 true |
| `test_invalid_block_dedup` | 重复添加同一失效块会自动去重 |
| `test_clear` | clear() 清空所有队列 |
| `test_invalid_region` | 无效区域（宽/高为0）被忽略 |

### 集成测试

| 测试用例 | 验证内容 |
|---------|---------|
| `test_add_child_marks_layout_invalid` | add_child 后布局自动失效 |
| `test_mark_invalid_adds_to_queue` | mark_invalid 添加到失效队列 |
| `test_repaints_adds_dirty_region` | repaint 添加脏区域 |
| `test_repaint_uses_specified_rect` | 指定区域重绘而非整个块 |
| `test_multiple_repaints_merge_regions` | 多次重绘合并区域 |
| `test_invisible_block_no_dirty_region` | 不可见块不产生脏区域 |
| `test_perform_update_two_phase` | 两阶段更新正确执行 |

## 待增强功能

| 功能 | g2 实现 | 当前状态 | 改进方向 |
|------|---------|---------|----------|
| 脏区域坐标传播 | 自动向上传播并取交集 | 无 | 可在 render 时处理 |
| 异步更新 | asyncExec | 同步 | Vello 渲染需要同步 |
| 增量重绘 | 只重绘 damage 区域 | 全量重绘 | 依赖 Vello 能力 |
| UpdateListener | 有监听器 | 已有接口 | 可增强 |

## 参考资料

- Eclipse Draw2D 源码：`org.eclipse.draw2d.UpdateManager`
- Eclipse Draw2D 源码：`org.eclipse.draw2d.DeferredUpdateManager`
- 本项目源码：`novadraw-scene/src/update/`
