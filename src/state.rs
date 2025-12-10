use crate::config::Config;
use crate::database::{postgres_db, redis_db};
use crate::error::AppError;
use sqlx::PgPool;
use std::sync::Arc;

pub type SharedAppState = Arc<AppState>;

pub struct AppState {
    pub db_pool: PgPool,
    pub redis_client: redis::Client,
    pub config: Config,
}

impl AppState {
    pub async fn new(config: &Config) -> Result<SharedAppState, AppError> {
        let db_pool = postgres_db::connect(&config.database_config).await?;
        let redis_client = redis_db::connect(&config.redis_config)?;

        Ok(Arc::new(AppState {
            db_pool,
            redis_client,
            config: config.clone(),
        }))
    }
}