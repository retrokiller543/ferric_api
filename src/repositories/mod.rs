#![allow(dead_code)]

use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use sqlx_utils::pool::get_db_pool;
use utoipa::{ToResponse, ToSchema};

pub mod oauth_clients;
pub mod oauth_token;
pub mod users;

/// Health information and stats about the database the server is connected to.
#[derive(Debug, Serialize, Deserialize, ToSchema, ToResponse)]
pub struct DatabaseHealth {
    pub connected: bool,
    /// Database version information.
    #[schema(
        example = "PostgreSQL 17.2 (Debian 17.2-1.pgdg120+1) on aarch64-unknown-linux-gnu, compiled by gcc (Debian 12.2.0-14) 12.2.0, 64-bit"
    )]
    pub version: String,
    /// Current active connections.
    pub connection_count: i64,
    /// The max connections of the database.
    pub max_connections: i32,
    /// Update of the database in the format of BigDecimal String
    #[schema(example = "990035.37549000")]
    pub uptime: String,
    /// Current size of the stored data in the database.
    pub size_mb: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_vacuum: Option<chrono::DateTime<chrono::Utc>>,
}

pub(crate) async fn check_db_health() -> crate::ApiResult<DatabaseHealth> {
    let pool = get_db_pool();

    // Check if we can connect to the database
    let version = sqlx::query_scalar::<_, String>("SELECT version()")
        .fetch_one(pool)
        .await?;

    // Get connection statistics
    let conn_stats = sqlx::query!(
        r#"
        SELECT
            count(*) as "current_connections!",
            (SELECT setting::int FROM pg_settings WHERE name = 'max_connections') as "max_connections!"
        FROM pg_stat_activity
        "#
    )
        .fetch_one(pool)
        .await?;

    // Get database size
    let db_size = sqlx::query_scalar::<_, String>(
        "SELECT pg_size_pretty(pg_database_size(current_database()))",
    )
    .fetch_one(pool)
    .await?;

    // Convert pretty size to MB numeric value
    let size_mb = if db_size.contains("MB") {
        db_size
            .replace("MB", "")
            .trim()
            .parse::<f64>()
            .unwrap_or(0.0)
    } else if db_size.contains("GB") {
        db_size
            .replace("GB", "")
            .trim()
            .parse::<f64>()
            .unwrap_or(0.0)
            * 1024.0
    } else if db_size.contains("kB") {
        db_size
            .replace("kB", "")
            .trim()
            .parse::<f64>()
            .unwrap_or(0.0)
            / 1024.0
    } else {
        0.0
    };

    // Get uptime and last vacuum
    let db_stats = sqlx::query!(
        r#"
        SELECT
            extract(epoch from (now() - pg_postmaster_start_time())) as "uptime_seconds!",
            (SELECT max(last_vacuum) FROM pg_stat_user_tables) as "last_vacuum"
        "#
    )
    .fetch_one(pool)
    .await?;

    let uptime = db_stats.uptime_seconds.to_string();

    Ok(DatabaseHealth {
        connected: true,
        version,
        connection_count: conn_stats.current_connections,
        max_connections: conn_stats.max_connections,
        uptime,
        size_mb,
        last_vacuum: db_stats.last_vacuum,
    })
}
