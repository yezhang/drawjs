# Layout App - 布局管理器验证

## 功能说明

验证 Draw2D 布局管理系统，包括 FlowLayout、BorderLayout、StackLayout、XYLayout、GridLayout 等布局类型。

## 运行方式

```bash
cargo run -p layout-app
```

## 场景说明

| 场景 | 名称 | 验证内容 |
|------|------|----------|
| 0 | FlowLayout 水平 | 元素水平排列 |
| 1 | FlowLayout 垂直 | 元素垂直排列 |
| 2 | FlowLayout 换行 | 元素超出宽度时换行 |
| 3 | BorderLayout | 方位布局（北、南、西、东、中） |
| 4 | BorderLayout 嵌套 | 嵌套的 BorderLayout |
| 5 | StackLayout | 堆叠布局，元素重叠显示 |
| 6 | XYLayout 绝对 | 绝对定位布局 |
| 7 | GridLayout | 网格布局 |
| 8 | 布局嵌套 | 多种布局的组合使用 |
| 9 | 布局约束 | 元素的尺寸约束 |

## 操作说明

- 按数字键 `0`-`9` 切换场景
- 按 `ESC` 退出程序

## 布局类型说明

### FlowLayout

元素按顺序排列，支持水平或垂直方向，超出容器时自动换行。

### BorderLayout

将容器划分为五个区域：

- **North**: 顶部区域
- **South**: 底部区域
- **West**: 左侧区域
- **East**: 右侧区域
- **Center**: 中央区域

### StackLayout

元素堆叠显示，后添加的元素在上层。

### XYLayout

绝对定位，每个元素指定精确的 X/Y 坐标。

### GridLayout

将容器划分为规则的网格，元素按网格单元排列。

## 依赖模块

- `novadraw-scene`: 场景图和 Figure 接口
