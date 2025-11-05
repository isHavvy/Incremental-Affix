use bevy::prelude::*;

use crate::incremental::action::spc::PlayerActionSpc;
use crate::incremental::stock::StockKind;
use crate::incremental::stats::PlayerActionsStats;
use crate::incremental::action::{Action, ActionAffinity, ActionProgress, AffinityTimer, CurrentAction};
use crate::incremental::DotPerSecond;

#[derive(Debug, Event)]
pub struct ChangeAction {
    pub action: Action
}

impl ChangeAction {
    pub fn new(action: Action) -> Self {
        ChangeAction {
            action
        }
    }
}

pub(in super) fn on_change_action(
    event: On<ChangeAction>,

    player_action_bonuses: Res<PlayerActionsStats>,

    mut current_action: ResMut<CurrentAction>,
    mut action_progress: ResMut<ActionProgress>,
    mut action_affinity: ResMut<ActionAffinity>,
    mut affinity_timer: ResMut<AffinityTimer>,

    mut spc: Single<&mut PlayerActionSpc>,
) {
    // Changing to current action. Disregard.
    if Some(event.action) == current_action.0 {
        return;
    }

    reset_player_action(&mut *action_progress, &mut *action_affinity, &mut *affinity_timer, &mut *current_action, &mut spc);

    current_action.set(event.action);
    action_progress.time_seconds = event.action.progress_time();

    match event.action {
        Action::Explore => {},
        Action::GatherWood => {
            let bonuses = &player_action_bonuses.gather_wood;
            spc.push_change(StockKind::Wood, bonuses.base_gain_per_second);
            spc.set_affinity_multiplier(bonuses.affinity.multiplier);
            action_affinity.affinity = bonuses.affinity;
            affinity_timer.unpause();
        },
        Action::GatherStone => {
            let bonuses = &player_action_bonuses.gather_stone;
            spc.push_change(StockKind::Stone, bonuses.base_gain_per_second);
            spc.set_affinity_multiplier(bonuses.affinity.multiplier);
            action_affinity.affinity = bonuses.affinity;
            affinity_timer.unpause();
        },
        Action::Hunt => {
            let bonuses = &player_action_bonuses.hunt;
            spc.push_change(StockKind::Carcass, bonuses.base_gain_per_second);
            spc.set_affinity_multiplier(bonuses.affinity.multiplier);
            action_affinity.affinity = bonuses.affinity;
            affinity_timer.unpause();
        },
        Action::RenderCarcass => {
            spc.push_change(StockKind::Carcass, (-0.2).per_second());
            spc.push_change(StockKind::Meat, 0.2.per_second());
            spc.push_change(StockKind::Bone, (0.2 / 5.0).per_second());
        },
        Action::CookMeat => {
            spc.push_change(StockKind::Meat, (-0.2).per_second());
            spc.push_change(StockKind::Wood, (-0.2).per_second());
            spc.push_change(StockKind::Food, 0.2.per_second());
        }
        Action::CreateFollowers => {},
    }
}

#[derive(Debug, Event)]
pub struct ResetPlayerAction;

pub fn on_reset_player_action(
    _event: On<ResetPlayerAction>,
    mut action_progress: ResMut<ActionProgress>,
    mut action_affinity: ResMut<ActionAffinity>,
    mut affinity_timer: ResMut<AffinityTimer>,
    mut current_action: ResMut<CurrentAction>,
    mut action_spc: Single<&mut PlayerActionSpc>,
) {
    reset_player_action(&mut *action_progress, &mut *action_affinity, &mut *affinity_timer, &mut *current_action, &mut *action_spc);
}

fn reset_player_action(
    action_progress: &mut ActionProgress,
    action_affinity: &mut ActionAffinity,
    affinity_timer: &mut AffinityTimer,
    current_action: &mut CurrentAction,
    action_spc: &mut PlayerActionSpc,
) {
    action_progress.reset();
    action_affinity.reset();
    affinity_timer.reset();
    current_action.reset();
    action_spc.reset();
}