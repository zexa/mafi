use serde::{Deserialize, Serialize};
use strum::{EnumIter, EnumString, EnumVariantNames};

#[cfg_attr(test, derive(EnumIter))]
#[derive(Debug, Clone, Serialize, Deserialize, EnumString, EnumVariantNames, PartialEq, Eq, Hash)]
pub enum Role {
    Godfather,
    MafiaGoon,
    Silencer,
    Escort,
    Medic,
    Vigilante,
    Sheriff,
    Townie,
    SerialKiller,
    Alien,
}
