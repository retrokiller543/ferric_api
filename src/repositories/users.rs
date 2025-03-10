use crate::models::user::User;
use crate::prelude::*;
use crate::{ApiResult, repository_method};
use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};
use chrono::NaiveDateTime;
use tracing::{Span, error};
use uuid::Uuid;

repository! {
    pub UsersRepository;
}

impl Repository<User> for UsersRepository {
    fn pool(&self) -> &Pool {
        self.pool
    }

    fn repository_span() -> Span {
        Span::current()
    }
}

repository_insert! {
    UsersRepository<User>;

    insert_query(user) {
        query!(
            "INSERT INTO users (username, password_hash, email)
             VALUES ($1, $2, $3)",
            user.username,
            user.password_hash,
            user.email,
        )
    }
}

repository_update! {
    UsersRepository<User>;

    update_query(user) {
        query!(
            "UPDATE users
             SET username = $1,
                 password_hash = $2,
                 email = $3,
                 updated_at = CURRENT_TIMESTAMP
             WHERE ext_id = $4",
            user.username,
            user.password_hash,
            user.email,
            user.ext_id
        )
    }
}

repository_delete! {
    UsersRepository<User>;

    delete_by_id_query(id) {
        query!(
            "DELETE FROM users
             WHERE ext_id = $1",
            id
        )
    }

    delete_by_filter_query(filter) {
        let mut builder = QueryBuilder::new("DELETE FROM users");

        filter.apply_filter(&mut builder);

        builder
    }
}

impl SelectRepository<User> for UsersRepository {
    fn get_all_query(&self) -> QueryAs<User> {
        query_as(
            "SELECT id, ext_id, username, password_hash, email, created_at, updated_at FROM users",
        )
    }

    fn get_by_id_query(&self, id: impl Into<Uuid>) -> QueryAs<User> {
        let id = id.into();

        query_as(
            "SELECT id, ext_id, username, password_hash, email, created_at, updated_at FROM users WHERE ext_id = $1",
        ).bind(id)
    }
}

#[derive(Debug, Default)]
pub struct UserSearchParams {
    pub username_contains: Option<String>,
    pub email_contains: Option<String>,
    pub created_after: Option<NaiveDateTime>,
    pub created_before: Option<NaiveDateTime>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

impl UsersRepository {
    #[tracing::instrument(skip_all, err, name = "UsersRepository::create_user", level = "debug")]
    pub(crate) async fn create_user(
        &self,
        username: &str,
        email: &str,
        password: &str,
    ) -> ApiResult<User> {
        let password_hash = self.hash_password(&password)?;
        let user = User {
            id: None,
            ext_id: None,
            username: username.to_string(),
            password_hash,
            email: email.to_string(),
            created_at: None,
            updated_at: None,
        };

        self.insert_ref(&user).await?;
        Ok(user)
    }

    #[tracing::instrument(skip_all, name = "UsersRepository::verify_password", level = "debug")]
    pub(crate) fn verify_password(&self, password: &impl AsRef<[u8]>, hash: &str) -> bool {
        let parsed_hash = match PasswordHash::new(hash) {
            Ok(pass) => pass,
            Err(error) => {
                error!(error = %error);

                return false;
            }
        };

        Argon2::default()
            .verify_password(password.as_ref(), &parsed_hash)
            .is_ok()
    }

    #[tracing::instrument(
        skip_all,
        err,
        name = "UsersRepository::hash_password",
        level = "debug"
    )]
    fn hash_password(&self, password: &impl AsRef<[u8]>) -> ApiResult<String> {
        let salt = SaltString::generate(&mut OsRng);

        let argon2 = Argon2::default();

        Ok(argon2.hash_password(password.as_ref(), &salt)?.to_string())
    }
}

