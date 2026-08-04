//! Crafting information

use bevy::prelude::*;
use smallvec::{SmallVec, smallvec};

use crate::{incremental::{item::item_database::ItemDatabase, stock::{StockKind, stockyard::Stockyard}}, ui::log::LogMessage};

use super::base::Base;

pub struct ItemCraftPlugin;

impl Plugin for ItemCraftPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_observer(on_craft_request)
        ;
    }
}

#[derive(Debug, Clone, Default, Component)]
pub struct Craft {
    // Item base that will be crafted
    pub base: Base,

    // Cost in stock resources to craft this item
    pub resources: SmallVec<[(StockKind, f64); 2]>,
}

impl Craft {
    pub fn starting_crafts() -> impl Iterator<Item = Self> {
        [
            Craft {
                base: Base::MakeshiftTools,
                resources: smallvec![(StockKind::BranchesAndPebbles, 1.0)]
            },

            Craft {
                base: Base::TestTools,
                resources: smallvec![]
            },
        ].into_iter()
    }
}

#[derive(Debug, Event)]
pub struct CraftRequest {
    pub craft: Craft,
}

/// Event for when an item has been crafted.
#[derive(Debug, Event)]
pub struct Crafted {
    /// Entity that contains the crafted [AffixiveItem] as a Component.
    pub crafted_item: Entity,
}

fn on_craft_request(
    event: On<CraftRequest>,
    mut commands: Commands,
    
    item_db: Res<ItemDatabase>,
    mut stockyard: ResMut<Stockyard>,

    mut messages: MessageWriter<LogMessage>,
) {
    // This has_sufficient_stock code will hopefully be extraneous since the crafting UI
    // should not allow the buttons for crafting these to be pressed when there's not enough
    // stock in the stockyard.

    let has_sufficient_stock = event.craft.resources.iter().all(|&(stock_kind, amount)| stockyard[stock_kind] >= amount);
    
    if !has_sufficient_stock {
        messages.write(LogMessage(format!("Unable to craft {}. Insufficient resources.", event.craft.base.to_string())));
        return;
    }

    for &(stock_kind, amount) in event.craft.resources.iter() {
        stockyard[stock_kind] -= amount;
    }

    let item = item_db.create_basic(event.craft.base);

    let item_entity = commands.spawn((
        item,
    )).id();

    commands.trigger(Crafted { crafted_item: item_entity });
}