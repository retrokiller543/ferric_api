use crate::ServerResult;
use crate::statics::DATABASE_URL;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use std::time::Duration;

#[inline]
#[tracing::instrument]
pub async fn db_pool() -> ServerResult<PgPool> {
    Ok(PgPoolOptions::new()
        .max_connections(21)
        .min_connections(5)
        .idle_timeout(Duration::from_secs(60 * 10))
        .max_lifetime(Duration::from_secs(60 * 60 * 24))
        .acquire_timeout(Duration::from_secs(20))
        .connect(&DATABASE_URL)
        .await?)
}
