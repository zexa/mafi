use std::collections::HashMap;

use serde::Serialize;

use crate::roles::role::Role;

use super::{action_type::ActionType, action_limitation::ActionLimitation, action_consequence::ActionConsequence, information_type::InformationType, effect_type::EffectType};

#[derive(Debug, Clone, Serialize)]
pub struct GameRules {
    available_actions: HashMap<Role, ActionType>,
    action_limitations: HashMap<ActionType, Vec<ActionLimitation>>,
    action_consequences: HashMap<ActionType, Vec<ActionConsequence>>,
}

impl GameRules {
    pub fn get_available_actions(&self) -> &HashMap<Role, ActionType> {
        &self.available_actions
    }

    pub fn get_action_limitations(&self) -> &HashMap<ActionType, Vec<ActionLimitation>> {
        &self.action_limitations
    }

    pub fn get_action_limitations_by_action_type(&self, action_type: ActionType) -> Vec<ActionLimitation> {
        match self.action_limitations.get(&action_type) {
            Some(limitations) => limitations.clone(),
            None => todo!(),
        }
    }

    pub fn get_action_consequences(&self) -> &HashMap<ActionType, Vec<ActionConsequence>> {
        &self.action_consequences
    }

    pub fn get_action_consequences_by_action_type(&self, action_type: ActionType) -> Vec<ActionConsequence> {
        match self.action_consequences.get(&action_type) {
            Some(consequence) => consequence.clone(),
            None => todo!(),
        }
    }
}

impl Default for GameRules {
    fn default() -> Self {
        let available_actions = HashMap::<Role, ActionType>::new();

        let mut action_limitations = HashMap::<ActionType, Vec<ActionLimitation>>::new();
        action_limitations.insert(
            ActionType::Shoot, 
            vec![
                ActionLimitation::IsOptional,
                ActionLimitation::TargetsLivingPlayers,
                ActionLimitation::TargetsAmountOfPlayers(1),
            ]);
        action_limitations.insert(
            ActionType::Heal, 
            vec![
                ActionLimitation::IsOptional,
                ActionLimitation::TargetsLivingPlayers,
                ActionLimitation::TargetsAmountOfPlayers(1),
            ]);
        action_limitations.insert(
            ActionType::Investigate, 
            vec![
                ActionLimitation::IsOptional,
                ActionLimitation::TargetsLivingPlayers,
                ActionLimitation::TargetsAmountOfPlayers(1),
            ]);
        action_limitations.insert(
            ActionType::Swap, 
            vec![
                ActionLimitation::IsOptional,
                ActionLimitation::TargetsLivingPlayers,
                ActionLimitation::TargetsAmountOfPlayers(2),
            ]);
        action_limitations.insert(
            ActionType::SelfConceptualize,
            vec![
                ActionLimitation::DoesNotTarget,
            ]);

        let mut action_consequences = HashMap::<ActionType, Vec<ActionConsequence>>::new();
        action_consequences.insert(
            ActionType::Investigate, 
            vec![
                ActionConsequence::ReturnsInformation{
                    default: InformationType::HasGun,
                    on_escort: InformationType::DoesntHaveGun,
                },
            ]);
        action_consequences.insert(
            ActionType::SelfConceptualize,
            vec![
                ActionConsequence::ReturnsInformation{
                    default: InformationType::IsActivated,
                    on_escort: InformationType::IsntActivated,
                },
            ]);
        action_consequences.insert(
            ActionType::Heal,
            vec![
                ActionConsequence::CausesEffectOnRole(Role::Alien, EffectType::Death)
            ]);
        
        Self { 
            available_actions,
            action_limitations,
            action_consequences,
        }
    }
}

#[cfg(test)]
mod test {
    use strum::IntoEnumIterator;

    use crate::{game::action_type::ActionType, roles::role::Role};

    use super::GameRules;

    #[test]
    fn action_types_contain_all_roles() {
        assert_eq!(Role::iter().len(), GameRules::default().get_available_actions().keys().len());
    }

    #[test]
    fn action_limitations_contain_all_action_types() {
        assert_eq!(ActionType::iter().len(), GameRules::default().get_action_limitations().keys().len());
    }

    #[test]
    fn action_consequences_contain_all_action_types() {
        assert_eq!(ActionType::iter().len(), GameRules::default().get_action_consequences().keys().len());
    }
}
