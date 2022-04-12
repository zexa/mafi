use serde::Serialize;

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub enum InformationType {
    IsActivated,
    IsntActivated,
    HasGun,
    DoesntHaveGun,
}
