//! The crafting screen.

use bevy::picking::hover::Hovered;
use bevy::prelude::*;
use bevy::ui_widgets::{Button, Activate};

use crate::incremental::item::{base::Base, item_database::ItemDatabase, Crafted};
use crate::incremental::stock::{StockKind, stockyard::Stockyard};
use crate::ui::{item::spawn_item_details, log::LogMessage, tooltip};
use super::Screen;

pub fn crafting_screen() -> impl Scene {
    let craft_base_buttons = const {
        [Base::MakeshiftTools, Base::TestTools, Base::StoneTools, Base::WoodenHunt,]
    }.into_iter()
    .map(craft_base_button)
    .collect::<Vec<_>>();

    bsn! {
        Node {
            display: Display::None,

            flex_direction: FlexDirection::Column,
        }

        Screen::Craft

        Children [
            Node
            Children [
                Text::new("Craft")
                TextColor::BLACK
                TextFont { font_size: px(32.0) }
            ],

            { craft_base_buttons }
        ]
    }
}

fn craft_base_button(base: Base) -> impl Scene {
    bsn! {
        Node {
            display: Display::Flex,
            border: UiRect::all(Val::Px(2.)),
            height: Val::Px(25.0),
            width: Val::Px(200.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            margin: UiRect::all(Val::Px(8.0)),
        }
        BorderColor::all(Color::BLACK)

        template_value(base)

        Button
        Hovered
        on(handle_craft_button_click)
        on(handle_craft_button_hover)
        on(handle_craft_button_out)

        Children [
            Text::new(base.to_string())
            TextColor::BLACK
        ]
    }
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
    let base = *base_query.get(event.entity).expect("This handler can only be on an entity with an item base.");
    let tooltip_content = spawn_item_details(commands.reborrow(), &db.create_basic(base));
    commands.trigger(tooltip::ShowTooltip { content: tooltip_content });
}

fn handle_craft_button_out(
    _event: On<Pointer<Out>>,
    mut commands: Commands,
) {
    commands.trigger(tooltip::HideTooltip);
}