use bevy::prelude::*;

use crate::incremental::{action::{CanChop, CanMine}, item::{affixive_item::AffixiveItem, modifier::ModifierKind}};

#[derive(Debug, EntityEvent)]
pub struct Equipped {
    #[event_target]
    pub(crate) item: Entity
}

pub fn on_equip(
    equipped: On<Equipped>,

    mut can_mine: ResMut<CanMine>,
    mut can_chop: ResMut<CanChop>,

    item_query: Query<&AffixiveItem>,
) {
    let item = item_query.get(equipped.item).unwrap();

    can_mine.0 = false;
    can_chop.0 = false;

    for (modifier, _) in item.modifiers() {
        match modifier.kind {
            ModifierKind::CanChopWood => { can_chop.0 = true; },
            ModifierKind::CanMineStone => { can_mine.0 = true; },
            _ => {}
        }
    }
}
