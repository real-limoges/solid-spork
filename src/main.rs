#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use solid_spork::{api::run_server, config::Config, state::AppStateInner};

#[tokio::main]
async fn main() -> anyhow::Result<()> {

    dotenvy::dotenv().expect("Failed to load .env file");
    tracing_subscriber::fmt().with_target(false).init();

    let config = Config::from_env()?;
    // I don't really like how this is implemented right now.
    let app_state = AppStateInner::new(&config).await?;

    println!(
        "Starting server on https://{} ...",
        app_state.config.server_addr
    );
    run_server(app_state).await?;

    Ok(())
}
