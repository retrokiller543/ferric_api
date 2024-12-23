use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Formatter};
use tosic_utils::wrap_external_type;
use utoipa::openapi::path::{Parameter, ParameterBuilder, ParameterIn};
use utoipa::openapi::{KnownFormat, RefOr, Required, Schema, SchemaFormat};
use utoipa::{openapi, IntoParams, PartialSchema, ToSchema};

wrap_external_type! {
    #[derive(Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
    pub struct RedirectUri(oauth2::RedirectUrl);
}

impl RedirectUri {
    pub fn new(url: impl Into<String>) -> Self {
        Self(oauth2::RedirectUrl::new(url.into()).expect("Failed to parse URL"))
    }
}

impl Debug for RedirectUri {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl PartialSchema for RedirectUri {
    fn schema() -> RefOr<Schema> {
        openapi::schema::ObjectBuilder::new()
            .schema_type(openapi::schema::Type::String)
            .format(Some(SchemaFormat::KnownFormat(KnownFormat::UriTemplate)))
            .title("Redirect URI".into())
            .description(Some(
                "URI that the server will redirect to after authentication is completed",
            ))
            .examples([
                "https://client.example.com/callback",
                "http://localhost/redirect",
            ])
            .into()
    }
}

impl ToSchema for RedirectUri {}

impl IntoParams for RedirectUri {
    fn into_params(parameter_in_provider: impl Fn() -> Option<ParameterIn>) -> Vec<Parameter> {
        let parameter_in = parameter_in_provider().unwrap_or_default();

        let param = ParameterBuilder::new()
            .parameter_in(parameter_in)
            .required(Required::True)
            .schema(Some(Self::schema()))
            .description(Some("Redirect URI to use"))
            .build();

        vec![param]
    }
}
