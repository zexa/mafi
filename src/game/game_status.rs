use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub enum GameStatus {
    Ongoing,
    Finished,
}
