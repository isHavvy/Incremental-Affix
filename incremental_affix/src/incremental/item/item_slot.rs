use bevy::prelude::*;

#[derive(Debug, Component, Clone, Default)]
pub struct ItemSlot {
    pub tag: ItemSlotTag,
    pub item: Option<Entity>,
}

#[derive(Debug, Component, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ItemSlotTag {
    #[default] // To allow for ItemSlot to be used in BSN.
    Tool,
    Hunt,
}

impl std::fmt::Display for ItemSlotTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match *self {
            ItemSlotTag::Tool => "Tools",
            ItemSlotTag::Hunt => "Hunting Gear",
        })
    }
}