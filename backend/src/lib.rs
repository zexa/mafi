use std::{sync::Arc, net::TcpListener};

use axum::{
    routing::{get, post, delete},
    Router, extract::{Extension, extractor_middleware},
};
use game::{controller::{get_game, start_game, end_game, get_all_games, get_user_player}, game_repository::GameRepository, game::Game};
use lobby::{controller::{create_lobby, delete_lobby, get_lobby, join_lobby, get_all_lobbies, add_role_to_lobby, remove_role_from_lobby}, lobby_repository::LobbyRepository, lobby::Lobby};
use roles::controller::get_all_roles;
use tracing::info;
use user::{controller::register_user, user_repository::UserRepository, user::User};

mod lobby;
mod roles;
mod user;
mod app_exception;
mod game;

pub fn get_router() -> Router {
    let game_router = Router::new()
        .route("/lobby/:lobby_uuid/game/:game_uuid/player/me", get(get_user_player))
        .route("/lobby/:lobby_uuid/game/:game_uuid", get(get_game))
        .route("/lobby/:lobby_uuid/game/:game_uuid", delete(end_game))
        .route_layer(extractor_middleware::<Game>());

    let lobby_aware_router = Router::new()
        .route("/lobby/:lobby_uuid", get(get_lobby))
        .route("/lobby/:lobby_uuid", delete(delete_lobby))
        .route("/lobby/:lobby_uuid/role", delete(remove_role_from_lobby))
        .route("/lobby/:lobby_uuid/role", post(add_role_to_lobby))
        .route("/lobby/:lobby_uuid/join", post(join_lobby))
        .route("/lobby/:lobby_uuid/game", post(start_game))
        .route("/lobby/:lobby_uuid/games", get(get_all_games))
        .merge(game_router)
        .route_layer(extractor_middleware::<Lobby>());

    let lobbies_router = Router::new()
        .route("/lobbies", get(get_all_lobbies))
        .route("/lobby", post(create_lobby))
        .merge(lobby_aware_router)
        .route_layer(extractor_middleware::<User>());

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/ping", get(|| async { "pong" }))
        .route("/user", post(register_user))
        .route("/roles", get(get_all_roles))
        .merge(lobbies_router)
        .layer(Extension(Arc::new(LobbyRepository::default())))
        .layer(Extension(Arc::new(UserRepository::default())))
        .layer(Extension(Arc::new(GameRepository::default())));

    app
}

pub async fn start(listener: TcpListener) -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    info!("Mafi backend started");
    
    axum::Server::from_tcp(listener)
        .unwrap()
        .serve(get_router().into_make_service())
        .await?;

    Ok(())
}
