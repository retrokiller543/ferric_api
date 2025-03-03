use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use std::path::Path;
use tosic_utils::env::env_util;

#[inline]
pub(crate) async fn db_pool() -> PgPool {
    PgPoolOptions::new()
        .max_connections(21)
        .min_connections(5)
        .connect(&env_util!("DATABASE_URL"))
        .await
        .expect("Failed to connect to database")
}

// generated by `sqlx migrate build-script`
#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("No .env file to load from");
    // trigger recompilation when a new migration is added
    println!("cargo:rerun-if-changed=migrations");

    let migrator = sqlx::migrate::Migrator::new(Path::new("./migrations"))
        .await
        .expect("Failed to find migrations");

    let pool = db_pool().await;
    migrator.run(&pool).await.expect("Failed to run migrations")
}
