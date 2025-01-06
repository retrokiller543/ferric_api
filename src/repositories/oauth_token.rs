use crate::models::oauth_token::OAuthToken;
use crate::models::Model;
use crate::repositories::repository;
use crate::ApiResult;
use sqlx::{query, query_as};

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
