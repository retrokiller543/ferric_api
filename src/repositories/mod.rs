#![allow(dead_code)]

use sqlx::postgres::PgArguments;
use sqlx::query::Query;
use sqlx::Postgres;

mod filter;
pub mod oauth_clients;
pub mod oauth_token;
pub mod users;

pub type PgQuery<'a> = Query<'a, Postgres, PgArguments>;

/// Creates a new database repository, either just creates a basic new type and statics to interact
/// with the main database pool.
///
/// If a database model is provided it will also try to implement the [`crate::traits::Repository`] trait.
///
/// # Examples
///
/// ```
/// use ferric_api::repositories::repository;
/// use ferric_api::traits::Model;
///
///
/// struct Person {
///     id: String,
///     name: String
/// }
///
/// impl Model for Person {
///     type Id = String;
///
///     fn get_id(&self) -> Option<Self::Id> {
///         Some(self.id)
///     }
/// }
///
/// repository!{
///     PersonRepository<Person>;
/// }
/// ```
///
macro_rules! repository {
    {
        $( #[$meta:meta] )*
        $vis:vis $ident:ident;
    } => {
        $(#[$meta])*
        #[derive(Clone, Copy, Debug)]
        $vis struct $ident {
            pool: &'static ::sqlx::PgPool,
        }

        ::paste::paste! {
            $vis static [<$ident:snake:upper>]: ::tokio::sync::OnceCell<$ident> = ::tokio::sync::OnceCell::const_new();

            #[inline(always)]
            #[tracing::instrument(level = "debug")]
            async fn [<init_ $ident:snake:lower>]() -> crate::ApiResult<$ident> {
                $ident::new().await.map_err(|err| crate::error::ApiError::Generic(Box::new(err)))
            }

            #[inline(always)]
            #[tracing::instrument(level = "debug")]
            $vis async fn [<get_ $ident:snake>]() -> crate::ApiResult<&'static $ident> {
                [<$ident:snake:upper>].get_or_try_init([<init_ $ident:snake:lower>]).await
            }
        }

        impl $ident {
            $vis async fn new() -> crate::ServerResult<Self> {
                let pool = crate::setup::database::get_db_pool().await?;
                Ok(Self {
                    pool
                })
            }
        }
    };

    {
        $( #[$meta:meta] )*
        $vis:vis $ident:ident<$model:ty>;
    } => {
        crate::repositories::repository!(
            $(#[$meta])*
            $vis $ident<$model>;

            #[inline]
            fn insert_one(_model: &$model) -> crate::repositories::PgQuery {
                unimplemented!("Insert is not implemented for this repository");
            }

            #[inline]
            fn update_one(_model: &$model) -> crate::repositories::PgQuery {
                unimplemented!("Update is not implemented for this repository");
            }

            #[inline]
            fn delete_one_by_id(_model: &<$model as Model>::Id) -> crate::repositories::PgQuery {
                unimplemented!("Delete is not implemented for this repository");
            }
        );
    };

    {
        $( #[$meta:meta] )*
        $vis:vis $ident:ident<$model:ty>;

        insert_one($model_name:ident) $block:block;

        $($tokens:tt)*
    } => {
        crate::repositories::repository!(
            $(#[$meta])*
            $vis $ident<$model>;

            #[inline]
            fn insert_one($model_name: &$model) -> crate::repositories::PgQuery $block

            #[inline]
            fn update_one(_model: &$model) -> crate::repositories::PgQuery {
                unimplemented!("Update is not implemented for this repository");
            }

            #[inline]
            fn delete_one_by_id(_model: &<$model as Model>::Id) -> crate::repositories::PgQuery {
                unimplemented!("Delete is not implemented for this repository");
            }

            $($tokens)*
        );
    };

    {
        $( #[$meta:meta] )*
        $vis:vis $ident:ident<$model:ty>;

        update_one($model_name:ident) $block:block;

        $($tokens:tt)*
    } => {
        crate::repositories::repository!(
            $(#[$meta])*
            $vis $ident<$model>;

            #[inline]
            fn insert_one(_model: &$model) -> crate::repositories::PgQuery {
                unimplemented!("Insert is not implemented for this repository");
            }

            #[inline]
            fn update_one($model_name: &$model) -> crate::repositories::PgQuery $block

            #[inline]
            fn delete_one_by_id(_model: &<$model as Model>::Id) -> crate::repositories::PgQuery {
                unimplemented!("Delete is not implemented for this repository");
            }

            $($tokens)*
        );
    };

    {
        $( #[$meta:meta] )*
        $vis:vis $ident:ident<$model:ty>;

        delete_one_by_id($id_name:ident) $block:block;

        $($tokens:tt)*
    } => {
        crate::repositories::repository!(
            $(#[$meta])*
            $vis $ident<$model>;

            #[inline]
            fn insert_one(_model: &$model) -> crate::repositories::PgQuery {
                unimplemented!("Insert is not implemented for this repository");
            }

            #[inline]
            fn update_one(_model: &$model) -> crate::repositories::PgQuery {
                unimplemented!("Update is not implemented for this repository");
            }

            #[inline]
            fn delete_one_by_id($id_name: &<$model as Model>::Id) -> crate::repositories::PgQuery $block

            $($tokens)*
        );
    };

    {
        $( #[$meta:meta] )*
        $vis:vis $ident:ident<$model:ty>;

        update_one($model_name:ident) $update_block:block;
        delete_one_by_id($id_name:ident) $delete_block:block;

        $($tokens:tt)*
    } => {
        crate::repositories::repository!(
            $(#[$meta])*
            $vis $ident<$model>;

            #[inline]
            fn insert_one(_model: &$model) -> crate::repositories::PgQuery {
                unimplemented!("Insert is not implemented for this repository");
            }

            #[inline]
            fn update_one($model_name: &$model) -> crate::repositories::PgQuery $update_block

            #[inline]
            fn delete_one_by_id($id_name: &<$model as Model>::Id) -> crate::repositories::PgQuery $delete_block

            $($tokens)*
        );
    };

    {
        $( #[$meta:meta] )*
        $vis:vis $ident:ident<$model:ty>;

        insert_one($insert_model_name:ident) $insert_block:block;
        update_one($update_model_name:ident) $update_block:block;
        delete_one_by_id($id_name:ident) $delete_block:block;

        $($tokens:tt)*
    } => {
        crate::repositories::repository!(
            $(#[$meta])*
            $vis $ident<$model>;

            #[inline]
            fn insert_one($insert_model_name: &$model) -> crate::repositories::PgQuery $insert_block

            #[inline]
            fn update_one($update_model_name_name: &$model) -> crate::repositories::PgQuery $update_block

            #[inline]
            fn delete_one_by_id($id_name: &<$model as Model>::Id) -> crate::repositories::PgQuery $delete_block

            $($tokens)*
        );
    };

    {
        $( #[$meta:meta] )*
        $vis:vis $ident:ident<$model:ty>;

        insert_one($insert_model_name:ident) $insert_block:block;
        update_one($update_model_name:ident) $update_block:block;

        $($tokens:tt)*
    } => {
        crate::repositories::repository!(
            $(#[$meta])*
            $vis $ident<$model>;

            #[inline]
            fn insert_one($insert_model_name: &$model) -> crate::repositories::PgQuery $insert_block

            #[inline]
            fn update_one($update_model_name_name: &$model) -> crate::repositories::PgQuery $update_block

            #[inline]
            fn delete_one_by_id(_: &<$model as Model>::Id) -> crate::repositories::PgQuery {
                unimplemented!("Delete is not implemented for this repository");
            }

            $($tokens)*
        );
    };

    {
        $( #[$meta:meta] )*
        $vis:vis $ident:ident<$model:ty>;

        insert_one($insert_model_name:ident) $insert_block:block;
        delete_one_by_id($id_name:ident) $delete_block:block;

        $($tokens:tt)*
    } => {
        crate::repositories::repository!(
            $(#[$meta])*
            $vis $ident<$model>;

            #[inline]
            fn insert_one($insert_model_name: &$model) -> crate::repositories::PgQuery $insert_block

            #[inline]
            fn update_one(_: &$model) -> crate::repositories::PgQuery {
                unimplemented!("Update is not implemented for this repository");
            }

            #[inline]
            fn delete_one_by_id($id_name: &<$model as Model>::Id) -> crate::repositories::PgQuery $delete_block

            $($tokens)*
        );
    };

    {
        $( #[$meta:meta] )*
        $vis:vis $ident:ident<$model:ty>;

        $($tokens:tt)*
    } => {
        crate::repositories::repository!($(#[$meta])* $vis $ident;);

        impl crate::traits::repository::Repository<$model> for $ident {
            #[inline]
            fn pool(&self) -> &::sqlx::PgPool {
                self.pool
            }
            $($tokens)*
        }
    }
}
pub(crate) use repository;
