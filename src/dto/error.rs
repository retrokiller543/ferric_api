use crate::dto;
use actix_oauth::impl_responder;
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json::json;
use std::borrow::Cow;
use utoipa::{ToResponse, ToSchema};

dto! {
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
    pub struct ErrorDTO<'a> {
        /// The error message we got.
        #[serde(skip_serializing_if = "str::is_empty")]
        pub error: Cow<'a, str>,
        /// Status code in a human-readable format.
        #[serde(skip_serializing_if = "str::is_empty")]
        pub code: Cow<'a, str>,
        /// Stacktrace of the request after the error occurred.
        #[cfg(debug_assertions)]
        #[serde(skip_serializing_if = "str::is_empty")]
        pub stack_trace: Cow<'a, str>,
    }
}
