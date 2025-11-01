use bevy::prelude::*;
use bevy::platform::collections::HashMap;

use crate::incremental::stock::{StockKind, Stockyard};
use crate::incremental::{DotPerSecond as _, PerSecond};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
#[expect(unused)]
pub enum StockSystems {
    PreStockConsume,
    Consume,
    PostStockConsume,
    Produce,
}

/// How much an effect modifies the stocks in the stockyard.
/// 
/// Each tick, the `consumes`
#[derive(Debug, Component)]
pub struct StockProducerConsumer {
    pub consumes: Vec<(StockKind, PerSecond)>,

    /// Percent between 0.0 and 1.0 of how much of the stocks consumed exist.
    /// 
    /// E.g. if there's only 5 of a stock and two modifiers are trying to consume
    /// 10 each, this will be set to 0.25.
    consumption_fullfilled: f64,

    pub produces: Vec<(StockKind, PerSecond)>,
}

impl Default for StockProducerConsumer {
    fn default() -> Self {
        Self {
            consumes: vec![],
            consumption_fullfilled: 0.0,
            produces: vec![],
        }
    }
}

#[derive(Debug, Resource, Default, Deref, DerefMut)]
pub struct StockyardConsumption(HashMap<StockKind, f64>);

// This algorithm consumes conservatively. It's possible that a modifier
// consuming multiple kinds could starve another modifier even if
// the modifier is ultimately consuming zero cause another kind it consumes
// is unfilled at a lower rate.
//
// M_Starved: A: 10/tick
// M_Starver: A: 10/tick, B: 10/tick
// A: 10, B: 0
//
// M_Starved will consume 5 and get 50%.
// M_Starver will consume 0 and get 0%.
// A afterwards will have 5.
pub fn consume_modifiers(
    mut stockyard: ResMut<Stockyard>,
    mut consumption_table: ResMut<StockyardConsumption>,

    mut modifier_query: Query<&mut StockProducerConsumer>,
) {
    // Reset the table instead of allocating a new one every tick.
    for value in consumption_table.values_mut() {
        *value = 0.0;
    }

    // Sum into the consumption table the total consumption per stock kind.
    for (stock_kind, consumption) in modifier_query.iter().flat_map(|m| &m.consumes).copied() {
        let entry = consumption_table.entry(stock_kind).or_insert(0.0);
        *entry += consumption.per_tick();
    }

    // Remove the total consumption from the stockyard, replacing it with the percentage actually consumed.
    for (stock_kind, entry) in consumption_table.iter_mut() {
        *entry = stockyard[*stock_kind].consume_check(*entry);
    }

    // Find out what percentage is consumable, then consume it.
    for mut modifier in modifier_query.iter_mut() {
        modifier.consumption_fullfilled = modifier.consumes
            .iter()
            .map(|(stock_kind, _)| stock_kind)
            .fold(1.0, |min_consumption_percentage, stock_kind| f64::min(min_consumption_percentage, consumption_table[stock_kind]));

        // Actually consume what can be consumed.
        for (stock_kind, consumption) in modifier.consumes.iter().copied() {
            stockyard[stock_kind] -= consumption.per_tick() * modifier.consumption_fullfilled;
        }
    }
}

pub fn produce_modifiers (
    mut stockyard: ResMut<Stockyard>,

    pc_query: Query<&mut StockProducerConsumer>,
) {
    for (stock_kind, production) in pc_query.iter().flat_map(|pc| pc.produces.iter()) {
        stockyard[stock_kind] += production.per_tick();
    }
}

/// Marker component for the follower modifier.
/// 
/// Only one entity should be tagged with this component,
/// and said entity must have a StockModifier component.
#[derive(Debug, Component)]
pub struct FollowerModifier;

pub fn update_follower_modifier(
    stockyard: Res<Stockyard>,

    mut modifier: Single<&mut StockProducerConsumer, With<FollowerModifier>>,
) {
    let followers = stockyard[StockKind::Followers].current;

    // First and only must be a (Food, PerSecond)
    modifier.consumes[0].1 = (followers * 0.2).per_second();

    let godpower = if followers < 10.0 {
        followers / 10.0
    } else {
        followers.log10() + 1.0
    };

    modifier.produces[0].1 = godpower.per_second();
}

pub fn init_follower_stockyard_producer_consumer(
    mut commands: Commands,
) {
    commands.spawn((
        FollowerModifier,
        StockProducerConsumer {
            consumes: vec![(StockKind::Food, Default::default())],
            consumption_fullfilled: 1.0,
            produces: vec![(StockKind::Godpower, Default::default())],
        }
    ));
}

// #TODO(Havvy): This should be in action/stock_modifier.rs
pub struct PlayerActionModifier {
    has_affinity: bool,
    affinity_multiplier: f64,
    base_changes: Vec<(StockKind, f64)>,
}

#[expect(unused)]
fn changes_per_second(player_action_modifier: &PlayerActionModifier) -> Vec<(StockKind, f64)> {
    if player_action_modifier.has_affinity {
        player_action_modifier.base_changes.iter().copied().map(|(sk, base_change)| (sk, base_change * player_action_modifier.affinity_multiplier)).collect()
    } else {
        player_action_modifier.base_changes.clone()
    }
}