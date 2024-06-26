
# Drop Copy App

**Version**: 0.1.0  
**Author**: Pedro Serrano Faria (pedroserrano2@gmail.com)

## Overview

The Drop Copy App is a Rust-based application designed to manage multiple Drop Copy sessions. It supports configurations via YAML files and provides custom logging for both raw and human-readable FIX protocol messages.

## Features

- Manages multiple Drop Copy sessions simultaneously
- Custom logging of FIX protocol messages in both raw and human-readable formats
- Configurable via YAML files
- Command-line interface for easy usage

## Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
- [QuickFIX library](https://github.com/quickfix/quickfix)
- [Clap](https://crates.io/crates/clap)

### Steps

1. Clone the repository:

    ```sh
    git clone https://github.com/pedrosfaria2/drop_copy_rust.git
    cd drop_copy_rust
    ```

2. Build the project:

    ```sh
    cargo build
    ```

3. Run the tests:

    ```sh
    cargo test
    ```

## Usage

### Running the Application

To run the Drop Copy App, you need to provide a YAML configuration file. Below is an example command:

```sh
cargo run -- --config config.yaml
```

### Configuration File

The configuration file should be in YAML format and include paths to session settings files. Here is an example `config.yaml`:

```yaml
settings:
  - "session1.cfg"
  - "session2.cfg"
  - "session3.cfg"
```

### Command-Line Interface

The Drop Copy App provides a command-line interface with the following options:

```sh
USAGE:
    drop_copy_app.exe --config <FILE>

OPTIONS:
    -c, --config <FILE>    Sets a custom config file. Example: --config config.yaml
    -h, --help             Print help information
    -V, --version          Print version information

EXAMPLES:

To start the Drop Copy App with a custom configuration file:
    drop_copy_app.exe --config config.yaml

For more information, visit https://github.com/pedrosfaria2/drop_copy_rust
```

## Code Structure

- **main.rs**: Entry point of the application, sets up the command-line interface, and starts sessions.
- **config.rs**: Handles loading and parsing of the configuration file.
- **session.rs**: Manages the Drop Copy sessions and implements the FIX protocol interactions.
- **handler.rs**: Processes the incoming FIX messages.
- **custom_logger.rs**: Custom logging implementation for raw and human-readable messages.

## Example

```rust
// Example usage in main.rs

use config::AppConfig;
use session::start_sessions;
use handler::MessageHandler;
use std::sync::Arc;
use clap::{Arg, Command, ArgAction};

fn main() -> Result<(), quickfix::QuickFixError> {
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
            "EXAMPLES:\n\n             To start the Drop Copy App with a custom configuration file:\n             drop_copy_app.exe --config config.yaml\n\n             For more information, visit https://github.com/pedrosfaria2/drop_copy_rust \n\n\n"
        )
        .get_matches();

    println!("{}", "=".repeat(60));
    println!("Drop Copy App v0.1.0");
    println!("Author: Pedro Serrano Faria <pedroserrano2@gmail.com>");
    println!("{}", "=".repeat(60));
    println!();

    let config_file = matches.get_one::<String>("config").expect("Config file is required");
    let config = AppConfig::load(config_file).expect("Failed to load config");
    println!("Loaded config file: {}", config_file);
    println!("Configuration: {:#?}", config);

    let handler = Arc::new(MessageHandler::new());
    start_sessions(config, handler)?;

    Ok(())
}
```

## License

This project is licensed under the MIT License.
