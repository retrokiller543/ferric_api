#![allow(dead_code)]

use crate::ApiResult;
use sqlx::postgres::PgArguments;
use sqlx::query::Query;
use sqlx::{PgPool, Postgres};

mod filter;
pub(crate) mod oauth_clients;
pub(crate) mod oauth_token;
pub(crate) mod users;

pub(crate) type PgQuery<'a> = Query<'a, Postgres, PgArguments>;

/// Default repository methods, each repository may extend this themselves
///
/// Many batch operations are auto implemented as long as its defined how the query to insert one
/// item for example is implemented
pub(crate) trait Repository<M>
where
    M: Model,
{
    fn pool(&self) -> &PgPool;

    /// Inserts a single model instance.
    /// Must be implemented (e.g., via `sqlx::query!`).
    fn insert_one(model: &M) -> PgQuery<'_>;

    /// Similarly, implement an `update_one` function that returns the `Query`.
    /// Example placeholder:
    ///
    /// ```ignore
    /// fn update_one(model: &M) -> Query<Postgres, PgArguments> {
    ///     sqlx::query!(
    ///         r#"
    ///         UPDATE my_table
    ///         SET col1 = $1, col2 = $2
    ///         WHERE id = $3
    ///         "#,
    ///         model.col1,
    ///         model.col2,
    ///         model.id(),
    ///     )
    /// }
    /// ```
    fn update_one(model: &M) -> PgQuery<'_>;

    /// Similarly, implement a `delete_one_by_id` function that returns the `Query`.
    /// Example placeholder:
    ///
    /// ```ignore
    /// fn delete_one_by_id(id: &M::Id) -> Query<Postgres, PgArguments> {
    ///     sqlx::query!(
    ///         "DELETE FROM my_table WHERE id = $1",
    ///         id
    ///     )
    /// }
    /// ```
    fn delete_one_by_id(id: &M::Id) -> PgQuery<'_>;

    #[tracing::instrument(skip_all, level = "debug")]
    async fn get_all(&self) -> ApiResult<Vec<M>> {
        unimplemented!("This method has not been implemented for this repository")
    }

    #[tracing::instrument(skip_all, level = "debug")]
    async fn get_by_id(&self, _: impl Into<M::Id>) -> ApiResult<Option<M>> {
        unimplemented!("This method has not been implemented for this repository")
    }

    /// Update call
    #[tracing::instrument(skip_all, level = "debug")]
    async fn update(&self, model: &M) -> ApiResult<()> {
        Self::update_one(model).execute(self.pool()).await?;
        Ok(())
    }

    /// Delete call
    #[tracing::instrument(skip_all, level = "debug")]
    async fn delete_by_id(&self, id: impl Into<M::Id>) -> ApiResult<()> {
        Self::delete_one_by_id(&id.into())
            .execute(self.pool())
            .await?;
        Ok(())
    }

    /// Simple insert call
    #[tracing::instrument(skip_all, level = "debug")]
    async fn insert(&self, model: &M) -> ApiResult<()> {
        Self::insert_one(model).execute(self.pool()).await?;
        Ok(())
    }

    #[tracing::instrument(skip_all, level = "debug")]
    async fn insert_many(&self, models: impl IntoIterator<Item = M>) -> ApiResult<()> {
        self.insert_batch::<DEFAULT_BATCH_SIZE>(models).await
    }

    #[tracing::instrument(skip_all, level = "debug")]
    async fn insert_batch<const N: usize>(
        &self,
        models: impl IntoIterator<Item = M>,
    ) -> ApiResult<()> {
        BatchOperator::<M, N>::execute_query(models, self.pool(), Self::insert_one).await
    }

    #[tracing::instrument(skip_all, level = "debug")]
    async fn update_many(&self, models: impl IntoIterator<Item = M>) -> ApiResult<()> {
        self.update_batch::<DEFAULT_BATCH_SIZE>(models).await
    }

    #[tracing::instrument(skip_all, level = "debug")]
    async fn update_batch<const N: usize>(
        &self,
        models: impl IntoIterator<Item = M>,
    ) -> ApiResult<()> {
        BatchOperator::<M, N>::execute_query(models, self.pool(), Self::update_one).await
    }

    #[tracing::instrument(skip_all, level = "debug")]
    async fn delete_many(&self, ids: impl IntoIterator<Item = M::Id>) -> ApiResult<()> {
        self.delete_batch::<DEFAULT_BATCH_SIZE>(ids).await
    }

    #[tracing::instrument(skip_all, level = "debug")]
    async fn delete_batch<const N: usize>(
        &self,
        ids: impl IntoIterator<Item = M::Id>,
    ) -> ApiResult<()> {
        BatchOperator::<M::Id, N>::execute_query(ids, self.pool(), Self::delete_one_by_id).await
    }

    /// `save` should:
    /// - Insert if the model has no ID
    /// - Update if the model has an ID
    #[tracing::instrument(skip_all, level = "debug")]
    async fn save(&self, model: &M) -> ApiResult<()> {
        if model.get_id().is_none() {
            self.insert(model).await
        } else {
            self.update(model).await
        }
    }

    /// `save_all` is like `save`, but for a batch of models.
    /// It:
    /// - Inserts any model that has no ID
    /// - Updates any model that has an ID
    /// All in a single transaction.
    #[tracing::instrument(skip_all, level = "debug")]
    async fn save_all(&self, models: impl IntoIterator<Item = M>) -> ApiResult<()> {
        self.save_batch::<DEFAULT_BATCH_SIZE>(models).await
    }

    #[tracing::instrument(skip_all, level = "debug")]
    async fn save_batch<const N: usize>(
        &self,
        models: impl IntoIterator<Item = M>,
    ) -> ApiResult<()> {
        BatchOperator::<M, N>::execute_batch(models, |batch| async {
            let mut update = Vec::new();
            let mut insert = Vec::new();

            for model in batch {
                if model.get_id().is_some() {
                    update.push(model);
                } else {
                    insert.push(model);
                }
            }

            match (update.is_empty(), insert.is_empty()) {
                // Both non-empty => run them concurrently
                (false, false) => {
                    futures::try_join!(self.update_many(update), self.insert_many(insert))?;
                }
                // Only update
                (false, true) => {
                    self.update_many(update).await?;
                }
                // Only insert
                (true, false) => {
                    self.insert_many(insert).await?;
                }
                // Neither => no-op
                (true, true) => {}
            }

            Ok(())
        })
        .await
    }
}

/// Creates a new database repository, either just creates a basic new type and statics to interact
/// with the main database pool.
///
/// If a database model is provided it will also try to implement the [`Repository`] trait.
///
/// # Examples
///
/// ```
/// use crate::repositories::repository;
/// use crate::models::Model;
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

        impl crate::repositories::Repository<$model> for $ident {
            #[inline]
            fn pool(&self) -> &::sqlx::PgPool {
                self.pool
            }
            $($tokens)*
        }
    }
}
use crate::models::Model;
use crate::utils::batch::{BatchOperator, DEFAULT_BATCH_SIZE};
pub(crate) use repository;
