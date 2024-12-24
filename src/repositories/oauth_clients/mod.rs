use crate::models::oauth_client::OAuthClient;
use crate::repositories::{repository, Repository};
use crate::ApiResult;
use actix_oauth::types::GrantType;
use sqlx::{query, query_as};

repository! {
    pub(crate) OAuthClientsRepository;
}

#[allow(dead_code)]
impl OAuthClientsRepository {
    pub(crate) async fn filter(
        &self,
        grant_types: Option<Vec<GrantType>>,
        scopes: Option<Vec<String>>,
    ) -> ApiResult<Vec<OAuthClient>> {
        let mut query = "SELECT
                            client_id,
                            client_secret,
                            redirect_uri,
                            grant_types as \"grant_types: Vec<GrantType>\",
                            scopes,
                            created_at
                         FROM oauth_client WHERE 1=1"
            .to_string();

        if grant_types.is_some() {
            query.push_str(" AND grant_types && $1");
        }
        if scopes.is_some() {
            query.push_str(" AND scopes && $2");
        }

        let result = match (grant_types, scopes) {
            (Some(gt), Some(sc)) => {
                query_as::<_, OAuthClient>(&query)
                    .bind(gt)
                    .bind(sc)
                    .fetch_all(self.pool)
                    .await?
            }
            (Some(gt), None) => {
                query_as::<_, OAuthClient>(&query)
                    .bind(gt)
                    .fetch_all(self.pool)
                    .await?
            }
            (None, Some(sc)) => {
                query_as::<_, OAuthClient>(&query)
                    .bind(sc)
                    .fetch_all(self.pool)
                    .await?
            }
            (None, None) => {
                query_as::<_, OAuthClient>(&query)
                    .fetch_all(self.pool)
                    .await?
            }
        };

        Ok(result)
    }
}

impl Repository<OAuthClient, String> for OAuthClientsRepository {
    async fn get_all(&self) -> ApiResult<Vec<OAuthClient>> {
        Ok(query_as!(
            OAuthClient,
            "SELECT
                client_id,
                client_secret,
                redirect_uri,
                grant_types as \"grant_types: Vec<GrantType>\",
                scopes,
                created_at
             FROM oauth_client"
        )
        .fetch_all(self.pool)
        .await?)
    }

    async fn get_by_id(&self, client_id: impl Into<String>) -> ApiResult<Option<OAuthClient>> {
        let client_id = client_id.into();

        Ok(query_as!(
            OAuthClient,
            "SELECT
                client_id,
                client_secret,
                redirect_uri,
                grant_types as \"grant_types: Vec<GrantType>\",
                scopes,
                created_at
             FROM oauth_client
             WHERE client_id = $1",
            client_id
        )
        .fetch_optional(self.pool)
        .await?)
    }
    async fn insert(&self, client: &OAuthClient) -> ApiResult<()> {
        query!(
            "INSERT INTO oauth_client (client_id, client_secret, redirect_uri, grant_types, scopes)
             VALUES ($1, $2, $3, $4, $5)",
            client.client_id,
            client.client_secret,
            client.redirect_uri,
            &client.grant_types as _,
            &client.scopes as _
        )
        .execute(self.pool)
        .await?;
        Ok(())
    }
    async fn update(&self, client: &OAuthClient) -> ApiResult<()> {
        query!(
            "UPDATE oauth_client
             SET client_secret = $1,
                 redirect_uri = $2,
                 grant_types = $3,
                 scopes = $4
             WHERE client_id = $5",
            client.client_secret,
            client.redirect_uri,
            &client.grant_types as _,
            &client.scopes as _,
            client.client_id
        )
        .execute(self.pool)
        .await?;
        Ok(())
    }
    async fn delete_by_id(&self, client_id: impl Into<String>) -> ApiResult<()> {
        let client_id = client_id.into();

        query!(
            "DELETE FROM oauth_client
             WHERE client_id = $1",
            client_id
        )
        .execute(self.pool)
        .await?;
        Ok(())
    }
}
