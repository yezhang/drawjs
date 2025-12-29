mod app_window;
mod scene_manager;

use env_logger;

use crate::app_window::start_app;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let _ = start_app();
    Ok(())
}

//fn main() {
//    let num = novadraw::add(1, 2);
//    println!("Hello, world! {num:?}");
//}
