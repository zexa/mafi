use serde::{Deserialize, Serialize};
use strum::{EnumString, EnumVariantNames};

#[derive(Debug, Clone, Serialize, Deserialize, EnumString, EnumVariantNames, PartialEq)]
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
