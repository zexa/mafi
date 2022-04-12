use std::{sync::Arc, str::FromStr};

use async_trait::async_trait;
use axum::{extract::{FromRequest, RequestParts, Extension, TypedHeader}, headers::Cookie};
use hyper::StatusCode;
use serde::Serialize;
use uuid::Uuid;

use crate::app_exception::AppException;

use super::user_repository::UserRepository;

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct User {
    name: String,
    secret: Uuid,
}

impl User {
    pub fn new(name: String, secret: Uuid) -> Self { Self { name, secret } }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_secret(&self) -> &Uuid {
        &self.secret
    }
}

#[async_trait]
impl<B> FromRequest<B> for User
where
    B: Send,
{
    type Rejection = AppException;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let Extension(user_repository) = Extension::<Arc<UserRepository>>::from_request(req)
            .await
            .map_err(|e| {
                tracing::error!(%e, "Could not get user_repository from container");

                AppException::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string())
            })?;

        let TypedHeader(cookies) = TypedHeader::<Cookie>::from_request(req)
            .await
            .map_err(|e| {
                tracing::error!(%e, "Could not get cookie header");

                AppException::new(StatusCode::INTERNAL_SERVER_ERROR, "Missing cookies".to_string())
            })?;

        let secret = cookies
            .get("SESSION")
            .ok_or_else(|| {
                tracing::error!("Could not get SESSION cookie from request");

                AppException::new(StatusCode::UNAUTHORIZED, "Missing SESSION cookie".to_string())
            })?;
        let secret = Uuid::from_str(secret).map_err(|e| {
            tracing::error!(%e, "SESSION cookie is an invalid uuid");

            AppException::new(StatusCode::UNAUTHORIZED, "SESSION cookie invalid".to_string())
        })?;

        let user = {
            use crate::user::user_repository::Inner;

            user_repository
                .get_by_secret(&secret)
                .await
                    .map_err(|e| {
                    tracing::error!(%e, "Could not get user from user_repository");

                    AppException::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string())
                })?
                .ok_or_else(|| {
                    tracing::error!("User not found in user_repository");

                    AppException::new(StatusCode::UNAUTHORIZED, "Invalid secret".to_string())
                })?
        };

        Ok(user)
    }
}
