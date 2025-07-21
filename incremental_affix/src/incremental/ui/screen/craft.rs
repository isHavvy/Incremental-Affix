//! The crafting screen.

use bevy::{prelude::*};

use crate::incremental::{item::{Base, ItemDatabase}, StockKind, Stockyard};
use super::Screen;

#[derive(Debug, Component)]
struct CorrespondingItem(Entity);

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
        ChildOf(screen),
        Node {
            height: Val::Px(100.0),
            width: Val::Px(200.0),
            ..default()
        },
        Button,
        children![
            Text("Makeshift Tools".to_string()),
        ],
        Base::MakeshiftTools,
    ));
}

pub fn handle_craft_button_click(
    mut commands: Commands,
    item_db: Res<ItemDatabase>,
    mut stockyard: ResMut<Stockyard>,
    button_query: Query<(&Base, &Interaction), (Changed<Interaction>, With<Button>,)>,
    inventory_screen: Res<super::inventory::InventoryScreen>,
) {
    for (base, interaction) in button_query.iter() {
        if *interaction != Interaction::Pressed {
            continue;
        }

        match base {
            Base::MakeshiftTools => {
                stockyard[StockKind::BranchesAndPebbles] -= 1;

                let item = item_db.create_basic(*base);
                let name = item_db.item_name(&item).to_owned();

                let item_entity = commands.spawn((
                    item_db.create_basic(*base),
                )).id();

                commands.spawn((
                    Text(name),
                    CorrespondingItem(item_entity),
                    ChildOf(inventory_screen.get()),
                ));
            },
        }
    }
}