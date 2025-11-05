use bevy::prelude::*;
use bevy::platform::collections::HashMap;

use crate::incremental::stock::{StockKind, Stockyard};
use crate::incremental::{DotPerSecond as _, PerSecond};

// #[TODO(Havvy)]: Make sure these systems happen in this order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub enum StockSystems {
    PreConsume,
    Consume,
    PostConsume,
    Produce,
}

/// An effect that consumes or produces stocks over time.
#[derive(Debug, Component)]
pub struct StockyardProducerConsumer {
    pub consumes: Vec<(StockKind, PerSecond)>,

    /// Percent between 0.0 and 1.0 of how much of the stocks consumed exist.
    /// 
    /// E.g. if there's only 5 of a stock and two modifiers are trying to consume
    /// 10 each, this will be set to 0.25.
    consumption_fullfilled: f64,

    pub produces: Vec<(StockKind, PerSecond)>,
}

impl Default for StockyardProducerConsumer {
    fn default() -> Self {
        Self {
            consumes: vec![],
            consumption_fullfilled: 0.0,
            produces: vec![],
        }
    }
}

impl StockyardProducerConsumer {
    /// The percentage of the consumption that was fulfilled during the consume 
    pub fn consumption_fullfilled(&self) -> f64 {
        self.consumption_fullfilled
    }
}

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
    mut consumption_table: Local<HashMap<StockKind, f64>>,

    mut modifier_query: Query<&mut StockyardProducerConsumer>,
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

    pc_query: Query<&mut StockyardProducerConsumer>,
) {
    for (stock_kind, production) in pc_query.iter().flat_map(|pc| pc.produces.iter()) {
        stockyard[stock_kind] += production.per_tick();
    }
}

/// Marker component for the follower Stockyard Producer Consumer entity.
/// 
/// Only one entity should be tagged with this component,
/// and said entity must have a StockModifier component.
#[derive(Debug, Component)]
pub struct FollowerSpc;

pub fn update_follower_modifier(
    stockyard: Res<Stockyard>,

    mut modifier: Single<&mut StockyardProducerConsumer, With<FollowerSpc>>,
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
        FollowerSpc,
        StockyardProducerConsumer {
            consumes: vec![(StockKind::Food, Default::default())],
            consumption_fullfilled: 1.0,
            produces: vec![(StockKind::Godpower, Default::default())],
        }
    ));
}