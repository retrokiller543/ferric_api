use crate::{error::ApiError, services::health::check_health};
use actix_helper_utils::generate_endpoint;

generate_endpoint! {
    /// Basic health endpoint to see if server up and running and working as expected.
    ///
    /// Add more to the inner service to improve the check.
    fn health;
    method: get;
    path: "/health";
    error: ApiError;
    docs: {
        tag: "health",
        context_path: "/",
        responses: {
            (status = 200, description = "Everything works just fine!")
        }
    }
    {
        check_health().await
    }
}
