use sqlx::PgPool;

use crate::models::note::*;

pub async fn create(pool: &PgPool, input: CreateNote) -> Result<Note, sqlx::Error> {
    sqlx::query_as!(
        Note,
        r#"
        INSERT INTO notes (title, content)
        VALUES ($1, $2)
        RETURNING id, title, content
        "#,
        input.title,
        input.content
    )
    .fetch_one(pool)
    .await
    .map(|note| {
        println!("Created note: {:?}", note);
        note
    })
}

pub async fn get(pool: &PgPool, id: i32) -> Option<Note> {
    sqlx::query_as!(
        Note,
        r#"
        SELECT id, title, content
        FROM notes
        WHERE id = $1
        "#,
        id
    )
    .fetch_optional(pool)
    .await
    .ok()
    .flatten()
}

pub async fn list(p0: &PgPool) -> Vec<Note> {
    sqlx::query_as!(
        Note,
        r#"
        SELECT id, title, content
        FROM notes
        ORDER BY id DESC
        "#
    )
    .fetch_all(p0)
    .await
    .unwrap_or_default()
}

pub async fn delete(p0: &PgPool, p1: i32) {
    sqlx::query!(
        r#"
        DELETE FROM notes
        WHERE id = $1
        "#,
        p1
    )
    .execute(p0)
    .await
    .ok();  
}
