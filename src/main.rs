use solid_spork::{
    api::run_server, 
    config::Config, 
    state::AppState
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {

    // I'm using a .env file for configuration.
    dotenvy::dotenv().expect("Failed to load .env file");
    tracing_subscriber::fmt().with_target(false).init();

    // Load the configuration . AppState is an Arc
    let config = Config::from_env()?;
    let app_state = AppState::new(&config).await?;

    println!("Starting server on https://{} ...",  app_state.config.server_addr);
    run_server(app_state).await?;

    Ok(())
}
