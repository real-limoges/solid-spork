use crate::error::AppError;
use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub redis_url: String,
    pub server_addr: String,
}

impl Config {
    pub fn from_env() -> Result<Self, AppError> {
        Ok(Self {
            database_url: get_env_var("DATABASE_URL")?,
            redis_url: get_env_var("REDIS_URL")?,
            server_addr: get_env_var("SERVER_ADDR")?,
        })
    }
}

fn get_env_var(key: &str) -> Result<String, AppError> {
    env::var(key).map_err(|_| AppError::Config(format!("Missing environment variable {}", key)))
}