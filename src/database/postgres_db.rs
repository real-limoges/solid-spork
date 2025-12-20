use crate::config::DbConfig;
use crate::error::AppError;
use sqlx::{
    PgPool,
    postgres::{PgConnectOptions, PgPoolOptions},
};

// A very practical way to set up Postgres.
pub async fn connect(database_config: &DbConfig) -> Result<PgPool, AppError> {
    let connect_options = PgConnectOptions::new()
        .host(&database_config.database_host)
        .port(database_config.database_port.parse::<u16>()?)
        .username(&database_config.database_user)
        .password(&database_config.database_pass)
        .database(&database_config.database_name);

    let pool = PgPoolOptions::new().max_connections(5);

    let pool = pool
        .connect_with(connect_options)
        .await
        .map_err(AppError::Sqlx)?;

    Ok(pool)
}
