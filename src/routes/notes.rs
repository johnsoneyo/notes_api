
use axum::{Router, routing::{get, post}};

use crate::handlers::notes::*;

pub fn notes_routes() -> Router<crate::AppState> {
    Router::new()
        .route("/notes", post(create_note).get(list_notes))
        .route("/notes/{id}", get(get_note).delete(delete_note))
}