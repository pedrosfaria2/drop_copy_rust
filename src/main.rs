mod config;
mod custom_logger;
mod handler;
mod session;
mod utils;

use clap::{Arg, ArgAction, Command};
use config::AppConfig;
use handler::MessageHandler;
use session::start_sessions;
use std::sync::Arc;

fn main() -> Result<(), quickfix::QuickFixError> {
    // Set up the command-line arguments and options using clap
    let matches = Command::new("Drop Copy App")
        .version("0.1.0")
        .author("Pedro Serrano Faria <pedroserrano2@gmail.com>")
        .about("Manages Drop Copy sessions")
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .value_name("FILE")
                .help("Sets a custom config file. Example: --config config.yaml")
                .long_help("Specify the path to the custom configuration file for the Drop Copy sessions. The configuration file should be in YAML format and include all necessary session settings.")
                .action(ArgAction::Set)
                .required(true),
        )
        .after_help(
            "EXAMPLES:\n\n\
             To start the Drop Copy App with a custom configuration file:\n\
             drop_copy_app.exe --config config.yaml\n\n\
             For more information, visit https://github.com/pedrosfaria2/drop_copy_rust \n\n\n"
        )
        .get_matches();

    // Print application header
    println!("{}", "=".repeat(60));
    println!("Drop Copy App v0.1.0");
    println!("Author: Pedro Serrano Faria <pedroserrano2@gmail.com>");
    println!("{}", "=".repeat(60));
    println!();

    // Get the config file path from the command-line arguments
    let config_file = matches
        .get_one::<String>("config")
        .expect("Config file is required");
    // Load the configuration from the specified file
    let config = AppConfig::load(config_file).expect("Failed to load config");
    println!("Loaded config file: {}", config_file);
    println!("Configuration: {:#?}", config);

    // Create a new message handler
    let handler = Arc::new(MessageHandler::new());
    // Start the sessions using the loaded configuration and message handler
    start_sessions(config, handler)?;

    Ok(())
}
