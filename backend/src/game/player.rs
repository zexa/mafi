use serde::Serialize;

use crate::{user::user::User, roles::role::Role};

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct Player {
    user: User,
    role: Role,
}

impl Player {
    pub fn new(user: User, role: Role) -> Self {
        Self {
            user,
            role,
        }
    }

    pub fn get_user(&self) -> &User {
        &self.user
    }
}
