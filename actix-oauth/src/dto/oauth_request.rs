use crate::types::{AuthorizationCode, ClientId, ClientSecret, Password, RefreshToken, Username};
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json::json;
use utoipa::openapi::path::{Parameter, ParameterBuilder, ParameterIn};
use utoipa::openapi::{KnownFormat, ObjectBuilder, Required, SchemaFormat, Type};
use utoipa::{IntoParams, ToResponse, ToSchema};

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, ToSchema, ToResponse)]
#[serde(tag = "grant_type", rename_all = "snake_case")]
pub enum OauthRequest {
    #[schema(example = json!({
        "grant_type": "password",
        "username": "john-doe",
        "password": "password"
    }))]
    Password {
        username: Username,
        password: Password,
    },
    #[schema(example = json!({
        "grant_type": "authorization_code",
        "code": "abc",
        "redirect_url": "http://localhost/redirect",
        "client_id": "client_id",
        "client_secret": "client-secret"
    }))]
    AuthorizationCode {
        code: AuthorizationCode,
        redirect_url: String,
        client_id: ClientId,
        client_secret: ClientSecret,
    },
    #[schema(example = json!({
        "grant_type": "client_credentials",
        "client_id": "client_id",
        "client_secret": "client-secret"
    }))]
    ClientCredentials {
        client_id: ClientId,
        client_secret: ClientSecret,
    },
    #[schema(example = json!({
        "grant_type": "refresh_token",
        "refresh_token": "token"
    }))]
    RefreshToken {
        client_id: Option<ClientId>,
        client_secret: Option<ClientSecret>,
        refresh_token: RefreshToken,
    },
}

impl IntoParams for OauthRequest {
    fn into_params(parameter_in_provider: impl Fn() -> Option<ParameterIn>) -> Vec<Parameter> {
        let parameter_in = parameter_in_provider().unwrap_or(ParameterIn::Query);

        // Common grant_type parameter
        let grant_type_param = ParameterBuilder::new()
            .name("grant_type")
            .parameter_in(parameter_in.clone())
            .required(Required::True)
            .schema(Some(
                ObjectBuilder::new()
                    .schema_type(Type::String)
                    .enum_values(Some(vec![
                        "password",
                        "authorization_code",
                        "client_credentials",
                        "refresh_token",
                    ]))
                    .build(),
            ))
            .description(Some("Type of grant being requested"))
            .build();

        // Collect parameters for each variant
        let mut parameters = vec![grant_type_param];

        let mut params = Username::into_params(|| Some(parameter_in.clone()));
        params.extend(Password::into_params(|| Some(parameter_in.clone())));
        params.extend(AuthorizationCode::into_params(|| {
            Some(parameter_in.clone())
        }));
        params.push(
            ParameterBuilder::new()
                .name("redirect_url")
                .parameter_in(parameter_in.clone())
                .required(Required::True)
                .schema(Some(
                    ObjectBuilder::new()
                        .schema_type(Type::String)
                        .format(Some(SchemaFormat::KnownFormat(KnownFormat::UriTemplate)))
                        .build(),
                ))
                .description(Some("Redirect URL for authorization code"))
                .build(),
        );
        params.extend(ClientId::into_params(|| Some(parameter_in.clone())));
        params.extend(ClientSecret::into_params(|| Some(parameter_in.clone())));
        params.extend(ClientId::into_params(|| Some(parameter_in.clone())));
        params.extend(ClientSecret::into_params(|| Some(parameter_in.clone())));
        params.extend(ClientId::into_params(|| Some(parameter_in.clone())));
        params.extend(ClientSecret::into_params(|| Some(parameter_in.clone())));
        params.extend(RefreshToken::into_params(|| Some(parameter_in.clone())));

        parameters.extend(params);

        parameters
    }
}
