use sqlx::PgPool;
use std::sync::LazyLock;
use tokio::sync::OnceCell;
use tosic_utils::env::env_util;

pub static PORT: LazyLock<u32> = LazyLock::new(|| env_util!("PORT", 8000, u32));
pub static BASE_URL: LazyLock<String> =
    LazyLock::new(|| env_util!("BASE_URL", "http://localhost:8000"));
pub static DATABASE_URL: LazyLock<String> = LazyLock::new(|| env_util!("DATABASE_URL"));
pub static EXTERNAL_RESOURCES: LazyLock<Vec<(&'static str, &'static str)>> = LazyLock::new(|| {
    let Some(env_str) = option_env!("EXTERNAL_RESOURCES") else {
        return Vec::new();
    };

    env_str
        .trim()
        .split(';')
        .filter_map(|res| {
            let (key, value) = res.trim().split_once(':')?;
            /*let key = parts.next()?;
            let value = parts.next()?;*/
            Some((key, value))
        })
        .collect()
});
/// Main Database pool for the API
pub static DB_POOL: OnceCell<PgPool> = OnceCell::const_new();
