use crate::engine::item::ModifierKind;

#[derive(Debug, Clone, Copy)]
pub(crate) enum GameModifierKind {
    InventoryBase,
    InventoryHeight,
    IncreasedVolume,
    InventorySkillGain,
    ArmorArmorIncrease,
    FootwearSpeedIncrease,
}

impl ModifierKind for GameModifierKind {
    fn display_actual(&self, actual: i32) -> String {
        fn sign(n: i32) -> char {
            if n > 0 { '+' } else { '-' }
        }

        match self {
            Self::InventoryBase => format!("{}{} Inventory Base", sign(actual), actual),
            Self::InventoryHeight => format!("{}{} Inventory Height", sign(actual), actual),
            Self::IncreasedVolume => format!("{}{}% Increased Volume", sign(actual), actual),
            Self::InventorySkillGain => format!("Skills in inventory gain {}% of earned experience", actual),

            Self::ArmorArmorIncrease => format!("{}{} Armor", sign(actual), actual),

            Self::FootwearSpeedIncrease => format!("{}{} Travel Speed", sign(actual), actual),
        }
    }
}