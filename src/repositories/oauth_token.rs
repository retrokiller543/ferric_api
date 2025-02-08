use crate::models::oauth_token::OAuthToken;
use crate::ApiResult;
use sqlx::{query, query_as, QueryBuilder};
use sqlx_utils::repository;
use sqlx_utils::sql_filter;
use sqlx_utils::traits::{Repository, SqlFilter};
use uuid::Uuid;

sql_filter! {
    #[derive(Default, Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
    pub struct OauthTokenFilter {
        SELECT * FROM oauth_token
        WHERE
            ?token = String
            AND ?user_ext_id = Uuid
            AND ?token_type as token_types IN Vec<String>
            AND expires_at > "CURRENT_TIMESTAMP"
    }
}

repository! {
    pub OauthTokenRepository<OAuthToken>;

    insert_one(model) {
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
    };

    #[tracing::instrument(skip_all, level = "debug")]
    async fn get_by_id(&self, id: impl Into<i64>) -> sqlx_utils::Result<Option<OAuthToken>> {
        let id = id.into();

        Ok(
            query_as!(
                OAuthToken,
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
                id
            )
            .fetch_optional(self.pool)
            .await?
        )
    }

    #[tracing::instrument(skip_all, level = "debug")]
    async fn get_by_any_filter(&self, filter: impl SqlFilter<'_>) -> sqlx_utils::Result<Vec<OAuthToken>> {
        let mut query = QueryBuilder::new("SELECT * FROM oauth_token WHERE ");

        filter.apply_filter(&mut query);

        Ok(query.build_query_as().fetch_all(self.pool).await?)
    }
}

impl OauthTokenRepository {
    #[tracing::instrument(skip_all)]
    pub(crate) async fn get_by_token(
        &self,
        token: impl Into<String>,
    ) -> ApiResult<Option<OAuthToken>> {
        let token = token.into();

        Ok(query_as!(
            OAuthToken,
            "SELECT
                id,
                token,
                client_id,
                user_ext_id,
                token_type as \"token_type: _\",
                scopes,
                expires_at,
                created_at
             FROM oauth_token WHERE token = $1 AND expires_at > CURRENT_TIMESTAMP",
            token
        )
        .fetch_optional(self.pool)
        .await?)
    }

    pub(crate) async fn get_by_filter(
        &self,
        filter: OauthTokenFilter,
    ) -> ApiResult<Vec<OAuthToken>> {
        self.get_by_any_filter(filter).await.map_err(Into::into)
    }
}
