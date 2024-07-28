# 本脚本用于编译 webgpu.cpp 文件为 hello_module.js

emcc \
	-v \
	webgpu.cpp \
	-s EXPORTED_FUNCTIONS='["_draw", "_main"]' \
	-s MODULARIZE=1 \
	-s EXPORT_NAME='HelloInit' \
	-o webgpu.js
