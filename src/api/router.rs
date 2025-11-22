#![allow(unused_imports)]

use crate::state::SharedAppState;
use axum::{
    Router,
    routing::{get, post},
};
use std::net::SocketAddr;

pub async fn run_server(app_state: SharedAppState) -> anyhow::Result<()> {
    let app = create_router(app_state.clone());
    let addr: SocketAddr = app_state
        .config
        .server_addr
        .parse()
        .expect("Failed to parse SERVER_ADDR!");

    println!("Server Listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}

// Add more routes here! 
fn create_router(app_state: SharedAppState) -> Router {
    Router::new()
        .route("/", get(crate::api::handlers::root_handler))
        .route("/do_something", get(crate::api::handlers::do_something_handler))
        .with_state(app_state)
}