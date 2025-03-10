//#![deny(missing_docs)]
#![allow(dead_code)]
//! Traits used in the project

use crate::mod_def;

mod_def! {
    pub mod from_model;
    pub mod into_dto;
    pub mod repository_rls;
}
