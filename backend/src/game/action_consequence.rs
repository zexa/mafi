use serde::Serialize;

use crate::roles::role::Role;

use super::{effect_type::EffectType, information_type::InformationType};

#[derive(Debug, Clone, Serialize)]
pub enum ActionConsequence {
    ReturnsInformation{
        default: InformationType,
        on_escort: InformationType,
    },
    CausesEffectOnRole(Role, EffectType),
    CausesEffect(EffectType),
}
