use actix_web::body::{BoxBody, MessageBody};
use actix_web::dev::ServiceResponse;
use actix_web::http::header::{HeaderValue, TryIntoHeaderValue};
use actix_web::http::{StatusCode, header};
use actix_web::middleware::ErrorHandlerResponse;
use actix_web::{HttpResponse, ResponseError};
#[cfg(debug_assertions)]
use std::backtrace;
use thiserror::Error;
use tracing::error;
use tracing_core::dispatcher::SetGlobalDefaultError;
use validator::ValidationErrors;

use crate::dto::Error;

/// Error that could occur when the API is running and need to be sent back to the client
#[derive(Error, Debug)]
#[allow(dead_code)]
pub enum ApiError {
    #[error("Error: {0}")]
    Basic(String),
    #[error(transparent)]
    Validation(#[from] ValidationErrors),
    #[cfg_attr(debug_assertions, error("Database error occurred: {0}"))]
    #[cfg_attr(not(debug_assertions), error("Database error occurred"))]
    Postgres(#[from] sqlx::Error),
    #[cfg_attr(debug_assertions, error("Database error occurred: {0}"))]
    #[cfg_attr(not(debug_assertions), error("Database error occurred"))]
    Sql(#[from] sqlx_utils::Error),
    #[error("Failed to hash password")]
    Argon2(#[from] argon2::password_hash::errors::Error),
    #[error(transparent)]
    Llm(#[from] tosic_llm::error::LlmError),
    #[error(transparent)]
    Serde(#[from] serde_json::Error),
    #[error(transparent)]
    Generic(#[from] Box<dyn std::error::Error + Send>),
    #[error("An unknown error occurred")]
    InternalError,
}

/// Errors that occurs on the server level and could cause the entire server to go down
#[derive(Error, Debug)]
#[allow(dead_code)]
pub enum ServerError {
    #[error("Error: {0}")]
    Basic(String),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Api(#[from] ApiError),
    #[error(transparent)]
    SetGlobalDefault(#[from] SetGlobalDefaultError),
    #[error(transparent)]
    Postgres(#[from] sqlx::Error),
    #[error(transparent)]
    Generic(#[from] Box<dyn std::error::Error + Send>),
}

impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::Validation(..) => StatusCode::BAD_REQUEST,
            _ => {
                error!("An error occurred: {self}");
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        let mut res = HttpResponse::new(self.status_code());

        let mime = mime::APPLICATION_JSON.try_into_value().unwrap();
        res.headers_mut().insert(header::CONTENT_TYPE, mime);
        let code = self.status_code().to_string();

        let msg = self.to_string();
        let error = {
            #[cfg(debug_assertions)]
            let backtrace = backtrace::Backtrace::capture().to_string();
            Error {
                code: code.into(),
                error: msg.into(),
                #[cfg(debug_assertions)]
                stack_trace: backtrace.into(),
            }
        };

        let json = serde_json::to_string(&error).unwrap();

        res.set_body(BoxBody::new(json))
    }
}

pub fn default_error_handler<B: MessageBody + 'static>(
    res: ServiceResponse<B>,
) -> actix_web::Result<ErrorHandlerResponse<BoxBody>> {
    let (req, res) = res.into_parts();

    let mut headers = res.headers().clone();

    // Extract content type to check if the body is already JSON
    let content_type = headers
        .get(header::CONTENT_TYPE)
        .and_then(|h| h.to_str().ok());

    if matches!(content_type, Some(ct) if ct == mime::APPLICATION_JSON.as_ref() || ct == mime::TEXT_HTML_UTF_8.as_ref())
    {
        // If content type is already JSON, return the original response
        let orig_res = ServiceResponse::new(req, res)
            .map_into_boxed_body()
            .map_into_right_body();
        return Ok(ErrorHandlerResponse::Response(orig_res));
    }

    // Extract status and body before modifying
    let status = res.status();
    let body = res.into_body();
    let body_bytes = body.try_into_bytes().unwrap_or_default();

    // Convert the body to a string, fallback to a default error message
    let error_message = String::from_utf8_lossy(&body_bytes).to_string();

    // Build the JSON error response
    let error = {
        #[cfg(debug_assertions)]
        let backtrace = backtrace::Backtrace::capture().to_string();
        Error {
            code: status.to_string().into(),
            error: error_message.into(),
            #[cfg(debug_assertions)]
            stack_trace: backtrace.into(),
        }
    };

    let json = serde_json::to_string(&error).unwrap();

    // Update the response with the JSON body and proper content type
    headers.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static(mime::APPLICATION_JSON.as_ref()),
    );
    let mut res = HttpResponse::new(status);
    for (name, val) in headers {
        res.headers_mut().insert(name, val);
    }
    let new_res = res.set_body(BoxBody::new(json));

    // Create the final service response
    let final_res = ServiceResponse::new(req, new_res)
        .map_into_boxed_body()
        .map_into_right_body();

    Ok(ErrorHandlerResponse::Response(final_res))
}
