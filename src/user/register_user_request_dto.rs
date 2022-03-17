use serde::Deserialize;
use uuid::Uuid;

use super::user::User;

#[derive(Debug, Clone, Deserialize)]
pub struct RegisterUserRequestDto {
    name: String,
}

impl Into<User> for RegisterUserRequestDto {
    fn into(self) -> User {
        User::new(self.name, Uuid::new_v4())
    }
}
