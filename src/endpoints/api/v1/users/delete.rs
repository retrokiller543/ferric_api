use crate::error::ApiError;
use crate::repositories::users::UsersRepository;
use crate::traits::repository::Repository;
use actix_helper_utils::generate_endpoint;
use actix_web::{web, HttpResponse};
use uuid::Uuid;

generate_endpoint! {
    fn delete_user;
    method: delete;
    path: "/{id}";
    error: ApiError;
    docs: {
        tag: "user",
        context_path: "/users",
    }
    params: {
        repo: web::Data<UsersRepository>,
        id: web::Path<Uuid>
    }
    {
        repo.delete_by_id(id.into_inner()).await?;
        Ok(HttpResponse::Ok())
    }
}
