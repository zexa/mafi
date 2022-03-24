use std::{sync::Arc, collections::HashMap, str::FromStr};

use anyhow::format_err;
use async_trait::async_trait;
use axum::extract::{FromRequest, RequestParts, Extension, Path};
use hyper::StatusCode;
use rand::{prelude::SliceRandom, thread_rng};
use serde::Serialize;
use uuid::Uuid;

use crate::{lobby::lobby::Lobby, user::user::User, app_exception::AppException};

use super::{player::Player, game_repository::GameRepository, game_status::GameStatus};


#[derive(Debug, Clone, Serialize)]
pub struct Game {
    uuid: Uuid,
    lobby_uuid: Uuid,
    players: Vec<Player>,
    game_status: GameStatus,
}

impl Game {
    pub fn from_lobby(lobby: &Lobby, starting_user: &User) -> anyhow::Result<Self> {
        let mut users = lobby.get_players().clone();
        let mut roles = lobby.get_roles().clone();

        if users.len() != roles.len() {
            return Err(format_err!("Lobby must have the same ammount of players and roles"));
        }

        if lobby.get_owner().get_secret() != starting_user.get_secret() {
            return Err(format_err!("Only the lobby owner can start the game"));
        }

        users.shuffle(&mut thread_rng());
        roles.shuffle(&mut thread_rng());
        let mut players: Vec<Player> = vec![];

        while let Some(user) = users.pop() {
            let role = roles.pop().ok_or_else(|| {
                tracing::error!("Could not construct Vec<Player>");

                format_err!("The developer sucks, some theoretically impossible scenario happened")
            })?;

            let player = Player::new(user, role);

            players.push(player);
        }

        let game = Self {
            uuid: Uuid::new_v4(),
            lobby_uuid: lobby.get_uuid().clone(),
            players,
            game_status: GameStatus::Ongoing,
        };

        Ok(game)
    }

    pub fn get_uuid(&self) -> &Uuid {
        &self.uuid
    }

    pub fn get_lobby_uuid(&self) -> &Uuid {
        &self.lobby_uuid
    }

    pub fn set_game_status(&mut self, game_status: GameStatus) {
        self.game_status = game_status;
    }

    pub fn get_user_player(&self, user: &User) -> Option<Player> {
        self.players
            .clone()
            .into_iter()
            .find(|p| p.get_user().get_secret() == user.get_secret())
    }
}

#[async_trait]
impl<B> FromRequest<B> for Game
where
    B: Send,
{
    type Rejection = AppException;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        use crate::game::game_repository::Inner;

        let Extension(game_repository) = Extension::<Arc<GameRepository>>::from_request(req)
            .await
            .map_err(|e| {
                tracing::error!(%e, "Could not get game_repository from container");

                AppException::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string())
            })?;

        let Path(hm) = Path::<HashMap<String, String>>::from_request(req)
            .await
            .map_err(|e| {
                tracing::error!(%e, "Could not get lobby uuid from container");

                AppException::new(StatusCode::BAD_REQUEST, "Missing lobby uuid".to_string())
            })?;

        let uuid = hm.get("game_uuid").ok_or_else(|| {
            tracing::error!("Could not find :game_uuid param in request");

            AppException::new(StatusCode::BAD_REQUEST, "Missing lobby uuid".to_string())
        })?;
        let uuid = Uuid::from_str(uuid).map_err(|e| {
            tracing::error!(%e, "Could not parse game_uuid into a uuid");

            AppException::new(StatusCode::BAD_REQUEST, "Missing lobby uuid".to_string())
        })?;

        let game = game_repository
            .get_by_uuid(&uuid)
            .await
            .map_err(|e| {
                tracing::error!(%e, "Could not get game from game_repository");

                AppException::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string())

            })?
            .ok_or_else(|| {
                tracing::error!("Game not found in game_repository");

                AppException::new(StatusCode::NOT_FOUND, "Lobby not found".to_string())
            })?;

        Ok(game)
    }
}
