use humantime;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::fs;
use toml;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub database: Option<DatabaseConfig>,
    pub server: Option<HTTPServerConfig>,
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database: String,
    pub retry: u16,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct HTTPServerConfig {
    pub host: String,
    pub port: u16,
    pub timeout: String,
    pub idle_timeout: String,
    pub debug: bool,
}

impl HTTPServerConfig {
    pub fn get_timeout(&self) -> Duration {
        self.timeout
            .parse::<humantime::Duration>()
            .map(|d| d.into())
            .unwrap_or(Duration::from_secs(4))
    }

    pub fn get_idle_timeout(&self) -> Duration {
        self.idle_timeout
            .parse::<humantime::Duration>()
            .map(|d| d.into())
            .unwrap_or(Duration::from_secs(60))
    }
}

impl Config {
    pub async fn new() -> Self {
        let config_path = if cfg!(debug_assertions) {
            println!("[DEV MODE]");
            "./src/config/dev.toml"
        } else {
            println!("[PROD MODE]");
            "prod.toml"
        };

        match fs::read_to_string(config_path).await {
            Ok(content) => toml::from_str(&content).unwrap_or_else(|err| {
                eprintln!("Error parsing config: {:?}", err);
                Self::default()
            }),
            Err(err) => {
                eprintln!("Error reading config file: {:?}", err);
                Self::default()
            }
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            database: None,
            server: None,
        }
    }
}
