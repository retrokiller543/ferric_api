//! Helper trait to define the transition from a database model into a DTO

use crate::traits::IsDTO;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::collections::{BTreeMap, HashMap};
use std::hash::Hash;

/// Helper trait to convert from a database Model into a new DTO
pub trait FromModel<Model>
where
    Self: Serialize + DeserializeOwned,
{
    /// Takes in a model and consumes it to create a new DTO
    fn from_model(model: Model) -> Self;
}

impl<T: IsDTO> FromModel<T> for T {
    fn from_model(model: T) -> Self {
        model
    }
}

/// Blanket impl for converting a `Vec<Model>` into a `Vec<DTO>`
impl<DTO, Model> FromModel<Vec<Model>> for Vec<DTO>
where
    DTO: FromModel<Model> + Serialize + DeserializeOwned,
{
    #[inline]
    fn from_model(model: Vec<Model>) -> Self {
        model.into_iter().map(DTO::from_model).collect()
    }
}

/// Blanket impl for converting a slice of `Model` into a `Vec<DTO>`
impl<DTO, Model> FromModel<&[Model]> for Vec<DTO>
where
    Model: Clone,
    DTO: FromModel<Model> + Serialize + DeserializeOwned,
{
    #[inline]
    fn from_model(model: &[Model]) -> Self {
        model.iter().cloned().map(DTO::from_model).collect()
    }
}

/// Blanket impl for `Option<Model>` -> `Option<DTO>`
impl<DTO, Model> FromModel<Option<Model>> for Option<DTO>
where
    DTO: FromModel<Model> + Serialize + DeserializeOwned,
{
    #[inline]
    fn from_model(model: Option<Model>) -> Self {
        model.map(DTO::from_model)
    }
}

/// Blanket impl for `Result<Model, E>` -> `Result<DTO, E>`
impl<DTO, E, Model> FromModel<Result<Model, E>> for Result<DTO, E>
where
    DTO: FromModel<Model> + Serialize + DeserializeOwned,
    E: Serialize + DeserializeOwned,
{
    #[inline]
    fn from_model(model: Result<Model, E>) -> Self {
        model.map(DTO::from_model)
    }
}

/// Blanket impl for `HashMap<K, V>` -> `HashMap<DK, DV>`
impl<K, V, DK, DV> FromModel<HashMap<K, V>> for HashMap<DK, DV>
where
    K: Serialize + DeserializeOwned + Eq + Hash,
    V: Serialize + DeserializeOwned,
    DK: FromModel<K> + Serialize + DeserializeOwned + Eq + Hash,
    DV: FromModel<V> + Serialize + DeserializeOwned,
{
    #[inline]
    fn from_model(model: HashMap<K, V>) -> Self {
        model
            .into_iter()
            .map(|(k, v)| (DK::from_model(k), DV::from_model(v)))
            .collect()
    }
}

/// Blanket impl for `BTreeMap<K, V>` -> `BTreeMap<DK, DV>`
impl<K, V, DK, DV> FromModel<BTreeMap<K, V>> for BTreeMap<DK, DV>
where
    K: Serialize + DeserializeOwned + Ord,
    V: Serialize + DeserializeOwned,
    DK: FromModel<K> + Serialize + DeserializeOwned + Ord,
    DV: FromModel<V> + Serialize + DeserializeOwned,
{
    #[inline]
    fn from_model(model: BTreeMap<K, V>) -> Self {
        model
            .into_iter()
            .map(|(k, v)| (DK::from_model(k), DV::from_model(v)))
            .collect()
    }
}
