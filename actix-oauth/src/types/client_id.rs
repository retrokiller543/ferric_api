use crate::utils::random_string;
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Formatter};
use tosic_utils::wrap_external_type;
use utoipa::openapi::path::{Parameter, ParameterBuilder, ParameterIn};
use utoipa::openapi::{RefOr, Required, Schema};
use utoipa::{openapi, IntoParams, PartialSchema, ToSchema};

wrap_external_type! {
    #[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
    pub struct ClientId(oauth2::ClientId);
}

impl Debug for ClientId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl ClientId {
    pub fn new(id: impl Into<String>) -> Self {
        let id = id.into();

        Self(oauth2::ClientId::new(id))
    }

    pub fn new_random() -> Self {
        Self::new(random_string(128))
    }
}

impl PartialSchema for ClientId {
    fn schema() -> RefOr<Schema> {
        openapi::schema::ObjectBuilder::new()
            .schema_type(openapi::schema::Type::String)
            .title("Client Id".into())
            .description(Some("Part of the client credentials"))
            .into()
    }
}

impl ToSchema for ClientId {}

impl IntoParams for ClientId {
    fn into_params(parameter_in_provider: impl Fn() -> Option<ParameterIn>) -> Vec<Parameter> {
        let parameter_in = parameter_in_provider().unwrap_or_default();

        let param = ParameterBuilder::new()
            .parameter_in(parameter_in)
            .required(Required::True)
            .schema(Some(Self::schema()))
            .description(Some("ID part of the client credentials"))
            .build();

        vec![param]
    }
}
