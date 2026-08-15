//! The crafting screen.

use bevy::picking::hover::Hovered;
use bevy::prelude::*;
use bevy::ui_widgets::{Button, Activate};
use itertools::Itertools;

use crate::incremental::item::craft::{Recipe, CraftRequest};
use crate::incremental::item::item_database::ItemDatabase;
use crate::ui::screen::screen_title;
use crate::ui::{item::spawn_item_details, tooltip};
use super::Screen;

pub struct CraftScreenPlugin;

impl Plugin for CraftScreenPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Update, on_new_recipe)
        ;
    }
}

/// Marker component for the [Node] that contains the recipe craft buttons
#[derive(Debug, Clone, Copy, Default, Component)]
struct CraftList;

#[derive(Debug, Clone, Component, FromTemplate)]
#[relationship(relationship_target = CorrespondingCraftButton)]
pub struct CraftButtonOf(pub Entity);

#[derive(Debug, Clone, Component)]
#[relationship_target(relationship = CraftButtonOf)]
pub struct CorrespondingCraftButton(Entity);

pub fn crafting_screen() -> impl Scene {
    bsn! {
        Node {
            display: Display::None,

            flex_direction: FlexDirection::Column,
        }

        Screen::Craft

        Children [
            Node
            Children [
                screen_title("Craft")
            ],

            // ---

            Node
            CraftList
            Children []
        ]
    }
}

fn craft_base_button(recipe_entity: Entity, recipe: &Recipe) -> impl Scene + use<> {
    let contents = craft_base_button_text(&recipe);

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

        Button
        Hovered
        on(handle_craft_button_click)
        on(handle_craft_button_hover)
        on(handle_craft_button_out)

        CraftButtonOf(recipe_entity)
        Children [
            { contents }
        ]
    }
}

fn craft_base_button_text(recipe: &Recipe) -> impl SceneList + use<> {
    let mut scenes: Vec<Box<dyn Scene>> = Vec::with_capacity(3);

    let base_text = recipe.base.to_string();
    let resource_text = recipe.resources
    .iter()
    .map(|&(stock, amount)| format!("{} - {}", stock, amount))
    .join("  ");

    scenes.push(Box::new(bsn! {
        Text::new(base_text)
        TextColor::BLACK
    }));

    if recipe.resources.len() > 0 {
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
    event: On<Activate>,
    mut commands: Commands,

    craft_button_of_query: Query<&CraftButtonOf>,
) {
    let recipe = craft_button_of_query.get(event.entity).expect("Craft button must have CraftButtonOf component.").0;

    commands.trigger(CraftRequest { recipe });
}

fn handle_craft_button_hover(
    event: On<Pointer<Over>>,
    mut commands: Commands,

    db: Res<ItemDatabase>,

    craft_button_of_query: Query<&CraftButtonOf>,
    recipe_query: Query<&Recipe>,
) {
    let recipe = craft_button_of_query.get(event.entity).expect("Craft button must have CraftButtonOf component.").0;
    let recipe = recipe_query.get(recipe).expect("Entity of CraftButtonOf must have a Recipe component.");

    let tooltip_content = spawn_item_details(commands.reborrow(), &db.create_basic(recipe.base));
    commands.trigger(tooltip::ShowTooltip { content: tooltip_content });
}

fn handle_craft_button_out(
    _event: On<Pointer<Out>>,
    mut commands: Commands,
) {
    commands.trigger(tooltip::HideTooltip);
}

fn on_new_recipe(
    mut commands: Commands,

    recipe_query: Query<(Entity, &Recipe), Added<Recipe>>,
    craft_list: Single<Entity, With<CraftList>>,
) {
    for (entity, recipe) in recipe_query.iter() {
        eprintln!("Found a recipe");
        commands.spawn_scene(bsn! {
            craft_base_button(entity, recipe)
            ChildOf({ *craft_list })
        });
    }
}