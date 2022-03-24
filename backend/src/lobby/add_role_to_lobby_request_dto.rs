use serde::Deserialize;

use crate::roles::role::Role;

#[derive(Debug, Clone, Deserialize)]
pub struct AddRoleToLobbyRequestDto {
    role: Role,
}

impl AddRoleToLobbyRequestDto {
    pub fn get_role(&self) -> &Role {
        &self.role
    }
}