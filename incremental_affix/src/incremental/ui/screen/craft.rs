//! The crafting screen.

use bevy::picking::hover::Hovered;
use bevy::{prelude::*};
use bevy::ui_widgets::{observe, Activate, Button};

use crate::incremental::item::bases::Base;
use crate::incremental::ui::item::spawn_item_details;
use crate::incremental::ui::screen::inventory::spawn_inventory_item;
use crate::incremental::ui::tooltip;
use crate::incremental::{item::ItemDatabase, StockKind, Stockyard};
use super::Screen;

pub fn spawn_crafting_screen(mut commands: Commands, parent_node: Entity) {
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
                font_size: 32.,
                ..default()
            }
        )],

        ChildOf(screen)
    ));

    for base in const { [Base::MakeshiftTools, Base::SecondaryTools] }.into_iter() {
        commands.spawn((
            Node {
                width: px(200),

                border: px(1).all(),
                margin: px(5).bottom(),

                ..default()
            },
            BorderColor::all(Color::BLACK),

            Button,
            Hovered::default(),
            base,
            observe(handle_craft_button_click),
            observe(handle_craft_button_hover),
            observe(handle_craft_button_out),

            children![(
                Text::new(base.to_string()),
                TextColor(Color::BLACK),

                
            )],

            ChildOf(screen),
        ));
    };
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
            let name = item_db.item_name(&item).to_string();

            let item_entity = commands.spawn((
                item,
            )).id();

            spawn_inventory_item(commands.reborrow(), &*inventory_screen, item_entity, name);
        },

        Base::SecondaryTools => {
            let item = item_db.create_basic(*base);
            let name = item_db.item_name(&item).to_string();

            let item_entity = commands.spawn((
                item,
            )).id();

            spawn_inventory_item(commands.reborrow(), &*inventory_screen, item_entity, name);
        }
    }
}

fn handle_craft_button_hover(
    event: On<Pointer<Over>>,
    mut commands: Commands,

    db: Res<ItemDatabase>,
    base_query: Query<&Base>,
) {
    let base = base_query.get(event.entity).expect("This handler can only be on an entity with an item base.").clone();
    let tooltip_content = spawn_item_details(commands.reborrow(), &db.create_basic(base), &db);
    commands.trigger(tooltip::ShowTooltip { content: tooltip_content });
}

fn handle_craft_button_out(
    _event: On<Pointer<Out>>,
    mut commands: Commands,
) {
    commands.trigger(tooltip::HideTooltip);
}