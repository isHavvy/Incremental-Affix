use std::{borrow::Cow, fmt::Display, ops::Deref};

use bevy::{ecs::component::Component, platform::collections::HashMap};

use crate::incremental::item::{base::{AffixiveItemBase, Base}, item_slot::ItemSlotTag, modifier::{Affix, Implicit, Modifier, ModifierValue, Prefix, Suffix}};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Quality {
    /// An item with fixed affixes.
    #[expect(unused)]
    FixedArtifact,

    /// An item with the specific number of prefixes and suffixes.
    /// For example, an item with Quality::Qaulity(2) will have 2
    /// prefixes and 2 suffixes for a total of 4 affixes.
    Quality(u8),
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct ImplicitIndex(pub usize);

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum ItemTag {
    Tool,
    Hunt,
}

impl Display for ItemTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = match self {
            ItemTag::Tool => "Tool",
            ItemTag::Hunt => "Hunting Weapon",
        };

        f.write_str(string)?;
        Ok(())
    }
}

impl From<ItemSlotTag> for ItemTag {
    fn from(slot_tag: ItemSlotTag) -> Self {
        match slot_tag {
            ItemSlotTag::Tool => ItemTag::Tool,
            ItemSlotTag::Hunt => Self::Hunt
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum PushAffixError {
    AffixiveItemIsFixed,
    AffixiveItemQualityTooLow,
}

#[derive(Debug, Component)]
pub(crate) struct AffixiveItem {
    base: Base,
    name: Cow<'static, str>,
    implicits: Vec<Implicit>,
    prefixes: Vec<Prefix>,
    suffixes: Vec<Suffix>,
    quality: Quality,
    pub tags: Vec<ItemTag>,
}

impl AffixiveItem {
    pub(crate) fn new(bases: &HashMap<Base, AffixiveItemBase>, implicits_db: &[Implicit], base: Base, quality: Quality) -> Self {
        let item_base = &bases[&base];

        let mut implicits: Vec<Implicit>= item_base.implicits
            .iter()
            .map(|ix| &implicits_db[ix.0])
            .cloned()
            .collect();

        for implicit in &mut implicits {
            implicit.randomize_actual();
        }

        Self {
            base,
            name: item_base.name.clone(),
            implicits,
            prefixes: vec![],
            suffixes: vec![],
            quality,
            tags: item_base.tags.clone(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn base(&self) -> Base {
        self.base
    }

    pub fn implicits(&self) -> impl Iterator<Item=&Affix> {
        self.implicits.iter().map(Deref::deref)
    }

    pub fn prefixes(&self) -> impl Iterator<Item=&Affix> {
        self.prefixes.iter().map(Deref::deref)
    }

    pub fn suffixes(&self) -> impl Iterator<Item=&Affix> {
        self.suffixes.iter().map(Deref::deref)
    }

    #[allow(unused, reason = "Debug function")]
    pub fn display(&self, bases: &HashMap<Base, AffixiveItemBase>) -> String {
        let mut output = String::new();

        let name: &str = &self.name;

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

    pub fn increase_quality_to(&mut self, new_quality: u8) {
        let Quality::Quality(current_quality) = self.quality else { return; };
        if current_quality < new_quality {
            self.quality = Quality::Quality(new_quality)
        }
    }

    /// Attempt to attach a prefix to this item.
    /// 
    /// Return Ok(()) if the prefix was added.
    pub fn try_push_prefix(&mut self, prefix: Prefix) -> Result<(), PushAffixError> {
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
    pub fn try_push_suffix(&mut self, suffix: Suffix) -> Result<(), PushAffixError> {
        match self.quality {
            Quality::FixedArtifact => Err(PushAffixError::AffixiveItemIsFixed),
            Quality::Quality(quality) if quality as usize == self.suffixes.len() => Err(PushAffixError::AffixiveItemQualityTooLow),
            _ => {
                self.suffixes.push(suffix);
                Ok(())
            }
        }
    }

    pub fn modifiers(&self) -> impl Iterator<Item=(&Modifier, ModifierValue)> {
        self.implicits.iter().map(|implicit| &**implicit)
        .chain({ let x = self.prefixes.iter().map(|prefix| &**prefix); x })
        .chain(self.suffixes.iter().map(|suffix| &**suffix))
        .flat_map(|affix| affix.modifiers())
    }
}
