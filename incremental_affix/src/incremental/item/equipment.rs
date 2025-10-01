use bevy::prelude::*;

#[derive(Debug, Default, Resource)]
pub struct Slots {
    tools: Option<Entity>,
}

// impl Default for Slots {
//     fn default() -> Self {
//         Self {
//             tools: ItemSlot::new(crate::engine::item::AffixiveItemTag::Armor, 0),
//         }
//     }
// }