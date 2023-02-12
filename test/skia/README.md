尝试在 webworker 中使用 CanvasKit.MakeCanvas。

离屏画布
https://developers.google.com/web/updates/2018/08/offscreen-canvas
在 Worker 中独立使用 Canvas。

```js
// file: worker.js

function getGradientColor(percent) {
    const canvas = new OffscreenCanvas(100, 1);
    const ctx = canvas.getContext('2d');
    const gradient = ctx.createLinearGradient(0, 0, canvas.width, 0);
    gradient.addColorStop(0, 'red');
    gradient.addColorStop(1, 'blue');
    ctx.fillStyle = gradient;
    ctx.fillRect(0, 0, ctx.canvas.width, 1);
    const imgd = ctx.getImageData(0, 0, ctx.canvas.width, 1);
    const colors = imgd.data.slice(percent * 4, percent * 4 + 4);
    return `rgba(${colors[0]}, ${colors[1]}, ${colors[2]}, ${colors[])`;
}

getGradientColor(40);  // rgba(152, 0, 104, 255 )
```

将主线程中的 Canvas 传递到 Worker 中。
```js
const offscreen = document.querySelector('canvas').transferControlToOffscreen();
const worker = new Worker('myworkerurl.js');
worker.postMessage({ canvas: offscreen }, [offscreen]);
```

在 webworker 中使用 WebAssembly 模块的方法：https://www.sitepen.com/blog/using-webassembly-with-web-workers

WebWorker 的使用：
https://developer.mozilla.org/zh-CN/docs/Web/API/Web_Workers_API/Using_web_workers

WebAssembly 的使用：
https://developer.mozilla.org/en-US/docs/WebAssembly/Using_the_JavaScript_API
```js
fetch('simple.wasm').then(response =>
  response.arrayBuffer()
).then(bytes =>
  WebAssembly.instantiate(bytes, importObject)
).then(results => {
  results.instance.exports.exported_func();
});
```

在 Webpack 环境下使用 WebAssembly、WebWorker，https://pspdfkit.com/blog/2020/webassembly-in-a-web-worker/
```js
// `wasm.worker.js`

import { expose } from "comlink";
import addWasm from "./wasm/add.wasm";
import addJS from "./wasm/add.js";

const sum = async (a, b) =>
  new Promise(async resolve => {
    const wasm = await fetch(addWasm);
    const buffer = await wasm.arrayBuffer();
    const _instance = await addJS({
      wasmBinary: buffer
    });

    resolve(_instance._add(a, b))
  });

expose(sum);
```

CanvasKit 接口：
https://skia.googlesource.com/skia/+/312535b47d389e55a6666ea82638458245a421e0/modules/canvaskit/canvaskit/types/index.d.ts

https://skia.googlesource.com/skia/+/312535b47d389e55a6666ea82638458245a421e0/modules/canvaskit/gpu.js

初次使用时，遇到 canvaskit.wasm 被重复下载的问题。
原因：CanvasKit 会首先尝试 WebAssembly.instantiateStreaming 方式下载，下载出错后，使用 fetch 以及 arrayBuffer 的形式下载 WebAssembly。
instantiateStreaming 下载出错原因，在 DevTools 中的 Console 中有提示“Incorrect response MIME type. Expected 'application/wasm'.”。
解决方法是：??