impl UsersRepository {
    #[tracing::instrument(
        skip_all,
        err,
        name = "UsersRepository::find_by_email",
        level = "debug"
    )]
    pub async fn find_by_email(&self, email: &str) -> ApiResult<Option<User>> {
        Ok(query_as!(
            User,
            "SELECT id, ext_id, username, password_hash, email, created_at, updated_at
             FROM users
             WHERE email = $1",
            email
        )
        .fetch_optional(self.pool)
        .await?)
    }

    #[tracing::instrument(
        skip_all,
        err,
        name = "UsersRepository::find_by_username",
        level = "debug"
    )]
    pub async fn find_by_username(&self, username: impl AsRef<str>) -> ApiResult<Option<User>> {
        Ok(query_as!(
            User,
            "SELECT id, ext_id, username, password_hash, email, created_at, updated_at
             FROM users
             WHERE username = $1 OR email = $1",
            username.as_ref()
        )
        .fetch_optional(self.pool)
        .await?)
    }

    #[tracing::instrument(skip_all, err, name = "UsersRepository::search", level = "debug")]
    pub async fn search(&self, params: &UserSearchParams) -> ApiResult<Vec<User>> {
        let mut query = "SELECT id, ext_id, username, password_hash, email, created_at, updated_at 
                        FROM users WHERE 1=1"
            .to_string();
        let mut bindings = vec![];

        if let Some(username) = &params.username_contains {
            query.push_str(" AND username ILIKE $1");
            bindings.push(format!("%{}%", username));
        }

        if let Some(email) = &params.email_contains {
            let param_num = bindings.len() + 1;
            query.push_str(&format!(" AND email ILIKE ${}", param_num));
            bindings.push(format!("%{}%", email));
        }

        if let Some(after) = &params.created_after {
            let param_num = bindings.len() + 1;
            query.push_str(&format!(" AND created_at > ${}", param_num));
            bindings.push(after.to_string());
        }

        if let Some(before) = &params.created_before {
            let param_num = bindings.len() + 1;
            query.push_str(&format!(" AND created_at < ${}", param_num));
            bindings.push(before.to_string());
        }

        query.push_str(" ORDER BY created_at DESC");

        if let Some(limit) = params.limit {
            let param_num = bindings.len() + 1;
            query.push_str(&format!(" LIMIT ${}", param_num));
            bindings.push(limit.to_string());
        }

        if let Some(offset) = params.offset {
            let param_num = bindings.len() + 1;
            query.push_str(&format!(" OFFSET ${}", param_num));
            bindings.push(offset.to_string());
        }

        let mut query_builder = sqlx::query_as::<_, User>(&query);
        for binding in bindings {
            query_builder = query_builder.bind(binding);
        }

        Ok(query_builder.fetch_all(self.pool).await?)
    }

    /*#[tracing::instrument(skip_all, err, name = "UsersRepository::find_by_email", level = "debug")]
    async fn find_by_email_inner(&self, tx: &mut impl Executor<'_, Database = Database>, (email,): (impl AsRef<str>,)) -> ApiResult<Option<User>> {
        Ok(query_as!(
            User,
            "SELECT id, ext_id, username, password_hash, email, created_at, updated_at
             FROM users
             WHERE email = $1",
            email.as_ref()
        )
            .fetch_optional(tx)
            .await?)
    }

    pub async fn find_by_email_with_context(&self, username: impl AsRef<str> + Send, context: UserContext) -> ApiResult<Option<User>> {
        self.execute_with_context(context, Self::find_by_email_inner, (username, )).await
    }

    pub async fn find_by_email2(&self, username: impl AsRef<str>) -> ApiResult<Option<User>> {
        self.find_by_email_inner(&mut self.pool(), (username,)).await
    }

    async fn execute_with_context<'c, Func, Args, Ex, Fut, Res, Err, DB>(&self, context: UserContext, function: Func, args: Args) -> Result<Res, Err>
    where
        Func: Fn(&Self, &mut Ex, Args) -> Fut + Send + 'static,
        Args: Tuple + Send + 'static,
        Ex: Executor<'c, Database = DB> + Send + 'static,
        Fut: Future<Output = Result<Res, Err>> + Send  + 'static,
        Res: Send + 'static,
        Err: From<sqlx::Error> + Send + 'static,
        DB: DatabaseTrait
    {
        self.with_context(context, move |tx| Box::pin(async move {
            function(&self, tx, args).await
        })).await
    }*/

    repository_method! {
         pub async fn find_by_email2(&self, tx, username: impl AsRef<str> + Send) -> ApiResult<Option<User>> {
            Ok(query_as!(
            User,
            "SELECT id, ext_id, username, password_hash, email, created_at, updated_at
             FROM users
             WHERE email = $1",
            username.as_ref()
        )
            .fetch_optional(tx)
            .await?)
        }
    }

    #[tracing::instrument(
        skip(self),
        err,
        name = "UsersRepository::get_all_with_context",
        level = "debug"
    )]
    pub async fn get_all_with_context(&self, context: &UserContext) -> ApiResult<Vec<User>> {
        self.with_context(context, move |mut tx| async move {
            let result = sqlx::query_as::<_, User>(
                "SELECT id, ext_id, username, password_hash, email, created_at, updated_at FROM users"
            )
                .fetch_all(&mut *tx)
                .await.map_err(Into::into);

            (tx, result)
        }).await
    }

    #[tracing::instrument(
        skip(self, ext_id),
        err,
        name = "UsersRepository::get_by_ext_id_with_context",
        level = "debug"
    )]
    pub async fn get_by_ext_id_with_context(
        &self,
        ext_id: impl Into<Uuid>,
        context: &UserContext,
    ) -> ApiResult<Option<User>> {
        let id = ext_id.into();
        self.with_context(context, move |mut tx| async move {
            let result = sqlx::query_as::<_, User>(
                "SELECT id, ext_id, username, password_hash, email, created_at, updated_at
                 FROM users
                 WHERE ext_id = $1",
            )
            .bind(id)
            .fetch_optional(&mut *tx)
            .await
            .map_err(Into::into);

            (tx, result)
        })
        .await
    }
}
