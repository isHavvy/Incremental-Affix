use std::ops::Deref;

#[derive(Debug, Clone, Copy)]
pub(crate) enum ModifierKind {
    InventoryBase,
    InventoryHeight,
}

#[derive(Debug, Clone, Copy)]
pub struct Modifier {
    kind: ModifierKind,
    min: i32,
    max: i32,
}

fn sign(n: i32) -> char {
    if n > 0 { '+' } else { '-' }
}

impl Modifier {
    fn display_actual(&self, actual: i32) -> String {
        match self.kind {
            ModifierKind::InventoryBase => format!("{}{} Inventory Base", sign(actual), actual),
            ModifierKind::InventoryHeight => format!("{}{} Inventory Height", sign(actual), actual),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Affix {
    name: String,
    modifier: Modifier,
    modifier_actual: i32,
    hybrid_modifier: Option<Modifier>,
    hybrid_modifier_actual: i32,
}

impl Affix {
    fn display(&self) -> String {
        let mut output = String::new();

        output.push_str(&self.modifier.display_actual(self.modifier_actual));
        if let Some(hybrid_modifier) = self.hybrid_modifier {
            output.push('\n');
            output.push_str(&hybrid_modifier.display_actual(self.hybrid_modifier_actual));
        }

        output
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Implicit(Affix);

impl Deref for Implicit {
    type Target = Affix;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug)]
pub(crate) struct Prefix(Affix);

#[derive(Debug)]
pub(crate) struct Suffix(Affix);

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(crate) enum Quality {
    /// An item with fixed affixes.
    FixedArtifact,

    /// An item with the specific number of prefixes and suffixes.
    /// For example, an item with Quality::Qaulity(2) will have 2
    /// prefixes and 2 suffixes for a total of 4 affixes.
    Quality(i8),
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct AffixiveItemBaseIndex(usize);

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct ImplicitIndex(usize);

#[derive(Debug, Clone, Copy)]
enum AffixiveItemBaseTag {
    Inventory,
}

#[derive(Debug)]
pub(crate) struct AffixiveItemBase {
    name: String,
    tags: Vec<AffixiveItemBaseTag>,
    implicits: Vec<ImplicitIndex>,
}

#[derive(Debug)]
pub(crate) struct AffixiveItem {
    base: AffixiveItemBaseIndex,
    implicits: Vec<Implicit>,
    prefixes: Vec<Prefix>,
    suffixes: Vec<Suffix>,
    quality: Quality,
}

impl AffixiveItem {
    fn display(&self, implicits: &[Implicit], bases: &[AffixiveItemBase]) -> String {
        let mut output = String::new();

        let name: &str = &bases[self.base.0].name;

        match self.quality {
            Quality::FixedArtifact => {
                output.push_str(&format!("Artifact: {}", name));
            },

            Quality::Quality(quality) => {
                for _ in 0..quality {
                    output.push('~');
                }

                output.push_str(name);

                for _ in 0..quality {
                    output.push('~');
                }
            }
        }

        output.push_str("\n===\n");

        for implicit in self.implicits.iter() {
            output.push_str(&implicit.display())
        }

        output.push_str("\n===\n");

        output
    }
}

fn initialize_bases() -> Vec<AffixiveItemBase> {
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

fn initialize_implicits() -> Vec<Implicit> {
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

fn initialize_suffixes() -> Vec<Suffix> {
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

    return suffixes.into_iter().map(Suffix).collect()
}

pub fn make_backpack() -> (Vec<Implicit>, Vec<AffixiveItemBase>, AffixiveItem) {
    let implicits = initialize_implicits();
    let bases = initialize_bases();

    let backpack_base = &bases[3];

    let backpack = AffixiveItem {
        base: AffixiveItemBaseIndex(3),
        implicits: backpack_base.implicits.iter().map(|ix| implicits[ix.0].clone()).collect(),
        prefixes: vec![],
        suffixes: vec![],
        quality: Quality::Quality(0),
    };

    println!("{}", backpack.display(&implicits, &bases));
    (implicits, bases, backpack)
}