//! Crafting information

use bevy::prelude::*;
use smallvec::{SmallVec, smallvec};

use crate::incremental::{item::item_database::ItemDatabase, log::LogEntry, stock::{StockKind, stockyard::Stockyard}};

use super::base::Base;

pub struct ItemCraftPlugin;

impl Plugin for ItemCraftPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, setup_crafts.in_set(super::super::IncrementalStartupSystemSet))
        .add_observer(on_craft_request)
        ;
    }
}

/// Description of how to craft an item.
#[derive(Debug, Clone, Default, Component)]
pub struct Recipe {
    // Item base that will be crafted
    pub base: Base,

    // Cost in stock resources to craft this item
    pub resources: SmallVec<[(StockKind, f64); 2]>,
}

/// Event to fire when the player tries to craft something.
#[derive(Debug, Event)]
pub struct CraftRequest {
    pub recipe: Entity,
}

/// Event for when an item has been crafted.
#[derive(Debug, Event)]
pub struct Crafted {
    /// Entity that contains the crafted [AffixiveItem] as a Component.
    pub crafted_item: Entity,
}

fn setup_crafts(
    mut commands: Commands,
) {
    commands.spawn_scene_list(bsn_list!(
        Recipe {
            base: Base::MakeshiftTools,
            resources: smallvec![(StockKind::BranchesAndPebbles, 1.0)]
        },

        Recipe {
            base: Base::TestTools,
            resources: smallvec![]
        },
    ));
}

fn on_craft_request(
    event: On<CraftRequest>,
    mut commands: Commands,
    
    item_db: Res<ItemDatabase>,
    mut stockyard: ResMut<Stockyard>,

    mut messages: MessageWriter<LogEntry>,

    recipe_query: Query<&Recipe>,
) {
    let recipe = recipe_query.get(event.recipe).expect("CraftRequest event's recipe entity must have a Recipe component.");

    // This has_sufficient_stock code will hopefully be extraneous since the crafting UI
    // should not allow the buttons for crafting these to be pressed when there's not enough
    // stock in the stockyard.

    let has_sufficient_stock = recipe.resources.iter().all(|&(stock_kind, amount)| stockyard[stock_kind] >= amount);
    
    if !has_sufficient_stock {
        messages.write(format!("Unable to craft {}. Insufficient resources.", recipe.base.to_string()).into());
        return;
    }

    for &(stock_kind, amount) in recipe.resources.iter() {
        stockyard[stock_kind] -= amount;
    }

    let item = item_db.create_basic(recipe.base);

    let item_entity = commands.spawn((
        item,
    )).id();

    commands.trigger(Crafted { crafted_item: item_entity });
}