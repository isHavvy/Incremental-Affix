use crate::item::*;

use super::modifiers::GameModifierKind;

pub(crate) fn initialize_bases() -> Vec<AffixiveItemBase> {
    let mut bases = vec![];

    let inventory_tags = vec![AffixiveItemTag::Inventory];
    let boot_tags = vec![AffixiveItemTag::Footwear, AffixiveItemTag::Armor];

    bases.push(AffixiveItemBase {
        name: "Bag".to_string(),
        level: 1,
        tags: inventory_tags.clone(),
        implicits: vec![ImplicitIndex(0)],
    });

    bases.push(AffixiveItemBase {
        name: "Satchel".to_string(),
        level: 1,
        tags: inventory_tags.clone(),
        implicits: vec![ImplicitIndex(1)],
    });

    bases.push(AffixiveItemBase {
        name: "Purse".to_string(),
        level: 1,
        tags: inventory_tags.clone(),
        implicits: vec![ImplicitIndex(2)],
    });

    bases.push(AffixiveItemBase {
        name: "Backpack".to_string(),
        level: 1,
        tags: inventory_tags.clone(),
        implicits: vec![ImplicitIndex(3)],
    });

    bases.push(AffixiveItemBase {
        name: "Rucksack".to_string(),
        level: 1,
        tags: inventory_tags.clone(),
        implicits: vec![ImplicitIndex(4)],
    });

    bases.push(AffixiveItemBase {
        name: "Pocket Dimension".to_string(),
        level: 1,
        tags: inventory_tags.clone(),
        implicits: vec![ImplicitIndex(5)],
    });

    bases.push(AffixiveItemBase {
        name: "Sandals".to_string(),
        level: 1,
        tags: boot_tags.clone(),
        implicits: vec![ImplicitIndex(6)],
    });

    bases
}

pub(crate) fn initialize_implicits() -> Vec<super::Implicit> {
    let mut mods = vec![];

    mods.push(Affix {
        name: "BagInventory".to_string(),
        modifier: Modifier { kind: GameModifierKind::InventoryBase, min: 1, max: 1 },
        modifier_actual: 1,
        hybrid_modifier: Some(Modifier { kind: GameModifierKind::InventoryHeight, min: 1, max: 1 }),
        hybrid_modifier_actual: 1,
    });

    mods.push(Affix {
        name: "SatchelInventory".to_string(),
        modifier: Modifier { kind: GameModifierKind::InventoryBase, min: 1, max: 1 },
        modifier_actual: 1,
        hybrid_modifier: Some(Modifier { kind: GameModifierKind::InventoryHeight, min: 3, max: 3 }),
        hybrid_modifier_actual: 3,
    });

    mods.push(Affix {
        name: "PurseInventory".to_string(),
        modifier: Modifier { kind: GameModifierKind::InventoryBase, min: 2, max: 2 },
        modifier_actual: 2,
        hybrid_modifier: Some(Modifier { kind: GameModifierKind::InventoryHeight, min: 3, max: 3 }),
        hybrid_modifier_actual: 3,
    });

    mods.push(Affix {
        name: "BackpackInventory".to_string(),
        modifier: Modifier { kind: GameModifierKind::InventoryBase, min: 3, max: 3 },
        modifier_actual: 3,
        hybrid_modifier: Some(Modifier { kind: GameModifierKind::InventoryHeight, min: 3, max: 3 }),
        hybrid_modifier_actual: 3,
    });

    mods.push(Affix {
        name: "RucksackInventory".to_string(),
        modifier: Modifier { kind: GameModifierKind::InventoryBase, min: 3, max: 3 },
        modifier_actual: 3,
        hybrid_modifier: Some(Modifier { kind: GameModifierKind::InventoryHeight, min: 5, max: 5 }),
        hybrid_modifier_actual: 5,
    });

    mods.push(Affix {
        name: "PocketDimensionInventory".to_string(),
        modifier: Modifier { kind: GameModifierKind::InventoryBase, min: 3, max: 3 },
        modifier_actual: 3,
        hybrid_modifier: Some(Modifier { kind: GameModifierKind::InventoryHeight, min: 9, max: 9 }),
        hybrid_modifier_actual: 9,
    });

    mods.push(Affix {
        name: "SandalFootwear".to_string(),
        modifier: Modifier { kind: GameModifierKind::ArmorArmorIncrease, min: 2, max: 5 },
        modifier_actual: 0,
        hybrid_modifier: None,
        hybrid_modifier_actual: 0,
    });

    mods.into_iter().map(Implicit).collect()
}

