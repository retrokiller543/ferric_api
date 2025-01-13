use crate::models::oauth_token::OAuthToken;
use crate::repositories::repository;
use crate::traits::model::Model;
use crate::traits::{Repository, SqlFilter};
use crate::ApiResult;
use sqlx::{query, query_as, QueryBuilder};
use sqlx_filter_macro::sql_filter;
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
    pub(crate) OauthTokenRepository<OAuthToken>;

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

    type Filter<'args> = OauthTokenFilter;

    #[tracing::instrument(skip_all, level = "debug")]
    async fn get_by_id(&self, id: impl Into<i64>) -> ApiResult<Option<OAuthToken>> {
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
    async fn get_by_any_filter(&self, filter: impl SqlFilter<'_>) -> ApiResult<Vec<OAuthToken>> {
        let mut query = QueryBuilder::new("SELECT * FROM oauth_token WHERE ");

        filter.apply_filter(&mut query);

        Ok(query.build_query_as().fetch_all(self.pool).await?)
    }

    #[tracing::instrument(skip_all, level = "debug")]
    async fn get_by_filter(&self, filter: Self::Filter<'_>) -> ApiResult<Vec<OAuthToken>> {
        let mut query = QueryBuilder::new("SELECT * FROM oauth_token WHERE ");

        filter.apply_filter(&mut query);

        Ok(query.build_query_as().fetch_all(self.pool).await?)
    }

    #[tracing::instrument(skip_all, level = "debug")]
    async fn insert_many(&self, tokens: impl IntoIterator<Item = OAuthToken>) -> ApiResult<()> {
        let tokens: Vec<_> = tokens.into_iter().collect();

        if tokens.is_empty() {
            return Ok(());
        }

        // Create the base query
        let mut query = String::from(
            "INSERT INTO oauth_token (token, client_id, user_ext_id, token_type, scopes, expires_at) VALUES "
        );

        // Generate the placeholders for each row
        let mut param_count = 1;
        let values: Vec<String> = tokens
            .iter()
            .map(|_| {
                let placeholders = format!(
                    "(${},${},${},${},${},${})",
                    param_count,
                    param_count + 1,
                    param_count + 2,
                    param_count + 3,
                    param_count + 4,
                    param_count + 5
                );
                param_count += 6;
                placeholders
            })
            .collect();

        // Join all value placeholders with commas
        query.push_str(&values.join(","));

        // Build the query with all parameters
        let mut q = sqlx::query(&query);

        // Bind all parameters for each token
        for token in tokens {
            q = q.bind(token.token)
                .bind(token.client_id)
                .bind(token.user_ext_id)
                .bind(token.token_type)
                .bind(token.scopes)
                .bind(token.expires_at);
        }

        // Execute the query within the transaction
        q.execute(self.pool).await?;
        Ok(())
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
        self.get_by_any_filter(filter).await
    }
}
