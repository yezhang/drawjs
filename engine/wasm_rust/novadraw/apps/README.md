# Novadraw Apps - 验证应用程序集

## 概述

本目录包含 Novadraw 图形引擎的各个功能验证 App，每个 App 专注于验证一个特定功能模块。

## App 列表

| App | 功能 | 场景数 | 运行命令 |
|-----|------|--------|----------|
| **shape-app** | 形状展示 | 3 | `cargo run -p shape-app` |
| **transform-app** | 坐标变换 | 10 | `cargo run -p transform-app` |
| **clip-app** | 裁剪机制 | 10 | `cargo run -p clip-app` |
| **layout-app** | 布局管理 | 10 | `cargo run -p layout-app` |
| **event-app** | 事件处理 | 10 | `cargo run -p event-app` |
| **border-app** | Border 装饰器 | 10 | `cargo run -p border-app` |
| **editor** | 综合编辑器 | - | `cargo run -p editor` |

## 通用操作

所有 App 共享相同的操作方式：

```sh
- 按数字键 `0`-`9` 切换场景
- 按 `ESC` 退出程序
- 部分场景支持鼠标交互
```

## 优先级说明

| 优先级 | App | 说明 |
|--------|-----|------|
| P0 | transform-app, clip-app | 变换和裁剪是渲染核心 |
| P1 | layout-app, event-app | 布局和事件是交互基础 |
| P2 | border-app, shape-app | 可渐进式扩展 |

## 架构设计

```sh
novadraw/ (workspace)
├── novadraw-math/      ← 数学运算
├── novadraw-core/      ← 核心数据类型
├── novadraw-render/    ← 渲染后端
├── novadraw-scene/     ← 场景图、Figure 接口
└── apps/
    ├── shape-app/      ← 形状验证
    ├── transform-app/   ← 变换验证
    ├── clip-app/        ← 裁剪验证
    ├── layout-app/      ← 布局验证
    ├── event-app/       ← 事件验证
    ├── border-app/      ← Border 验证
    └── editor/         ← 综合编辑器
```

## 运行所有测试

```sh
# 检查所有包
cargo check --workspace

# 运行测试
cargo test --workspace
```

## 添加新 App

1. 在 `apps/` 下创建新目录
2. 添加 `Cargo.toml` 和 `src/main.rs`
3. 在 workspace `Cargo.toml` 中添加成员
4. 更新本 README
