#![allow(dead_code)]

use crate::ApiResult;
use sqlx::postgres::PgArguments;
use sqlx::query::Query;
use sqlx::{PgPool, Postgres};

mod filter;
pub(crate) mod oauth_clients;
pub(crate) mod oauth_token;
pub(crate) mod users;

type PgQuery<'a> = Query<'a, Postgres, PgArguments>;

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

    /// Simple insert call
    #[tracing::instrument(skip_all, level = "debug")]
    async fn insert(&self, model: &M) -> ApiResult<()> {
        Self::insert_one(model).execute(self.pool()).await?;
        Ok(())
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

    #[tracing::instrument(skip_all, level = "debug")]
    async fn insert_many(&self, models: impl IntoIterator<Item = M>) -> ApiResult<()> {
        let mut tx = self.pool().begin().await?;

        for model in models {
            Self::insert_one(&model).execute(&mut *tx).await?;
        }

        tx.commit().await?;

        Ok(())
    }

    #[tracing::instrument(skip_all, level = "debug")]
    async fn insert_batch<const N: usize>(
        &self,
        models: impl IntoIterator<Item = M>,
    ) -> ApiResult<()> {
        let mut buf = Vec::with_capacity(N);
        for model in models {
            buf.push(model);
            if buf.len() == N {
                let mut tx = self.pool().begin().await?;

                for m in buf.drain(..) {
                    Self::insert_one(&m).execute(&mut *tx).await?;
                }

                tx.commit().await?;
            }
        }

        let mut tx = self.pool().begin().await?;

        for m in buf.drain(..) {
            Self::insert_one(&m).execute(&mut *tx).await?;
        }

        tx.commit().await?;

        Ok(())
    }

    #[tracing::instrument(skip_all, level = "debug")]
    async fn update_many(&self, models: impl IntoIterator<Item = M>) -> ApiResult<()> {
        let mut tx = self.pool().begin().await?;

        for model in models {
            Self::update_one(&model).execute(&mut *tx).await?;
        }

        tx.commit().await?;
        Ok(())
    }

    #[tracing::instrument(skip_all, level = "debug")]
    async fn delete_many(&self, ids: impl IntoIterator<Item = M::Id>) -> ApiResult<()> {
        let mut tx = self.pool().begin().await?;

        for id in ids {
            Self::delete_one_by_id(&id).execute(&mut *tx).await?;
        }

        tx.commit().await?;
        Ok(())
    }

    /// `save_all` is like `save`, but for a batch of models.
    /// It:
    /// - Inserts any model that has no ID
    /// - Updates any model that has an ID
    /// All in a single transaction.
    #[tracing::instrument(skip_all, level = "debug")]
    async fn save_all(&self, models: impl IntoIterator<Item = M>) -> ApiResult<()> {
        let mut tx = self.pool().begin().await?;

        for model in models {
            if model.get_id().is_none() {
                Self::insert_one(&model).execute(&mut *tx).await?;
            } else {
                Self::update_one(&model).execute(&mut *tx).await?;
            }
        }

        tx.commit().await?;
        Ok(())
    }
}

macro_rules! repository {
    (
        $( #[$meta:meta] )*
        $vis:vis $ident:ident;
    ) => {
        $(#[$meta])*
        #[derive(Clone, Copy, Debug)]
        $vis struct $ident {
            pool: &'static ::sqlx::PgPool,
        }

        ::paste::paste! {

            pub(crate) static [<$ident:snake:upper>]: ::tokio::sync::OnceCell<$ident> = ::tokio::sync::OnceCell::const_new();

            #[inline(always)]
            #[tracing::instrument(level = "debug")]
            pub(crate) async fn [<init_ $ident:snake:lower>]() -> crate::ApiResult<$ident> {
                $ident::new().await.map_err(|err| crate::error::ApiError::Generic(Box::new(err)))
            }

            #[inline(always)]
            #[tracing::instrument(level = "debug")]
            pub(crate) async fn [<get_ $ident:snake>]() -> crate::ApiResult<&'static $ident> {
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

    (
        $( #[$meta:meta] )*
        $vis:vis $ident:ident<$model:ty>;

        $($tokens:tt)*
    ) => {
        use sqlx::query;

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
pub(crate) use repository;
