
# Drop Copy App

**Version**: 0.1.0  
**Author**: Pedro Serrano Faria (pedroserrano2@gmail.com)

## Overview

The Drop Copy App is a Rust-based application designed to manage multiple Drop Copy sessions. It supports configurations via YAML files and provides custom logging for both raw and human-readable FIX protocol messages.

## FIX Protocol

The Financial Information Exchange (FIX) protocol is an electronic communications protocol initiated in 1992 for international real-time exchange of information related to securities transactions and markets. FIX is a widely used protocol by various financial institutions, enabling the exchange of trading information.

### QuickFIX-rs Library

This application utilizes the `quickfix-rs` library, which is a Rust implementation of the QuickFIX engine. It handles the low-level details of the FIX protocol, allowing developers to focus on the business logic of their applications.

## Features

- Manages multiple Drop Copy sessions simultaneously
- Custom logging of FIX protocol messages in both raw and human-readable formats
- Configurable via YAML files
- Command-line interface for easy usage


## Logging

All logs, including raw and human-readable messages, are consolidated into a single file for each type. This makes it easier for users to track and analyze the sessions. The log files are stored in the `logs` directory:

- `logs/raw.log`: Contains raw FIX messages.
- `logs/human_readable.log`: Contains human-readable FIX messages with delimiters replaced for better readability.

## Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
- [QuickFIX library](https://github.com/quickfix/quickfix)
- [Clap](https://crates.io/crates/clap)
- [CMake](https://cmake.org/)

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

## License

This project is licensed under the MIT License.

## Important Notice

This code is an example implementation of a Drop Copy application using the QuickFIX-rs library. If you intend to use this code in a production environment, please review it thoroughly and make the necessary adaptations, improvements, and implementations specific to your use case.
