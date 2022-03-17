use anyhow::format_err;
use async_trait::async_trait;
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::lobby::lobby::Lobby;

use super::game::Game;

#[async_trait]
pub trait Inner {
    async fn insert(&self, i: Game) -> anyhow::Result<()>;
    async fn get_all(&self) -> anyhow::Result<Vec<Game>>;
    async fn get_by_lobby(&self, l: &Lobby) -> anyhow::Result<Vec<Game>>;
    async fn get_by_uuid(&self, u: &Uuid) -> anyhow::Result<Option<Game>>;
    async fn update(&self, g: Game) -> anyhow::Result<()>;
}

pub struct GameRepository {
    games: RwLock<Vec<Game>>,
}

impl GameRepository {
    pub fn new(games: RwLock<Vec<Game>>) -> Self {
        Self {
            games,
        }
    }
}

impl Default for GameRepository {
    fn default() -> Self {
        Self::new(RwLock::new(vec![]))
    }
}

#[async_trait]
impl Inner for GameRepository {
    async fn insert(&self, i: Game) -> anyhow::Result<()> {
        let mut games = self.games.write().await;
        games.push(i);

        Ok(())
    }

    async fn get_all(&self) -> anyhow::Result<Vec<Game>> {
        let games = self.games.read().await;

        Ok(games.clone())
    }

    async fn get_by_lobby(&self, l: &Lobby) -> anyhow::Result<Vec<Game>> {
        let found = self
            .get_all()
            .await?
            .into_iter()
            .filter(|g| g.get_lobby_uuid() == l.get_uuid())
            .collect();

        Ok(found)
    }

    async fn get_by_uuid(&self, u: &Uuid) -> anyhow::Result<Option<Game>> {
        let games = self.get_all().await?;
        let found = games.into_iter().find(|g| g.get_uuid() == u);

        Ok(found)
    }

    async fn update(&self, g: Game) -> anyhow::Result<()> {
        let mut games = self.games.write().await;
        let position = games
            .iter()
            .position(|game| game.get_uuid() == g.get_uuid())
            .ok_or_else(|| {
                format_err!("Could not find game")
            })?;
        games[position] = g;

        Ok(())
    }
}
