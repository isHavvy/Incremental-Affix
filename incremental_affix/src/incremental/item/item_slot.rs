use bevy::prelude::*;

#[derive(Debug, Component)]
pub struct ItemSlot {
    pub tag: ItemSlotTag,
    pub item: Option<Entity>,
}

#[derive(Debug, Component, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ItemSlotTag {
    Tool,
}