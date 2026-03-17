mod app_window;
mod scene_manager;

use tracing_subscriber::{fmt, EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

use crate::app_window::start_app;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化 tracing
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| "novadraw=info".into());

    tracing_subscriber::registry()
        .with(fmt::layer().with_target(false))
        .with(filter)
        .init();

    let _ = start_app();
    Ok(())
}
