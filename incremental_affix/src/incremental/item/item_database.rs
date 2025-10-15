use bevy::{platform::collections::HashMap, prelude::Resource};

use crate::incremental::item::affixive_item::{AffixiveItem, ItemTag, Quality};
use crate::incremental::item::base::{AffixiveItemBase, Base};
use crate::incremental::item::modifier::{Implicit, Prefix, Suffix};

use super::affixive_item::PushAffixError;

pub type PrefixTable = Vec<usize>;

#[derive(Debug, Resource)]
pub struct ItemDatabase {
    bases: HashMap<Base, AffixiveItemBase>,
    implicits: Vec<Implicit>,
    prefixes: Vec<Prefix>,
    #[expect(unused)] suffixes: Vec<Suffix>,

    prefix_table: HashMap<Base, PrefixTable>,
}

impl ItemDatabase {
    pub fn new() -> Self {
        let bases = super::base::initialize();
        let implicits = super::modifier::initialize_implicits();
        let prefixes = super::modifier::initialize_prefixes();
        let suffixes = super::modifier::initialize_suffixes();

        let mut prefix_table: HashMap<Base, PrefixTable> = HashMap::new();
        prefix_table.insert(Base::StoneTools, vec![0]);
        prefix_table.insert(Base::MakeshiftTools, vec![0]);
        prefix_table.insert(Base::TestTools, vec![0]);

        Self { bases, implicits, prefixes, suffixes, prefix_table }
    }

    pub fn item_has_tag(&self, item: &AffixiveItem, tag: ItemTag) -> bool {
        item.tags.contains(&tag)
    }

    #[allow(unused, reason = "Debug function")]
    pub fn display_item(&self, item: &AffixiveItem) -> String {
        item.display(&self.bases)
    }

    fn prefix_table(&self, item: &AffixiveItem) -> &PrefixTable {
        &self.prefix_table[&item.base()]
    }

    /// Make a new item with no modifiers or modifier slots of the specified base.
    pub fn create_basic(&self, base: Base) -> AffixiveItem {
        let implicits = &*self.implicits;
        AffixiveItem::new(&self.bases, implicits, base, Quality::Quality(0))
    }

    /// Try to push a random prefix that can be put onto the item onto it.
    pub fn try_push_random_prefix(&self, item: &mut AffixiveItem) -> Result<(), PushAffixError> {
        let mut prefix = self.prefixes[self.prefix_table(&item)[0]].clone();
        prefix.randomize_actual();
        let res = item.try_push_prefix(prefix);

        eprintln!("{}", self.display_item(&item));
        eprintln!("{:?}", &item);

        res
    }
}

impl Default for ItemDatabase {
    fn default() -> Self {
        Self::new()
    }
}