//! Contains almost all configurations and setup needed to run the app, this includes macros to actually construct the
//! app and server so that we can more simply run the app in tests in the same way we run the app in production

pub mod app;
pub mod database;
pub mod server;

use tracing_appender::non_blocking::WorkerGuard;

use crate::env::init_env;
use crate::logging::init_tracing;
use crate::ServerResult;

#[inline]
pub async fn setup() -> ServerResult<WorkerGuard> {
    init_env()?;

    init_tracing()
}
