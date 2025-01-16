use crate::models::user::User;
use crate::repositories::{repository, PgQuery};
use crate::traits::model::Model;
use crate::traits::repository::Repository;
use crate::ApiResult;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use chrono::NaiveDateTime;
use sqlx::{query, query_as, PgPool};
use tracing::error;

repository! {
    pub UsersRepository;
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
    #[tracing::instrument(skip_all)]
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

        self.insert(&user).await?;
        Ok(user)
    }

    #[tracing::instrument(skip_all)]
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

    #[tracing::instrument(skip_all)]
    fn hash_password(&self, password: &impl AsRef<[u8]>) -> ApiResult<String> {
        let salt = SaltString::generate(&mut OsRng);

        let argon2 = Argon2::default();

        Ok(argon2.hash_password(password.as_ref(), &salt)?.to_string())
    }
}

impl Repository<User> for UsersRepository {
    #[inline]
    fn pool(&self) -> &PgPool {
        self.pool
    }

    #[inline]
    fn insert_one(model: &User) -> PgQuery<'_> {
        query!(
            "INSERT INTO users (username, password_hash, email)
             VALUES ($1, $2, $3)",
            model.username,
            model.password_hash,
            model.email,
        )
    }

    #[inline]
    fn update_one(model: &User) -> PgQuery<'_> {
        query!(
            "UPDATE users
             SET username = $1,
                 password_hash = $2,
                 email = $3,
                 updated_at = CURRENT_TIMESTAMP
             WHERE ext_id = $4",
            model.username,
            model.password_hash,
            model.email,
            model.ext_id
        )
    }

    #[inline]
    fn delete_one_by_id(id: &<User as Model>::Id) -> PgQuery<'_> {
        query!(
            "DELETE FROM users
             WHERE ext_id = $1",
            id
        )
    }

    #[tracing::instrument(skip_all)]
    async fn get_all(&self) -> ApiResult<Vec<User>> {
        Ok(query_as!(
            User,
            "SELECT id, ext_id, username, password_hash, email, created_at, updated_at FROM users"
        )
        .fetch_all(self.pool)
        .await?)
    }

    #[tracing::instrument(skip_all)]
    async fn get_by_id(&self, id: impl Into<<User as Model>::Id>) -> ApiResult<Option<User>> {
        let id = id.into();
        Ok(query_as!(
            User,
            "SELECT id, ext_id, username, password_hash, email, created_at, updated_at
             FROM users
             WHERE ext_id = $1",
            id
        )
        .fetch_optional(self.pool)
        .await?)
    }
}

impl UsersRepository {
    #[tracing::instrument(skip_all)]
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

    #[tracing::instrument(skip_all)]
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

    #[tracing::instrument(skip_all)]
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
}
