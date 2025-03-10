use crate::error::ApiError;
use crate::prelude::*;
use crate::repositories::users::UsersRepository;
use actix_helper_utils::generate_endpoint;
use actix_web::{HttpResponse, web};
use uuid::Uuid;

generate_endpoint! {
    /// Delete a user given a external ID (UUID)
    ///
    /// TODO: Currently there is no authentication, this will be changed in the future
    fn delete_user;
    method: delete;
    path: "/{id}";
    error: ApiError;
    docs: {
        tag: "user",
        context_path: "/users",
        responses: {
            (status = 200, description = "Deleted the user"),
            (status = 500, description = "Error deleting user", body = ErrorDTO)
        }
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
