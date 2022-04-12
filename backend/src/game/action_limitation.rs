use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub enum ActionLimitation {
    IsOptional,
    TargetsLivingPlayers,
    TargetsAmountOfPlayers(i8),
    DoesNotTarget,
}
