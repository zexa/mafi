use async_trait::async_trait;
use tokio::sync::RwLock;
use uuid::Uuid;

use super::user::User;

#[async_trait]
pub trait Inner {
    async fn insert(&self, i: User) -> anyhow::Result<()>;
    async fn get_all(&self) -> anyhow::Result<Vec<User>>;
    async fn get_by_name(&self, name: &str) -> anyhow::Result<Option<User>>;
    async fn get_by_secret(&self, secret: &Uuid) -> anyhow::Result<Option<User>>;
}

pub struct UserRepository {
    users: RwLock<Vec<User>>
}

impl UserRepository {
    pub fn new(users: RwLock<Vec<User>>) -> Self { Self { users } }
}

impl Default for UserRepository {
    fn default() -> Self {
        Self::new(RwLock::new(vec![]))
    }
}

#[async_trait]
impl Inner for UserRepository {
    async fn insert(&self, i: User) -> anyhow::Result<()> {
        let mut users = self.users.write().await;
        users.push(i);
        Ok(())
    }

    async fn get_all(&self) -> anyhow::Result<Vec<User>> {
        let users = self.users.read().await;

        Ok(users.clone())
    }

    async fn get_by_name(&self, name: &str) -> anyhow::Result<Option<User>> {
        let found = self.get_all().await?.into_iter().find(|u| u.get_name() == name);

        Ok(found)
    }

    async fn get_by_secret(&self, secret: &Uuid) -> anyhow::Result<Option<User>> {
        let found = self.get_all().await?.into_iter().find(|u| u.get_secret() == secret);

        Ok(found)
    }
}
