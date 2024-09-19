use crate::engine::{item::{AffixiveItemBase, AffixiveItemTag}, item_slot::SwapFailReason};

use super::{AffixiveItem, ItemSlot};

fn percent(n: i32) -> f32 {
    (n as f32) / 100.0
}

#[derive(Debug)]
pub struct Player {
    // Equipment Slots
    slot_inventory: ItemSlot,
    slot_footwear: ItemSlot,

    // Calculated Stats
    inventory_volume: f32,
    inventory_skill_gain_percent: i32,
    speed_multiplier: f32,

    armor: i32,

    // Other Stats
    #[expect(unused)]
    level: u8,
}

impl Player {
    pub fn new() -> Self {
        Self {
            slot_inventory: ItemSlot::new(AffixiveItemTag::Inventory, 1),
            slot_footwear: ItemSlot::new(AffixiveItemTag::Footwear, 1),

            inventory_volume: 9.0,
            inventory_skill_gain_percent: 0,
            speed_multiplier: 1.0,

            armor: 0,

            level: 1
        }
    }

    pub fn try_equipping(&mut self, replacement_item: AffixiveItem, bases: &[AffixiveItemBase]) -> Result<Option<AffixiveItem>, SwapFailReason> {
        let replaced_item = 'replaced_item: {
            for item_slot in [&mut self.slot_footwear, &mut self.slot_inventory] {
                match item_slot.can_hold(&replacement_item, bases) {
                    Ok(_) => break 'replaced_item Ok(item_slot.replace_item(Some(replacement_item))),
                    Err(SwapFailReason::NotEquippableInSlot) => continue,
                    Err(error) => break 'replaced_item Err(error)
                }
            }

            Err(SwapFailReason::NotEquippableInSlot)
        }?;

        self.recalculate_bonuses();
        Ok(replaced_item)
    }

    pub fn recalculate_bonuses(&mut self) {
        let slots = vec![
            &self.slot_inventory,
            &self.slot_footwear,
        ];

        let mut inventory_base_bonus = 0;
        let mut inventory_height_bonus = 0;
        let mut inventory_volume_bonus = 0;
        let mut inventory_skill_gain_bonus = 0;
        let mut armor_increase = 0;
        let mut speed_increase = 0;

        let modifiers = slots.iter().flat_map(|slot| slot.modifiers());
        for (modifier, modifier_value) in modifiers {
            match modifier.kind {
                super::modifiers::GameModifierKind::InventoryBase => { inventory_base_bonus +=  modifier_value; },
                super::modifiers::GameModifierKind::InventoryHeight => { inventory_height_bonus += modifier_value; },
                super::modifiers::GameModifierKind::IncreasedVolume => { inventory_volume_bonus += modifier_value; },
                super::modifiers::GameModifierKind::InventorySkillGain => { inventory_skill_gain_bonus += modifier_value; },
                super::modifiers::GameModifierKind::ArmorArmorIncrease => { armor_increase += modifier_value; },
                super::modifiers::GameModifierKind::FootwearSpeedIncrease => { speed_increase += modifier_value; },
            }
        }

        let base = 3.0 + inventory_base_bonus as f32;
        let height = 3.0 + inventory_height_bonus as f32;
        self.inventory_volume = base * height * (1.0 + percent(inventory_volume_bonus));

        self.inventory_skill_gain_percent = inventory_skill_gain_bonus;

        self.speed_multiplier = 1.0 + percent(speed_increase);

        self.armor = armor_increase;
    }

    pub fn get_inventory_volume(&self) -> u32 {
        self.inventory_volume as u32
    }
}

impl Default for Player {
    fn default() -> Self {
        Self::new()
    }
}