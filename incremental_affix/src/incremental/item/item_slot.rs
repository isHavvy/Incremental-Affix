use bevy::prelude::*;

#[derive(Debug, Component)]
pub struct ItemSlot {
    pub tag: ItemSlotTag,
    pub item: Option<Entity>,
}

#[derive(Debug, Component, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ItemSlotTag {
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