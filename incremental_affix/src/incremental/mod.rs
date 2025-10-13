use std::ops::{Deref, DerefMut};

use bevy::prelude::*;

use crate::incremental::item::item_database::ItemDatabase;

pub mod action;
pub mod item;
pub mod stock;
pub mod critical;

pub struct IncrementalPlugin;

impl Plugin for IncrementalPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
        .init_resource::<ExplorationProgress>()
        .init_resource::<ItemDatabase>()
        .insert_resource(TickTimer(Timer::from_seconds(const { 1.0 / 20.0 }, TimerMode::Repeating)))

        .add_plugins((
            action::ActionPlugin,
            stock::StockPlugin,
            item::ItemPlugin,
        ))

        ;
    }
}

/// For the early game Explore action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Resource)]
pub struct ExplorationProgress(u32);

#[derive(Resource)]
struct TickTimer(Timer);

impl Deref for TickTimer {
    type Target = Timer;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for TickTimer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}