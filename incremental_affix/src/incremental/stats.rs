//! ...

use bevy::prelude::*;

use crate::incremental::action::Action;

/// Stats for all player actions
#[derive(Debug, Resource)]
pub struct PlayerActionsStats {
    pub gather_wood: PlayerActionStats,
    pub gather_stone: PlayerActionStats,
}

impl Default for PlayerActionsStats {
    fn default() -> Self {
        Self {
            gather_wood: Default::default(),
            gather_stone: Default::default(),
        }
    }
}

impl PlayerActionsStats {
    pub fn get_bonuses(&self, action: Action) -> Option<&PlayerActionStats> {
        match action {
            Action::Explore => None,
            Action::GatherWood => Some(&self.gather_wood),
            Action::GatherStone => Some(&self.gather_stone),
            Action::CreateFollowers => None,
        }
    }
}

/// Stats for a specific action
#[derive(Debug)]
pub struct PlayerActionStats {
    pub base_gain_per_second: f64,
}

impl Default for PlayerActionStats {
    fn default() -> Self {
        Self {
            base_gain_per_second: 0.0,
        }
    }
}

impl PlayerActionStats {
    pub fn has_base_gain(&self) -> bool {
        self.base_gain_per_second != 0.0
    }
}