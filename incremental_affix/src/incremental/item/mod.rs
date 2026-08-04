pub mod equipment;
pub mod item_slot;
pub mod affixive_item;
pub mod modifier;
pub mod base;
pub mod item_database;
pub mod craft;

use bevy::prelude::*;

use crate::incremental::item::item_database::ItemDatabase;

pub struct ItemPlugin;

impl bevy::prelude::Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        app
        .init_resource::<ItemDatabase>()

        .add_plugins(craft::ItemCraftPlugin)

        .add_observer(equipment::on_equip)
        ;
    }
}