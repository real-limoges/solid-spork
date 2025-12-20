use crate::config::RedisConfig;
use crate::error::AppError;
use redis;

// Stupidly simple Redis connection.
pub fn connect(redis_config: &RedisConfig) -> Result<redis::Client, AppError> {
    redis::Client::open(redis_config.redis_url.clone()).map_err(AppError::Redis)
}
