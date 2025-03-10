use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt::{Debug, Formatter};
use tosic_utils::wrap_external_type;
use utoipa::openapi::path::{Parameter, ParameterBuilder, ParameterIn};
use utoipa::openapi::{KnownFormat, RefOr, Required, Schema, SchemaFormat};
use utoipa::{IntoParams, PartialSchema, ToSchema, openapi};

wrap_external_type! {
    #[derive(Clone, Hash, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
    pub struct Password(String);
}

impl Debug for Password {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "Password([redacted])")
    }
}

impl PartialSchema for Password {
    fn schema() -> RefOr<Schema> {
        openapi::schema::ObjectBuilder::new()
            .schema_type(openapi::schema::Type::String)
            .format(Some(SchemaFormat::KnownFormat(KnownFormat::Password)))
            .title("User password".into())
            .description(Some("Password used to log user in"))
            .examples(["superSecretePassword", "somePassword"])
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
            .example(Some(Value::String(String::from("somePassword"))))
            .build();

        vec![param]
    }
}

impl AsRef<str> for Password {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl AsRef<[u8]> for Password {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}
