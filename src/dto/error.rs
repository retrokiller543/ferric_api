use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use utoipa::{ToResponse, ToSchema};

/// Error sent back to the client.
#[cfg_attr(
    debug_assertions,
    doc = "\nWhen debug mode is on this also includes a stacktrace to help debug issues."
)]
#[derive(Serialize, Deserialize, ToSchema, ToResponse)]
#[response(
    description = "Error was sent back to the client",
    content_type = "application/json"
)]
#[response(
    example = json!({"error": "Error: This is a error message", "code": "400 Bad Request"}),
)]
pub(crate) struct Error<'a> {
    /// The error message we got.
    pub(crate) error: Cow<'a, str>,
    /// Status code in a human-readable format.
    pub(crate) code: Cow<'a, str>,
    #[cfg(debug_assertions)]
    /// Stacktrace of the request after the error occurred.
    pub(crate) stack_trace: Cow<'a, str>,
}
