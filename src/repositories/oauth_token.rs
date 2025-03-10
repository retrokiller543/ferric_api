use crate::models::oauth_token::{OAuthToken, TokenType};
use crate::prelude::{RepositoryRls, UserContext};
use crate::{ApiResult, repository_method};
use sqlx_utils::filter::*;
use sqlx_utils::prelude::*;
use tracing::{Span, error};
use uuid::Uuid;

sql_filter! {
    #[derive(Default, Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
    pub struct OauthTokenFilter<OauthTokenRepository> {
        SELECT
            id,
            token,
            client_id,
            user_ext_id,
            token_type,
            scopes,
            expires_at,
            created_at
        FROM
            oauth_token
        WHERE
            ?token = String
            AND ?user_ext_id = Uuid
            AND ?token_type as token_types IN Vec<TokenType>
            AND expires_at > "CURRENT_TIMESTAMP"
    }
}

repository! {
    pub OauthTokenRepository;
}

impl Repository<OAuthToken> for OauthTokenRepository {
    fn pool(&self) -> &Pool {
        self.pool
    }

    fn repository_span() -> Span {
        Span::current()
    }
}

repository_insert! {
    OauthTokenRepository<OAuthToken>;

    insert_query(model) {
        query!(
            "INSERT INTO oauth_token (token, client_id, user_ext_id, token_type, scopes, expires_at)
             VALUES ($1, $2, $3, $4, $5, $6)",
            model.token,
            model.client_id,
            model.user_ext_id,
            model.token_type as _,
            model.scopes as _,
            model.expires_at,
        )
    }
}

repository_update! {
    OauthTokenRepository<OAuthToken>;

    update_query(model) {
        error!(model = ?model, "OauthTokenRepository is not supposed to update any tokens!");

        query("SELECT NULL as null")
    }
}

impl SelectRepository<OAuthToken> for OauthTokenRepository {
    fn get_all_query(&self) -> QueryAs<OAuthToken> {
        query_as(
            "SELECT
                    id,
                    token,
                    client_id,
                    user_ext_id,
                    token_type as \"token_type: _\",
                    scopes,
                    expires_at,
                    created_at
                 FROM oauth_token WHERE expires_at > CURRENT_TIMESTAMP",
        )
    }

    fn get_by_id_query(&self, id: impl Into<i64>) -> QueryAs<OAuthToken> {
        query_as(
            "SELECT
                    id,
                    token,
                    client_id,
                    user_ext_id,
                    token_type as \"token_type: _\",
                    scopes,
                    expires_at,
                    created_at
                 FROM oauth_token WHERE id = $1 AND expires_at > CURRENT_TIMESTAMP",
        )
        .bind(id.into())
    }
}

impl OauthTokenRepository {
    repository_method! {
        pub(crate) async fn get_by_token(
            &self,
            tx,
            token: impl Into<String> + Send,
        ) -> ApiResult<OAuthToken> {
            let token = token.into();

            let filter = equals("token", Some(token)).and(greater_than_raw("expires_at", Raw("CURRENT_TIMESTAMP")));

            let mut builder = Self::prepare_filter_query(filter);

            builder.build_query_as().fetch_one(tx).await.map_err(Into::into)
        }
    }

    fn save_query<'a>(&self, model: &'a OAuthToken) -> Query<'a> {
        if model.get_id().is_none() {
            Self::insert_query(model)
        } else {
            Self::update_query(model)
        }
    }

    pub async fn save_with_context(
        &self,
        model: &OAuthToken,
        context: &UserContext,
    ) -> ApiResult<()> {
        self.with_context(context, move |mut tx| async move {
            let connection = &mut *tx;

            let result = self.save_query(model).execute(connection).await;

            (tx, result)
        })
        .await?;

        Ok(())
    }
}
