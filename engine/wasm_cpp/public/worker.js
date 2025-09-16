// worker.js 本执行实际的 WebAssembly 模块，执行绘图操作。
//
self.onmessage = async (event) => {
  const data = event.data;

  console.log("Worker received:", data);

  // 加载 WebGPU 模块
  importScripts("novadraw.js");

  let ModuleOptions = {
    print: function (text) {
      console.log(text);
    },
    printErr: function (text) {
      console.error(text);
    },
    onRuntimeInitialized: function () {
      console.log("onRuntimeInitialized(), in web worker");
      // 通知主线程已经初始化完成
      self.postMessage("WebAssembly module initialized");
    },
  };
  NovaDrawInit(ModuleOptions).then(function (Module) {
    console.log("NovaDrawInit(), in web worker");
  });
};

self.onerror = (error) => {
  console.error("Worker error:", error.message);
};
