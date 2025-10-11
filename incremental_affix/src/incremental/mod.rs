//! We call them stocks and not resources because Bevy already uses Resource as a term.
//! 
//! Anywhere we call them stocks in the codebase, we refer to them as resources to the player.

use std::{collections::HashMap, fmt::Write, ops::{AddAssign, Deref, DerefMut, Index, IndexMut, SubAssign}};

use bevy::prelude::*;

use crate::incremental::item::item_database::ItemDatabase;

pub mod action;
pub mod item;
pub mod ui;

pub struct IncrementalPlugin;

impl Plugin for IncrementalPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
        .init_resource::<ExplorationProgress>()
        .init_resource::<Stockyard>()
        .init_resource::<ItemDatabase>()
        .init_resource::<item::equipment::Slots>()
        .insert_resource(TickTimer(Timer::from_seconds(1.0, TimerMode::Repeating)))
        .add_systems(FixedUpdate, (tick_stockyard_system,))
        ;
    }
}

/// For the early game Explore action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Resource)]
pub struct ExplorationProgress(u32);

/// A numeric resource controlled by the player.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Component)]
pub enum StockKind {
    BranchesAndPebbles,
    Godpower,
    Followers,
    Wood,
    Stone,
}

impl StockKind {
    pub const LIST: &'static [Self] = &[
        Self::BranchesAndPebbles,
        Self::Godpower,
        Self::Followers,
        Self::Wood,
        Self::Stone,
    ];
}

impl ToString for StockKind {
    fn to_string(&self) -> String {
        match self {
            StockKind::BranchesAndPebbles => "Branches and Pebbles",
            StockKind::Godpower => "Godpower",
            StockKind::Followers => "Followers",
            StockKind::Wood => "Wood",
            StockKind::Stone => "Stone",
        }.to_string()
    }
}

/// A numeric resource held by the player.
pub struct Stock {
    current: i64,
    maximum: Option<i64>,
    pub change: i64,

    /// Whether or not the stock has changed since UI has last looked at it.
    has_changed: bool,
}

impl Stock {
    fn new(current: i64, maximum: Option<i64>) -> Self {
        Self { current, maximum, change: 0, has_changed: true }
    }
}

impl AddAssign<i64> for Stock {
    fn add_assign(&mut self, change: i64) {
        self.has_changed = true;
        self.current = i64::clamp(self.current + change, 0, self.maximum.unwrap_or(i64::MAX));
    }
}

impl SubAssign<i64> for Stock {
    fn sub_assign(&mut self, neg_change: i64) {
        self.has_changed = true;
        self.current = i64::clamp(self.current - neg_change, 0, self.maximum.unwrap_or(i64::MAX));
    }
}

impl PartialEq<i32> for Stock {
    fn eq(&self, value: &i32) -> bool {
        self.current.eq(&(*value as i64))
    }
}

impl PartialOrd<i32> for Stock {
    fn partial_cmp(&self, value: &i32) -> Option<std::cmp::Ordering> {
        self.current.partial_cmp(&(*value as i64))
    }
}

impl Stock {
    /// Push to a string the amount of stock is held and the maximum.
    fn push_str(&self, string: &mut String) {
        write!(string, "{}.{:0>2}", self.current / 100, self.current % 100).expect("Writing to a string should work.");

        if let Some(maximum) = self.maximum {
            write!(string, "/ {}", maximum / 100).expect("Writing to a string should work.");
        }
    }

    /// Check if the stock has changed since last time calling this function.
    fn has_changed(&mut self) -> bool {
        std::mem::replace(&mut self.has_changed, false)
    }
}

#[derive(bevy::prelude::Resource)]
pub struct Stockyard(HashMap<StockKind, Stock>);

impl Default for Stockyard {
    fn default() -> Self {
        let mut resources = HashMap::new();

        resources.insert(StockKind::BranchesAndPebbles, Stock::new(0, None));
        resources.insert(StockKind::Godpower, Stock::new(0, None));
        resources.insert(StockKind::Followers, Stock::new(0, Some(10 * 100)));
        resources.insert(StockKind::Wood, Stock::new(0, Some(100 * 100)));
        resources.insert(StockKind::Stone, Stock::new(0, Some(100 * 100)));

        Self(resources)
    }
}

impl Index<StockKind> for Stockyard {
    type Output = Stock;

    fn index(&self, index: StockKind) -> &Self::Output {
        &self.0[&index]
    }
}

impl IndexMut<StockKind> for Stockyard {
    fn index_mut(&mut self, index: StockKind) -> &mut Self::Output {
        self.0.get_mut(&index).expect("All keys for a Resources map must map to a value.")
    }
}

fn tick_stockyard_system(time: Res<Time>, mut tick_timer: ResMut<TickTimer>, mut stockyard: ResMut<Stockyard>) {
    if tick_timer.tick(time.delta()).just_finished() {
        for (_key, stock) in &mut stockyard.0 {
            debug!("{}", stock.change);
            stock.current += stock.change;
        }
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