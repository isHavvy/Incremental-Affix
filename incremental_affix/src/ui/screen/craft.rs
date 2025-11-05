//! The crafting screen.

use bevy::picking::hover::Hovered;
use bevy::prelude::*;
use bevy::ui_widgets::{observe, Activate, Button};

use crate::incremental::item::{base::Base, item_database::ItemDatabase, Crafted};
use crate::incremental::stock::{StockKind, stockyard::Stockyard};
use crate::ui::{item::spawn_item_details, log::LogMessage, tooltip};
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

    for base in const { [Base::MakeshiftTools, Base::TestTools, Base::StoneTools, Base::WoodenHunt,] }.into_iter() {
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

fn handle_craft_button_click(
    activate: On<Activate>,
    mut commands: Commands,

    item_db: Res<ItemDatabase>,
    mut stockyard: ResMut<Stockyard>,

    mut messages: MessageWriter<LogMessage>,

    base_query: Query<&Base>,
) {
    let base = base_query.get(activate.entity).expect("Craft button must have a base.");

    match base {
        Base::MakeshiftTools => {
            if stockyard[StockKind::BranchesAndPebbles] == 0.0 {
                messages.write(LogMessage("Unable to craft. Missing branches and pebbles.".into()));
                return;
            }
            stockyard[StockKind::BranchesAndPebbles] -= 1.0;
        },

        Base::TestTools => {},

        Base::StoneTools => {
            if stockyard[StockKind::Stone] < 5.0 || stockyard[StockKind::Wood] < 5.0 {
                messages.write(LogMessage("Unable to craft stone tools. Need 5 stone and 5 wood.".into()));
                return;
            }

            stockyard[StockKind::Stone] -= 5.0;
            stockyard[StockKind::Wood] -= 5.0;
        },

        Base::WoodenHunt => {
            if stockyard[StockKind::Wood] < 5.0 {
                messages.write(LogMessage("Unable to craft wooden hunting weapon. Need 5 wood.".into()));
                return;
            }

            stockyard[StockKind::Wood] -= 5.0;
        }
    }

    let item = item_db.create_basic(*base);

    let item_entity = commands.spawn((
        item,
    )).id();

    commands.trigger(Crafted { crafted_item: item_entity });
}

fn handle_craft_button_hover(
    event: On<Pointer<Over>>,
    mut commands: Commands,

    db: Res<ItemDatabase>,
    base_query: Query<&Base>,
) {
    let base = base_query.get(event.entity).expect("This handler can only be on an entity with an item base.").clone();
    let tooltip_content = spawn_item_details(commands.reborrow(), &db.create_basic(base));
    commands.trigger(tooltip::ShowTooltip { content: tooltip_content });
}

fn handle_craft_button_out(
    _event: On<Pointer<Out>>,
    mut commands: Commands,
) {
    commands.trigger(tooltip::HideTooltip);
}