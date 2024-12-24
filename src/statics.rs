use sqlx::PgPool;
use std::sync::LazyLock;
use tokio::sync::OnceCell;
use tosic_utils::env::env_util;

pub(crate) static PORT: LazyLock<u32> = LazyLock::new(|| env_util!("PORT", 8000, u32));
pub(crate) static BASE_URL: LazyLock<String> =
    LazyLock::new(|| env_util!("BASE_URL", "http://localhost:8000"));
pub(crate) static DATABASE_URL: LazyLock<String> = LazyLock::new(|| env_util!("DATABASE_URL"));
/// Main Database pool for the API
pub(crate) static DB_POOL: OnceCell<PgPool> = OnceCell::const_new();
