//! A DTO (Data Transfer Object) is a object that is sent over the wire, this can be either a request
//! or repose to and from the API or also any requests sent to other servers.
//! It's recommended to re-export all DTO's to be visible at `crate::dto::*` to make things simpler
//! and paths not to complex.

pub(crate) mod error;
pub(crate) use error::*;
