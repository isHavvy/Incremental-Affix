//! ...

use bevy::prelude::*;

use crate::incremental::{action::Action, affinity::Affinity};

/// Stats for all player actions
#[derive(Debug, Resource, Default)]
pub struct PlayerActionsStats {
    pub gather_wood: PlayerActionStats,
    pub gather_stone: PlayerActionStats,
    pub hunt: PlayerActionStats,
}

impl PlayerActionsStats {
    pub fn get_bonuses(&self, action: Action) -> Option<&PlayerActionStats> {
        match action {
            Action::Explore => None,
            Action::GatherWood => Some(&self.gather_wood),
            Action::GatherStone => Some(&self.gather_stone),
            Action::Hunt => Some(&self.hunt),
            Action::RenderCarcass => None,
            Action::CookMeat => None,
            Action::CreateFollowers => None,
        }
    }
}

/// Stats for a specific action
#[derive(Debug, Default)]
pub struct PlayerActionStats {
    pub base_gain_per_second: f64,
    pub affinity: Affinity,
}

impl PlayerActionStats {
    pub fn has_base_gain(&self) -> bool {
        self.base_gain_per_second != 0.0
    }
}