use bevy::prelude::*;

use crate::incremental::{action::{ChopSpeed, MineSpeed}, item::{affixive_item::AffixiveItem, modifier::ModifierKind}};

#[derive(Debug, EntityEvent)]
pub struct Equipped {
    #[event_target]
    pub(crate) item: Entity
}

pub fn on_equip(
    equipped: On<Equipped>,

    mut can_mine: ResMut<MineSpeed>,
    mut can_chop: ResMut<ChopSpeed>,

    item_query: Query<&AffixiveItem>,
) {
    let item = item_query.get(equipped.item).unwrap();

    can_mine.set(0.);
    can_chop.set(0.);

    for (modifier, _) in item.modifiers() {
        match modifier.kind {
            ModifierKind::CanChopWood => { can_chop.set(1.) },
            ModifierKind::CanMineStone => { can_mine.set(1.); },
            _ => {}
        }
    }
}
