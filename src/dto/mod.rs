//! A DTO (Data Transfer Object) is a object that is sent over the wire, this can be either a request
//! or repose to and from the API or also any requests sent to other servers.
//! It's recommended to re-export all DTO's to be visible at `crate::dto::*` to make things simpler
//! and paths not to complex.
#![allow(unused_imports)]

mod_def! {
    pub(crate) mod error;
    pub mod llm;
    pub(crate) mod user;
}

use crate::mod_def;

#[macro_export]
macro_rules! dto {
    {
        $(#[$meta:meta])*
        $vis:vis struct $ident:ident$(<$($lifetime:lifetime $(,)?)? $($generic:ident),*>)? {
            $($(#[$field_meta:meta])* $field_vis:vis $field:ident: $field_type:ty),* $(,)?
        }
    } => {
        $(#[$meta])*
        $vis struct $ident$(<$($lifetime,)? $($generic),*>)? {
            $($(#[$field_meta])* $field_vis $field: $field_type),*
        }

        ::actix_oauth::impl_responder!($ident$(<$($lifetime,)? $($generic),*>)?);
    };

    {
        $(#[$meta:meta])*
        $vis:vis struct $ident:ident$(<$($lifetime:lifetime $(,)?)? $($generic:ident),*>)? => $model:ty {
            $($(#[$field_meta:meta])* $field_vis:vis $field:ident: $field_type:ty),*
        }
    } => {
        $crate::dto! {
            $(#[$meta])*
            $vis struct $ident$(<$($lifetime,)? $($generic),*>)? {
                $($(#[$field_meta])* $field_vis $field: $field_type),*
            }
        }

        ::paste::paste! {
            impl$(<$($lifetime $(,)?)? $($generic),*>)? $crate::traits::FromModel<Vec<$model$(<$($lifetime $(,)?)? $($generic),*>)?>> for [<$ident Collection>]$(<$($lifetime $(,)?)? $($generic),*>)? {
                fn from_model(model: Vec<$model$(<$($lifetime $(,)?)? $($generic),*>)?>) -> Self {
                    let dto: Vec<_> = model.into_dto();
                    dto.into()
                }
            }
        }
    };

    {
        $(#[$meta:meta])*
        $vis:vis struct $ident:ident$(<$($lifetime:lifetime $(,)?)? $($generic:ident),*>)? => $model:ty {
            $($(#[$field_meta:meta])* $field_vis:vis $field:ident: $field_type:ty),*
        }

        $($tt:tt)*
    } => {
        $crate::dto! {
            $(#[$meta])*
            $vis struct $ident$(<$($lifetime,)? $($generic),*>)? => $model {
                $($(#[$field_meta])* $field_vis $field: $field_type),*
            }
        }

        impl$(<$($lifetime $(,)?)? $($generic),*>)? $crate::traits::FromModel<$model$(<$($lifetime $(,)?)? $($generic),*>)?> for $ident {
            $($tt)*
        }
    };
}
