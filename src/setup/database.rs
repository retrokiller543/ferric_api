use crate::{ServerResult, DATABASE_URL, DB_POOL};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::time::Duration;

#[inline]
pub(crate) async fn db_pool() -> ServerResult<PgPool> {
    Ok(PgPoolOptions::new()
        .max_connections(21)
        .min_connections(5)
        .idle_timeout(Duration::from_secs(60 * 10))
        .max_lifetime(Duration::from_secs(60 * 60 * 24))
        .acquire_timeout(Duration::from_secs(20))
        .connect(&*DATABASE_URL)
        .await?)
}

#[inline(always)]
pub(crate) async fn get_db_pool() -> ServerResult<&'static PgPool> {
    DB_POOL.get_or_try_init(db_pool).await
}
