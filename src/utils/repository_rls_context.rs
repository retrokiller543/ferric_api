/*
#[tracing::instrument(skip_all, err, level = "debug")]
    async fn find_by_email_inner<'e, E>(&self, email: &str, executor: E) -> ApiResult<Option<User>>
    where
        E: sqlx::Executor<'e, Database = Postgres>,
    {
        Ok(query_as!(
            User,
            "SELECT id, ext_id, username, password_hash, email, created_at, updated_at
             FROM users
             WHERE email = $1",
            email
        )
        .fetch_optional(executor)
        .await?)
    }

    /// Find by email using the pool directly (no RLS)
    #[tracing::instrument(skip_all, err, level = "debug")]
    pub async fn find_by_email(&self, email: &str) -> ApiResult<Option<User>> {
        self.find_by_email_inner(email, self.pool).await
    }

    /// Find by email with RLS context
    #[tracing::instrument(skip_all, err, level = "debug")]
    pub async fn find_by_email_with_context(&self, email: &str, context: UserContext) -> ApiResult<Option<User>> {
        self.with_context(context, |tx| {
            let email = email.to_string(); // Clone to ensure ownership for the async block
            Box::pin(async move {
                self.find_by_email_inner(&email, tx).await
            })
        }).await
    }
*/

#[macro_export]
macro_rules! repository_method {
    // Main pattern that matches a standard repository method definition
    {
        $(#[$method_meta:meta])*
        $vis:vis async fn $method_name:ident(&self, $executor_ident:ident, $($param_name:ident: $param_ty:ty),* $(,)?) -> $return_ty:ty
        $block:block
    } => {
        // 1. Create the inner method with a generic executor
        // The inner method name is the original name plus "_inner"
        ::paste::paste! {
            #[tracing::instrument(skip_all, err, level = "debug")]
            async fn [<$method_name _inner>]<'e, E>(&self, $executor_ident: E, $($param_name: $param_ty),*) -> $return_ty
            where
                E: ::sqlx::Executor<'e, Database = ::sqlx_utils::types::Database>,
            $block

        }

        $(#[$method_meta])*
        #[tracing::instrument(skip_all, err, parent = &Self::repository_span(), level = "debug")]
        $vis async fn $method_name(&self, $($param_name: $param_ty),*) -> $return_ty {
            ::paste::paste! {
                self.[<$method_name _inner>](self.pool(), $($param_name,)*).await
            }
        }

        ::paste::paste! {
            $(#[$method_meta])*
            #[tracing::instrument(skip(self, $($param_name),*), err, parent = &Self::repository_span(), level = "debug")]
            $vis async fn [<$method_name _with_context>](&self, $($param_name: $param_ty,)* context: &crate::types::UserContext) -> $return_ty {
                self.with_context(context, move |mut tx| async {
                    let result = self.[<$method_name _inner>](&mut *tx, $($param_name,)*).await;

                    (tx, result)
                }).await
            }
        }
    };
}
