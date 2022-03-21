use std::sync::Arc;

use axum::{response::IntoResponse, extract::Extension, Json};
use hyper::StatusCode;

use crate::{app_exception::AppException, lobby::lobby::Lobby, user::user::User, game::game_status::GameStatus};

use super::{game_repository::GameRepository, game::Game};

pub async fn start_game(
    user: User,
    lobby: Lobby,
    Extension(game_repository): Extension<Arc<GameRepository>>,
) -> Result<impl IntoResponse, AppException> {
    use crate::game::game_repository::Inner;

    let game = Game::from_lobby(&lobby, &user)
        .map_err(|e| {
            tracing::error!(%e, "Could not start game");

            AppException::new(StatusCode::BAD_REQUEST, e.to_string())
        })?;

    game_repository.insert(game.clone()).await.map_err(|e| {
        tracing::error!(%e, "Could not start game");

        AppException::new(StatusCode::INTERNAL_SERVER_ERROR, "Database issues".to_string())
    })?;

    Ok(Json(game))
}

pub async fn get_game(
    game: Game,
) -> Result<impl IntoResponse, AppException> {
    Ok(Json(game))
}

pub async fn get_all_games(
    lobby: Lobby,
    Extension(game_repository): Extension<Arc<GameRepository>>,
) -> Result<impl IntoResponse, AppException> {
    use crate::game::game_repository::Inner;
    let games = game_repository
        .get_by_lobby(&lobby)
        .await
        .map_err(|e| {
            tracing::error!(%e, "Could not get games from game_repository");

            AppException::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_string()
            )
        })?;

    Ok(Json(games))
}

pub async fn end_game(
    mut game: Game,
    lobby: Lobby,
    user: User,
    Extension(game_repository): Extension<Arc<GameRepository>>,
) -> Result<impl IntoResponse, AppException> {
    use crate::game::game_repository::Inner;

    if lobby.get_owner().get_secret() != user.get_secret() {
        tracing::error!("User tried to end game that does not belong to him");

        let exception = AppException::new(
            StatusCode::BAD_REQUEST,
            "Cannot end game that you're not the owner of".to_string()
        );

        return Err(exception);
    }

    game.set_game_status(GameStatus::Finished);

    game_repository.update(game.clone()).await.map_err(|e| {
        tracing::error!(%e, "Could not update game in game_repository");

        AppException::new(
            StatusCode::BAD_REQUEST,
            "Cannot end game that you're not the owner of".to_string()
        )
    })?;

    Ok(StatusCode::OK)
}

pub async fn get_user_player(
    game: Game,
    user: User,
) -> Result<impl IntoResponse, AppException> {
    let player = game.get_user_player(&user).ok_or_else(|| {
        tracing::error!("Non player user tried to get their role");

        AppException::new(
            StatusCode::BAD_REQUEST,
            "You are not a player in this game".to_string()
        )
    })?;

    Ok(Json(player))
}
