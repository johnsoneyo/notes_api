use sqlx::PgPool;

#[derive(Clone)]
pub struct DbState {
    pub pool: PgPool,
}
