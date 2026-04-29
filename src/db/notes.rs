

use sqlx::PgPool;

use crate::models::note::*;

pub async fn create(pool: &PgPool, input: CreateNote) -> Note {
    // simplified example
    Note {
        id: 1,
        title: input.title,
        content: input.content,
    }
}

pub async fn get(pool: &PgPool, id: i64) -> Option<Note> {
    // query DB here
    None
}

pub async fn list(p0: &PgPool) ->  Vec<Note> {
    vec![
        Note {
            id: 1,
            title: "First note".into(),
            content: "This is a dummy note".into(),
        },
        Note {
            id: 2,
            title: "Second note".into(),
            content: "Another dummy note".into(),
        },
        Note {
            id: 3,
            title: "Rust Axum note".into(),
            content: "Learning Axum + SQLx".into(),
        },
    ]
}

pub async fn delete(p0: &PgPool, p1: i64) {
    todo!()
}