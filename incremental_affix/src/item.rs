use std::ops::{Deref, DerefMut};

use anymap::AnyMap;
use rand::Rng as _;

#[derive(Debug, Clone, Copy)]
pub(crate) enum ModifierKind {
    InventoryBase,
    InventoryHeight,
    IncreasedVolume,
    InventorySkillGain,
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
            ModifierKind::IncreasedVolume => format!("{}{}% Increased Volume", sign(actual), actual),
            ModifierKind::InventorySkillGain => format!("Skills in inventory gain {}% of earned experience", actual),
        }
    }

    fn random_modifier_value(&self) -> i32 {
        rand::thread_rng().gen_range(self.min..self.max + 1)
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
    /// Construct a new affix with the given modifier. Sets the hybrid to `None`.
    fn new(name: String, modifier: Modifier) -> Self {
        Self {
            name,
            modifier,
            modifier_actual: if modifier.min == modifier.max { modifier.min } else { 0 },
            hybrid_modifier: None,
            hybrid_modifier_actual: 0,
        }
    }

    fn display(&self) -> String {
        let mut output = String::new();

        output.push_str(&self.modifier.display_actual(self.modifier_actual));
        if let Some(hybrid_modifier) = self.hybrid_modifier {
            output.push('\n');
            output.push_str(&hybrid_modifier.display_actual(self.hybrid_modifier_actual));
        }

        output
    }

    pub(crate) fn randomize_actual(&mut self) {
        self.modifier_actual = self.modifier.random_modifier_value();
        if let Some(modifier) = self.hybrid_modifier {
            self.hybrid_modifier_actual = modifier.random_modifier_value();
        }
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

impl DerefMut for Implicit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Prefix(Affix);

impl Deref for Prefix {
    type Target = Affix;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Prefix {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Suffix(Affix);

impl Deref for Suffix {
    type Target = Affix;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Suffix {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

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
pub(crate) struct AffixiveItemBaseIndex(usize);

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

#[derive(Debug, Clone, Copy)]
pub(crate) enum PushAffixError {
    AffixiveItemIsFixed,
    AffixiveItemQualityTooLow,
}

#[derive(Debug)]
pub(crate) struct AffixiveItem {
    base_ix: AffixiveItemBaseIndex,
    implicits: Vec<Implicit>,
    prefixes: Vec<Prefix>,
    suffixes: Vec<Suffix>,
    quality: Quality,
}

impl AffixiveItem {
    pub(crate) fn new(bases: &[AffixiveItemBase], implicits: &[Implicit], base_ix: AffixiveItemBaseIndex, quality: Quality) -> Self {
        let base = &bases[base_ix.0];
        let implicits = base.implicits.iter().map(|ix| implicits[ix.0].clone()).collect();

        Self {
            base_ix,
            implicits,
            prefixes: vec![],
            suffixes: vec![],
            quality,
        }
    }
    pub(crate) fn display(&self, bases: &[AffixiveItemBase]) -> String {
        let mut output = String::new();

        let name: &str = &bases[self.base_ix.0].name;

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

        let mut has_affix = false;

        for prefix in self.prefixes.iter() {
            has_affix = true;
            output.push_str(&prefix.display());
            output.push('\n');
        }

        for suffix in self.suffixes.iter() {
            has_affix = true;
            output.push_str(&suffix.display());
            output.push('\n');
        }

        if has_affix {
            output.push_str("===\n");
        }

        output
    }

    /// Attempt to attach a prefix to this item.
    /// 
    /// Return Ok(()) if the prefix was added.
    pub(crate) fn try_push_prefix(&mut self, prefix: Prefix) -> Result<(), PushAffixError> {
        match self.quality {
            Quality::FixedArtifact => Err(PushAffixError::AffixiveItemIsFixed),
            Quality::Quality(quality) if quality as usize == self.prefixes.len() => Err(PushAffixError::AffixiveItemQualityTooLow),
            _ => {
                self.prefixes.push(prefix);
                Ok(())
            }
        }
    }

    /// Attempt to attach a suffix to this item.
    /// 
    /// Return Ok(()) if the suffix was added.
    pub(crate) fn try_push_suffix(&mut self, suffix: Suffix) -> Result<(), PushAffixError> {
        match self.quality {
            Quality::FixedArtifact => Err(PushAffixError::AffixiveItemIsFixed),
            Quality::Quality(quality) if quality as usize == self.suffixes.len() => Err(PushAffixError::AffixiveItemQualityTooLow),
            _ => {
                self.suffixes.push(suffix);
                Ok(())
            }
        }
    }
}

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

pub struct DropTable<T> {
    total_weight: i32,
    weights: Vec<i32>,
    modifiers: Vec<T>,
}

impl<T: Clone> DropTable<T> {
    pub fn random(&self) -> T {
        let r = rand::thread_rng().gen_range(0..self.total_weight);

        let ix = self.weights.partition_point(|weight| weight <= &r);
        self.modifiers[ix].clone()
    }
}

struct DropTableBuilder<T> {
    total_weight: i32,
    weights: Vec<i32>,
    modifiers: Vec<T>,
}

impl<T: Clone> DropTableBuilder<T> {
    fn new() -> Self {
        Self {
            total_weight: 0,
            weights: vec![],
            modifiers: vec![],
        }
    }

    fn build(self) -> DropTable<T> {
        DropTable {
            total_weight: self.total_weight,
            weights: self.weights,
            modifiers: self.modifiers,
        }
    }

    fn add_loot(mut self, loot: T, weight: i32) -> Self {
        self.modifiers.push(loot);
        self.weights.push(self.total_weight + weight);
        
        Self {
            total_weight: self.total_weight + weight,
            weights: self.weights,
            modifiers: self.modifiers,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SuffixOrPrefix {
    Suffix,
    Prefix,
}

pub(crate) struct InventoryModifierPrefixes(DropTable<Prefix>);

impl Deref for InventoryModifierPrefixes {
    type Target = DropTable<Prefix>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub(crate) struct InventoryModifierSuffixes(DropTable<Suffix>);

impl Deref for InventoryModifierSuffixes {
    type Target = DropTable<Suffix>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct SuffixOrPrefixDropTable(DropTable<SuffixOrPrefix>);

impl Deref for SuffixOrPrefixDropTable {
    type Target = DropTable<SuffixOrPrefix>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub(crate) struct StorageBasesDropTable(DropTable<AffixiveItemBaseIndex>);

impl Deref for StorageBasesDropTable {
    type Target = DropTable<AffixiveItemBaseIndex>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn initialize_drop_tables(bases: &[AffixiveItemBase], prefixes: Vec<Prefix>, suffixes: Vec<Suffix>) -> AnyMap {
    let mut drop_tables = AnyMap::new();

    let mut base_drop_table = DropTableBuilder::new();
    for (ix, _) in bases.iter().enumerate() {
        base_drop_table = base_drop_table.add_loot(AffixiveItemBaseIndex(ix), 50);
    }
    let base_drop_table = base_drop_table.build();

    drop_tables.insert(StorageBasesDropTable(base_drop_table));

    let prefix_drop_table = DropTableBuilder::new()
        .add_loot(prefixes[0].clone(), 50)
        .add_loot(prefixes[1].clone(), 50)
        .add_loot(prefixes[2].clone(), 50)
        .add_loot(prefixes[3].clone(), 50)
        .build();

    drop_tables.insert(InventoryModifierPrefixes(prefix_drop_table));

    let suffix_drop_table = DropTableBuilder::new()
        .add_loot(suffixes[0].clone(), 50)
        .add_loot(suffixes[1].clone(), 50)
        .add_loot(suffixes[2].clone(), 50)
        .add_loot(suffixes[3].clone(), 50)
        .build();

    drop_tables.insert(InventoryModifierSuffixes(suffix_drop_table));

    let prefix_or_suffix_table = DropTableBuilder::new()
        .add_loot(SuffixOrPrefix::Suffix, 1)
        .add_loot(SuffixOrPrefix::Prefix, 1)
        .build();

    drop_tables.insert(SuffixOrPrefixDropTable(prefix_or_suffix_table));

    return drop_tables;
}