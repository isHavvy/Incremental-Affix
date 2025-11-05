use std::time::Duration;

use bevy::prelude::*;

use crate::incremental::affinity::Affinity;
use crate::incremental::item::item_slot::ItemSlot;
use crate::incremental::item::{affixive_item::AffixiveItem, modifier::ModifierKind};
use crate::incremental::stats::PlayerActionsStats;
use crate::incremental::DotPerSecond;
use crate::stats_builder::StatsBuilder;

#[derive(Debug, EntityEvent)]
pub struct Equipped {
    #[event_target]
    pub(crate) item: Entity
}

pub fn on_equip(
    _equipped: On<Equipped>,

    mut player_actions_stats: ResMut<PlayerActionsStats>,

    item_query: Query<&AffixiveItem>,
    item_slot_query: Query<&ItemSlot>,
) {
    let mut base_wood = StatsBuilder::default();
    let mut wood_affinity_chance = StatsBuilder::default();
    let mut wood_affinity_multiplier = StatsBuilder::default();
    let mut wood_affinity_time = StatsBuilder::default();

    let mut base_stone = StatsBuilder::default();
    let mut stone_affinity_chance = StatsBuilder::default();
    let mut stone_affinity_multiplier = StatsBuilder::default();
    let mut stone_affinity_time = StatsBuilder::default();

    let mut base_hunt = StatsBuilder::default();

    // Base values.
    wood_affinity_chance.add_offset(0.5);
    wood_affinity_multiplier.add_offset(2.0);
    wood_affinity_time.add_offset(1.0);

    stone_affinity_chance.add_offset(0.5);
    stone_affinity_multiplier.add_offset(2.0);
    stone_affinity_time.add_offset(1.0);

    for item_slot in item_slot_query.iter() {
        let Some(item) = item_slot.item else { continue; };
        let item = item_query.get(item).unwrap();

        for (modifier, value) in item.modifiers() {
            match modifier.kind {
                ModifierKind::WoodBase => { base_wood.set_base(value as f64 / 100.0); },
                ModifierKind::WoodBaseGain => { base_wood.add_offset(value as f64 / 100.0); },
                ModifierKind::WoodMultiplier => { base_wood.add_multiplier_percent(value); }
                ModifierKind::WoodAffinityChanceMultiplier => { wood_affinity_chance.add_multiplier_percent(value); },
                ModifierKind::WoodAffinityMultiplier => { wood_affinity_multiplier.add_multiplier_percent(value); },
                ModifierKind::WoodAffinityTimeMultiplier => { wood_affinity_time.add_multiplier_percent(value); },

                ModifierKind::StoneBase => { base_stone.set_base(value as f64 / 100.0); },
                ModifierKind::StoneBaseGain => {base_stone.add_offset(value as f64 / 100.0); },
                ModifierKind::StoneMultiplier => { base_stone.add_multiplier_percent(value); }
                ModifierKind::StoneAffinityChanceMultiplier => { stone_affinity_chance.add_multiplier_percent(value); },
                ModifierKind::StoneAffinityMultiplier => { stone_affinity_multiplier.add_multiplier_percent(value); },
                ModifierKind::StoneAffinityTimeMultiplier => { stone_affinity_time.add_multiplier_percent(value); },

                ModifierKind::ToolMultiplier => {
                    base_wood.add_multiplier_percent(value);
                    base_stone.add_multiplier_percent(value);
                },
                ModifierKind::ToolAffinityChanceMultiplier => {
                    wood_affinity_chance.add_multiplier_percent(value);
                    stone_affinity_chance.add_multiplier_percent(value);
                },
                ModifierKind::ToolAffinityMultiplier => {
                    wood_affinity_multiplier.add_multiplier_percent(value);
                    stone_affinity_multiplier.add_multiplier_percent(value);
                },
                ModifierKind::ToolAffinityTimeMultiplier => {
                    wood_affinity_time.add_multiplier_percent(value);
                    stone_affinity_time.add_multiplier_percent(value);
                },

                ModifierKind::HuntBase => {
                    base_hunt.set_base(value as f64 / 100.0);
                }
            }
        }
    }

    player_actions_stats.gather_wood.base_gain_per_second = base_wood.calculate().per_second();
    player_actions_stats.gather_wood.affinity = Affinity {
        chance: wood_affinity_chance.calculate(),
        multiplier: wood_affinity_multiplier.calculate(),
        time: Duration::from_secs_f64(wood_affinity_time.calculate()),
    };

    player_actions_stats.gather_stone.base_gain_per_second = base_stone.calculate().per_second();
    player_actions_stats.gather_stone.affinity = Affinity {
        chance: stone_affinity_chance.calculate(),
        multiplier: stone_affinity_multiplier.calculate(),
        time: Duration::from_secs_f64(stone_affinity_time.calculate()),
    };

    player_actions_stats.hunt.base_gain_per_second = base_hunt.calculate().per_second();
}