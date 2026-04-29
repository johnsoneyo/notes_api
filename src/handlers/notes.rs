use crate::{AppState, db::notes as db, models::note::*};
use axum::http::StatusCode;
use axum::{
    Json,
    extract::{Path, State},
};

pub async fn create_note(
    State(state): State<AppState>,
    Json(payload): Json<CreateNote>,
) -> Json<Note> {
    let note = db::create(&state.db.pool, payload).await;
    Json(note.unwrap())
}

pub async fn get_note(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<Note>, StatusCode> {
    match db::get(&state.db.pool, id).await {
        Some(note) => Ok(Json(note)),
        None => Err(StatusCode::NOT_FOUND),
    }
}

/// GET /notes
pub async fn list_notes(State(state): State<AppState>) -> Json<Vec<Note>> {
    let notes = db::list(&state.db.pool).await;
    Json(notes)
}

/// DELETE /notes/:id
pub async fn delete_note(State(state): State<AppState>, Path(id): Path<i32>) -> &'static str {
    db::delete(&state.db.pool, id).await;
    "deleted"
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{config::Config, db::DbState, http::HttpState};
    use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
    use std::time::Duration;

    fn test_state() -> AppState {
        let connect_options = PgConnectOptions::new()
            .host("127.0.0.1")
            .port(1)
            .username("postgres")
            .password("postgres")
            .database("notes_api_test");
        let pool = PgPoolOptions::new()
            .max_connections(1)
            .acquire_timeout(Duration::from_millis(50))
            .connect_lazy_with(connect_options);

        AppState {
            db: DbState { pool },
            config: Config {
                database_url: "test database disabled".to_string(),
                port: 5432,
            },
            http: HttpState {
                client: reqwest::Client::new(),
            },
        }
    }

    #[tokio::test]
    async fn get_note_returns_not_found_when_note_is_missing() {
        let response = get_note(State(test_state()), Path(42)).await;

        assert!(matches!(response, Err(StatusCode::NOT_FOUND)));
    }

    #[tokio::test]
    async fn list_notes_returns_empty_vec_when_query_fails() {
        let Json(notes) = list_notes(State(test_state())).await;

        assert!(notes.is_empty());
    }

    #[tokio::test]
    async fn delete_note_returns_deleted_message() {
        let response = delete_note(State(test_state()), Path(42)).await;

        assert_eq!(response, "deleted");
    }

    #[tokio::test]
    #[should_panic]
    async fn create_note_panics_when_insert_fails() {
        let payload = CreateNote {
            title: "Test note".to_string(),
            content: "Test content".to_string(),
        };

        let _ = create_note(State(test_state()), Json(payload)).await;
    }
}
