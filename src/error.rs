use actix_web::body::BoxBody;
use actix_web::http::header::TryIntoHeaderValue;
use actix_web::http::{header, StatusCode};
use actix_web::{HttpResponse, ResponseError};
#[cfg(debug_assertions)]
use std::backtrace;
use thiserror::Error;
use tracing_core::dispatcher::SetGlobalDefaultError;
use validator::ValidationErrors;

use crate::dto::Error;

/// Error that could occur when the API is running and need to be sent back to the client
#[derive(Error, Debug)]
#[allow(dead_code)]
pub(crate) enum ApiError {
    #[error("Error: {0}")]
    Basic(String),
    #[error(transparent)]
    Validation(#[from] ValidationErrors),
    #[error(transparent)]
    Generic(#[from] Box<dyn std::error::Error>),
}

/// Errors that occurs on the server level and could cause the entire server to go down
#[derive(Error, Debug)]
#[allow(dead_code)]
pub(crate) enum ServerError {
    #[error("Error: {0}")]
    Basic(String),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Api(#[from] ApiError),
    #[error(transparent)]
    SetGlobalDefault(#[from] SetGlobalDefaultError),
    #[error(transparent)]
    Generic(#[from] Box<dyn std::error::Error>),
}

impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::Validation(..) => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
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
