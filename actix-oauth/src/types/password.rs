use serde::{Deserialize, Serialize};
use tosic_utils::wrap_external_type;
use utoipa::openapi::path::{Parameter, ParameterBuilder, ParameterIn};
use utoipa::openapi::{KnownFormat, RefOr, Required, Schema, SchemaFormat};
use utoipa::{openapi, IntoParams, PartialSchema, ToSchema};

wrap_external_type! {
    #[derive(Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
    pub struct Password(String);
}

impl PartialSchema for Password {
    fn schema() -> RefOr<Schema> {
        openapi::schema::ObjectBuilder::new()
            .schema_type(openapi::schema::Type::String)
            .format(Some(SchemaFormat::KnownFormat(KnownFormat::Password)))
            .title("User password".into())
            .description(Some("Password used to log user in"))
            .into()
    }
}

impl ToSchema for Password {}

impl IntoParams for Password {
    fn into_params(parameter_in_provider: impl Fn() -> Option<ParameterIn>) -> Vec<Parameter> {
        let parameter_in = parameter_in_provider().unwrap_or_default();

        let param = ParameterBuilder::new()
            .parameter_in(parameter_in)
            .required(Required::True)
            .schema(Some(Self::schema()))
            .description(Some("User Password"))
            .build();

        vec![param]
    }
}
