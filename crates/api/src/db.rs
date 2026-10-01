//! Database connectivity for the API.
//!
//! We start with PostgreSQL: the conceptual graph is the *domain model*; physical
//! storage can evolve when scale/query patterns justify it.

use sqlx::postgres::{PgPool, PgPoolOptions};

pub async fn connect(database_url: &str) -> Result<PgPool, sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;
    // Fail fast if the DB is unreachable at boot rather than on first request.
    sqlx::query("SELECT 1").execute(&pool).await?;
    Ok(pool)
}
