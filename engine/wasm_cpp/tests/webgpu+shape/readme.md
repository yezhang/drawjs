分层架构

```
src/
├── core/           # 基础模块（无外部依赖）
│   ├── math.js     # 数学工具
│   └── gpu.js      # WebGPU 初始化
├── data/           # 数据管理
│   └── scene.js    # 场景图（依赖 core/gpu.js）
├── render/         # 渲染模块
│   └── pipeline.js # 渲染管线（依赖 core/gpu.js 和 data/scene.js）
└── ui/             # 交互模块
    └── tools.js    # 绘图工具（依赖 data/scene.js）
```
