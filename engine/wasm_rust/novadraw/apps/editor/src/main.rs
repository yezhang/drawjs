// 第一部分，模块声明，解释项目结构
mod app_window;
// 第二部分，引入外部crate
use log::{debug, error, log_enabled, info, Level};
use env_logger;
// 引入内部workspace模块中的crate
use novadraw;
// 第三部分，引入标准库
// 第四部分，引入内部模块的项
use crate::app_window::start_app;
// 第五部分，重新导出（塑造公共API）

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化日志
    env_logger::init();
    let _ = start_app(); 
    
    Ok(())
}

//fn main() {
//    let num = novadraw::add(1, 2);
//    println!("Hello, world! {num:?}");
//}
