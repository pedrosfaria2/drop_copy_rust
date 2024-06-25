mod config;
mod session;
mod handler;
mod custom_logger;
mod utils;

use config::AppConfig;
use session::start_session;
use handler::MessageHandler;

fn main() -> Result<(), quickfix::QuickFixError> {
    let current_dir = std::env::current_dir().expect("Failed to get current directory");
    println!("Current directory: {:?}", current_dir);

    let config = AppConfig::load("config.yaml").expect("Failed to load config");
    println!("Loaded config: {:?}", config);

    let handler = MessageHandler::new();
    start_session(config, handler)?;
    Ok(())
}
