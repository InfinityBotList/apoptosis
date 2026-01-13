use std::{fs::File, sync::LazyLock};
use serde::{Deserialize, Serialize};
use crate::Error;

/// Global config object
pub static CONFIG: LazyLock<Config> = LazyLock::new(|| Config::load().expect("Failed to load config"));

#[derive(Serialize, Deserialize)]
pub struct Config {
    /// Base URL for CDN
    pub cdn_url: String,
    /// Path on disk for CDN storage
    pub cdn_path: String,
    /// Proxy URL for outgoing requests
    pub proxy_url: String,
}

impl Config {
    pub fn load() -> Result<Self, Error> {
        // Open config.yaml
        let file = File::open("config.yaml");

        match file {
            Ok(file) => {
                // Parse config.yaml
                let cfg: Config = serde_yaml::from_reader(file)?;

                // Return config
                Ok(cfg)
            }
            Err(e) => {
                // Print error
                println!("config.yaml could not be loaded: {}", e);

                // Exit
                std::process::exit(1);
            }
        }
    }
}