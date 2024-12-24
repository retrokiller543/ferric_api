use crate::ServerResult;
/*use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::fmt::Layer as FmtLayer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{filter::LevelFilter, EnvFilter, Layer, Registry};*/
use tosic_utils::logging::init_tracing as tracing_config;
use tracing_appender::non_blocking::WorkerGuard;
/*#[inline]
/// Initialize tracing with default settings
pub fn init_tracing() -> ServerResult<()> {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        #[cfg(not(debug_assertions))]
        let level = LevelFilter::INFO;

        #[cfg(debug_assertions)]
        let level = LevelFilter::DEBUG;

        EnvFilter::builder()
            .with_default_directive(level.into())
            .from_env_lossy()
    });
    let def_layer = FmtLayer::new()
        .with_line_number(true)
        .with_span_events(FmtSpan::CLOSE)
        .with_level(true)
        .with_target(true)
        .with_thread_names(true)
        .compact()
        .with_filter(filter);

    let subscriber = Registry::default().with(def_layer);

    tracing::subscriber::set_global_default(subscriber)?;

    Ok(())
}
*/

/// Initialize tracing with default settings
#[inline]
pub fn init_tracing() -> ServerResult<WorkerGuard> {
    let guard = tracing_config("api.logs")?;

    Ok(guard)
}
