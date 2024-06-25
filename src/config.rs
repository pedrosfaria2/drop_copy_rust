use serde::{Deserialize};
use std::fs::File;
use std::io::Read;
use std::error::Error;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub settings: String,
}

impl AppConfig {
    pub fn load(filename: &str) -> Result<AppConfig, Box<dyn Error>> {
        let mut file = File::open(filename)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let config: AppConfig = serde_yaml::from_str(&contents)?;
        Ok(config)
    }
}
