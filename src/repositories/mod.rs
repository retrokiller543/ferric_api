#![allow(dead_code)]

use crate::ApiResult;

mod filter;
pub(crate) mod oauth_clients;
pub(crate) mod users;

/// Default repository methods, each repository may extend this themselves
#[allow(dead_code)]
pub(crate) trait Repository<Model, Id> {
    async fn get_all(&self) -> ApiResult<Vec<Model>>;
    async fn get_by_id(&self, id: impl Into<Id>) -> ApiResult<Option<Model>>;

    async fn save(&self, _: &Model) -> ApiResult<()> {
        unimplemented!("This method has not been implemented for this repository")
    }

    async fn insert(&self, item: &Model) -> ApiResult<()>;
    async fn update(&self, item: &Model) -> ApiResult<()>;

    async fn delete_by_id(&self, id: impl Into<Id>) -> ApiResult<()>;

    async fn insert_many(&self, _: &[Model]) -> ApiResult<()> {
        unimplemented!("This method has not been implemented for this repository")
    }
    async fn update_many(&self, _: &[Model]) -> ApiResult<()> {
        unimplemented!("This method has not been implemented for this repository")
    }
    async fn delete_many(&self, _: &[Id]) -> ApiResult<()> {
        unimplemented!("This method has not been implemented for this repository")
    }
}

macro_rules! repository {
    (
        $( #[$meta:meta] )*
        $vis:vis $ident:ident;
    ) => {
        $(#[$meta])*
        #[derive(Clone, Copy, Debug)]
        $vis struct $ident {
            pool: &'static ::sqlx::PgPool,
        }

        impl $ident {
            $vis async fn new() -> crate::ServerResult<Self> {
                let pool = crate::setup::database::get_db_pool().await?;
                Ok(Self {
                    pool
                })
            }
        }
    };
}
pub(crate) use repository;
