//! Player's numerical resources such as their wood and stone.
//!
//! We call them stocks and not resources because Bevy already uses Resource as a term.
//! Anywhere we call them stocks in the codebase, we refer to them as resources to the player.

use std::fmt::Write as _;
use std::ops::*;

use bevy::prelude::*;
use bevy::platform::collections::HashMap;

use crate::incremental::TickTimer;

pub struct StockPlugin;

impl Plugin for StockPlugin {
    fn build(&self, app: &mut App) {
        app
        .init_resource::<Stockyard>()
        .add_systems(FixedUpdate, tick_stockyard_system)
        ;
    }
}

/// A numeric resource controlled by the player.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Component)]
pub enum StockKind {
    // #[TODO(Havvy)]: Move this out of Resources.
    // It's currently here to show up in the resources sidebar.
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
#[derive(Debug)]
pub struct Stock {
    /// Amount of stock held, in hundredths
    current: i64,
    maximum: Option<i64>,

    player_action_base_modifier: f64,
    player_action_affinity_multiplier: f64,
    player_action_has_affinity: bool,

    /// Whether or not the stock has changed since `has_changed`
    /// 
    /// Both changes to the actual value and changes to the change per tick
    /// will set this to true.
    has_changed: bool,
}

/// Constructors
impl Stock {
    fn new(current: i64, maximum: Option<i64>) -> Self {
        Self {
            current,
            maximum,

            player_action_base_modifier: 0.0,
            player_action_affinity_multiplier: 1.0,
            player_action_has_affinity: false,

            has_changed: true,
        }
    }
}

impl AddAssign<i64> for Stock {
    fn add_assign(&mut self, change: i64) {
        if change == 0 { return; }

        self.has_changed = true;
        self.current = i64::clamp(self.current + change, 0, self.maximum.unwrap_or(i64::MAX));
    }
}

impl SubAssign<i64> for Stock {
    fn sub_assign(&mut self, neg_change: i64) {
        if neg_change == 0 { return; }

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

/// Reading stock values to strings.
impl Stock {
    /// Push to a string the amount of stock is held and the maximum.
    pub fn push_str_current_and_maximum(&self, string: &mut String) {
        write!(string, "{}.{:0>2}", self.current / 100, self.current % 100).expect("Writing to a string should work.");

        if let Some(maximum) = self.maximum {
            write!(string, "/{}", maximum / 100).expect("Writing to a string should work.");
        }
    }

    /// Push to a string the change per second of the stock.
    pub fn push_str_change_per_second(&self, string: &mut String) {
        let change = self.get_change_per_tick();

        string.push('(');

        if change > 0.0 {
            string.push('+');
        } else if change < 0.0 {
            string.push('-');
        }

        let _ = write!(string, "{})", change / 5.0);
    }
}

/// Modifying formula values for automatic stock updating per tick.
impl Stock {
    pub fn get_change_per_tick(&self) -> f64 {
        let mut modifier = self.player_action_base_modifier;

        if self.player_action_has_affinity {
            modifier *= self.player_action_affinity_multiplier;
        }

        modifier
    }

    pub fn set_player_action_base_modifier(&mut self, modifier_per_second: f64) {
        self.player_action_base_modifier = modifier_per_second;
        self.has_changed = true;
    }

    /// Sets the multiplier to the player's action base modifier if the player's action is affine.
    /// 
    /// Multiplier is not modified. You probably don't want to pass a multiplier less than 1.0.
    pub fn set_player_action_affinity_multiplier(&mut self, multiplier: f64) {
        self.player_action_affinity_multiplier = multiplier;
        self.has_changed = true;
    }

    pub fn set_player_action_has_affinity(&mut self, has_affinity: bool) {
        self.player_action_has_affinity = has_affinity;
        self.has_changed = true;
    }

    pub fn reset_player_action_modifiers(&mut self) {
        self.player_action_base_modifier = 0.0;
        self.player_action_affinity_multiplier = 1.0;
        self.player_action_has_affinity = false;
    }
}

/// Change detection
impl Stock {
    /// Check if the stock has changed since last time calling this function.
    pub fn has_changed(&mut self) -> bool {
        std::mem::replace(&mut self.has_changed, false)
    }
}

#[derive(Debug, Resource, Deref, DerefMut)]
pub struct Stockyard(HashMap<StockKind, Stock>);

impl Stockyard {
    pub fn reset_player_action_modifiers(&mut self) {
        for stock in self.values_mut() {
            stock.reset_player_action_modifiers();
        }
    }
}

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
        self.0.get_mut(&index).expect("The Stockyard stock map should contain values for each kind of stock.")
    }
}

fn tick_stockyard_system(
    time: Res<Time>,
    mut tick_timer: ResMut<TickTimer>,
    mut stockyard: ResMut<Stockyard>
) {
    if tick_timer.tick(time.delta()).just_finished() {
        for stock in &mut stockyard.values_mut() {
            *stock += stock.get_change_per_tick() as _;
        }
    }
}