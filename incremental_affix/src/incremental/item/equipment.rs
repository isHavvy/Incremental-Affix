use bevy::prelude::*;

use crate::incremental::item::{affixive_item::AffixiveItem, modifier::ModifierKind};
use crate::incremental::stats::PlayerActionsStats;
use crate::stats_builder::StatsBuilder;

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
    let mut base_wood = StatsBuilder::default();
    let mut base_stone = StatsBuilder::default();

    let item = item_query.get(equipped.item).unwrap();

    for (modifier, value) in item.modifiers() {
        match modifier.kind {
            ModifierKind::WoodBase => { base_wood.set_base(value as f64 / 100.0); },
            ModifierKind::WoodBaseGain => { base_wood.add_offset(value as f64 / 100.0); },
            ModifierKind::WoodMultiplier => { base_wood.add_multiplier(value as f64 / 100.0); }

            ModifierKind::StoneBase => { base_stone.set_base(value as f64 / 100.0); },
            ModifierKind::StoneBaseGain => {base_stone.add_offset(value as f64 / 100.0); },
            ModifierKind::StoneMultiplier => { base_stone.add_multiplier(value as f64 / 100.0); }

            _ => { /* unimplemented stuff */ }
        }
    }

    player_actions_stats.gather_wood.base_gain_per_second = base_wood.calculate();
    player_actions_stats.gather_stone.base_gain_per_second = base_stone.calculate();
}