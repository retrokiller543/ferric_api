use crate::types::Scope;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::fmt::Display;
use std::ops::{Deref, DerefMut};
use utoipa::openapi::{RefOr, Schema};
use utoipa::{PartialSchema, ToSchema, openapi};
#[derive(Clone, Debug, Default, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct Scopes(Vec<String>);

impl Scopes {
    pub fn new() -> Self {
        Self(Vec::new())
    }
}

impl Display for Scopes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.join(" "))
    }
}

impl Deref for Scopes {
    type Target = Vec<String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Scopes {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<oauth2::Scope> for Scopes {
    fn from(value: oauth2::Scope) -> Self {
        value.split(" ").collect()
    }
}

impl From<Vec<String>> for Scopes {
    fn from(value: Vec<String>) -> Self {
        Self(value)
    }
}

impl From<Scope> for Scopes {
    fn from(value: Scope) -> Self {
        value.into_scopes().into()
    }
}

impl FromIterator<String> for Scopes {
    fn from_iter<T: IntoIterator<Item = String>>(iter: T) -> Self {
        let mut scopes = Scopes::new();

        for i in iter {
            scopes.push(i)
        }

        scopes
    }
}

impl<'a> FromIterator<&'a str> for Scopes {
    fn from_iter<T: IntoIterator<Item = &'a str>>(iter: T) -> Self {
        let mut scopes = Scopes::new();

        for i in iter {
            scopes.push(i.to_string())
        }

        scopes
    }
}

impl Serialize for Scopes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let joined_scopes = self.0.join(" ");
        serializer.serialize_str(&joined_scopes)
    }
}

impl<'de> Deserialize<'de> for Scopes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let scopes = s.split_whitespace().collect();
        Ok(scopes)
    }
}

impl PartialSchema for Scopes {
    fn schema() -> RefOr<Schema> {
        openapi::schema::ObjectBuilder::new()
            .schema_type(openapi::schema::Type::String)
            .title("OAuth Scope".into())
            .description(Some("A space delimited string of scopes"))
            .examples(["read write openid"])
            .into()
    }
}

impl ToSchema for Scopes {}
