use crate::dto::UserDTO;
use crate::error::ApiError;
use crate::repositories::users::UsersRepository;
use crate::traits::into_dto::IntoDTO;
use crate::traits::repository::Repository;
use actix_helper_utils::generate_endpoint;
use actix_web::web;
use uuid::Uuid;

generate_endpoint! {
    fn get_user_by_id;
    method: get;
    path: "/{id}";
    return_type: Option<UserDTO>;
    error: ApiError;
    docs: {
        tag: "user",
        context_path: "/users"
        responses: {
            (status = 200, response = UserDTO)
        }
    }
    params: {
        repo: web::Data<UsersRepository>,
        id: web::Path<Uuid>
    }
    {
        let user = repo.get_by_id(id.into_inner()).await?;
        Ok(user.into_dto())
    }
}
