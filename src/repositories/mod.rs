use crate::ApiResult;

mod filter;
pub(crate) mod oauth_clients;

/// Default repository methods, each repository may extend this themselves
pub(crate) trait Repository<Model, Id> {
    async fn get_all(&self) -> ApiResult<Vec<Model>>;
    async fn insert(&self, item: &Model) -> ApiResult<()>;
    async fn get_by_id(&self, id: impl Into<Id>) -> ApiResult<Option<Model>>;
    async fn update(&self, item: &Model) -> ApiResult<()>;
    async fn delete_by_id(&self, id: impl Into<Id>) -> ApiResult<()>;
}
