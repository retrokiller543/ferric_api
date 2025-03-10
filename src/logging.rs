use crate::ServerResult;
use crate::constants::{LOG_DIR_ENV_VAR, LOG_FILE_ENV_VAR, LOG_PATH_ENV_VAR};
use std::path::Path;
use tosic_utils::logging::init_tracing_layered;
use tracing_appender::non_blocking::WorkerGuard;

/// Guard type returned by tracing initialization to keep the non-blocking logger alive
pub type TracingGuard = Option<WorkerGuard>;

/// Initialize tracing with support for both stdout and file-based logging.
///
/// # Logging Configuration
///
/// This function configures tracing based on environment variables with the following priority:
///
/// 1. [`LOG_PATH_ENV_VAR`] - Complete path to the log file (directory + filename)
///    - If this path points to a file, it will be used directly
///    - Takes highest priority when set
///
/// 2. Individual components:
///    - [`LOG_FILE_ENV_VAR`] - Just the log filename
///    - [`LOG_DIR_ENV_VAR`] - Directory where logs should be stored
///    - If only filename is provided, defaults to "logs" directory
///
/// # Behavior
///
/// - Stdout logging is always enabled
/// - File logging is only enabled if valid logging paths are determined
/// - Returns a guard that must be kept alive for the duration of the program
///   to ensure non-blocking logging works correctly
///
/// # Returns
///
/// Returns [`TracingGuard`] wrapped in [`ServerResult`] which must be stored in a variable
/// to prevent premature logger shutdown.
///
/// # Examples
///
/// ```
/// use ferric_api::logging::init_tracing;
///
/// // Basic usage - store the guard to keep logger alive
/// let _guard = init_tracing()?;
/// ```
#[inline]
pub fn init_tracing() -> ServerResult<TracingGuard> {
    let log_path_result = std::env::var(LOG_PATH_ENV_VAR).ok();
    let log_file_str = std::env::var(LOG_FILE_ENV_VAR).ok();
    let log_dir_str = std::env::var(LOG_DIR_ENV_VAR).ok();

    let (log_dir, log_file) = if let Some(ref log_path_str) = log_path_result {
        let log_path = Path::new(log_path_str);

        if log_path.is_file() || !log_path.exists() {
            let file_name = log_path.file_name();
            let parent_dir = log_path.parent();
            (parent_dir, file_name)
        } else {
            (Some(log_path), None)
        }
    } else {
        let file = log_file_str.as_ref().map(AsRef::as_ref);

        let dir = log_dir_str.as_ref().map(Path::new);

        (dir, file)
    };

    let logging_paths = match (log_dir, log_file) {
        (Some(dir), Some(file)) => Some((dir, file)),
        (None, Some(file)) => Some((Path::new("logs"), file)),
        (Some(dir), None) => Some((dir, "ferric_api.log".as_ref())),
        _ => None,
    };

    let guard = init_tracing_layered(logging_paths)?;

    Ok(guard)
}
