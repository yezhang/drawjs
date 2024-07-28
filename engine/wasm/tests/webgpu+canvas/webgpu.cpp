
#include <webgpu/webgpu_cpp.h>

#include <cstdlib>

#include <emscripten.h>
#include <emscripten/console.h>
#include <emscripten/html5.h>
#include <emscripten/html5_webgpu.h>

extern "C" {
EMSCRIPTEN_KEEPALIVE void draw() {
  // 插入绘制逻辑
}
}
int main(int argc, char *argv[]) {
  emscripten_console_log("Hello, WebGPU!");
  return 0;
}
