#![deny(missing_docs)]
#![allow(dead_code)]
//! Traits used in the project
use crate::mod_def;

mod_def! {
    pub mod repository;
}

mod_def! {
    pub mod model;
}

mod_def! {
    pub mod into_dto;
}

mod_def! {
    pub mod from_model;
}
