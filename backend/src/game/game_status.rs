use serde::Serialize;

use crate::roles::role::Role;

use super::game_stage::GameStage;

#[derive(Debug, Clone, Serialize)]
pub enum GameStatus {
    Night(Role), // Role here symbolizes who's turn it currently is
    Day,
    Voting,
    Finished,
}
