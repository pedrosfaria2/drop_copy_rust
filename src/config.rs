use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::io::Read;

// Struct to represent the application configuration
#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub settings: Vec<String>, // A vector of strings representing session configuration file paths
}

impl AppConfig {
    // Loads the configuration from a specified YAML file
    pub fn load(filename: &str) -> Result<AppConfig, Box<dyn Error>> {
        let mut file = File::open(filename)?; // Open the file
        let mut contents = String::new();
        file.read_to_string(&mut contents)?; // Read the file contents into a string
        let config: AppConfig = serde_yaml::from_str(&contents)?; // Deserialize the YAML string into AppConfig
        Ok(config) // Return the loaded configuration
    }
}
