pub mod screen;
pub mod log;
mod stocks;
pub mod tooltip;
pub mod item;

use bevy::prelude::*;

use crate::incremental::action::KnownActions;

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
            screen::population::PopulationScreenPlugin,
        ))

        .add_observer(screen::inventory::on_item_craft)
        ;
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    known_actions: Res<KnownActions>,
) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    commands.spawn(Camera2d);

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
    let(screen1, screen2) = screen::screens_ui(known_actions);
    let mut screen1 = commands.queue_spawn_scene(screen1);
    screen1.insert(ChildOf(right_of_sidebar));
    let mut screen2 = commands.queue_spawn_scene(screen2);
    screen2.insert(ChildOf(right_of_sidebar));
    commands.queue_spawn_scene(bsn!{
        log::log_ui()
        ChildOf(right_of_sidebar)
    });
}