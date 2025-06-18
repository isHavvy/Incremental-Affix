use std::{collections::HashMap, ops::{AddAssign, Index, IndexMut}};

use bevy::prelude::*;

pub mod actions;
pub mod screens;

pub struct IncrementalPlugin;

impl Plugin for IncrementalPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
        .init_resource::<Stockyard>()
        .insert_resource(TickTimer(Timer::from_seconds(1.0, TimerMode::Repeating)))
        .add_systems(Update, (tick_stockyard_system,))
        ;
    }
}

/// A numeric resource controlled by the player.
/// 
/// Because Bevy already takes Resource.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Component)]
pub enum StockKind {
    Godpower,
    Followers,
    Wood,
}

impl StockKind {
    pub const LIST: &'static [Self] = &[
        Self::Godpower,
        Self::Followers,
        Self::Wood
    ];
}

impl ToString for StockKind {
    fn to_string(&self) -> String {
        match self {
            StockKind::Godpower => "Godpower".to_string(),
            StockKind::Followers => "Followers".to_string(),
            StockKind::Wood => "Wood".to_string(),
        }
    }
}

pub struct Stock {
    pub current: i64,
    pub maximum: Option<i64>,
    pub change: i64,
}

impl Stock {
    fn new(current: i64, maximum: Option<i64>) -> Self {
        Self { current, maximum, change: 0, }
    }
}

impl AddAssign<i64> for Stock {
    fn add_assign(&mut self, change: i64) {
        self.current = i64::min(self.maximum.unwrap_or(i64::MAX), self.current + change);
    }
}

#[derive(bevy::prelude::Resource)]
pub struct Stockyard(HashMap<StockKind, Stock>);

impl Default for Stockyard {
    fn default() -> Self {
        let mut resources = HashMap::new();

        resources.insert(StockKind::Godpower, Stock::new(0, None));
        resources.insert(StockKind::Followers, Stock::new(0, Some(10 * 100)));
        resources.insert(StockKind::Wood, Stock::new(0, Some(100 * 100)));

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
    if tick_timer.0.tick(time.delta()).just_finished() {
        for (_key, stock) in &mut stockyard.0 {
            debug!("{}", stock.change);
            stock.current += stock.change;
        }
    }
}

#[derive(Resource)]
struct TickTimer(Timer);