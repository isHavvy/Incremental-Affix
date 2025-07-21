pub mod screen;
pub mod log;
mod resouces;

use bevy::prelude::*;

use crate::incremental::{self, ui::screen::{action::ActionProgressBar, Screen}};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
        .init_resource::<ActionProgressBar>()
        .add_systems(Startup, setup)
        .add_systems(Update, (
            screen::action::handle_action_click,
            screen::craft::handle_craft_button_click,
            screen::handle_screen_click,
            resouces::update_resources_sidebar,
            screen::action::update_action_progress_bar
        ))
        ;
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    action_progress: Res<incremental::action::ActionProgress>,
    action_progress_bar: ResMut<ActionProgressBar>,
) {
    let font = asset_server.load::<Font>("fonts/FiraSans-Bold.ttf");

    commands.spawn(Camera2d::default());

    let root_node = commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            //justify_content: JustifyContent::Center,
            ..default()
        },
    ))
    .id();

    let sidebar = commands.spawn((
        Node {
            width: Val::Percent(20.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            border: UiRect::right(Val::Px(2.0)),
            ..default()
        },
        BackgroundColor(Color::WHITE.into()),
        BorderColor(Color::BLACK),
        ChildOf(root_node)
    ))
    .id();

    resouces::setup_resources_sidebar(&mut commands, sidebar, font.clone());

    let right_of_sidebar =     commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            border: UiRect::bottom(Val::Px(2.0)),
            ..default()
        },
        ChildOf(root_node)
    )).id();

    // Top bar with the screen switching stuff.
    let screen_select_bar = commands.spawn((
        Node {
            height: Val::Px(48.0),
            width: Val::Percent(100.0),
            ..default()
        },
        BackgroundColor(Color::srgb(0.0, 0.8, 0.0).into()),
        ChildOf(right_of_sidebar)
    )).id();

    setup_screens_bar(commands.reborrow(), screen_select_bar, font.clone());

    let screen_container = commands.spawn((
        Node {
            height: Val::Percent(100.0),
            width: Val::Percent(100.0),
            ..default()
        },
        ChildOf(right_of_sidebar),
    )).id();

    let action_screen = commands.reborrow().spawn((
        Node {
            flex_direction: FlexDirection::Column,
            height: Val::Percent(100.0),
            ..default()
        },
        BackgroundColor(Color::WHITE.into()),
        Screen::Act,
        ChildOf(screen_container)
    )).id();

    screen::action::initialize_actions(commands.reborrow(), action_screen, font, &action_progress, action_progress_bar);

    screen::craft::spawn_crafting_screen(commands.reborrow(), screen_container);

    screen::inventory::spawn_inventory_screen(commands.reborrow(), screen_container);

    commands.spawn((
        Node {
            flex_direction: FlexDirection::Column,
            height: Val::Percent(100.0),
            display: Display::None,
            ..default()
        },
        BackgroundColor(Color::WHITE),
        Screen::Population,
        ChildOf(screen_container)
    ));

    log::GameLogPlugin::make_log_ui(commands, right_of_sidebar);
}

fn setup_screens_bar(mut commands: Commands, bar: Entity, font: Handle<Font>) {
    for screen in Screen::LIST.iter().cloned() {
        commands.spawn((
            Button,
            Node {
                border: UiRect::all(Val::Px(2.)),
                height: Val::Px(40.0),
                width: Val::Auto,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect::right(Val::Px(5.0)),
                ..default()
            },
            BorderColor(Color::BLACK),
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