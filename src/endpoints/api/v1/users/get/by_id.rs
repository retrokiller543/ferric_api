use crate::dto::UserDTO;
use crate::error::ApiError;
use crate::models::user::User;
use crate::prelude::*;
use crate::repositories::users::UsersRepository;
use crate::traits::into_dto::IntoDTO;
use actix_helper_utils::generate_endpoint;
use actix_web::web;
use uuid::Uuid;

generate_endpoint! {
    /// Gets users by their external ID (UUID)
    #[tracing::instrument(skip(repo, authenticated_user))]
    fn get_user_by_id;
    method: get;
    path: "/{id}";
    return_type: Option<UserDTO>;
    error: ApiError;
    docs: {
        tag: "user",
        context_path: "/users"
        responses: {
            (status = 200, description = "Successfully fetched user", body = UserDTO)
        },
        security: [
            ("bearer_token" = [])
        ]
    }
    params: {
        repo: web::Data<UsersRepository>,
        authenticated_user: User,
        user_context: UserContext,
        id: web::Path<Uuid>
    }
    {
        let id = id.into_inner();

        if Some(id) == user_context.user_id() {
            Ok(Some(authenticated_user).into_dto())
        } else {
            let user = repo.get_by_ext_id_with_context(id, &user_context).await?;
            Ok(user.into_dto())
        }
    }
}
