use crate::error::ApiError;
use crate::repositories::{DatabaseHealth, check_db_health};
use crate::state::AppState;
use crate::{ApiResult, dto};
use actix_web::{HttpResponse, Responder};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::process;
#[cfg(target_os = "linux")]
use sysinfo::Process;
use sysinfo::System;
use tracing::error;
use utoipa::{ToResponse, ToSchema};

dto! {
    /// Health and stats about the server
    #[derive(serde::Deserialize, Serialize, Debug, ToSchema, ToResponse)]
    pub struct ServerHealth {
        /// When the server was started.
        start_time: chrono::DateTime<Utc>,
        /// Stats about the process the server is running on.
        process_stats: ProcessStats,
        /// Information about the database.
        database: DatabaseHealth,
    }
}

/// Stats about the process the server is running on.
#[derive(Debug, Serialize, Deserialize, ToSchema, ToResponse)]
pub struct ProcessStats {
    /// Process id of server.
    pub pid: u32,
    /// Memory usage in kb (1024 bytes).
    pub memory_usage_kb: u64,
    #[cfg_attr(target_os = "linux", doc("The thread count of the server."))]
    #[cfg(target_os = "linux")]
    pub thread_count: usize,
    /// How long the process has been running for.
    pub uptime_seconds: u64,
}

pub fn get_process_stats() -> ProcessStats {
    let pid = process::id();
    let mut system = System::new_all();
    system.refresh_all();

    let process = system.process(sysinfo::Pid::from(pid as usize));
    #[cfg(target_os = "linux")]
    let thread_count = |process: &Process| {
        let tasks = process.tasks();

        if let Some(tasks) = tasks {
            tasks.len()
        } else {
            0
        }
    };

    ProcessStats {
        memory_usage_kb: process.map(|p| p.memory()).unwrap_or(0) / 1024,
        #[cfg(target_os = "linux")]
        thread_count: process.map(|p| thread_count(p)).unwrap_or(0),
        pid,
        uptime_seconds: process.map(|p| p.run_time()).unwrap_or(0),
    }
}

pub(crate) async fn check_health(state: &AppState) -> ApiResult<ServerHealth> {
    let database_health = match check_db_health().await {
        Ok(database_health) => database_health,
        Err(e) => {
            let message = "Failed to check health of database, database is most likely not running or cant be connected".to_string();
            error!(error = ?e, "{}", message);
            return Err(ApiError::FailedDependency {
                message,
                error: Some(Box::new(e)),
            });
        }
    };

    let process_stats = get_process_stats();

    let response = ServerHealth {
        database: database_health,
        start_time: state.started_at().clone(),
        process_stats,
    };

    Ok(response)
}
