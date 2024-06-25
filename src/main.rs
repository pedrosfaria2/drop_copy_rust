mod config;
mod session;
mod handler;
mod custom_logger;
mod utils;

use config::AppConfig;
use session::start_sessions;
use handler::MessageHandler;
use std::sync::Arc;

fn main() -> Result<(), quickfix::QuickFixError> {
    let current_dir = std::env::current_dir().expect("Failed to get current directory");
    println!("Current directory: {:?}", current_dir);

    let config = AppConfig::load("config.yaml").expect("Failed to load config");
    println!("Loaded config: {:?}", config);

    let handler = Arc::new(MessageHandler::new());
    start_sessions(config, handler)?;
    Ok(())
}
