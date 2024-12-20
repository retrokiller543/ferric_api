use serde::{Deserialize, Serialize};
use tosic_utils::wrap_external_type;
use utoipa::openapi::path::{Parameter, ParameterBuilder, ParameterIn};
use utoipa::openapi::{RefOr, Required, Schema};
use utoipa::{openapi, IntoParams, PartialSchema};

wrap_external_type! {
    #[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
    pub struct AccessToken(oauth2::AccessToken);
}

impl AccessToken {
    pub fn new(token: String) -> Self {
        Self(oauth2::AccessToken::new(token))
    }
}

impl PartialSchema for AccessToken {
    fn schema() -> RefOr<Schema> {
        openapi::schema::ObjectBuilder::new()
            .schema_type(openapi::schema::Type::String)
            .description(Some("A Access token that can be used to access systems"))
            .title("Access Token".into())
            .into()
    }
}

impl utoipa::ToSchema for AccessToken {}

impl IntoParams for AccessToken {
    fn into_params(parameter_in_provider: impl Fn() -> Option<ParameterIn>) -> Vec<Parameter> {
        let parameter_in = parameter_in_provider().unwrap_or_default();

        let access_token_params = ParameterBuilder::new()
            .parameter_in(parameter_in)
            .required(Required::True)
            .schema(Some(Self::schema()))
            .description(Some(
                "Access token that can be used to access resources on the server",
            ))
            .build();

        vec![access_token_params]
    }
}
