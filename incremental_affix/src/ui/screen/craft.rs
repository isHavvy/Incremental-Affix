//! The crafting screen.

use bevy::picking::hover::Hovered;
use bevy::prelude::*;
use bevy::ui_widgets::{Button, Activate};
use itertools::Itertools;

use crate::incremental::item::craft::{Craft, CraftRequest};
use crate::incremental::item::item_database::ItemDatabase;
use crate::ui::{item::spawn_item_details, tooltip};
use super::Screen;

pub fn crafting_screen() -> impl Scene {
    let craft_base_buttons = Craft::starting_crafts()
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

fn craft_base_button(craft: Craft) -> impl Scene {
    let contents = craft_base_button_text(&craft);

    bsn! {
        Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            border: UiRect::all(Val::Px(2.)),
            min_height: Val::Px(25.0),
            width: Val::Px(200.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            margin: UiRect::all(Val::Px(8.0)),
        }
        BorderColor::all(Color::BLACK)

        template_value(craft.clone())

        Button
        Hovered
        on(handle_craft_button_click)
        on(handle_craft_button_hover)
        on(handle_craft_button_out)

        Children [
            { contents }
        ]
    }
}

fn craft_base_button_text(craft: &Craft) -> impl SceneList + use<> {
    let mut scenes: Vec<Box<dyn Scene>> = Vec::with_capacity(3);

    let base_text = craft.base.to_string();
    let resource_text = craft.resources
    .iter()
    .map(|&(stock, amount)| format!("{} - {}", stock, amount))
    .join("  ");

    scenes.push(Box::new(bsn! {
        Text::new(base_text)
        TextColor::BLACK
    }));

    if craft.resources.len() > 0 {
        scenes.push(Box::new(bsn! {
            Node {
                margin: UiRect::left(px(5))
            }
            Text::new(resource_text)
            TextColor::BLACK
            TextFont { font_size: px(12) }
        }));
    }

    scenes
}

fn handle_craft_button_click(
    activate: On<Activate>,
    mut commands: Commands,

    craft_query: Query<&Craft>,
) {
    let craft = craft_query.get(activate.entity).expect("Craft button must have a base.");

    commands.trigger(CraftRequest {
        craft: craft.clone()
    });
}

fn handle_craft_button_hover(
    event: On<Pointer<Over>>,
    mut commands: Commands,

    db: Res<ItemDatabase>,
    craft_query: Query<&Craft>,
) {
    let craft = craft_query.get(event.entity).expect("This handler can only be on an entity with an item base.");
    let tooltip_content = spawn_item_details(commands.reborrow(), &db.create_basic(craft.base));
    commands.trigger(tooltip::ShowTooltip { content: tooltip_content });
}

fn handle_craft_button_out(
    _event: On<Pointer<Out>>,
    mut commands: Commands,
) {
    commands.trigger(tooltip::HideTooltip);
}