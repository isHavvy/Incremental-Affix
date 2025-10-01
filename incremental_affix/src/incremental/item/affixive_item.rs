use bevy::ecs::component::Component;

use crate::incremental::item::{item_slot::ItemSlotTag, modifier::{Implicit, Modifier, ModifierValue, Prefix, Suffix}};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Quality {
    /// An item with fixed affixes.
    #[expect(unused)]
    FixedArtifact,

    /// An item with the specific number of prefixes and suffixes.
    /// For example, an item with Quality::Qaulity(2) will have 2
    /// prefixes and 2 suffixes for a total of 4 affixes.
    Quality(i8),
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct AffixiveItemBaseIndex(pub usize);

impl AffixiveItemBaseIndex {
    pub fn inner(self) -> usize {
        self.0
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct ImplicitIndex(pub usize);

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum ItemTag {
    Tool,
}

impl From<ItemSlotTag> for ItemTag {
    fn from(slot_tag: ItemSlotTag) -> Self {
        match slot_tag {
            ItemSlotTag::Tool => ItemTag::Tool,
        }
    }
}

#[derive(Debug)]
pub(crate) struct AffixiveItemBase {
    pub name: String,
    pub level: u8,
    pub tags: Vec<ItemTag>,
    pub implicits: Vec<ImplicitIndex>,
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum PushAffixError {
    AffixiveItemIsFixed,
    AffixiveItemQualityTooLow,
}

#[derive(Debug, Component)]
pub(crate) struct AffixiveItem {
    base_ix: AffixiveItemBaseIndex,
    implicits: Vec<Implicit>,
    prefixes: Vec<Prefix>,
    suffixes: Vec<Suffix>,
    quality: Quality,
    pub tags: Vec<ItemTag>,
}

impl AffixiveItem {
    pub(crate) fn new(bases: &[AffixiveItemBase], implicits: &[Implicit], base_ix: AffixiveItemBaseIndex, quality: Quality) -> Self {
        let base = &bases[base_ix.0];
        let mut implicits: Vec<Implicit> = base.implicits.iter().map(|ix| implicits[ix.0].clone()).collect();
        for implicit in &mut implicits {
            implicit.randomize_actual();
        }

        Self {
            base_ix,
            implicits,
            prefixes: vec![],
            suffixes: vec![],
            quality,
            tags: base.tags.clone(),
        }
    }

    pub(crate) fn name<'bases>(&self, bases: &'bases [AffixiveItemBase]) -> &'bases str {
        &bases[self.base_ix.0].name
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

    #[expect(unused)]
    pub fn level(&self, bases: &[AffixiveItemBase]) -> u8 {
        let base = &bases[self.base_ix.inner()];

        base.level
    }

    /// Attempt to attach a prefix to this item.
    /// 
    /// Return Ok(()) if the prefix was added.
    #[expect(unused)]
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
    #[expect(unused)]
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

    #[expect(unused)]
    pub fn modifiers(&self) -> impl Iterator<Item=(&Modifier, ModifierValue)> {
        self.implicits.iter().map(|implicit| &**implicit)
        .chain({ let x = self.prefixes.iter().map(|prefix| &**prefix); x })
        .chain(self.suffixes.iter().map(|suffix| &**suffix))
        .flat_map(|affix| affix.modifiers())
    }
}
