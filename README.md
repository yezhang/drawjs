# drawjs

a port of eclipse draw2d

本项目构建可以运行在浏览器端的图形编辑框架。

使用 Rust 语言编写，编译为 WebAssembly 后，在 `<canvas/>` HTML 标签上运行。

# 路线图（RoadMap）

1. [ ]梳理 draw2d 的java类继承关系
2. [ ]选择核心类范围
3. [ ]使用 Rust 实现核心类
4. [ ]建立 Rust 与 `<canvas/>` 通信机制
5. [ ]验证核心类在 `<canvas/>` 标签的可用性。