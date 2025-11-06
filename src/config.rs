use crate::error::AppError;
use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_config: DbConfig,
    pub redis_config: RedisConfig,
    pub server_addr: String,
}

impl Config {
    pub fn from_env() -> Result<Self, AppError> {
        Ok(Self {
            database_config: DbConfig::from_env()?,
            redis_config: RedisConfig::from_env()?,
            server_addr: get_env_var("SERVER_ADDR")?,
        })
    }
}

fn get_env_var(key: &str) -> Result<String, AppError> {
    env::var(key).map_err(|_| AppError::Config(format!("Missing environment variable {}", key)))
}

#[derive(Debug, Clone)]
pub struct DbConfig {
    pub database_user: String,
    pub database_pass: String,
    pub database_host: String,
    pub database_port: String,
    pub database_name: String,
}

impl DbConfig {
    pub fn from_env() -> Result<Self, AppError> {
        Ok(Self {
            database_user: get_env_var("DATABASE_USER")?,
            database_pass: get_env_var("DATABASE_PASS")?,
            database_host: get_env_var("DATABASE_HOST")?,
            database_port: get_env_var("DATABASE_PORT")?,
            database_name: get_env_var("DATABASE_NAME")?,
        })
    }
}
#[derive(Debug, Clone)]
pub struct RedisConfig {
    pub redis_url: String,
}
impl RedisConfig {
    pub fn from_env() -> Result<Self, AppError> {
        Ok(Self {
            redis_url: get_env_var("REDIS_URL")?,
        })
    }
}