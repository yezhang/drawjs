# Bounds 迁移计划

本文档记录将本项目的 Figure bounds 系统迁移到与 Eclipse Draw2D 一致所需的改进项。

## 1. 当前实现 vs d2 完整能力对比

### 1.1 Bounds 相关能力对比

| 能力 | d2 实现 | 本项目实现 | 差距 |
|------|---------|-----------|------|
| **bounds 存储** | Rectangle (x,y,width,height) | Rect (x,y,width,height) | ✅ 已实现 |
| **bounds 含义** | 绝对坐标（相对于坐标根） | 相对坐标（相对于父节点） | ⚠️ 需重构 |
| **坐标传播** | `primTranslate()` 自动传播到子节点 | 无 | ❌ 未实现 |
| **useLocalCoordinates** | 支持切换坐标模式 | 有定义但未使用 | ⚠️ 部分实现 |
| **坐标根概念** | ScalableFreeformLayeredPane, Viewport 等 | 无 | ❌ 未实现 |
| **translateFromParent** | 递归坐标转换 | 部分实现（命中测试中） | ⚠️ 需完善 |
| **translateToParent** | 递归坐标转换 | 无 | ❌ 未实现 |

### 1.2 Figure Trait 能力对比

| 方法 | d2 | 本项目 | 状态 |
|------|-----|--------|------|
| `bounds()` | ✅ | ✅ | 已实现 |
| `setBounds()` | ✅ | ❌ | 未实现 |
| `containsPoint()` | ✅ | ✅ | 已实现 |
| `hit_test()` | ✅ | ✅ | 已实现 |
| `useLocalCoordinates()` | ✅ | ✅ | 有定义未使用 |
| `translate(int,int)` | ✅ | ❌ | 未实现 |
| `primTranslate(int,int)` | ✅ | ❌ | 未实现 |
| `translateFromParent()` | ✅ | ❌ | 未实现 |
| `translateToParent()` | ✅ | ❌ | 未实现 |
| `translateToAbsolute()` | ✅ | ❌ | 未实现 |
| `translateToRelative()` | ✅ | ❌ | 未实现 |
| `isCoordinateSystem()` | ✅ | ❌ | 未实现 |
| `getClientArea()` | ✅ | ✅ | 部分实现 |
| `getInsets()` | ✅ | ✅ | 有定义 |
| `isOpaque()` | ✅ | ✅ | 已实现 |
| `fireFigureMoved()` | ✅ | ❌ | 未实现 |
| `fireCoordinateSystemChanged()` | ✅ | ❌ | 未实现 |

### 1.3 布局相关能力对比

| 能力 | d2 | 本项目 | 状态 |
|------|-----|--------|------|
| LayoutManager | ✅ | ✅ | 已实现 |
| setConstraints | ✅ | ✅ | 已实现 |
| invalidate() | ✅ | 部分 | 需完善 |
| validate() | ✅ | 部分 | 需完善 |
| revalidate() | ✅ | ❌ | 未实现 |
| invalidateTree() | ✅ | ❌ | 未实现 |

### 1.4 命中测试能力对比

| 能力 | d2 | 本项目 | 状态 |
|------|-----|--------|------|
| findFigureAt | ✅ | ❌ | 未实现 |
| findMouseEventTargetAt | ✅ | ✅ | 已实现 |
| 逆序遍历子节点 | ✅ | ✅ | 已实现 |
| 坐标转换 | ✅ | ⚠️ | 部分实现 |
| 路径追踪 | ✅ | ✅ | 已实现 |
| 剪枝（clientArea） | ✅ | ❌ | 未实现 |
| TreeSearch 过滤 | ✅ | ❌ | 未实现 |

---

## 2. 执行清单

### 阶段 1：基础坐标系统（短期）

| # | 任务 | 优先级 | 状态 |
|---|------|--------|------|
| 1.1 | 修复命中测试坐标转换（当前测试失败） | P0 | ⏳ pending |
| 1.2 | 实现 `translate()` 坐标传播，统一 bounds 为绝对坐标 | P1 | ⏳ pending |
| 1.3 | 实现 `setBounds()` 方法和 erase/repaint 语义 | P2 | ⏳ pending |

### 阶段 2：坐标转换方法（中期）

| # | 任务 | 优先级 | 状态 |
|---|------|--------|------|
| 2.1 | 实现 `translateFromParent/translateToParent` 坐标转换方法 | P3 | ⏳ pending |
| 2.2 | 实现 `translateToAbsolute/translateToRelative` 递归转换 | P4 | ⏳ pending |

### 阶段 3：布局失效机制（中期）

| # | 任务 | 优先级 | 状态 |
|---|------|--------|------|
| 3.1 | 实现 `invalidate/revalidate/invalidateTree` 布局失效机制 | P5 | ⏳ pending |

