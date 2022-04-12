use super::{player::Player, action_type::ActionType};

pub struct AddActionToGameRequestDto {
    executor: Player,
    targets: Vec<Player>,
    action_type: ActionType,
}
