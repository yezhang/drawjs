文章地址：
[《Easy Scalable Text Rendering on the GPU》](https://medium.com/@evanwallace/easy-scalable-text-rendering-on-the-gpu-c3f4d782c5ac)

### 核心原理：

- GPU stencil 具有'invert' 特征，可以翻转像素。
- 本文避免了 stencil buffer 的方法：使用加法混合（additive blending），颜色使用 1/255（原因是 GPU 中的颜色是 8 bit，从 0 到 1）。

### 具体步骤

- [x] 读取 glyph 的控制点数据（在线点、离线点）
  - glyph.path.commands
- [x] 准备三角形绘图数据（存储缓冲区）
  - 多个轮廓
  - topology = triangle-list，使用 drawIndexed 绘制。
- [-] 使用 stencil 方法绘制“多边形”字体（在线点）

  - [ ] 阶段1，绘制 stencil 模板

    - render pass 配置
      - stencilLoadOp = 'clear'; // 如果是初次 renderpass 的 stencil 附件，默认是0，可以使用 'load' 配置。
      - stencilClearValue = 0
      - stencilLoadOp = 'store'
    - pipeline 配置
      - depthStencil 配置
        - depthCompare = 'always'
        - depthWriteEnabled = false, // 如果 format 中没有深度，可以省略这个参数；本案例中不涉及 depth 的写入，所以考虑设置 false。
        - stencilWriteMask = 0x1
        - stencilReadMask = 0x1
        - stencilFront/stencilBack
          - compare = 'always'
          - passOp = 'invert'
    - 执行绘制
      所有三角形只调用一次 draw，并触发多次 stencil 翻转

  - [ ] 阶段2，绘制字体颜色
    - render pass 配置
      - stencilLoadOp = 'load'
      - stencilStoreOp = 'discard' // passOp = keep 时，StoreOp 可以保持为 store，具有相同效果（都是不写入）
      - stencilReadOnly = true
    - pipeline 配置
      - depthStencil 配置
        - depthCompare = 'always'
        - depthWriteEnabled = false
        - stencilWriteMask = 0x1
        - stencilReadMask = 0x1
        - stencilFront/stencilBack
          - compare = 'equal'
          - passOp = 'keep'
    - 执行
      pass.setStencilReference(1.0)
      pass.draw(), 绘制一个超出范围的大三角形

- [ ] 使用 add blending 方法绘制“多边形”字体（在线点）

### 一些发现

使用 encoder 创建 renderpass 时，在colorAttachments 处填写的 view，其所在 context 必须与 encoder 所在 device 一致。即，

- colorAttachments[0].view = context.getCurrentTexture().createView()
  - 条件：context.configure({device,...}) <-- 这里的 device-A
- encoder = device.createCommandEncoder() <-- 这里的 device-B
- device-A，必须与 device-B 一致。

### 参考资料

- webgpu-utils 工具使用说明，https://greggman.github.io/webgpu-utils/docs/
