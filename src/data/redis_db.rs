use crate::error::AppError;
use redis;

pub fn connect(redis_url: &str) -> Result<redis::Client, AppError> {
    redis::Client::open(redis_url).map_err(|e| AppError::Redis(e))
}