use serde::Serialize;

use super::player::Player;

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub enum EffectType {
    Shot,
    Healed,
    Investigated,
    Swapped(Player),
    Death,
}
