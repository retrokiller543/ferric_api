use crate::models::oauth_client::OAuthClient;
use crate::prelude::*;
use actix_oauth::types::GrantType;
use tracing::Span;

repository! {
    pub OauthClientsRepository;
}

impl Repository<OAuthClient> for OauthClientsRepository {
    fn pool(&self) -> &Pool {
        self.pool
    }

    fn repository_span() -> Span {
        Span::current()
    }
}

repository_insert! {
    OauthClientsRepository<OAuthClient>;

    insert_query(client) {
        query!(
            "INSERT INTO oauth_client (client_id, client_secret, redirect_uri, grant_types, scopes)
             VALUES ($1, $2, $3, $4, $5)",
            client.client_id,
            client.client_secret,
            client.redirect_uri,
            &client.grant_types as _,
            &client.scopes as _
        )
    }
}

repository_update! {
    OauthClientsRepository<OAuthClient>;

    update_query(client) {
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
    }
}

repository_delete! {
    OauthClientsRepository<OAuthClient>;

    delete_by_id_query(id) {
        query!(
            "DELETE FROM oauth_client
             WHERE client_id = $1",
            id
        )
    }

    delete_by_filter_query(filter) {
        let mut builder = QueryBuilder::new("DELETE FROM oauth_client WHERE ");

        filter.apply_filter(&mut builder);

        builder
    }
}

impl SelectRepository<OAuthClient> for OauthClientsRepository {
    fn get_all_query(&self) -> QueryAs<OAuthClient> {
        query_as(
            "SELECT
                client_id,
                client_secret,
                redirect_uri,
                grant_types,
                scopes,
                created_at
             FROM oauth_client",
        )
    }

    fn get_by_id_query(&self, id: impl Into<String>) -> QueryAs<OAuthClient> {
        let id = id.into();

        query_as(
            "SELECT
                client_id,
                client_secret,
                redirect_uri,
                grant_types,
                scopes,
                created_at
             FROM oauth_client
             WHERE client_id = $1",
        )
        .bind(id)
    }
}

#[allow(dead_code)]
impl OauthClientsRepository {
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
