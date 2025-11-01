use std::ops::{Deref, DerefMut};

use bevy::prelude::*;

use item::item_database::ItemDatabase;
use stats::PlayerActionsStats;

pub mod action;
pub mod stats;
pub mod item;
pub mod stock;
pub mod affinity;

pub struct IncrementalPlugin;

impl IncrementalPlugin {
    pub const TICKS_PER_SECOND: f32 = 20.0;
}

impl Plugin for IncrementalPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
        .init_resource::<ExplorationProgress>()
        .init_resource::<PlayerActionsStats>()
        .init_resource::<ItemDatabase>()
        .insert_resource(TickTimer(Timer::from_seconds(const { 1.0 / Self::TICKS_PER_SECOND }, TimerMode::Repeating)))

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

#[derive(Debug, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
pub struct PerSecond(f64);

pub trait DotPerSecond {
    fn per_second(self) -> PerSecond;
}

impl<T> DotPerSecond for T where T: Into<PerSecond> {
    fn per_second(self) -> PerSecond {
        self.into()
    }
}

impl From<i32> for PerSecond {
    fn from(value: i32) -> Self {
        Self(value as _)
    }
}

impl From<f32> for PerSecond {
    fn from(value: f32) -> Self {
        Self(value as _)
    }
}

impl From<f64> for PerSecond {
    fn from(value: f64) -> Self {
        Self(value)
    }
}

impl PerSecond {
    pub fn per_tick(&self) -> f64 {
        self.0 / IncrementalPlugin::TICKS_PER_SECOND as f64
    }
}