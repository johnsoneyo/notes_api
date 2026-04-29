
use axum::Router;
use tokio::net::TcpListener;
use routes::notes::notes_routes;
use crate::app::AppState;

mod routes;
mod handlers;
mod models;
mod db;
mod app;

mod state;

#[tokio::main]
async fn main() {
    // 🔥 MUST BE FIRST LINE
    dotenvy::dotenv().ok();
    let state = AppState::new().await;

    let app = Router::new()
        .merge(notes_routes())
        .with_state(state);

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}