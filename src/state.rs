use crate::config::Config;
use crate::data::{postgres_db, redis_db};
use crate::error::AppError;
use sqlx::PgPool;
use std::sync::Arc;

pub type AppState = Arc<AppStateInner>;

pub struct AppStateInner {
    pub db_pool: PgPool,
    pub redis_client: redis::Client,
    pub config: Config,
}

impl AppStateInner {
    pub async fn new(config: &Config) -> Result<AppState, AppError> {
        let db_pool = postgres_db::connect(&config.database_url).await?;
        let redis_client = redis_db::connect(&config.redis_url)?;

        Ok(Arc::new(AppStateInner {
            db_pool,
            redis_client,
            config: config.clone(),
        }))
    }
}