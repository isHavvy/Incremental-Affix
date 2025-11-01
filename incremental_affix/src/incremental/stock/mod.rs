//! Player's numerical resources such as their wood and stone.
//!
//! We call them stocks and not resources because Bevy already uses Resource as a term.
//! Anywhere we call them stocks in the codebase, we refer to them as resources to the player.

use std::fmt::Write as _;
use std::ops::*;

use bevy::prelude::*;
use bevy::platform::collections::HashMap;

use crate::incremental::{IncrementalPlugin, TickTimer};
use crate::incremental::stock::producer_consumer::{consume_modifiers, init_follower_stockyard_producer_consumer, produce_modifiers, update_follower_modifier, StockSystems, StockyardConsumption};

pub mod producer_consumer;

pub struct StockPlugin;

impl Plugin for StockPlugin {
    fn build(&self, app: &mut App) {
        app
        .init_resource::<Stockyard>()
        .init_resource::<StockyardConsumption>()
        .add_systems(Startup, init_follower_stockyard_producer_consumer)
        .add_systems(FixedUpdate, tick_stockyard_system)
        .add_systems(FixedUpdate, consume_modifiers.in_set(StockSystems::Consume))
        .add_systems(FixedUpdate, update_follower_modifier.in_set(StockSystems::PostStockConsume).after(StockSystems::Consume))
        .add_systems(FixedUpdate, produce_modifiers.in_set(StockSystems::Produce).after(StockSystems::PostStockConsume))
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

impl ToString for StockKind {
    fn to_string(&self) -> String {
        match self {
            StockKind::BranchesAndPebbles => "Branches and Pebbles",
            StockKind::Godpower => "Godpower",
            StockKind::Followers => "Followers",
            StockKind::Wood => "Wood",
            StockKind::Stone => "Stone",
            StockKind::Carcass => "Carcasses",
            StockKind::Meat => "Meat",
            StockKind::Bone => "Bones",
            StockKind::Food => "Food",
        }.to_string()
    }
}

/// A numeric resource held by the player.
#[derive(Debug)]
pub struct Stock {
    /// Amount of stock held, in hundredths
    current: f64,
    maximum: Option<f64>,

    player_action_base_modifier: f64,
    player_action_affinity_multiplier: f64,
    player_action_has_affinity: bool,
    player_action_active: bool,

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

            player_action_base_modifier: 0.0,
            player_action_affinity_multiplier: 1.0,
            player_action_has_affinity: false,
            player_action_active: true,

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
        if !self.player_action_active {
            return 0.0;
        }

        let mut modifier = self.player_action_base_modifier / (IncrementalPlugin::TICKS_PER_SECOND as f64);

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
        self.player_action_active = true;

        if self.player_action_base_modifier == 0.0 { return; }

        self.has_changed = true;
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
pub struct Stockyard {
    #[deref]
    stocks: HashMap<StockKind, Stock>,
    stop_player_action_when_empty: Option<Vec<StockKind>>,
}

impl Stockyard {
    pub fn reset_player_action_modifiers(&mut self) {
        self.stop_player_action_when_empty = None;
        for stock in self.values_mut() {
            stock.reset_player_action_modifiers();
        }
    }

    pub fn get_stocks_mut<const N: usize>(&mut self, stocks: [&StockKind; N]) -> [&mut Stock; N] {
        // The unwrap will not panic because every stock kind has an associated value in the stockyard hashmap.
        self.stocks.get_many_mut(stocks).map(Option::unwrap)
    }

    pub fn set_stop_player_action_when_empty(&mut self, stock_kind: Vec<StockKind>) {
        self.stop_player_action_when_empty = Some(stock_kind);
    }
}

impl Default for Stockyard {
    fn default() -> Self {
        let mut stocks = HashMap::new();

        stocks.insert(StockKind::BranchesAndPebbles, Stock::new(0.0, None));
        stocks.insert(StockKind::Godpower, Stock::new(10.0, None));
        stocks.insert(StockKind::Followers, Stock::new(0.0, Some(10.0)));
        stocks.insert(StockKind::Wood, Stock::new(0.0, Some(100.0)));
        stocks.insert(StockKind::Stone, Stock::new(0.0, Some(100.0)));
        stocks.insert(StockKind::Carcass, Stock::new(0.0, Some(10.0)));
        stocks.insert(StockKind::Bone, Stock::new(0.0, Some(100.0)));
        stocks.insert(StockKind::Meat, Stock::new(0.0, Some(100.0)));
        stocks.insert(StockKind::Food, Stock::new(0.0, Some(100.0)));

        Self {
            stocks,
            stop_player_action_when_empty: None,
        }
    }
}

impl Index<StockKind> for Stockyard {
    type Output = Stock;

    fn index(&self, index: StockKind) -> &Self::Output {
        &self.stocks[&index]
    }
}

impl Index<&StockKind> for Stockyard {
    type Output = Stock;

    fn index(&self, index: &StockKind) -> &Self::Output {
        &self.stocks[index]
    }
}

impl IndexMut<StockKind> for Stockyard {
    fn index_mut(&mut self, index: StockKind) -> &mut Self::Output {
        self.stocks.get_mut(&index).expect("The Stockyard stocks map should contain values for each StockKind.")
    }
}

impl IndexMut<&StockKind> for Stockyard {
    fn index_mut(&mut self, index: &StockKind) -> &mut Self::Output {
        self.stocks.get_mut(index).expect("The Stockyard stocks map should contain values for each StockKind.")
    }
}

fn tick_stockyard_system(
    time: Res<Time>,
    mut tick_timer: ResMut<TickTimer>,
    mut stockyard: ResMut<Stockyard>,
) {
    // This whole block is so badly written.
    if let Some(ref stock_kind) = stockyard.stop_player_action_when_empty {
        let player_action_active = stock_kind.iter().copied().all(|stock_kind| stockyard[stock_kind] != 0.0);
        for stock in &mut stockyard.values_mut() {
            stock.player_action_active = player_action_active;
        }
    }

    if tick_timer.tick(time.delta()).just_finished() {
        for stock in &mut stockyard.values_mut() {
            *stock += stock.get_change_per_tick() as _;
        }
    }
}