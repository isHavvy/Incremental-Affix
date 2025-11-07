//! Player's numerical resources such as their wood and stone.
//!
//! We call them stocks and not resources because Bevy already uses Resource as a term.
//! Anywhere we call them stocks in the codebase, we refer to them as resources to the player.

use std::fmt::Write as _;
use std::ops::*;

use bevy::prelude::*;

use crate::incremental::stock::stockyard::{tick_stockyard_system, Stockyard};
use crate::incremental::{IncrementalPlugin, PerSecond};
use crate::incremental::stock::producer_consumer::{consume_modifiers, init_follower_stockyard_producer_consumer, produce_modifiers, update_follower_modifier, StockSystems};

pub mod producer_consumer;
pub mod stockyard;

pub struct StockPlugin;

impl Plugin for StockPlugin {
    fn build(&self, app: &mut App) {
        app
        .init_resource::<Stockyard>()
        .add_systems(Startup, init_follower_stockyard_producer_consumer)
        .add_systems(FixedUpdate, tick_stockyard_system)
        .add_systems(FixedUpdate, consume_modifiers.in_set(StockSystems::Consume))
        .add_systems(FixedUpdate, update_follower_modifier.in_set(StockSystems::PostConsume).after(StockSystems::Consume))
        .add_systems(FixedUpdate, produce_modifiers.in_set(StockSystems::Produce).after(StockSystems::PostConsume))
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
    Carcass,
    Meat,
    Bone,
    Food,
}

impl StockKind {
    pub const LIST: &'static [Self] = &[
        Self::BranchesAndPebbles,
        Self::Godpower,
        Self::Followers,
        Self::Wood,
        Self::Stone,
        Self::Carcass,
        Self::Bone,
        Self::Meat,
        Self::Food,
    ];
}

impl std::fmt::Display for StockKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            StockKind::BranchesAndPebbles => "Branches and Pebbles",
            StockKind::Godpower => "Godpower",
            StockKind::Followers => "Followers",
            StockKind::Wood => "Wood",
            StockKind::Stone => "Stone",
            StockKind::Carcass => "Carcasses",
            StockKind::Meat => "Meat",
            StockKind::Bone => "Bones",
            StockKind::Food => "Food",
        })
    }
}

/// A numeric resource held by the player.
#[derive(Debug)]
pub struct Stock {
    /// Amount of stock held, in hundredths
    current: f64,
    maximum: Option<f64>,

    /// Whether or not the stock has changed since `has_changed`
    /// 
    /// Both changes to the actual value and changes to the change per tick
    /// will set this to true.
    has_changed: bool,
}

/// Constructors
impl Stock {
    fn new(current: f64, maximum: Option<f64>) -> Self {
        Self {
            current,
            maximum,

            has_changed: true,
        }
    }
    
    /// Try to consume the amount of stock.
    /// If there's not enough stock, it will instead consume all of it.
    /// 
    /// Returned value is the percentage of stock consumed.
    /// `1.0` if the total amount is consumed.
    /// `0.0` if the stock is empty and the amount is non-zero. 
    fn consume_check(&self, amount: f64) -> f64 {
        if amount <= self.current {
            1.0
        } else {
            // This cannot divide by zero since if `amount` is zero,
            // it would be less than or equal to `self.current` and
            // this else block would not be taken.
            self.current / amount
        }
    }
}

impl AddAssign<f64> for Stock {
    fn add_assign(&mut self, change: f64) {
        if change == 0.0 { return; }

        self.has_changed = true;
        self.current = f64::clamp(self.current + change, 0.0, self.maximum.unwrap_or(f64::MAX));
    }
}

impl SubAssign<f64> for Stock {
    fn sub_assign(&mut self, neg_change: f64) {
        if neg_change == 0.0 { return; }

        self.has_changed = true;
        self.current = f64::clamp(self.current - neg_change, 0.0, self.maximum.unwrap_or(f64::MAX));
    }
}

impl PartialEq<f64> for Stock {
    fn eq(&self, value: &f64) -> bool {
        self.current.eq(value)
    }
}

impl PartialOrd<f64> for Stock {
    fn partial_cmp(&self, value: &f64) -> Option<std::cmp::Ordering> {
        self.current.partial_cmp(value)
    }
}

impl PartialEq<u32> for Stock {
    fn eq(&self, value: &u32) -> bool {
        self.current.eq(&(*value as f64))
    }
}

impl PartialOrd<u32> for Stock {
    fn partial_cmp(&self, value: &u32) -> Option<std::cmp::Ordering> {
        self.current.partial_cmp(&(*value as f64))
    }
}

/// Reading stock values to strings.
impl Stock {
    /// Push to a string the amount of stock is held and the maximum.
    pub fn push_str_current_and_maximum(&self, string: &mut String) {
        let _ = write!(string, "{:0>2.2}", self.current);

        if let Some(maximum) = self.maximum {
            let _ = write!(string, "/{}", maximum);
        }
    }

    /// Push to a string the change per second of the stock.
    pub fn push_str_change_per_second(&self, string: &mut String) {
        let change = self.get_change_per_tick() * (IncrementalPlugin::TICKS_PER_SECOND as f64);

        string.push('(');

        if change > 0.0 {
            string.push('+');
        }

        let _ = write!(string, "{:.2})", change);
    }
}

/// Modifying formula values for automatic stock updating per tick.
impl Stock {
    pub fn get_change_per_tick(&self) -> f64 {
        0.0
    }
}

/// Change detection
impl Stock {
    /// Check if the stock has changed since last time calling this function.
    pub fn has_changed(&mut self) -> bool {
        std::mem::replace(&mut self.has_changed, false)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct StockPerSecond {
    pub kind: StockKind,
    pub per_second: PerSecond,
}

impl StockPerSecond {
    pub const fn new(kind: StockKind, per_second: PerSecond) -> Self {
        Self{ kind, per_second, }
    }

    /// Construct a StockPerSecond with a change of 0 per second.
    pub const fn none(kind: StockKind) -> Self {
        Self { kind, per_second: PerSecond(0.0), }
    }

    pub const fn is_sign_negative(&self) -> bool {
        self.per_second.0.is_sign_negative()
    }

    pub const fn is_sign_positive(&self) -> bool {
        self.per_second.0.is_sign_positive()
    }

    pub fn negate(self) -> Self {
        Self {
            kind: self.kind,
            per_second: -self.per_second
        }
    }
}

impl Mul<f64> for StockPerSecond {
    type Output = StockPerSecond;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::Output {
            kind: self.kind,
            per_second: self.per_second * rhs,
        }
    }
}

impl Mul<u32> for StockPerSecond {
    type Output = StockPerSecond;

    fn mul(self, rhs: u32) -> Self::Output {
        Self::Output {
            kind: self.kind,
            per_second: self.per_second * rhs as f64,
        }
    }
}