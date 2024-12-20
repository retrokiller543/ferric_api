use serde::{Deserialize, Serialize};
use tosic_utils::wrap_external_type;
use utoipa::openapi::path::{Parameter, ParameterBuilder, ParameterIn};
use utoipa::openapi::{RefOr, Required, Schema};
use utoipa::{openapi, IntoParams, PartialSchema, ToSchema};

wrap_external_type! {
    #[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
    pub struct AuthorizationCode(oauth2::AuthorizationCode);
}

impl PartialSchema for AuthorizationCode {
    fn schema() -> RefOr<Schema> {
        openapi::schema::ObjectBuilder::new()
            .schema_type(openapi::schema::Type::String)
            .title("Authorization Code".into())
            .description(Some("A code that can be exchanged into a Access Token"))
            .into()
    }
}

impl ToSchema for AuthorizationCode {}

impl IntoParams for AuthorizationCode {
    fn into_params(parameter_in_provider: impl Fn() -> Option<ParameterIn>) -> Vec<Parameter> {
        let parameter_in = parameter_in_provider().unwrap_or_default();

        let param = ParameterBuilder::new()
            .parameter_in(parameter_in)
            .required(Required::True)
            .schema(Some(Self::schema()))
            .description(Some(
                "Authorization code that can be exchanged for an Access Token",
            ))
            .build();

        vec![param]
    }
}
