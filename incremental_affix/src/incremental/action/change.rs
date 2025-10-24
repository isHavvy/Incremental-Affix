use bevy::prelude::*;

use crate::incremental::{action::{Action, ActionAffinity, ActionProgress, AffinityTimer, CurrentAction}, stats::PlayerActionsStats, stock::{StockKind, Stockyard}};

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

    mut stockyard: ResMut<Stockyard>,
    player_action_bonuses: Res<PlayerActionsStats>,

    mut current_action: ResMut<CurrentAction>,
    mut action_progress: ResMut<ActionProgress>,
    mut action_affinity: ResMut<ActionAffinity>,
    mut affinity_timer: ResMut<AffinityTimer>,
) {
    // Changing to current action. Disregard.
    if Some(event.action) == current_action.0 {
        return;
    }

    reset_player_action(&mut *action_progress, &mut *action_affinity, &mut *affinity_timer, &mut *stockyard, &mut *current_action);

    current_action.set(event.action);
    action_progress.time_seconds = event.action.progress_time();

    match event.action {
        Action::Explore => {},
        Action::GatherWood => {
            let stock = &mut stockyard[StockKind::Wood];
            let bonuses = &player_action_bonuses.gather_wood;
            stock.set_player_action_base_modifier(bonuses.base_gain_per_second);
            stock.set_player_action_affinity_multiplier(bonuses.affinity.multiplier);
            action_affinity.affinity = bonuses.affinity;
            affinity_timer.unpause();
        },
        Action::GatherStone => {
            let stock = &mut stockyard[StockKind::Stone];
            let bonuses = &player_action_bonuses.gather_stone;
            stock.set_player_action_base_modifier(bonuses.base_gain_per_second);
            stock.set_player_action_affinity_multiplier(bonuses.affinity.multiplier);
            action_affinity.affinity = bonuses.affinity;
            affinity_timer.unpause();
        },
        Action::Hunt => {
            let stock = &mut stockyard[StockKind::Carcass];
            let bonuses = &player_action_bonuses.hunt;
            stock.set_player_action_base_modifier(bonuses.base_gain_per_second);
            stock.set_player_action_affinity_multiplier(bonuses.affinity.multiplier);
            action_affinity.affinity = bonuses.affinity;
            affinity_timer.unpause();
        },
        Action::RenderCarcass => {
            let [
                stock_carcass,
                stock_meat,
                stock_bones,
            ] = stockyard.get_stocks_mut([&StockKind::Carcass, &StockKind::Meat, &StockKind::Bone]);

            stock_carcass.set_player_action_base_modifier(-0.2);
            stock_meat.set_player_action_base_modifier(0.2);
            stock_bones.set_player_action_base_modifier(0.2 / 5.0);
            stockyard.set_stop_player_action_when_empty(vec![StockKind::Carcass]);
        },
        Action::CookMeat => {
            let [
                stock_meat,
                stock_wood,
                stock_food
            ] = stockyard.get_stocks_mut([&StockKind::Meat, &StockKind::Wood, &StockKind::Food]);

            stock_meat.set_player_action_base_modifier(-0.2);
            stock_wood.set_player_action_base_modifier(-0.2);
            stock_food.set_player_action_base_modifier(0.2);
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
    mut stockyard: ResMut<Stockyard>,
    mut current_action: ResMut<CurrentAction>
) {
    reset_player_action(&mut *action_progress, &mut *action_affinity, &mut *affinity_timer, &mut *stockyard, &mut *current_action);
}

fn reset_player_action(
    action_progress: &mut ActionProgress,
    action_affinity: &mut ActionAffinity,
    affinity_timer: &mut AffinityTimer,
    stockyard: &mut Stockyard,
    current_action: &mut CurrentAction
) {
    action_progress.reset();
    action_affinity.reset();
    affinity_timer.reset();
    stockyard.reset_player_action_modifiers();
    current_action.reset();
}