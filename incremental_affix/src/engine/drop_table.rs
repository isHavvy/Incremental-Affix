use std::ops::Deref;

use anymap::AnyMap;
use rand::Rng;

use crate::item::{AffixiveItemBase, AffixiveItemBaseIndex, Prefix, Suffix};

use super::item::{AffixiveItemBaseTagMap, ModifierKind};

pub struct DropTable<T> {
    total_weight: i32,
    weights: Vec<i32>,
    modifiers: Vec<T>,
}

impl<T: Clone> DropTable<T> {
    pub fn random(&self) -> T {
        let r = rand::thread_rng().gen_range(0..self.total_weight);

        let ix = self.weights.partition_point(|weight| weight <= &r);
        self.modifiers[ix].clone()
    }
}

struct DropTableBuilder<T> {
    total_weight: i32,
    weights: Vec<i32>,
    modifiers: Vec<T>,
}

impl<T: Clone> DropTableBuilder<T> {
    fn new() -> Self {
        Self {
            total_weight: 0,
            weights: vec![],
            modifiers: vec![],
        }
    }

    fn build(self) -> DropTable<T> {
        DropTable {
            total_weight: self.total_weight,
            weights: self.weights,
            modifiers: self.modifiers,
        }
    }

    fn add_loot(mut self, loot: T, weight: i32) -> Self {
        self.modifiers.push(loot);
        self.weights.push(self.total_weight + weight);
        
        Self {
            total_weight: self.total_weight + weight,
            weights: self.weights,
            modifiers: self.modifiers,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SuffixOrPrefix {
    Suffix,
    Prefix,
}

pub(crate) struct InventoryModifierPrefixes<MK: ModifierKind>(DropTable<Prefix<MK>>);

impl<MK> Deref for InventoryModifierPrefixes<MK> where MK: ModifierKind {
    type Target = DropTable<Prefix<MK>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub(crate) struct InventoryModifierSuffixes<MK: ModifierKind>(DropTable<Suffix<MK>>);

impl<MK> Deref for InventoryModifierSuffixes<MK> where MK: ModifierKind {
    type Target = DropTable<Suffix<MK>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct SuffixOrPrefixDropTable(DropTable<SuffixOrPrefix>);

impl Deref for SuffixOrPrefixDropTable {
    type Target = DropTable<SuffixOrPrefix>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub(crate) struct StorageBasesDropTable(DropTable<AffixiveItemBaseIndex>);

impl Deref for StorageBasesDropTable {
    type Target = DropTable<AffixiveItemBaseIndex>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn initialize_drop_tables<MK: ModifierKind>(bases: &[AffixiveItemBase], prefixes: AffixiveItemBaseTagMap<Vec<Prefix<MK>>>, suffixes: AffixiveItemBaseTagMap<Vec<Suffix<MK>>>) -> AnyMap {
    let mut drop_tables = AnyMap::new();

    let mut base_drop_table = DropTableBuilder::new();
    for (ix, _) in bases.iter().enumerate() {
        base_drop_table = base_drop_table.add_loot(AffixiveItemBaseIndex(ix), 50);
    }
    let base_drop_table = base_drop_table.build();

    drop_tables.insert(StorageBasesDropTable(base_drop_table));

    let inventory_prefix_drop_table = DropTableBuilder::new()
        .add_loot(prefixes.inventory[0].clone(), 50)
        .add_loot(prefixes.inventory[1].clone(), 50)
        .add_loot(prefixes.inventory[2].clone(), 50)
        .add_loot(prefixes.inventory[3].clone(), 50)
        .build();

    drop_tables.insert(InventoryModifierPrefixes(inventory_prefix_drop_table));

    let suffix_drop_table = DropTableBuilder::new()
        .add_loot(suffixes.inventory[0].clone(), 50)
        .add_loot(suffixes.inventory[1].clone(), 50)
        .add_loot(suffixes.inventory[2].clone(), 50)
        .add_loot(suffixes.inventory[3].clone(), 50)
        .build();

    drop_tables.insert(InventoryModifierSuffixes(suffix_drop_table));

    let prefix_or_suffix_table = DropTableBuilder::new()
        .add_loot(SuffixOrPrefix::Suffix, 1)
        .add_loot(SuffixOrPrefix::Prefix, 1)
        .build();

    drop_tables.insert(SuffixOrPrefixDropTable(prefix_or_suffix_table));

    return drop_tables;
}