pub(crate) fn initialize_suffixes() -> AffixiveItemBaseTagMap<Vec<super::Suffix>> {
    let mut suffixes = vec![];

    suffixes.push(Affix {
        name: "Holding".to_string(),
        modifier: Modifier { kind: GameModifierKind::InventoryBase, min: 1, max: 1 },
        modifier_actual: 1,
        hybrid_modifier: None,
        hybrid_modifier_actual: 0,
    });
    
    suffixes.push(Affix {
        name: "More Holding".to_string(),
        modifier: Modifier { kind: GameModifierKind::InventoryBase, min: 2, max: 2 },
        modifier_actual: 2,
        hybrid_modifier: None,
        hybrid_modifier_actual: 0,
    });

    suffixes.push(Affix::new("Tiny Pockets".to_string(), Modifier { kind: GameModifierKind::IncreasedVolume, min: 8, max: 15 }));
    suffixes.push(Affix::new("Pockets".to_string(), Modifier { kind: GameModifierKind::IncreasedVolume, min: 16, max: 24 }));
    suffixes.push(Affix::new("Large Tiny Pockets".to_string(), Modifier { kind: GameModifierKind::IncreasedVolume, min: 25, max: 34 }));
    suffixes.push(Affix::new("Gargantuan Pockets".to_string(), Modifier { kind: GameModifierKind::IncreasedVolume, min: 35, max: 45 }));

    let inventory_suffixes = suffixes.into_iter().map(Suffix).collect();

    let mut armor_suffixes = vec![];

    armor_suffixes.push(Affix::new("Protection 1".to_string(), Modifier { kind: GameModifierKind::ArmorArmorIncrease, min: 2, max: 5 }));

    let armor_suffixes = armor_suffixes.into_iter().map(Suffix).collect();

    let mut footwear_suffixes = vec![];

    footwear_suffixes.push(Affix::new("Speed 1".to_string(), Modifier { kind: GameModifierKind::FootwearSpeedIncrease, min: 1, max: 5 }));

    let feet_suffixes = footwear_suffixes.into_iter().map(Suffix).collect();

    AffixiveItemBaseTagMap {
        inventory: inventory_suffixes,
        armor: armor_suffixes,
        footwear: feet_suffixes,
    }
}

pub(crate) fn initialize_prefixes() -> AffixiveItemBaseTagMap<Vec<super::Prefix>> {
    let mut inventory = vec![];

    inventory.push(Affix::new("Student's".to_string(), Modifier { kind: GameModifierKind::InventorySkillGain, min: 1, max: 25 }));
    inventory.push(Affix::new("Teacher's".to_string(), Modifier { kind: GameModifierKind::InventorySkillGain, min: 26, max: 50 }));
    inventory.push(Affix::new("Professor's".to_string(), Modifier { kind: GameModifierKind::InventorySkillGain, min: 51, max: 75 }));
    inventory.push(Affix::new("Mentor's".to_string(), Modifier { kind: GameModifierKind::InventorySkillGain, min: 76, max: 100 }));

    let inventory = inventory.into_iter().map(Prefix).collect();

    let armor = vec![];
    let footwear = vec![];

    AffixiveItemBaseTagMap {
        inventory, armor, footwear,
    }
}
