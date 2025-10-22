#![allow(dead_code, unused_variables)]
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

#[derive(Debug, Clone)]
pub struct ShadowDbConfig {
    pub database_user: String,
    pub database_pass: String,
    pub database_host: String,
    pub database_port: String,
}

impl ShadowDbConfig {
    pub fn from_env() -> Result<Self, AppError> {
        Ok(Self {
            database_user: get_env_var("DATABASE_USER")?,
            database_pass: get_env_var("DATABASE_PASS")?,
            database_host: get_env_var("DATABASE_HOST")?,
            database_port: get_env_var("DATABASE_PORT")?,

        })
    }
}
#[derive(Debug, Clone)]
pub struct ShadowRedisConfig {
    pub redis_url: String,
}
impl ShadowRedisConfig {
    pub fn from_env() -> Result<Self, AppError> {
        Ok(Self {
            redis_url: get_env_var("REDIS_URL")?,
        })
    }
}