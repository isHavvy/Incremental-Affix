//! The crafting screen.

use bevy::{prelude::*};
use bevy::ui_widgets::{observe, Activate, Button};

use crate::incremental::{item::{Base, ItemDatabase}, StockKind, Stockyard};
use super::Screen;

pub fn spawn_crafting_screen(mut commands: Commands, parent: Entity) {
    let screen = commands.spawn((
        Node {
            flex_direction: FlexDirection::Column,
            height: Val::Percent(100.0),
            width: Val::Percent(100.0),
            display: Display::None,
            ..default()
        },
        BackgroundColor(Color::srgb_u8(238, 223, 187)),
        Screen::Craft,
        ChildOf(parent)
    )).id();

    commands.spawn((
        Node {
            height: px(100),
            width: px(200),
            ..default()
        },

        Button,
        observe(handle_craft_button_click),

        Base::MakeshiftTools,

        ChildOf(screen),
        children![
            Text("Makeshift Tools".to_string()),
        ],
    ));
}

pub fn handle_craft_button_click(
    activate: On<Activate>,
    mut commands: Commands,
    item_db: Res<ItemDatabase>,
    mut stockyard: ResMut<Stockyard>,
    base_query: Query<&Base>,
    inventory_screen: Res<super::inventory::InventoryScreen>,
) {
    let base = base_query.get(activate.entity).expect("Craft button must have a base.");

    match base {
        Base::MakeshiftTools => {
            stockyard[StockKind::BranchesAndPebbles] -= 1;

            let item = item_db.create_basic(*base);
            let name = item_db.item_name(&item).to_owned();

            let item_entity = commands.spawn((
                item_db.create_basic(*base),
            )).id();

            super::inventory::spawn_inventory_item(&mut commands, &*inventory_screen, item_entity, name);
        },
    }
}