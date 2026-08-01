mod incremental;
mod stats_builder;
mod ui;

//use bevy::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin, FrameTimeGraphConfig};
use bevy::prelude::*;

fn main() {
    App::new()
    .add_plugins((
        DefaultPlugins,

        incremental::IncrementalPlugin,

        ui::log::GameLogPlugin,
        ui::tooltip::TooltipPlugin,
        ui::UiPlugin,

        // FpsOverlayPlugin {
        //     config: FpsOverlayConfig {
        //         enabled: false,
        //         text_color: Color::BLACK,
        //         frame_time_graph_config: FrameTimeGraphConfig {
        //             enabled: false,
        //             ..default()
        //         },
        //         ..default()
        //     }
        // },
    ))
    .run();
}