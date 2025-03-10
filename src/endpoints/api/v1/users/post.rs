use crate::dto::user::create::UserCreateDTO;
use crate::error::ApiError;
use crate::prelude::*;
use crate::repositories::users::UsersRepository;
use actix_helper_utils::generate_endpoint;
use actix_web::http::StatusCode;
use actix_web::{HttpResponseBuilder, web};
use validator::Validate;

generate_endpoint! {
    /// Creates a new user.
    fn create_user;
    method: post;
    path: "";
    error: ApiError;
    docs: {
        tag: "user",
        context_path: "/users",
        request_body: {
            schema = UserCreateDTO
        }
        responses: {
            (status = 201, description = "User was created"),
            (status = 400, description = "Validation of the body failed", body = ErrorDTO),
            (status = 500, description = "Internal Server error", body = ErrorDTO),
        }
    }
    params: {
        repo: web::Data<UsersRepository>,
        web::Json(dto): web::Json<UserCreateDTO>
    }
    {
        match dto.validate() {
            Ok(_) => {}
            Err(error) => { return Err(error.into()); }
        }

        repo.create_user(&dto.username, &dto.email, &dto.password).await?;

        Ok(HttpResponseBuilder::new(StatusCode::CREATED))
    }
}
