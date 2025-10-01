use bevy::ecs::component::Component;

#[derive(Debug, Component)]
pub struct ItemSlot {
    tag: ItemSlotTag,
}

#[derive(Debug, Component, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ItemSlotTag {
    Tool,
}