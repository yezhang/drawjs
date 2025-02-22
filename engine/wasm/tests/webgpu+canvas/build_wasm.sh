# 本脚本用于编译 webgpu.cpp 文件为 webgpu.js

emcc \
	-v \
	webgpu.cpp \
	-s MODULARIZE=1 \
	-s EXPORT_NAME='HelloInit' \
	-s EXPORTED_FUNCTIONS='["_draw", "_main"]' \
	-s EXPORTED_RUNTIME_METHODS=ccall,cwrap \
	-s USE_WEBGPU=1 \
	-o webgpu.js
