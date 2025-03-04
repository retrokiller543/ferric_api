use crate::prelude::*;
use crate::services::health::ServerHealth;
use crate::state::AppState;
use crate::{error::ApiError, services::health::check_health};
use actix_helper_utils::generate_endpoint;
use actix_web::web;

generate_endpoint! {
    /// Health Endpoint
    ///
    /// Basic health endpoint to see if server up and running and working as expected.
    // Add more to the inner service to improve the check.
    fn health;
    method: get;
    path: "/health";
    error: ApiError;
    return_type: ServerHealth;
    docs: {
        tag: "health",
        context_path: "/",
        responses: {
            (status = 200, description = "Server is up and running and no direct issues found", body = ServerHealth),
            (status = 424, description = "Failed to check health of a dependency", body = Error),
            (status = 500, description = "Internal Server Error", body = Error),
        }
    }
    params: {
        state: web::Data<AppState>
    }
    {
        let state = state.into_inner();
        check_health(&state).await
    }
}
