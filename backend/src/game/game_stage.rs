use serde::Serialize;

use crate::roles::role::Role;

#[derive(Debug, Clone, Serialize)]
pub enum GameStage {
    Night(Role), // Role here symbolizes who's turn it currently is
    Day,
    Voting,
}
