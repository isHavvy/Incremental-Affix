use bevy::ecs::{component::Component, resource::Resource};

use affixive_item::{AffixiveItemBase, AffixiveItemBaseIndex, ImplicitIndex, Quality};
use modifier::Modifiers;

use crate::incremental::item::{affixive_item::{AffixiveItem, ItemTag}, modifier::{initialize_implicits, Implicit}};

pub mod equipment;
pub mod item_slot;
pub mod affixive_item;
pub mod modifier;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Component)]
pub enum Base {
    MakeshiftTools,
}

fn makeshift_tool() -> AffixiveItemBase {
    AffixiveItemBase {
        name: "Makeshift Tools".to_string(),
        level: 0,
        tags: vec![],
        implicits: vec![
            ImplicitIndex(0),
            ImplicitIndex(1),
        ],
    }
}

#[derive(Debug, Resource)]
pub struct ItemDatabase {
    bases: Vec<AffixiveItemBase>,
    implicits: Vec<Implicit>,
}

impl ItemDatabase {
    pub fn new() -> Self {
        let bases = vec![
            makeshift_tool()
        ];

        let implicits = initialize_implicits();

        Self { bases, implicits }
    }

    #[expect(unused)]
    pub fn get_base(&self, base: Base) -> &AffixiveItemBase {
        let ix = match base {
            Base::MakeshiftTools => 0,
        };

        &self.bases[ix]
    }

    pub fn get_base_ix(base: Base) -> AffixiveItemBaseIndex {
        AffixiveItemBaseIndex(match base {
            Base::MakeshiftTools => 0,
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

    pub fn display_item(&self, item: &AffixiveItem) -> String {
        item.display(&self.bases)
    }
}

impl Default for ItemDatabase {
    fn default() -> Self {
        Self::new()
    }
}