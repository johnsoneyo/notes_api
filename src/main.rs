use crate::app::AppState;
use axum::{Router, middleware, routing::get};
use routes::notes::notes_routes;
use tokio::net::TcpListener;

mod app;
mod db;
mod handlers;
mod models;
mod routes;

mod auth;
mod state;

use auth::middleware::auth_middleware;

async fn health_check() -> &'static str {
    "ok"
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let state = AppState::new().await;

    let protected_routes = notes_routes().layer(middleware::from_fn_with_state(
        state.clone(),
        auth_middleware,
    ));

    let app = Router::new()
        .route("/health", get(health_check))
        .merge(protected_routes)
        .with_state(state);

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
