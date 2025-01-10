//! Conversion trait between a database model and a DTO
//!
//! Also includes some blanket impls

use crate::traits::FromModel;
use actix_oauth::types::{ClientId, Username};
use chrono::NaiveDateTime;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::path::PathBuf;

/// Consider implementing [`FromModel`] instead of this trait to get more blanket implementations of
/// for example collection types and more.
#[diagnostic::on_unimplemented(
    message = "Consider implementing `IntoDTO<T> for {Self}` to define how the type should be converted"
)]
pub trait IntoDTO<DTO>
where
    DTO: Serialize + DeserializeOwned,
{
    /// Converts this model into its corresponding DTO.
    ///
    /// This consumes the model and produces a new DTO instance.
    fn into_dto(self) -> DTO;
}

impl<DTO, Model> IntoDTO<DTO> for Model
where
    DTO: FromModel<Model>,
{
    #[inline]
    fn into_dto(self) -> DTO {
        DTO::from_model(self)
    }
}

/// Marker trait
#[diagnostic::on_unimplemented(
    message = "Consider marking the type `{Self}` as a DTO",
    label = "Used here",
    note = "Common types in DTO's are valid but can not be wrapped in a Option"
)]
pub trait IsDTO: Serialize + DeserializeOwned {}

#[macro_export]
/// helper macro to mark a type as a dto, needed for it to be identity implemented with the [`IntoDTO`] trait
macro_rules! is_dto {
    ($ty:ty) => {
        impl IsDTO for $ty {}
    };
}

is_dto!(String);
is_dto!(PathBuf);
is_dto!(Username);
is_dto!(NaiveDateTime);
is_dto!(ClientId);
is_dto!(usize);
is_dto!(u8);
is_dto!(u16);
is_dto!(u32);
is_dto!(u64);
is_dto!(u128);
is_dto!(isize);
is_dto!(i8);
is_dto!(i16);
is_dto!(i32);
is_dto!(i64);
is_dto!(i128);
is_dto!(f32);
is_dto!(f64);
