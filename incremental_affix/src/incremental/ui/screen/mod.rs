use bevy::prelude::*;
use bevy::ui_widgets::{observe, Activate, Button};

use crate::incremental;
use crate::incremental::ui::screen::action::ActionProgressBar;

pub mod action;
pub mod craft;
pub mod inventory;

/// Kinds of screens in the game
#[derive(Debug, Clone, Copy, PartialEq, Eq, Component)]
pub enum Screen {
    Act,
    Population,
    Inventory,
    Craft,
}

impl Screen {
    pub const LIST: &[Self] = &[Self::Act, Self::Population, Self::Inventory, Self::Craft];
}

impl ToString for Screen {
    fn to_string(&self) -> String {
        match self {
            Self::Act => "Act".into(),
            Self::Population => "Population".into(),
            Self::Inventory => "Inventory".into(),
            Self::Craft => "Craft".into(),
        }
    }
}

pub fn spawn_screens_ui(
    mut commands: Commands,
    parent_ui_node: Entity,
    font: Handle<Font>,
    action_progress: Res<incremental::action::ActionProgress>,
    action_progress_bar: ResMut<ActionProgressBar>,
) {
    // Top bar with the screen switching stuff.
    let screen_select_bar = commands.spawn((
        Node {
            height: px(48),
            width: percent(100),
            ..default()
        },
        BackgroundColor(Color::srgb(0.0, 0.8, 0.0).into()),
        ChildOf(parent_ui_node)
    )).id();

    setup_screens_bar(commands.reborrow(), screen_select_bar, font.clone());

    let screen_container = commands.spawn((
        Node {
            flex_grow: 1.,

            padding: UiRect { left: px(10), right: px(0), top: px(10), bottom: px(0) },

            ..default()
        },

        ChildOf(parent_ui_node),
    )).id();

    action::initialize_actions_screen(commands.reborrow(), screen_container, font, &action_progress, action_progress_bar);
    craft::spawn_crafting_screen(commands.reborrow(), screen_container);
    inventory::spawn_inventory_screen(commands.reborrow(), screen_container);
    spawn_population_screen(commands.reborrow(), screen_container);
}

fn spawn_population_screen(mut commands: Commands, screen_container: Entity) {
    commands.spawn((
        Node {
            display: Display::None,

            flex_direction: FlexDirection::Column,
            flex_grow: 1.,

            //height: percent(100.0),
            ..default()
        },
        Screen::Population,
        ChildOf(screen_container),
    ));
}

/// The screens bar is the bar of buttons that allows changing the active screen.
pub fn setup_screens_bar(mut commands: Commands, bar: Entity, font: Handle<Font>) {
    for screen in Screen::LIST.iter().cloned() {
        commands.spawn((
            Node {
                height: px(40),
                width: Val::Auto,
                border: px(2).all(),
                margin: px(5).right(),

                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                
                ..default()
            },
            BorderColor::all(Color::BLACK),

            Button,
            observe(on_activate_button_screen_change),

            ChildOf(bar),
            children![
                Text(screen.to_string()),
                TextColor(Color::BLACK),
                TextFont {
                    font: font.clone(),
                    font_size: 20.0,
                    ..default()
                },
            ],
            screen
        ));
    }
}

pub fn on_activate_button_screen_change(
    activate: On<Activate>,
    screen_query: Query<&Screen>,
    mut screen_node_query: Query<(&mut Node, &Screen), Without<Button>>,
) {
    let next_visible_screen = screen_query.get(activate.entity).expect("Screen button must have a screen entity.");

    for (mut screen_node, screen) in &mut screen_node_query {
        screen_node.display = if screen == next_visible_screen { Display::Block } else { Display::None };
    }
}