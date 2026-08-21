pub mod screen;
pub mod log;
mod stocks;
pub mod tooltip;
pub mod item;

use bevy::prelude::*;

use crate::incremental::action::KnownActions;
use crate::ui::stocks::stockyard_ui;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub struct UiSetupSystemSet;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app

        .add_systems(Startup, setup.in_set(UiSetupSystemSet))
        .add_systems(Update, (
            stocks::update_stockyard_sidebar,
        ))

        .add_plugins((
            log::LogUiPlugin,
            screen::action::ActionScreenPlugin,
            screen::population::PopulationScreenPlugin,
            screen::craft::CraftScreenPlugin,
        ))

        .add_observer(screen::inventory::on_item_craft)
        ;
    }
}

fn setup(
    mut commands: Commands,
    known_actions: Res<KnownActions>,
) {
    commands.spawn(Camera2d);

    commands.queue_spawn_scene(bsn! {
        #Root
        Node {
            flex_direction: FlexDirection::Row,
            width: percent(100),
            height: percent(100),
        }
        BackgroundColor(Color::srgb_u8(238, 223, 187))
        Children [
            #Sidebar
            Node {
                flex_direction: FlexDirection::Column,
                height: percent(100),
                border: { px(2).right() }
            }
            BorderColor::all(Color::BLACK)
            Children [ stockyard_ui() ],

            #ScreensAndScreensBarContainer // Yeah, this name sucks
            Node {
                flex_direction: FlexDirection::Column,
                flex_grow: 1.,

                border: { px(2).bottom() }
            }
            Children [
                { screen::screens_ui(known_actions) },
                log::log_ui()
            ]
        ]
    });
}