use uuid::Uuid;

use super::{player::Player, action_type::ActionType};

pub struct Action {
    uuid: Uuid,
    lobby_uuid: Uuid,
    executor: Player,
    targets: Vec<Player>,
    action_type: ActionType,
}
