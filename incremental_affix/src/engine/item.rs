use std::ops::{Deref, DerefMut};

use bevy::ecs::component::Component;
use rand::Rng;

pub trait ModifierKind: Clone + Copy + 'static {
    fn display_actual(&self, actual: i32) -> String;
}

pub type ModifierValue = i32;

#[derive(Debug, Clone, Copy)]
pub struct Modifier<MK> where MK: ModifierKind {
    pub kind: MK,
    pub min: ModifierValue,
    pub max: ModifierValue,
}

impl<MK: ModifierKind> Modifier<MK> {
    fn display_actual(&self, actual: ModifierValue) -> String {
        self.kind.display_actual(actual)
    }

    fn random_modifier_value(&self) -> ModifierValue {
        rand::thread_rng().gen_range(self.min..self.max + 1)
    }
}

#[derive(Debug, Clone)]
pub struct Affix<MK> where MK: ModifierKind {
    #[expect(unused)] // Have not implemented deeper inspection.
    pub name: String,
    pub modifier: Modifier<MK>,
    pub modifier_actual: ModifierValue,
    pub hybrid_modifier: Option<Modifier<MK>>,
    pub hybrid_modifier_actual: ModifierValue,
}

impl<MK: ModifierKind> Affix<MK> {
    /// Construct a new affix with the given modifier. Sets the hybrid to `None`.
    pub fn new(name: String, modifier: Modifier<MK>) -> Self {
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

    pub fn randomize_actual(&mut self) {
        self.modifier_actual = self.modifier.random_modifier_value();
        if let Some(modifier) = self.hybrid_modifier {
            self.hybrid_modifier_actual = modifier.random_modifier_value();
        }
    }

    pub fn modifiers(&self) -> impl Iterator<Item=(&Modifier<MK>, ModifierValue)> {
        [
            Some((&self.modifier, self.modifier_actual)),
            (self.hybrid_modifier.as_ref().map(|hybrid| (hybrid, self.hybrid_modifier_actual)))
        ].into_iter().filter_map(|x| x) }
}

#[derive(Debug, Clone)]
pub(crate) struct Implicit<MK: ModifierKind>(pub Affix<MK>);

impl<MK> Deref for Implicit<MK> where MK: ModifierKind {
    type Target = Affix<MK>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<MK> DerefMut for Implicit<MK> where MK: ModifierKind {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Prefix<MK: ModifierKind>(pub Affix<MK>);

impl<MK> Deref for Prefix<MK> where MK: ModifierKind {
    type Target = Affix<MK>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<MK> DerefMut for Prefix<MK> where MK: ModifierKind {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Suffix<MK: ModifierKind>(pub Affix<MK>);

impl<MK> Deref for Suffix<MK> where MK: ModifierKind {
    type Target = Affix<MK>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<MK> DerefMut for Suffix<MK> where MK: ModifierKind {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

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

// TODO(Havvy): This should be in game, not engine!
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AffixiveItemTag {
    Inventory,

    Armor,
    Footwear,
}

#[expect(unused)]
pub struct AffixiveItemBaseTagMap<T> {
    pub inventory: T,

    pub armor: T,
    pub footwear: T,
}

#[derive(Debug)]
pub(crate) struct AffixiveItemBase {
    pub name: String,
    pub level: u8,
    pub tags: Vec<AffixiveItemTag>,
    pub implicits: Vec<ImplicitIndex>,
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum PushAffixError {
    AffixiveItemIsFixed,
    AffixiveItemQualityTooLow,
}

#[derive(Debug, Component)]
pub(crate) struct AffixiveItem<MK: ModifierKind> {
    base_ix: AffixiveItemBaseIndex,
    implicits: Vec<Implicit<MK>>,
    prefixes: Vec<Prefix<MK>>,
    suffixes: Vec<Suffix<MK>>,
    quality: Quality,
    pub tags: Vec<AffixiveItemTag>,
}

impl<MK> AffixiveItem<MK> where MK: ModifierKind {
    pub(crate) fn new(bases: &[AffixiveItemBase], implicits: &[Implicit<MK>], base_ix: AffixiveItemBaseIndex, quality: Quality) -> Self {
        let base = &bases[base_ix.0];
        let mut implicits: Vec<Implicit<MK>> = base.implicits.iter().map(|ix| implicits[ix.0].clone()).collect();
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

    pub fn level(&self, bases: &[AffixiveItemBase]) -> u8 {
        let base = &bases[self.base_ix.inner()];

        base.level
    }

    /// Attempt to attach a prefix to this item.
    /// 
    /// Return Ok(()) if the prefix was added.
    pub fn try_push_prefix(&mut self, prefix: Prefix<MK>) -> Result<(), PushAffixError> {
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
    pub fn try_push_suffix(&mut self, suffix: Suffix<MK>) -> Result<(), PushAffixError> {
        match self.quality {
            Quality::FixedArtifact => Err(PushAffixError::AffixiveItemIsFixed),
            Quality::Quality(quality) if quality as usize == self.suffixes.len() => Err(PushAffixError::AffixiveItemQualityTooLow),
            _ => {
                self.suffixes.push(suffix);
                Ok(())
            }
        }
    }

    pub fn modifiers(&self) -> impl Iterator<Item=(&Modifier<MK>, ModifierValue)> {
        self.implicits.iter().map(|implicit| &**implicit)
        .chain({ let x = self.prefixes.iter().map(|prefix| &**prefix); x })
        .chain(self.suffixes.iter().map(|suffix| &**suffix))
        .flat_map(|affix| affix.modifiers())
    }
}
