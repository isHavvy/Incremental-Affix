use bevy::prelude::*;

use affixive_item::{AffixiveItemBase, AffixiveItemBaseIndex, Quality};
use modifier::Modifiers;
use bases::{Base, makeshift_tools, secondary_tools};

use crate::incremental::item::{affixive_item::{AffixiveItem, ItemTag}, modifier::{initialize_implicits, Implicit}};

pub mod equipment;
pub mod item_slot;
pub mod affixive_item;
pub mod modifier;
pub mod bases;

#[derive(Debug, Resource)]
pub struct ItemDatabase {
    bases: Vec<AffixiveItemBase>,
    implicits: Vec<Implicit>,
}

impl ItemDatabase {
    pub fn new() -> Self {
        let bases = vec![
            makeshift_tools(),
            secondary_tools(),
        ];

        let implicits = initialize_implicits();

        Self { bases, implicits }
    }

    pub fn get_base_ix(base: Base) -> AffixiveItemBaseIndex {
        AffixiveItemBaseIndex(match base {
            Base::MakeshiftTools => 0,
            Base::SecondaryTools => 1,
        })
    }

    /// Make a new item with no modifiers or modifier slots of the specified base.
    pub fn create_basic(&self, base: Base) -> AffixiveItem {
        let implicits = &*self.implicits;
        AffixiveItem::new(&self.bases, implicits, Self::get_base_ix(base), Quality::Quality(0))
    }

    pub fn item_has_tag(&self, item: &AffixiveItem, tag: ItemTag) -> bool {
        item.tags.contains(&tag)
    }

    pub fn item_name(&self, item: &AffixiveItem) -> &str {
        item.name(&self.bases)
    }

    #[allow(unused, reason = "Debug function")]
    pub fn display_item(&self, item: &AffixiveItem) -> String {
        item.display(&self.bases)
    }
}

impl Default for ItemDatabase {
    fn default() -> Self {
        Self::new()
    }
}