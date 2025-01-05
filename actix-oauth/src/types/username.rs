use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt::{Debug, Formatter};
use tosic_utils::wrap_external_type;
use utoipa::openapi::path::{Parameter, ParameterBuilder, ParameterIn};
use utoipa::openapi::{KnownFormat, RefOr, Required, Schema, SchemaFormat};
use utoipa::{openapi, IntoParams, PartialSchema, ToSchema};

wrap_external_type! {
    #[derive(Clone, Hash, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
    pub struct Username(String);
}

impl Debug for Username {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "Username([redacted])")
    }
}

impl PartialSchema for Username {
    fn schema() -> RefOr<Schema> {
        openapi::schema::ObjectBuilder::new()
            .schema_type(openapi::schema::Type::String)
            .format(Some(SchemaFormat::KnownFormat(KnownFormat::Password)))
            .title("Username".into())
            .description(Some("Username used to login"))
            .examples(["john-doe", "john.doe@example.com"])
            .into()
    }
}

impl ToSchema for Username {}

impl IntoParams for Username {
    fn into_params(parameter_in_provider: impl Fn() -> Option<ParameterIn>) -> Vec<Parameter> {
        let parameter_in = parameter_in_provider().unwrap_or_default();

        let param = ParameterBuilder::new()
            .parameter_in(parameter_in)
            .required(Required::True)
            .schema(Some(Self::schema()))
            .description(Some("Username used to login"))
            .example(Some(Value::String(String::from("john-doe"))))
            .build();

        vec![param]
    }
}

impl AsRef<str> for Username {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}
