pub mod screen;
pub mod log;
mod stocks;
pub mod tooltip;
pub mod item;

use bevy::prelude::*;

use crate::incremental::{self, action::KnownActions, ui::screen::action::ActionProgressBar};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app

        .add_systems(Startup, setup)
        .add_systems(Update, (
            stocks::update_resources_sidebar,
        ))

        .add_plugins((
            screen::action::ActionScreenPlugin,
        ))

        .add_observer(screen::inventory::on_item_craft)
        ;
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    known_actions: Res<KnownActions>,
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
    screen::spawn_screens_ui(commands.reborrow(), right_of_sidebar, font.clone(), action_progress, known_actions, action_progress_bar);
    log::GameLogPlugin::setup_log_ui(commands, right_of_sidebar);
}