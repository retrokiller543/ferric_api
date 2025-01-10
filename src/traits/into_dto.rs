//! Conversion trait between a database model and a DTO
//!
//! Also includes some blanket impls

use crate::traits::FromModel;
use actix_oauth::types::{ClientId, Username};
use chrono::NaiveDateTime;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::path::PathBuf;

/// A trait for converting database models into Data Transfer Objects (DTOs).
///
/// This trait facilitates the separation of concerns between your database models
/// and the data structures used for external communication. It provides a clean,
/// type-safe way to transform internal models into serializable DTOs.
///
/// # Type Parameters
///
/// * `DTO`: The target Data Transfer Object type. Must implement both [`Serialize`]
///   and [`DeserializeOwned`] to ensure it can be properly serialized for API responses.
///
/// # Examples
///
/// Basic implementation for a user model and DTO:
/// ```rust
/// use serde::{Serialize, Deserialize};
///
/// #[derive(Serialize, Deserialize)]
/// struct UserDTO {
///     id: i32,
///     username: String,
/// }
///
/// struct UserModel {
///     id: i32,
///     username: String,
///     password_hash: String,  // Sensitive data we don't want in DTO
/// }
///
/// impl IntoDTO<UserDTO> for UserModel {
///     fn into_dto(self) -> UserDTO {
///         UserDTO {
///             id: self.id,
///             username: self.username,
///         }
///     }
/// }
/// ```
///
/// Converting a collection of models:
/// ```rust
/// let users = vec![user_model1, user_model2];
/// let user_dtos: Vec<UserDTO> = users.into_dto();
/// ```
///
/// Working with optional values:
/// ```rust
/// let maybe_user: Option<UserModel> = Some(user_model);
/// let maybe_dto: Option<UserDTO> = maybe_user.into_dto();
/// ```
///
/// Converting a hashmap of models:
/// ```rust
/// let user_map: HashMap<i32, UserModel> = HashMap::new();
/// let dto_map: HashMap<i32, UserDTO> = user_map.into_dto();
/// ```
///
/// # Design Notes
///
/// ## Marker Trait
///
/// The trait [`IsDTO`] is used to mark types as a dto and with it also implement an identity implementation
/// of the [`IntoDTO`] trait. This allows us to use a collection like [`HashMap<K, V>`](HashMap) or
/// [`BTreeMap<K, V>`](BTreeMap) with types that are not designed to be a DTO but rather used inside
/// a DTO possibly, such as [`String`] or any number type (unsigned, signed or floats), this also
/// allows us to simply expand this list with the helper macro [`is_dto`](crate::is_dto).
///
/// The trait provides automatic implementations for common wrapper types:
///
/// * [`Vec<T>`]: Converts each element in the vector
/// * [`Option<T>`]: Maps the conversion over the optional value
/// * [`Result<T, E>`](Result): Maps the conversion over the Ok variant
/// * [`HashMap<K, V>`](HashMap): Converts both keys and values
/// * [`BTreeMap<K, V>`](BTreeMap): Converts both keys and values
/// * `&[T]`: Converts slices into vectors of DTOs (requires `Clone`)
///
/// These implementations make it ergonomic to work with collections and
/// container types without writing boilerplate conversion code.
///
/// # Best Practices
///
/// When implementing this trait:
/// * Ensure DTOs exclude sensitive information from the model
/// * Consider implementing reverse conversion (`From<DTO>` for `Model`) if needed
/// * Use descriptive field names in DTOs for better API documentation
/// * Consider versioning DTOs if your API might change over time
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
