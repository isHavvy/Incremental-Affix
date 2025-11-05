use std::ops::Range;

use bevy::{platform::collections::HashMap, prelude::Resource};
use rand::Rng;

use crate::incremental::item::affixive_item::{AffixiveItem, ItemTag, Quality};
use crate::incremental::item::base::{AffixiveItemBase, Base};
use crate::incremental::item::modifier::{Implicit, Prefix, Suffix};

use super::affixive_item::PushAffixError;

pub type AffixTable = Range<usize>;

#[derive(Debug, Resource)]
pub struct ItemDatabase {
    bases: HashMap<Base, AffixiveItemBase>,
    implicits: Vec<Implicit>,
    prefixes: Vec<Prefix>,
    suffixes: Vec<Suffix>,

    prefix_table: HashMap<Base, AffixTable>,
    suffix_table: HashMap<Base, AffixTable>,
}

impl ItemDatabase {
    pub fn new() -> Self {
        let bases = super::base::initialize();
        let implicits = super::modifier::initialize_implicits();
        let prefixes = super::modifier::initialize_prefixes();
        let suffixes = super::modifier::initialize_suffixes();

        let mut prefix_table: HashMap<Base, AffixTable> = HashMap::new();
        prefix_table.insert(Base::StoneTools, 0..prefixes.len());
        prefix_table.insert(Base::MakeshiftTools, 0..prefixes.len());
        prefix_table.insert(Base::TestTools, 0..prefixes.len());

        let mut suffix_table: HashMap<Base, AffixTable> = HashMap::new();
        suffix_table.insert(Base::StoneTools, 0..suffixes.len());
        suffix_table.insert(Base::MakeshiftTools, 0..suffixes.len());
        suffix_table.insert(Base::TestTools, 0..suffixes.len());

        Self { bases, implicits, prefixes, suffixes, prefix_table, suffix_table }
    }

    pub fn item_has_tag(&self, item: &AffixiveItem, tag: ItemTag) -> bool {
        item.tags.contains(&tag)
    }

    #[allow(unused, reason = "Debug function")]
    pub fn display_item(&self, item: &AffixiveItem) -> String {
        item.display(&self.bases)
    }

    fn prefix_table(&self, item: &AffixiveItem) -> &AffixTable {
        &self.prefix_table[&item.base()]
    }

    fn suffix_table(&self, item: &AffixiveItem) -> &AffixTable {
        &self.suffix_table[&item.base()]
    }

    /// Make a new item with no modifiers or modifier slots of the specified base.
    pub fn create_basic(&self, base: Base) -> AffixiveItem {
        let implicits = &*self.implicits;
        AffixiveItem::new(&self.bases, implicits, base, Quality::Quality(0))
    }

    /// Try to push a random prefix that can be put onto the item onto it.
    pub fn try_push_random_prefix(&self, item: &mut AffixiveItem) -> Result<(), PushAffixError> {
        let mut rng = rand::rng();
        let table = self.prefix_table(item);
        let index = rng.random_range(table.clone());
        let mut prefix = self.prefixes[index].clone();
        prefix.randomize_actual();
        item.try_push_prefix(prefix)
    }

    /// Try to push a random suffix that can be put onto the item onto it.
    pub fn try_push_random_suffix(&self, item: &mut AffixiveItem) -> Result<(), PushAffixError> {
        let mut rng = rand::rng();
        let table = self.suffix_table(item);
        let index = rng.random_range(table.clone());
        let mut suffix = self.suffixes[index].clone();
        suffix.randomize_actual();
        item.try_push_suffix(suffix)
    }
}

impl Default for ItemDatabase {
    fn default() -> Self {
        Self::new()
    }
}