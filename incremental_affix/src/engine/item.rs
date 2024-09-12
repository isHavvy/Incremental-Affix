use std::ops::{Deref, DerefMut};

use rand::Rng;

#[derive(Debug, Clone, Copy)]
pub(crate) enum ModifierKind {
    InventoryBase,
    InventoryHeight,
    IncreasedVolume,
    InventorySkillGain,
}

#[derive(Debug, Clone, Copy)]
pub struct Modifier {
    pub kind: ModifierKind,
    pub min: i32,
    pub max: i32,
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
    pub name: String,
    pub modifier: Modifier,
    pub modifier_actual: i32,
    pub hybrid_modifier: Option<Modifier>,
    pub hybrid_modifier_actual: i32,
}

impl Affix {
    /// Construct a new affix with the given modifier. Sets the hybrid to `None`.
    pub fn new(name: String, modifier: Modifier) -> Self {
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
pub(crate) struct Implicit(pub Affix);

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
pub(crate) struct Prefix(pub Affix);

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
pub(crate) struct Suffix(pub Affix);

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
pub(crate) struct AffixiveItemBaseIndex(pub usize);

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct ImplicitIndex(pub usize);

#[derive(Debug, Clone, Copy)]
pub enum AffixiveItemBaseTag {
    Inventory,
}

#[derive(Debug)]
pub(crate) struct AffixiveItemBase {
    pub name: String,
    pub tags: Vec<AffixiveItemBaseTag>,
    pub implicits: Vec<ImplicitIndex>,
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
