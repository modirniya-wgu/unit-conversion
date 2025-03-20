use serde::Deserialize;
use config::{Config, ConfigError, File};
use std::env;

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub log_level: String,
}

impl AppConfig {
    pub fn new() -> Result<Self, ConfigError> {
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());
        
        let mut s = Config::builder()
            // Start with default config
            .add_source(File::with_name("config/default").required(false))
            // Add environment specific config
            .add_source(File::with_name(&format!("config/{}", run_mode)).required(false))
            // Add local config
            .add_source(File::with_name("config/local").required(false))
            // Add environment variables
            .add_source(config::Environment::with_prefix("APP"))
            .build()?;

        // Convert to our config struct
        s.try_deserialize()
    }

    pub fn from_env() -> Self {
        Self {
            server: ServerConfig {
                host: env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
                port: env::var("PORT").unwrap_or_else(|_| "8080".to_string())
                    .parse::<u16>()
                    .expect("PORT must be a number"),
            },
            log_level: env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string()),
        }
    }
} 