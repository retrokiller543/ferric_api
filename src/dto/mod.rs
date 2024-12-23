//! A DTO (Data Transfer Object) is a object that is sent over the wire, this can be either a request
//! or repose to and from the API or also any requests sent to other servers.
//! It's recommended to re-export all DTO's to be visible at `crate::dto::*` to make things simpler
//! and paths not to complex.

pub(crate) mod error;

pub(crate) use error::*;
use serde::de::DeserializeOwned;
use serde::Serialize;

/// Conversion trait between database model and a DTO
pub(crate) trait IntoDTO<DTO>
where
    DTO: Serialize + DeserializeOwned,
{
    fn into_dto(self) -> DTO;
}

pub(crate) trait FromModel<Model>
where
    Model: IntoDTO<Self>,
    Self: Serialize + DeserializeOwned,
{
    fn from_model(model: Model) -> Self {
        model.into_dto()
    }
}

impl<Model, DTO> FromModel<Model> for DTO
where
    Model: IntoDTO<DTO>,
    DTO: Serialize + DeserializeOwned,
{
}

impl<DTO, Model> IntoDTO<Vec<DTO>> for Vec<Model>
where
    DTO: Serialize + DeserializeOwned,
    Model: IntoDTO<DTO>,
{
    fn into_dto(self) -> Vec<DTO> {
        self.into_iter().map(IntoDTO::into_dto).collect()
    }
}

impl<DTO, Model> IntoDTO<Option<DTO>> for Option<Model>
where
    DTO: Serialize + DeserializeOwned,
    Model: IntoDTO<DTO>,
{
    fn into_dto(self) -> Option<DTO> {
        self.map(IntoDTO::into_dto)
    }
}

impl<DTO, E, Model> IntoDTO<Result<DTO, E>> for Result<Model, E>
where
    DTO: Serialize + DeserializeOwned,
    E: Serialize + DeserializeOwned,
    Model: IntoDTO<DTO>,
{
    fn into_dto(self) -> Result<DTO, E> {
        self.map(IntoDTO::into_dto)
    }
}

use std::collections::HashMap;
use std::hash::Hash;

impl<DTOKey, DTOValue, ModelKey, ModelValue> IntoDTO<HashMap<DTOKey, DTOValue>>
    for HashMap<ModelKey, ModelValue>
where
    DTOKey: Serialize + DeserializeOwned + Hash + Eq + PartialEq,
    DTOValue: Serialize + DeserializeOwned,
    ModelKey: IntoDTO<DTOKey>,
    ModelValue: IntoDTO<DTOValue>,
{
    fn into_dto(self) -> HashMap<DTOKey, DTOValue> {
        self.into_iter()
            .map(|(key, value)| (key.into_dto(), value.into_dto()))
            .collect()
    }
}

impl<DTO, Model> IntoDTO<Vec<DTO>> for &[Model]
where
    DTO: Serialize + DeserializeOwned,
    Model: IntoDTO<DTO> + Clone,
{
    fn into_dto(self) -> Vec<DTO> {
        self.iter().cloned().map(IntoDTO::into_dto).collect()
    }
}
