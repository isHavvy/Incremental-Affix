pub mod item;
pub mod player;
pub mod modifiers;

use modifiers::GameModifierKind;

pub type Implicit = crate::item::Implicit<GameModifierKind>;
pub type Prefix = crate::item::Prefix<GameModifierKind>;
pub type Suffix = crate::item::Suffix<GameModifierKind>;
pub type AffixiveItem = crate::item::AffixiveItem<GameModifierKind>;
pub type ItemSlot = crate::engine::item_slot::ItemSlot<GameModifierKind>;