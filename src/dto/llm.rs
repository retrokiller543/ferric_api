use crate::dto;
use actix_web::FromRequest;
use serde::{Deserialize, Serialize};
use tosic_llm::types::LlmMessages;
use utoipa::ToSchema;
use validator::Validate;

dto! {
    /// Request to communicate with a LLM.
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
