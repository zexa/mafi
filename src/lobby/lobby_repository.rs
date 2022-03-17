use anyhow::format_err;
use async_trait::async_trait;
use tokio::sync::RwLock;
use uuid::Uuid;

use super::lobby::Lobby;

#[async_trait]
pub trait Inner {
    async fn get_all(&self) -> anyhow::Result<Vec<Lobby>>;

    async fn get_by_uuid(&self, uuid: Uuid) -> anyhow::Result<Option<Lobby>>;

    async fn insert(&self, lobby: Lobby) -> anyhow::Result<()>;

    async fn update(&self, lobby: Lobby) -> anyhow::Result<()>;

    async fn delete(&self, lobby: Lobby) -> anyhow::Result<()>;
}

pub struct LobbyRepository {
    lobbies: RwLock<Vec<Lobby>>,
}

impl LobbyRepository {
    pub fn new(lobbies: RwLock<Vec<Lobby>>) -> Self { 
        Self { 
            lobbies 
        } 
    }
}

impl Default for LobbyRepository {
    fn default() -> Self {
        Self::new(RwLock::new(vec![]))
    }
}

#[async_trait]
impl Inner for LobbyRepository {
    async fn get_all(&self) -> anyhow::Result<Vec<Lobby>> {
        let lock = self.lobbies.read().await;

        Ok(lock.clone())
    }

    async fn get_by_uuid(&self, uuid: Uuid) -> anyhow::Result<Option<Lobby>> {
        let result = self.get_all().await?.into_iter()
            .find(|l| l.get_uuid() == &uuid);
        
        Ok(result)
    }
    
    async fn insert(&self, lobby: Lobby) -> anyhow::Result<()> {
        let mut lobbies = self.lobbies.write().await;
        lobbies.push(lobby);

        Ok(())
    }

    async fn update(&self, lobby: Lobby) -> anyhow::Result<()> {
        let mut lobbies = self.lobbies.write().await;
        let index = lobbies
            .iter()
            .position(|l| l.get_uuid() == lobby.get_uuid())
            .ok_or_else(|| format_err!("Lobby does not exist"))?;

        lobbies[index] = lobby;

        Ok(())
    }

    async fn delete(&self, lobby: Lobby) -> anyhow::Result<()> {
        let mut lobbies = self.lobbies.write().await;
        let index = lobbies
            .iter()
            .position(|l| l.get_uuid() == lobby.get_uuid())
            .ok_or_else(|| format_err!("no such value"))?;
        lobbies.remove(index);

        Ok(())
    }
}
