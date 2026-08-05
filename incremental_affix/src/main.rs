mod incremental;
mod stats_builder;
mod ui;

use bevy::prelude::*;

fn main() {
    App::new()
    .add_plugins((
        DefaultPlugins,

        incremental::IncrementalPlugin,

        ui::log::LogUiPlugin,
        ui::tooltip::TooltipPlugin,
        ui::UiPlugin,
    ))
    .run();
}