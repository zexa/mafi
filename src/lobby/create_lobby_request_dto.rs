use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct CreateLobbyRequestDto {
    name: String,
}

impl CreateLobbyRequestDto {
    pub fn get_name(&self) -> &str {
        &self.name
    }
}