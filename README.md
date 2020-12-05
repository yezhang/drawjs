# drawjs

a port of eclipse draw2d

本项目构建可以运行在浏览器端的图形编辑框架。实现基于 `<canvas/>` 标签的 [GEF 框架](https://www.eclipse.org/gef/)。

使用 Rust 语言编写，编译为 WebAssembly 后，在 `<canvas/>` HTML 标签上运行。

由于 GEF 框架最新版 GEF5，主要目的平台是 JavaFX，无法方便应用于Web技术上。
因此，本项目采用 [GEF-legacy](https://github.com/eclipse/gef-legacy) 项目的代码框架。

# 路线图（RoadMap）

1. [ ]梳理 draw2d 的java类继承关系
2. [ ]选择核心类范围
3. [ ]使用 Rust 实现核心类
4. [ ]建立 Rust 与 `<canvas/>` 通信机制
5. [ ]验证核心类在 `<canvas/>` 标签的可用性。

# 进一步阅读
- [GEF4 wiki](https://wiki.eclipse.org/GEF/GEF4)
- [GEF4 + 1 = GEF5](http://nyssen.blogspot.com/2017/02/gef4-1-gef-5.html#Merger%20of%20MVC%20and%20MVC.FX)
- [The Draw2d Examples - A Hidden Treasure](http://nyssen.blogspot.com/2010/12/draw2d-examples-hidden-treasure.html)
- Alexander Nyßen（GEF Leader, https://projects.eclipse.org/projects/tools.gef/who） 的其他博客[文章](http://nyssen.blogspot.com)
