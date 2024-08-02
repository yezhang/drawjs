const worker = new Worker("worker.js");

worker.onmessage = (event) => {
  console.log("Result from worker:", event.data);
};

worker.onerror = (error) => {
  console.error("Worker error:", error.message);
};

async function main() {
  // 加载 WebWorker 模块
  // worker.js 本执行实际的 WebAssembly 模块，执行绘图操作。
  worker.postMessage("Hello from main.js");
}

main().catch(console.error);
