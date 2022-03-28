use serde::Deserialize;

use crate::roles::role::Role;

#[derive(Debug, Clone, Deserialize)]
pub struct RemoveRoleFromLobbyRequestDto {
    role: Role,
}

impl RemoveRoleFromLobbyRequestDto {
    pub fn get_role(&self) -> &Role {
        &self.role
    }
}
