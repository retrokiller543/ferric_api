use crate::types::Scopes;
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Formatter};
use tosic_utils::wrap_external_type;
use utoipa::openapi::{RefOr, Schema};
use utoipa::{openapi, PartialSchema, ToSchema};

wrap_external_type! {
    #[derive(Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
    pub struct Scope(oauth2::Scope);
}

impl Debug for Scope {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl Scope {
    pub fn new(s: impl Into<String>) -> Self {
        Self(oauth2::Scope::new(s.into()))
    }

    pub fn into_scopes(self) -> Vec<String> {
        self.split(" ").map(str::to_string).collect()
    }
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        self
    }
}

impl From<Scopes> for Scope {
    fn from(value: Scopes) -> Self {
        Self::new(value.join(" "))
    }
}

impl PartialSchema for Scope {
    fn schema() -> RefOr<Schema> {
        openapi::schema::ObjectBuilder::new()
            .schema_type(openapi::schema::Type::String)
            .title("OAuth Scope String".into())
            .description(Some("A space delimited string of scopes"))
            .into()
    }
}

impl ToSchema for Scope {}
