use std::sync::Arc;

use axum::{response::IntoResponse, extract::Extension, Json};
use axum_macros::debug_handler;
use hyper::StatusCode;

use crate::{lobby::{lobby::Lobby, remove_role_from_lobby_request_dto::RemoveRoleFromLobbyRequestDto}, app_exception::AppException, user::user::User};

use super::{lobby_repository::LobbyRepository, create_lobby_request_dto::CreateLobbyRequestDto, add_role_to_lobby_request_dto::AddRoleToLobbyRequestDto};

pub async fn create_lobby(
    user: User,
    Json(create_lobby_request): Json<CreateLobbyRequestDto>,
    Extension(lobby_repository): Extension<Arc<LobbyRepository>>,
) -> Result<impl IntoResponse, AppException> {
    use super::lobby_repository::Inner;

    let lobby = Lobby::from_create_lobby_request_dto(create_lobby_request, user);

    lobby_repository.insert(lobby.clone()).await.map_err(|e| {
        tracing::error!(%e, "Could not insert lobby into lobby_repository");

        AppException::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })?;

    Ok(Json(lobby))
}

pub async fn get_all_lobbies(
    Extension(lobby_repository): Extension<Arc<LobbyRepository>>,
) -> Result<impl IntoResponse, AppException> {
    use super::lobby_repository::Inner;

    let lobbies = lobby_repository.get_all().await.map_err(|e| {
        tracing::error!(%e, "Could not get all lobbies from lobby_repository");

        AppException::new(StatusCode::INTERNAL_SERVER_ERROR ,e.to_string())
    })?;

    Ok(Json(lobbies))
}

pub async fn get_lobby(
    lobby: Lobby,
) -> Result<impl IntoResponse, AppException> {
    Ok(Json(lobby))
}

pub async fn delete_lobby(
    lobby: Lobby,
    user: User,
    Extension(lobby_repository): Extension<Arc<LobbyRepository>>,
) -> Result<impl IntoResponse, AppException> {
    use super::lobby_repository::Inner;

    if lobby.get_owner().get_secret() != user.get_secret() {
        tracing::error!("Non owner tried to delete lobby");
        let exception = AppException::new(StatusCode::UNAUTHORIZED, "Only the owner can delete the lobby".to_string());
        return Err(exception);
    }

    lobby_repository
        .delete(lobby)
        .await
        .map_err(|e| {
            tracing::error!(%e, "Could not delete lobby from lobby_repository");

            AppException::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })?;

    Ok(StatusCode::OK)
}

pub async fn join_lobby(
    user: User,
    mut lobby: Lobby,
    Extension(lobby_repository): Extension<Arc<LobbyRepository>>,
) -> Result<impl IntoResponse, AppException> {
    use super::lobby_repository::Inner;

    if lobby.get_owner().get_secret() == user.get_secret() {
        tracing::error!("Owner tried to join its own lobby");
        let exception = AppException::new(StatusCode::BAD_REQUEST, "The owner cannot be a player".to_string());
        return Err(exception);
    }

    lobby.add_player(user).map_err(|e| {
        tracing::error!(%e, "Could not add player to lobby because it was already a member");

        AppException::new(StatusCode::BAD_REQUEST, "Could not add player to lobby because it is already a member".to_string())
    })?;
    
    lobby_repository
        .update(lobby)
        .await
        .map_err(|e| {
            tracing::error!(%e, "Could not update lobby in lobby_repository");

            AppException::new(StatusCode::INTERNAL_SERVER_ERROR, "Could not add player to lobby".to_string())
        })?;

    Ok(StatusCode::OK)
}

#[debug_handler]
pub async fn add_role_to_lobby(
    user: User,
    mut lobby: Lobby,
    Json(dto): Json<AddRoleToLobbyRequestDto>,
    Extension(repository): Extension<Arc<LobbyRepository>>,
) -> Result<impl IntoResponse, AppException> {
    use crate::lobby::lobby_repository::Inner;

    if lobby.get_owner().get_secret() != user.get_secret() {
        tracing::error!("Non owner tried to add role");
        let exception = AppException::new(
            StatusCode::BAD_REQUEST, 
            "Only the owner can add roles to the lobby".to_string()
        );
        return Err(exception);
    }

    lobby.add_role(dto.get_role().clone());

    repository
        .update(lobby)
        .await
        .map_err(|e| {
            tracing::error!(%e, "Could not update lobby in lobby_repository");

            AppException::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_string()
            )
        })?;

    Ok(StatusCode::OK)
}


#[debug_handler]
pub async fn remove_role_from_lobby(
    user: User,
    mut lobby: Lobby,
    Json(dto): Json<RemoveRoleFromLobbyRequestDto>,
    Extension(repository): Extension<Arc<LobbyRepository>>,
) -> Result<impl IntoResponse, AppException> {
    use crate::lobby::lobby_repository::Inner;

    if lobby.get_owner().get_secret() != user.get_secret() {
        tracing::error!("Non owner tried to remove role");
        let exception = AppException::new(
            StatusCode::BAD_REQUEST, 
            "Only the owner can remove roles from the lobby".to_string()
        );
        return Err(exception);
    }

    lobby.remove_role(dto.get_role().clone()).map_err(|_| {
        tracing::error!("Tried to remove role that does not exist in the lobby");

        AppException::new(
            StatusCode::BAD_REQUEST,
            "Role does not exist in the lobby".to_string()
        )
    })?;

    repository
        .update(lobby)
        .await
        .map_err(|e| {
            tracing::error!(%e, "Could not update lobby in lobby_repository");

            AppException::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_string()
            )
        })?;

    Ok(StatusCode::OK)
}