use axum::{
    Json,
    extract::{Path, State},
};
use axum::http::StatusCode;
use crate::{
    state::AppState,
    models::note::*,
    db::notes as db,
};

pub async fn create_note(
    State(state): State<AppState>,
    Json(payload): Json<CreateNote>,
) -> Json<Note> {
    let note = db::create(&state.db, payload).await;
    Json(note)
}

pub async fn get_note(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<Note>, StatusCode> {
    match db::get(&state.db, id).await {
        Some(note) => Ok(Json(note)),
        None => Err(StatusCode::NOT_FOUND),
    }
}

/// GET /notes
pub async fn list_notes(
    State(state): State<AppState>,
) -> Json<Vec<Note>> {
    let notes = db::list(&state.db).await;
    Json(notes)
}

/// DELETE /notes/:id
pub async fn delete_note(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> &'static str {
    db::delete(&state.db, id).await;
    "deleted"
}