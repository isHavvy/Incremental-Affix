use crate::item::*;

pub(crate) fn initialize_bases() -> Vec<AffixiveItemBase> {
    let mut bases = vec![];

    let inventory_tags = vec![AffixiveItemBaseTag::Inventory];

    bases.push(AffixiveItemBase {
        name: "Bag".to_string(),
        tags: inventory_tags.clone(),
        implicits: vec![ImplicitIndex(0)],
    });

    bases.push(AffixiveItemBase {
        name: "Satchel".to_string(),
        tags: inventory_tags.clone(),
        implicits: vec![ImplicitIndex(1)],
    });

    bases.push(AffixiveItemBase {
        name: "Purse".to_string(),
        tags: inventory_tags.clone(),
        implicits: vec![ImplicitIndex(2)],
    });

    bases.push(AffixiveItemBase {
        name: "Backpack".to_string(),
        tags: inventory_tags.clone(),
        implicits: vec![ImplicitIndex(3)],
    });

    bases.push(AffixiveItemBase {
        name: "Rucksack".to_string(),
        tags: inventory_tags.clone(),
        implicits: vec![ImplicitIndex(4)],
    });

    bases.push(AffixiveItemBase {
        name: "Pocket Dimension".to_string(),
        tags: inventory_tags.clone(),
        implicits: vec![ImplicitIndex(5)],
    });

    bases
}

pub(crate) fn initialize_implicits() -> Vec<Implicit> {
    let mut mods = vec![];

    mods.push(Affix {
        name: "BagInventory".to_string(),
        modifier: Modifier { kind: ModifierKind::InventoryBase, min: 1, max: 1 },
        modifier_actual: 1,
        hybrid_modifier: Some(Modifier { kind: ModifierKind::InventoryHeight, min: 1, max: 1 }),
        hybrid_modifier_actual: 1,
    });

    mods.push(Affix {
        name: "SatchelInventory".to_string(),
        modifier: Modifier { kind: ModifierKind::InventoryBase, min: 1, max: 1 },
        modifier_actual: 1,
        hybrid_modifier: Some(Modifier { kind: ModifierKind::InventoryHeight, min: 3, max: 3 }),
        hybrid_modifier_actual: 3,
    });

    mods.push(Affix {
        name: "PurseInventory".to_string(),
        modifier: Modifier { kind: ModifierKind::InventoryBase, min: 2, max: 2 },
        modifier_actual: 2,
        hybrid_modifier: Some(Modifier { kind: ModifierKind::InventoryHeight, min: 3, max: 3 }),
        hybrid_modifier_actual: 3,
    });

    mods.push(Affix {
        name: "BackpackInventory".to_string(),
        modifier: Modifier { kind: ModifierKind::InventoryBase, min: 3, max: 3 },
        modifier_actual: 3,
        hybrid_modifier: Some(Modifier { kind: ModifierKind::InventoryHeight, min: 3, max: 3 }),
        hybrid_modifier_actual: 3,
    });

    mods.push(Affix {
        name: "RucksackInventory".to_string(),
        modifier: Modifier { kind: ModifierKind::InventoryBase, min: 3, max: 3 },
        modifier_actual: 3,
        hybrid_modifier: Some(Modifier { kind: ModifierKind::InventoryHeight, min: 5, max: 5 }),
        hybrid_modifier_actual: 5,
    });

    mods.push(Affix {
        name: "PocketDimensionInventory".to_string(),
        modifier: Modifier { kind: ModifierKind::InventoryBase, min: 3, max: 3 },
        modifier_actual: 3,
        hybrid_modifier: Some(Modifier { kind: ModifierKind::InventoryHeight, min: 9, max: 9 }),
        hybrid_modifier_actual: 9,
    });

    mods.into_iter().map(Implicit).collect()
}

pub(crate) fn initialize_suffixes() -> Vec<Suffix> {
    let mut suffixes = vec![];

    suffixes.push(Affix {
        name: "Holding".to_owned(),
        modifier: Modifier { kind: ModifierKind::InventoryBase, min: 1, max: 1 },
        modifier_actual: 1,
        hybrid_modifier: None,
        hybrid_modifier_actual: 0,
    });
    
    suffixes.push(Affix {
        name: "More Holding".to_owned(),
        modifier: Modifier { kind: ModifierKind::InventoryBase, min: 2, max: 2 },
        modifier_actual: 2,
        hybrid_modifier: None,
        hybrid_modifier_actual: 0,
    });

    suffixes.push(Affix::new("Tiny Pockets".to_string(), Modifier { kind: ModifierKind::IncreasedVolume, min: 8, max: 15 }));
    suffixes.push(Affix::new("Pockets".to_string(), Modifier { kind: ModifierKind::IncreasedVolume, min: 16, max: 24 }));
    suffixes.push(Affix::new("Large Tiny Pockets".to_string(), Modifier { kind: ModifierKind::IncreasedVolume, min: 25, max: 34 }));
    suffixes.push(Affix::new("Gargantuan Pockets".to_string(), Modifier { kind: ModifierKind::IncreasedVolume, min: 35, max: 45 }));

    return suffixes.into_iter().map(Suffix).collect()
}

pub(crate) fn initialize_prefixes() -> Vec<Prefix> {
    let mut prefixes = vec![];

    prefixes.push(Affix::new("Student's".to_string(), Modifier { kind: ModifierKind::InventorySkillGain, min: 1, max: 25 }));
    prefixes.push(Affix::new("Teacher's".to_string(), Modifier { kind: ModifierKind::InventorySkillGain, min: 26, max: 50 }));
    prefixes.push(Affix::new("Professor's".to_string(), Modifier { kind: ModifierKind::InventorySkillGain, min: 51, max: 75 }));
    prefixes.push(Affix::new("Mentor's".to_string(), Modifier { kind: ModifierKind::InventorySkillGain, min: 76, max: 100 }));

    return prefixes.into_iter().map(Prefix).collect();
}
