use std::sync::Arc;

use axum::extract::Extension;
use axum::{response::IntoResponse, Json};
use axum_macros::debug_handler;
use hyper::StatusCode;

use crate::app_exception::AppException;
use crate::user::register_user_request_dto::RegisterUserRequestDto;
use crate::user::user::User;

use super::user_repository::UserRepository;

#[debug_handler]
pub async fn register_user(
    Json(register_user_request_dto): Json<RegisterUserRequestDto>,
    Extension(user_repository): Extension<Arc<UserRepository>>,
) -> Result<impl IntoResponse, AppException> {
    use crate::user::user_repository::Inner;
    let user: User = register_user_request_dto.into();
    user_repository.insert(user.clone()).await.map_err(|e| {
        tracing::error!(%e, "Could not insert user into user_repository");

        AppException::new(StatusCode::INTERNAL_SERVER_ERROR, "Could not register user".to_string())
    })?;

    Ok(Json(user))
}
