use std::ops::Not as _;

use super::item::{AffixiveItem, AffixiveItemBase, AffixiveItemTag, Modifier, ModifierKind, ModifierValue};

pub enum SwapFailReason {
    TooHighLevel,
    NotEquippableInSlot,
}

#[derive(Debug)]
pub struct ItemSlot<MK> where MK: ModifierKind {
    item: Option<AffixiveItem<MK>>,
    slot_kind: AffixiveItemTag,
    level: u8,
}

impl<MK> ItemSlot<MK> where MK: ModifierKind {
    pub fn new(slot_kind: AffixiveItemTag, level: u8) -> Self {
        Self {
            item: None, slot_kind, level,
        }
    }

    pub fn can_hold(&mut self, item: &AffixiveItem<MK>, bases: &[AffixiveItemBase]) -> Result<(), SwapFailReason> {
        if item.level(bases) > self.level {
            Err(SwapFailReason::TooHighLevel)
        } else if item.tags.contains(&self.slot_kind).not() {
            Err(SwapFailReason::NotEquippableInSlot)
        } else {
            Ok(())
        }
    }

    pub fn replace_item(&mut self, maybe_replacement_item: Option<AffixiveItem<MK>>) -> Option<AffixiveItem<MK>> {
        std::mem::replace(&mut self.item, maybe_replacement_item)
    }
    
    pub fn modifiers(&self) -> impl Iterator<Item=(&Modifier<MK>, ModifierValue)> {
        self.item.iter().flat_map(|item| item.modifiers())
    }
}