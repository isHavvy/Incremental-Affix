pub mod screen;
pub mod log;
mod stocks;

use bevy::prelude::*;

use crate::incremental::{self, ui::screen::{action::ActionProgressBar, setup_screens_bar, spawn_screens_ui, Screen}};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
        .init_resource::<ActionProgressBar>()
        .add_systems(Startup, setup)
        .add_systems(Update, (
            stocks::update_resources_sidebar,
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
            flex_direction: FlexDirection::Row,

            width: percent(100),
            height: percent(100),

            ..default()
        },
        BackgroundColor(Color::srgb_u8(238, 223, 187)),
    )).id();

    let sidebar = commands.spawn((
        Node {
            flex_direction: FlexDirection::Column,
            flex_grow: 0.,

            width: px(250),
            height: percent(100),

            border: px(2).right(),

            ..default()
        },
        BorderColor::all(Color::BLACK),

        ChildOf(root_node)
    ))
    .id();

    let right_of_sidebar = commands.spawn((
        Node {
            flex_direction: FlexDirection::Column,
            flex_grow: 1.,

            border: px(2).bottom(),

            ..default()
        },
        ChildOf(root_node)
    )).id();

    stocks::spawn_stocks_ui(&mut commands, sidebar, font.clone());
    screen::spawn_screens_ui(commands.reborrow(), right_of_sidebar, font.clone(), action_progress, action_progress_bar);
    log::GameLogPlugin::setup_log_ui(commands, right_of_sidebar);
}