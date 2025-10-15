use bevy::prelude::*;

use crate::incremental::item::{affixive_item::AffixiveItem, modifier::ModifierKind};
use crate::incremental::stats::PlayerActionsStats;

#[derive(Debug, EntityEvent)]
pub struct Equipped {
    #[event_target]
    pub(crate) item: Entity
}

pub fn on_equip(
    equipped: On<Equipped>,

    mut player_actions_stats: ResMut<PlayerActionsStats>,

    item_query: Query<&AffixiveItem>,
) {
    let item = item_query.get(equipped.item).unwrap();

    player_actions_stats.gather_wood.base_gain_per_second = 0.0;
    player_actions_stats.gather_stone.base_gain_per_second = 0.0;

    for (modifier, value) in item.modifiers() {
        match modifier.kind {
            ModifierKind::WoodBase => { player_actions_stats.gather_wood.base_gain_per_second = value as f64 / 100. },
            ModifierKind::StoneBase => { player_actions_stats.gather_stone.base_gain_per_second = value as f64 / 100. },
            _ => {}
        }
    }
}
