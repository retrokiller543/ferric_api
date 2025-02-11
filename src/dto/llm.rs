use crate::dto;
use actix_web::dev::Payload;
use actix_web::web::Json;
use actix_web::{Error, FromRequest, HttpRequest};
use futures_util::future::BoxFuture;
use serde::{Deserialize, Serialize};
use std::future::Future;
use std::pin::Pin;
use tosic_llm::gemini::GeminiContent;
use tosic_llm::types::LlmMessages;
use utoipa::ToSchema;
use validator::Validate;

dto! {
    #[derive(
        Default,
        Debug,
        Clone,
        Hash,
        Eq,
        PartialEq,
        Ord,
        PartialOrd,
        Serialize,
        Deserialize,
        ToSchema,
        Validate
    )]
    pub struct LlmRequest {
        pub contents: LlmMessages,
        #[serde(default)]
        pub stream: bool,
    }
}
