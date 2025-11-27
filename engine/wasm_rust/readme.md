# Rust 渲染引擎

- [ ] 验证 vello 对于不同中文字体的支持情况

## 开发环境配置

- [ ] 配置 vello 依赖项，初始化调试环境

## 配置明细说明

使用 cargo workspace 方式管理项目。
主应用是 bin 格式；
开发包是 lib 格式；

workspace 中的包是共享 Cargo.lock 和输出目录的。

使用 `cargo new xxx` 在工作空间下新建一个二进制包；
使用 `cargo new xxx --lib` 在工作工具下新增一个库；
