# 脚本作用：
# 1. 将 cpp 文件编译为 wasm 模块
# 2. 生成 html 启动文件；在 HTML 中，会自动将 wasm 文件以 async 方式加载
#
# 参数含义如下：
# -o webgpu.html: 输出文件名
# --shell-file html_template/shell_minimal.html: 使用模板文件
# -s MODULARIZE=1: 生成模块化的 js 文件
# -s EXPORT_NAME='HelloInit': 指定模块化的 js 文件的名称
#
emcc \
	webgpu.cpp \
	--shell-file html_template/shell_minimal.html \
	-s MODULARIZE=1 \
	-s EXPORT_NAME='HelloInit' \
	-o webgpu.html
