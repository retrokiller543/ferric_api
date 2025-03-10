use crate::types::UserContext;
use sqlx::Transaction;
use sqlx_utils::{
    traits::{Model, Repository},
    types::Database,
};
use std::future::Future;

/// Extension trait for Repository to work with Row Level Security
pub trait RepositoryRls<M>: Repository<M>
where
    M: Model,
{
    /// Execute a function within a transaction and with the given user context
    fn with_context<'a, 'b, F, Fut, R, E>(
        &'a self,
        context: &'a UserContext,
        f: F,
    ) -> impl Future<Output = Result<R, E>> + Send + 'a
    where
        F: FnOnce(Transaction<'b, Database>) -> Fut + Send + 'a,
        Fut: Future<Output = (Transaction<'b, Database>, Result<R, E>)> + Send,
        R: Send,
        E: From<sqlx::Error> + Send,
    {
        let pool = self.pool();

        async move {
            // Start a transaction
            let mut tx = pool.begin().await.map_err(E::from)?;

            // Set user context based on the provided context
            match context {
                UserContext::Authenticated {
                    ext_id: user_id, ..
                } => {
                    // Set user ID
                    sqlx::query("SELECT set_user_context($1)")
                        .bind(user_id)
                        .execute(&mut *tx)
                        .await
                        .map_err(E::from)?;

                    // Ensure system context is off
                    sqlx::query("SELECT set_system_context(FALSE)")
                        .execute(&mut *tx)
                        .await
                        .map_err(E::from)?;
                }
                UserContext::System => {
                    // Enable system context (bypasses RLS)
                    sqlx::query("SELECT set_system_context(TRUE)")
                        .execute(&mut *tx)
                        .await
                        .map_err(E::from)?;
                }
                UserContext::Anonymous => {
                    // Clear user ID and system context
                    sqlx::query("SELECT clear_user_context()")
                        .execute(&mut *tx)
                        .await
                        .map_err(E::from)?;

                    sqlx::query("SELECT set_system_context(FALSE)")
                        .execute(&mut *tx)
                        .await
                        .map_err(E::from)?;
                }
            }

            // Call the function with the transaction
            let (mut tx, result) = f(tx).await;

            // Clear context regardless of result
            let _ = sqlx::query("SELECT clear_user_context()")
                .execute(&mut *tx)
                .await;

            let _ = sqlx::query("SELECT set_system_context(TRUE)")
                .execute(&mut *tx)
                .await;

            // Commit or rollback the transaction
            match &result {
                Ok(_) => tx.commit().await.map_err(E::from)?,
                Err(_) => {
                    let _ = tx.rollback().await;
                }
            }

            result
        }
    }
}

impl<T, M> RepositoryRls<M> for T
where
    T: Repository<M>,
    M: Model,
{
}
