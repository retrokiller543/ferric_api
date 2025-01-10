//! Helper trait to define the transition from a database model into a DTO

use crate::traits::IsDTO;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::collections::{BTreeMap, HashMap};
use std::hash::Hash;

/// A trait for converting database models into Data Transfer Objects (DTOs).
///
/// This trait facilitates the separation of concerns between your database models
/// and the data structures used for external communication. It provides a clean,
/// type-safe way to transform internal models into serializable DTOs.
///
/// # Type Parameters
///
/// * `Model`: The Persistence object to be converted into a Data Transfer Object.
/// * `Self`: Must implement both [`Serialize`]
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
/// impl FromModel<UserModel> for UserDTO {
///     fn from_model(model: UserModel) -> UserDTO {
///         UserDTO {
///             id: model.id,
///             username: model.username,
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
/// of the [`FromModel`] trait. This allows us to use a collection like [`HashMap<K, V>`](HashMap) or
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
pub trait FromModel<Model>
where
    Self: Serialize + DeserializeOwned,
{
    /// Takes in a model and consumes it to create a new DTO
    fn from_model(model: Model) -> Self;
}

impl<T: IsDTO> FromModel<T> for T {
    #[inline(always)]
    fn from_model(model: T) -> Self {
        model
    }
}

impl<DTO, Model> FromModel<Vec<Model>> for Vec<DTO>
where
    DTO: FromModel<Model> + Serialize + DeserializeOwned,
{
    #[inline]
    fn from_model(model: Vec<Model>) -> Self {
        model.into_iter().map(DTO::from_model).collect()
    }
}

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

impl<DTO, Model> FromModel<Option<Model>> for Option<DTO>
where
    DTO: FromModel<Model> + Serialize + DeserializeOwned,
{
    #[inline]
    fn from_model(model: Option<Model>) -> Self {
        model.map(DTO::from_model)
    }
}

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
