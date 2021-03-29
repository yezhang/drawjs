# drawjs

a port of eclipse draw2d

本项目构建可以运行在浏览器端的图形编辑框架。实现基于 `<canvas/>` 标签的 [GEF 框架](https://www.eclipse.org/gef/)。

第一版本使用 TypeScript 实现该库。
第二版本使用 Rust/C++ 语言编写，编译为 WebAssembly 后，在 `<canvas/>` HTML 标签上运行。

由于 GEF 框架最新版 GEF5，主要目的平台是 JavaFX，无法方便应用于Web技术上。
因此，本项目采用 [GEF-legacy](https://github.com/eclipse/gef-legacy) 项目的代码框架。

# 路线图（RoadMap）

1. [ ]梳理 draw2d 的 Java 类继承关系
    1.1 [x] 绘制 IFigure 类图
        1.1.1 [ ] 绘制 Figure 类图（Implement）
    1.2 [x] 绘制 EventDispatcher 类图
        1.2.1 [ ] SWTEventDispatcher 类图（Implement）
    1.3 [x] 绘制 UpdateManager 类图
        1.3.1 [ ] DeferredUpdateManager 类图（Implement）
    1.4 [x] 绘制 LightweightSystem 类图
    1.5 [ ] 绘制 EventHandler 类图
2. [ ]选择核心类范围
3. [ ]使用 Rust 实现核心类
4. [ ]建立 Rust 与 `<canvas/>` 通信机制
5. [ ]验证核心类在 `<canvas/>` 标签的可用性。
6. [x]HTML Canvas 中支持的 Events 清单

# 设计

对于 `<canvas/>` 标签做一个抽象封装，可以支持后续替换为 2D 或 3D。
- 支持无限滚动。
- 缩放快速。
- 拖拽快速。
- 支持海量复杂图形的高性能绘制。（百万级别数量图形）
    - 计算机图形学（2D/3D）中的裁剪算法。


# 进一步阅读
- [GEF4 wiki](https://wiki.eclipse.org/GEF/GEF4)
- [GEF4 + 1 = GEF5](http://nyssen.blogspot.com/2017/02/gef4-1-gef-5.html#Merger%20of%20MVC%20and%20MVC.FX)
- [GEF4 MVC (0.1.0) 用于替换 GEF (MVC) 3.x](https://github.com/eclipse/gef/blob/master/CHANGELOG.md#gef4-mvc-010)
- [GEF4 FX (0.1.0)用于替换 Draw2D](https://github.com/eclipse/gef/blob/master/CHANGELOG.md#gef4-fx-010)
- [The Draw2d Examples - A Hidden Treasure](http://nyssen.blogspot.com/2010/12/draw2d-examples-hidden-treasure.html)
- Alexander Nyßen（GEF Leader, https://projects.eclipse.org/projects/tools.gef/who） 的其他博客[文章](http://nyssen.blogspot.com)
- draw2d 源码位置在[这里](https://download.eclipse.org/oomph/archive/simrel/gef.aggrcon/index/org.eclipse.draw2d.source_3.10.100.201606061308.html)
- [GEF 官方文章](https://www.eclipse.org/gef/reference/articles.html)
- Graphiti，https://www.eclipse.org/graphiti/documentation/overview.php

### Web 绘图技术
- 操作 Canvas 的 [3 种方法](https://compile.fi/canvas-filled-three-ways-js-webassembly-and-webgl/)
- Canvas API 相关的[库](https://developer.mozilla.org/en-US/docs/Web/API/Canvas_API#libraries)
- Camera 技术的使用：https://stackoverflow.com/questions/16919601/html5-canvas-camera-viewport-how-to-actually-do-it
- [WebAssembly](https://webassembly.org/)

### SWT 相关
- [SWT 中的 TraverseEvent](https://cloud.tencent.com/developer/article/1433531)
- [SWT 中的 Graphics Context](https://www.eclipse.org/articles/Article-SWT-graphics/SWT_graphics.html)

