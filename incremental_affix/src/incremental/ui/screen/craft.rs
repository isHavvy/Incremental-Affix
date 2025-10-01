//! The crafting screen.

use bevy::{prelude::*};
use bevy::ui_widgets::{observe, Activate, Button};

use crate::incremental::{item::{Base, ItemDatabase}, StockKind, Stockyard};
use super::Screen;

pub fn spawn_crafting_screen(mut commands: Commands, parent_node: Entity, font: Handle<Font>) {
    let screen = commands.spawn((
        Node {
            display: Display::None,

            flex_direction: FlexDirection::Column,

            ..default()
        },

        Screen::Craft,

        ChildOf(parent_node)
    )).id();

    commands.spawn((
        Node::default(),

        children![(
            Text::new("Craft"),
            TextColor(Color::BLACK),
            TextFont {
                font,
                font_size: 32.,
                ..default()
            }
        )],

        ChildOf(screen)
    ));

    commands.spawn((
        Node {
            width: px(200),

            border: px(1).all(),

            ..default()
        },
        BorderColor::all(Color::BLACK),

        Button,
        Base::MakeshiftTools,
        observe(handle_craft_button_click),

        children![(
            Text::new("Makeshift Tools"),
            TextColor(Color::BLACK),
        )],

        ChildOf(screen),
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