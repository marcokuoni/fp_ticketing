mod domain;
mod error;
mod http;
mod service;
mod state;

use std::net::SocketAddr;

use state::{AppState, InMemoryStore};
use tower_http::cors::{Any, CorsLayer};
use tracing::info;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .compact()
        .init();

    let state = AppState::new(InMemoryStore::default());
    let app = http::router(state).layer(
        CorsLayer::new()
            .allow_origin(Any)
            .allow_methods(Any)
            .allow_headers(Any),
    );

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("bind tcp listener");

    info!("API listening on {addr}");
    axum::serve(listener, app).await.expect("serve axum app");
}
