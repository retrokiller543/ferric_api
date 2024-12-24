use crate::dto::IntoDTO;
use crate::error::ApiError;
use crate::repositories::users::UsersRepository;
use crate::repositories::Repository;
use actix_helper_utils::generate_endpoint;
use actix_web::web;
use uuid::Uuid;

generate_endpoint! {
    fn get_user_by_id;
    method: get;
    path: "/{id}";
    error: ApiError;
    params: {
        repo: web::Data<UsersRepository>,
        id: web::Path<Uuid>
    }
    {
        let user = repo.get_by_id(id.into_inner()).await?;
        Ok(user.into_dto())
    }
}
