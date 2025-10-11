pub mod equipment;
pub mod item_slot;
pub mod affixive_item;
pub mod modifier;
pub mod base;
pub mod item_database;

use bevy::prelude::*;

use crate::incremental::item::item_database::ItemDatabase;

pub struct ItemPlugin;

impl bevy::prelude::Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        app
        .init_resource::<ItemDatabase>()
        .add_observer(equipment::on_equip)
        ;
    }
}

#[derive(Debug, Event)]
pub struct CraftEvent {
    /// Entity that contains the crafted affixive item as a component.
    pub crafted_item: Entity,
}