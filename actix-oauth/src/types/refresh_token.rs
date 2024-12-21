use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Formatter};
use tosic_utils::wrap_external_type;
use utoipa::openapi::path::{Parameter, ParameterBuilder, ParameterIn};
use utoipa::openapi::{RefOr, Required, Schema};
use utoipa::{openapi, IntoParams, PartialSchema};

wrap_external_type! {
    #[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
    pub struct RefreshToken(oauth2::RefreshToken);
}

impl Debug for RefreshToken {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl RefreshToken {
    pub fn new(token: String) -> Self {
        Self(oauth2::RefreshToken::new(token))
    }
}

impl PartialSchema for RefreshToken {
    fn schema() -> RefOr<Schema> {
        openapi::schema::ObjectBuilder::new()
            .schema_type(openapi::schema::Type::String)
            .title("Refresh Token".into())
            .description(Some(
                "A Refresh token that can be used to get a new Access Token",
            ))
            .examples([
                "pml2Hh2M8EZgiydw5cyz7AYafSupjqD5yJgIrqaBs8unZc9SmP",
                "L34vlICPyqsfuakxuXz1Z8X6C0ap4ewaODYdT1Hr10krL1czK9",
            ])
            .into()
    }
}

impl utoipa::ToSchema for RefreshToken {}

impl IntoParams for RefreshToken {
    fn into_params(parameter_in_provider: impl Fn() -> Option<ParameterIn>) -> Vec<Parameter> {
        let parameter_in = parameter_in_provider().unwrap_or_default();

        let param = ParameterBuilder::new()
            .parameter_in(parameter_in)
            .required(Required::True)
            .schema(Some(Self::schema()))
            .description(Some(
                "Refresh token that can be exchanged for a new Access Token",
            ))
            .build();

        vec![param]
    }
}
