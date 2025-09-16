# 脚本作用：
# 1. 将 cpp 文件编译为 wasm 模块和配套的 js 胶水文件；
# 2. 生成 html 启动文件；在 HTML 中，会自动将 wasm 文件以 async 方式加载
#
# 参数含义如下：
# -o webgpu.html: 输出文件名
# --shell-file html_template/shell_minimal.html: 使用模板文件
# -s MODULARIZE=1: 生成模块化的 js 文件
# -s EXPORT_NAME='HelloInit': 指定模块化的 js 文件的名称
#
# 注意事项：
# emscripten 默认 HTML 模板，提供了一个名称为 Module 的全局变量，用于给 wasm 的导出模块注入启动参数，
# 包括 print, canvas, setStatus 等。
# 在生成 webgpu.html 后，该 html 文件会以 async 方式加载 wasm 的 js 入口文件，即 webgpu.js。
# webgpu.js 会自动将 wasm 的导出模块注入到 Module 中，然后调用 Module 的 _main 函数。
# 默认 HTML 模板只支持非模块化导出，且导出名称是默认的 Module。
#
# 不同构建方式所需要的模板和配置：
# 1. 全局导出 & 默认导出名：直接使用默认 HTML 模板即可。
# 2. 全局导出 & 自定义导出名：修改 HTML 模板中的 Module 为 EXPORT_NAME。
# 也就是说，HTML 中要存在 EXPORT_NAME 变量，以便将模块初始化参数注入到模块中。
#
# 3. 模块导出 & 默认导出名：完全自定义模板，手动调用 Module() 函数并传递参数。注意代码加载顺序。
# 4. 模块导出 & 自定义导出名：完全自定义模板，手动调用 <ExportName>Init 并传递参数。注意代码加载顺序。
#
# 使用模块化导出：
# 默认 HTML 模板是不支持模块化导出的，因此当使用 MODULARIZE=1 时，需要自定义 HTML 模板文件。
#
# 使用自定义导出名称：
# 当使用 EXPORT_NAME 时，webgpu.js 导出文件会使用 EXPORT_NAME 作为模块名称，而不是默认的 Module。
# 但是 HTML 模板文件中仍然使用 Module，因此需要手动修改 HTML 模板文件，将 Module 替换为 EXPORT_NAME。
#
# 使用自定义初始化参数：
# 除了使用全局变量形式，也可以在模块初始化时，将 print, canvas, setStatue 参数传递给模块，然后模块内部使用参数。

# 方法1：使用全局导出 & 默认导出名
emcc \
	webgpu.cpp \
	--shell-file html_template/shell_minimal.html \
	-o webgpu.html
