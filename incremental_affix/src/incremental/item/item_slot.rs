use bevy::prelude::*;

#[derive(Debug, Component)]
#[expect(unused)]
pub struct ItemSlot {
    tag: ItemSlotTag,
    item: Option<Entity>,
}

#[derive(Debug, Component, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ItemSlotTag {
    Tool,
}