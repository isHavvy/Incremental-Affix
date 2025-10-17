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

impl ToString for ItemSlotTag {
    fn to_string(&self) -> String {
        match *self {
            ItemSlotTag::Tool => "Tools".into(),
            ItemSlotTag::Hunt => "Hunting Gear".into(),
        }
    }
}