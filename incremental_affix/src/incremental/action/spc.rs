//! Player's action's Stockyard Producer-Consumer

use bevy::prelude::*;

use crate::incremental::{stock::{producer_consumer::StockyardProducerConsumer, StockKind}, PerSecond};

/// The Stockyard Producer/Consumer for the player's action
#[derive(Debug, Component)]
pub(super) struct PlayerActionSpc {
    has_affinity: bool,
    affinity_multiplier: f64,
    base_changes: Vec<(StockKind, PerSecond)>,
}

impl Default for PlayerActionSpc {
    fn default() -> Self {
        Self {
            has_affinity: false,
            affinity_multiplier: 0.0,
            base_changes: vec![]
        }
    }
}

impl PlayerActionSpc {
    pub fn reset(&mut self) {
        self.has_affinity = false;
        self.affinity_multiplier = 0.0;
        self.base_changes.clear();
    }

    pub fn push_change(&mut self, stock_kind: StockKind, change: PerSecond) {
        self.base_changes.push((stock_kind, change));
    }

    pub fn set_affinity_multiplier(&mut self, affinity: f64) {
        self.affinity_multiplier = affinity;
    }

    pub fn enable_affinity(&mut self) {
        self.has_affinity = true;
    }

    pub fn disable_affinity(&mut self) {
        self.has_affinity = false;
    }

    pub fn affinity_multiplier(&self) -> f64 {
        if self.has_affinity {
            self.affinity_multiplier
        } else {
            1.0
        }
    }
}

pub(super) fn initialize_action_spc(
    mut commands: Commands
) {
    commands.spawn((
        PlayerActionSpc::default(),
        StockyardProducerConsumer::default(),
    ));
}

pub(super) fn preconsume(
    mut spc: Single<(&mut StockyardProducerConsumer, &PlayerActionSpc)>,
) {
    let (ref mut spc, action_spc) = *spc;
    spc.consumes.clear();
    spc.consumes.extend(
        action_spc.base_changes
        .iter().copied()
        .filter(|&(_sk, ps)| ps.is_sign_negative())
        .map(|(sk, ps)| (sk, -ps))
        .map(|(sk, ps)| (sk, ps * action_spc.affinity_multiplier()))
    );
}

pub(super) fn postconsume(
    mut spc: Single<(&mut StockyardProducerConsumer, &PlayerActionSpc)>,
) {
    let (ref mut spc, action_spc) = *spc;
    let consumption_fullfilled = spc.consumption_fullfilled();

    spc.produces.clear();
    spc.produces.extend(
        action_spc.base_changes
        .iter().copied()
        .filter(|&(_sk, ps)| ps.is_sign_positive())
        .map(|(sk, ps)| (sk, ps * consumption_fullfilled * action_spc.affinity_multiplier()))
    )
}

#[expect(unused)]
fn changes_per_second(player_action_modifier: &PlayerActionSpc) -> Vec<(StockKind, PerSecond)> {
    if player_action_modifier.has_affinity {
        player_action_modifier.base_changes.iter().copied().map(|(sk, base_change)| (sk, base_change * player_action_modifier.affinity_multiplier)).collect()
    } else {
        player_action_modifier.base_changes.clone()
    }
}