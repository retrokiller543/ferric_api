use serde::{Deserialize, Serialize};
use tosic_utils::wrap_external_type;
use utoipa::openapi::path::{Parameter, ParameterBuilder, ParameterIn};
use utoipa::openapi::{RefOr, Required, Schema};
use utoipa::{openapi, IntoParams, PartialSchema, ToSchema};

wrap_external_type! {
    #[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
    pub struct ClientSecret(oauth2::ClientSecret);
}

impl ClientSecret {
    pub fn new(secret: oauth2::ClientSecret) -> Self {
        Self(secret)
    }
}

impl PartialSchema for ClientSecret {
    fn schema() -> RefOr<Schema> {
        openapi::schema::ObjectBuilder::new()
            .schema_type(openapi::schema::Type::String)
            .title("Client Secret".into())
            .description(Some("Part of the client credentials"))
            .into()
    }
}

impl ToSchema for ClientSecret {}

impl IntoParams for ClientSecret {
    fn into_params(parameter_in_provider: impl Fn() -> Option<ParameterIn>) -> Vec<Parameter> {
        let parameter_in = parameter_in_provider().unwrap_or_default();

        let param = ParameterBuilder::new()
            .parameter_in(parameter_in)
            .required(Required::True)
            .schema(Some(Self::schema()))
            .description(Some("Secret part of the client credentials"))
            .build();

        vec![param]
    }
}
