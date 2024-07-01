# WebAssembly 的 CMake 配置文件
cmake_minimum_required(VERSION 3.5)
project(NovaDraw)

add_definitions(-std=c++11)

include_directories(include)
# 将项目根目录（即包含 CMakeLists.txt 的目录）添加到 include 搜索路径
include_directories(${CMAKE_SOURCE_DIR})
include_directories($(CMAKE_SOURCE_DIR)/src)
file(GLOB SOURCES src/*.cpp)



# 设置调试信息
set(CMAKE_BUILD_TYPE "Debug")

# 设置输出目录
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY "${CMAKE_SOURCE_DIR}/public")

# 添加可执行文件
add_executable(novadraw ${SOURCES})

# 设置生成 WebAssembly 的特定链接标志
message(STATUS "Setting compilation target to WASM")

execute_process(COMMAND em++ --cflags
    OUTPUT_VARIABLE EM_CFLAGS)
set_target_properties(novadraw PROPERTIES
    COMPILE_FLAGS "${EM_CFLAGS}"
    OUTPUT_NAME "novadraw"  # 确保所有生成的文件使用相同的基本名称
    SUFFIX ".js"  # 设置输出文件的后缀
)

# 链接标志 - 保证生成 novadraw.wasm 和 novadraw.js
set(LINK_FLAGS "")
string(APPEND LINK_FLAGS " -s WASM=1")
string(APPEND LINK_FLAGS " -s EXPORTED_FUNCTIONS='[\"_draw\", \"_main\"]'")
string(APPEND LINK_FLAGS " -s \"EXPORTED_RUNTIME_METHODS=['ccall', 'cwrap']\"")
string(APPEND LINK_FLAGS " -s ALLOW_MEMORY_GROWTH=1")
string(APPEND LINK_FLAGS " -s MODULARIZE=1")
string(APPEND LINK_FLAGS " -s EXPORT_NAME='NovaDrawInit'")
string(APPEND LINK_FLAGS " -o ${CMAKE_RUNTIME_OUTPUT_DIRECTORY}/novadraw.js")

# 将 LINK_FLAGS 应用于目标文件
set_target_properties(novadraw PROPERTIES LINK_FLAGS "${LINK_FLAGS}")
