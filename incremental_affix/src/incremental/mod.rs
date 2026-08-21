use std::{iter::Sum, ops::{Deref, DerefMut, Mul, Neg}};
use std::fmt::Display;

use bevy::prelude::*;

use item::item_database::ItemDatabase;
use stats::PlayerActionsStats;

pub mod action;
pub mod stats;
pub mod item;
pub mod stock;
pub mod affinity;
pub mod job;
pub mod story;
pub mod log;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub struct IncrementalStartupSystemSet;

pub struct IncrementalPlugin;

impl IncrementalPlugin {
    pub const TICKS_PER_SECOND: f32 = 20.0;
}

impl Plugin for IncrementalPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
        .init_resource::<PlayerActionsStats>()
        .init_resource::<ItemDatabase>()
        .insert_resource(TickTimer(Timer::from_seconds(const { 1.0 / Self::TICKS_PER_SECOND }, TimerMode::Repeating)))

        .add_plugins((
            log::LogPlugin,
            story::StoryPlugin,
            action::ActionPlugin,
            stock::StockPlugin,
            item::ItemPlugin,
            job::JobsPlugin,
        ))

        ;
    }
}

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

impl Mul<f64> for PerSecond {
    type Output = PerSecond;

    fn mul(self, rhs: f64) -> Self::Output {
        (self.0 * rhs).per_second()
    }
}

impl Neg for PerSecond {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self * -1.0
    }
}

impl Sum<PerSecond> for PerSecond {
    fn sum<I: Iterator<Item = PerSecond>>(iter: I) -> Self {
        iter.fold(0.per_second(), |lhs, rhs| Self(lhs.0 + rhs.0))
    }
}

/// This trait adds a `.per_second()` method as an alias for `.into::<PerSecond>()`.
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

impl Display for PerSecond {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0 == 0.0 {
            return Ok(());
        } else {
            write!(f, "{:+.2}/s", self.0)
        }
    }
}