### 阶段 4：坐标根与本地坐标模式（长期）

| # | 任务 | 优先级 | 状态 |
|---|------|--------|------|
| 4.1 | 实现 `isCoordinateSystem()` 方法 | P6 | ⏳ pending |
| 4.2 | 实现坐标根容器（FreeformLayer, Viewport, ScalableFreeformLayeredPane） | P6 | ⏳ pending |

### 阶段 5：高级命中测试（长期）

| # | 任务 | 优先级 | 状态 |
|---|------|--------|------|
| 5.1 | 实现 `findFigureAt` 和 TreeSearch 过滤 | P7 | ⏳ pending |
| 5.2 | 实现基于 `getClientArea()` 的剪枝 | P7 | ⏳ pending |

---

## 3. 任务详情

### 1.1 修复命中测试坐标转换（当前测试失败）

**问题**：当前命中测试中嵌套场景测试失败，路径未正确追踪

**根因分析**：
- 子节点 bounds 是相对于父节点的
- 命中测试需要将全局坐标转换为本地坐标
- 当前偏移量计算逻辑有误

**预期行为**：
```
场景：root(100,100) → parent(30,30) → child(-30,-30)
点击点 (120, 120) 应该命中 child

绝对坐标：
- root: (100, 100, 200, 150)
- parent: (130, 130, 100, 80)
- child: (100, 100, 60, 40)  // 130-30=100, 130-30=100
```

**关键代码路径**：
- `novadraw-scene/src/scene/hit_test.rs` - HitTester

---

### 1.2 实现 translate() 坐标传播

**目标**：统一 bounds 含义为绝对坐标

**设计**：
```rust
fn translate(&mut self, dx: f64, dy: f64) {
    self.bounds.x += dx;
    self.bounds.y += dy;

    if !self.use_local_coordinates() {
        // 默认模式：递归传播到所有子节点
        for &child_id in &self.children {
            // 递归传播
        }
    }
}
```

**影响范围**：
- `set_bounds()` 调用时传播偏移
- 子节点 bounds 始终保持绝对坐标

---

### 1.3 实现 setBounds() 方法

**目标**：实现完整的 bounds 设置语义

**设计**：
```rust
fn set_bounds(&mut self, rect: Rect) {
    let old_x = self.bounds.x;
    let old_y = self.bounds.y;

    let resize = rect.width != self.bounds.width || rect.height != self.bounds.height;
    let translate = rect.x != old_x || rect.y != old_y;

    if (resize || translate) && self.is_visible {
        self.erase();  // 擦除旧位置
    }

    if translate {
        let dx = rect.x - old_x;
        let dy = rect.y - old_y;
        self.prim_translate(dx, dy);
    }

    self.bounds = rect;

    if translate || resize {
        if resize {
            self.invalidate();
        }
        self.fire_figure_moved();
        self.repaint();
    }
}
```

---

## 4. 优先级说明

| 优先级 | 含义 | 适用场景 |
|--------|------|----------|
| P0 | 最高优先级 | 修复测试失败，影响核心功能 |
| P1 | 高优先级 | 基础 API 缺失，影响系统一致性 |
| P2 | 中高优先级 | 常用 API，影响用户体验 |
| P3 | 中优先级 | 完整 API，支持高级功能 |
| P4 | 中优先级 | 完整 API，支持高级功能 |
| P5 | 中低优先级 | 布局系统完整 |
| P6 | 低优先级 | 高级特性，支持缩放/滚动 |
| P7 | 低优先级 | 高级特性，支持过滤 |

---

## 5. 依赖关系

```
1.1 修复命中测试
    │
    ▼
1.2 实现 translate()  ◄─── 依赖 1.1
    │
    ▼
1.3 实现 setBounds()  ◄─── 依赖 1.2
    │
    ▼
2.1 坐标转换方法       ◄─── 依赖 1.3
    │
    ▼
2.2 递归坐标转换       ◄─── 依赖 2.1
    │
    ▼
3.1 布局失效机制       ◄─── 依赖 2.2
    │
    ▼
4.1 isCoordinateSystem ◄─── 依赖 3.1
    │
    ▼
4.2 坐标根容器         ◄─── 依赖 4.1
    │
    ▼
5.1 高级命中测试       ◄─── 依赖 4.2
```

---

## 6. 验收标准

每个任务完成后需满足：

1. **单元测试通过** - 所有相关测试 100% 通过
2. **编译无警告** - `cargo clippy` 无警告
3. **API 文档完整** - 所有 pub 方法有文档注释
4. **行为与 d2 一致** - 关键行为与 Eclipse Draw2D 相同

---

*创建时间：2024-01-15*
*基于 Eclipse Draw2D 源码分析*
