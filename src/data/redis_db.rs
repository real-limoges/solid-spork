use crate::error::AppError;
use redis;
use crate::config::RedisConfig;

// Stupidly simple Redis connection.
pub fn connect(redis_config: &RedisConfig) -> Result<redis::Client, AppError> {
    redis::Client::open(redis_config.redis_url.clone()).map_err(AppError::Redis)
}