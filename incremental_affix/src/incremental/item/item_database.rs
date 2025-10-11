use bevy::{platform::collections::HashMap, prelude::Resource};

use crate::incremental::item::{affixive_item::{AffixiveItem, ItemTag, Quality}, base::{AffixiveItemBase, Base}, modifier::Implicit};

#[derive(Debug, Resource)]
pub struct ItemDatabase {
    bases: HashMap<Base, AffixiveItemBase>,
    implicits: Vec<Implicit>,
}

impl ItemDatabase {
    pub fn new() -> Self {
        let bases = super::base::initialize();
        let implicits = super::modifier::initialize_implicits();

        Self { bases, implicits }
    }

    /// Make a new item with no modifiers or modifier slots of the specified base.
    pub fn create_basic(&self, base: Base) -> AffixiveItem {
        let implicits = &*self.implicits;
        AffixiveItem::new(&self.bases, implicits, base, Quality::Quality(0))
    }

    pub fn item_has_tag(&self, item: &AffixiveItem, tag: ItemTag) -> bool {
        item.tags.contains(&tag)
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