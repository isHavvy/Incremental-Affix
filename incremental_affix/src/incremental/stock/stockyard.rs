use std::ops::{Index, IndexMut};

use bevy::prelude::*;
use bevy::platform::collections::HashMap;

use crate::incremental::{stock::{Stock, StockKind}, TickTimer};

#[derive(Debug, Resource, Deref, DerefMut)]
pub struct Stockyard {
    #[deref]
    stocks: HashMap<StockKind, Stock>,
}

impl Stockyard {
    #[expect(unused)]
    pub fn get_stocks_mut<const N: usize>(&mut self, stocks: [&StockKind; N]) -> [&mut Stock; N] {
        // The unwrap will not panic because every stock kind has an associated value in the stockyard hashmap.
        self.stocks.get_many_mut(stocks).map(Option::unwrap)
    }
}

impl Default for Stockyard {
    fn default() -> Self {
        // This match exists to throw an error when a new stock kind is added.
        // That way we don't forget to add the stock to the stocks hashmap below.
        match StockKind::Godpower {
            StockKind::BranchesAndPebbles => {},
            StockKind::Godpower => {},
            StockKind::Followers => {},
            StockKind::Wood => {},
            StockKind::Stone => {},
            StockKind::Carcass => {},
            StockKind::Meat => {},
            StockKind::Bone => {},
            StockKind::Food => {},
        }

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

pub(super) fn tick_stockyard_system(
    time: Res<Time>,
    mut tick_timer: ResMut<TickTimer>,
    mut stockyard: ResMut<Stockyard>,
) {
    if tick_timer.tick(time.delta()).just_finished() {
        for stock in &mut stockyard.values_mut() {
            *stock += stock.get_change_per_tick() as _;
        }
    }
}