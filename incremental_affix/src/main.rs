mod incremental;
mod stats_builder;
mod ui;

use bevy::prelude::*;

use crate::{incremental::IncrementalStartupSystemSet, ui::UiSetupSystemSet};

fn main() {
    App::new()
    .add_plugins((
        DefaultPlugins,

        incremental::IncrementalPlugin,

        ui::tooltip::TooltipPlugin,

        ui::UiPlugin,
    ))
    .configure_sets(Startup, UiSetupSystemSet.after(IncrementalStartupSystemSet))
    .run();
}