use std::{sync::Arc, collections::HashMap, str::FromStr};

use async_trait::async_trait;
use axum::extract::{FromRequest, RequestParts, Extension, Path};
use hyper::StatusCode;
use serde::Serialize;
use uuid::Uuid;

use crate::{roles::role::Role, user::user::User, app_exception::AppException};

use super::{create_lobby_request_dto::CreateLobbyRequestDto, lobby_repository::{LobbyRepository, self}};

#[derive(Debug, Clone, Serialize)]
pub struct Lobby {
    uuid: Uuid,
    name: String,
    owner: User,
    roles: Vec<Role>,
    players: Vec<User>,
}

impl Lobby {
    pub fn new(uuid: Uuid, name: String, owner: User, roles: Vec<Role>, players: Vec<User>) -> Self { 
        Self { 
            uuid, 
            name, 
            owner,
            roles, 
            players 
        } 
    }

    pub fn from_create_lobby_request_dto(dto: CreateLobbyRequestDto, owner: User) -> Self {
        Self::new(
            Uuid::new_v4(),
            dto.get_name().to_string(),
            owner,
            vec![],
            vec![],
        )
    }

    pub fn get_uuid(&self) -> &Uuid {
        &self.uuid
    }

    pub fn get_owner(&self) -> &User {
        &self.owner
    }

    pub fn get_roles(&self) -> &Vec<Role> {
        &self.roles
    }

    pub fn get_players(&self) -> &Vec<User> {
        &self.players
    }

    pub fn add_player(&mut self, player: User) {
        self.players.push(player);
    }

    pub fn add_role(&mut self, role: Role) { 
        self.roles.push(role);
    }
}

#[async_trait]
impl<B> FromRequest<B> for Lobby
where
    B: Send,
{
    type Rejection = AppException;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        use lobby_repository::Inner;

        let Extension(lobby_repository) = Extension::<Arc<LobbyRepository>>::from_request(req)
            .await
            .map_err(|e| {
                tracing::error!(%e, "Could not get lobby_repository from container");

                AppException::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string())
            })?;

        let Path(hm) = Path::<HashMap<String, String>>::from_request(req)
            .await
            .map_err(|e| {
                tracing::error!(%e, "Could not get lobby uuid from container");

                AppException::new(StatusCode::BAD_REQUEST, "Missing lobby uuid".to_string())
            })?;

        let uuid = hm.get("lobby_uuid").ok_or_else(|| {
            tracing::error!("Could not find :lobby_uuid param in request");

            AppException::new(StatusCode::BAD_REQUEST, "Missing lobby uuid".to_string())
        })?;
        let uuid = Uuid::from_str(uuid).map_err(|e| {
            tracing::error!(%e, "Could not parse lobby_uuid into a uuid");

            AppException::new(StatusCode::BAD_REQUEST, "Missing lobby uuid".to_string())
        })?;

        let lobby = lobby_repository
            .get_by_uuid(uuid)
            .await
            .map_err(|e| {
                tracing::error!(%e, "Could not get lobby from lobby_repository");

                AppException::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string())
            })?
            .ok_or_else(|| {
                tracing::error!("Lobby not found in lobby_repository");

                AppException::new(StatusCode::NOT_FOUND, "Lobby not found".to_string())
            })?;

        Ok(lobby)
    }
}
