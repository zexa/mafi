use serde::Serialize;

use crate::roles::role::Role;

use strum::EnumIter;

#[cfg_attr(test, derive(EnumIter))]
#[derive(Debug, Clone, Serialize, PartialEq, Eq, Hash)]
pub enum ActionType {
    Shoot,
    Heal,
    Investigate,
    Swap,
    SelfConceptualize, // Alien
}
