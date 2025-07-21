use bevy::ecs::{component::Component, resource::Resource};

use crate::engine::item::{AffixiveItem, AffixiveItemBase, AffixiveItemBaseIndex, Modifier, ModifierKind, Quality};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Modifiers {
    WoodBaseGain,
    WoodMultiplier,
    WoodAffinityChanceMultiplier,
    WoodAffinityMultiplier,
    WoodAffinityTimeMultiplier,

    StoneBaseGain,
    StoneMultiplier,
    StoneAffinityChanceMultiplier,
    StoneAffinityMultiplier,
    StoneAffinityTimeMultiplier,
}

impl ModifierKind for Modifiers {
    fn display_actual(&self, actual: i32) -> String {
        fn sign(n: i32) -> char {
            if n > 0 { '+' } else { '-' }
        }

        match *self {
            Modifiers::WoodBaseGain => format!("{}{} Wood chopped per second", sign(actual), actual),
            Modifiers::WoodMultiplier => format!("{}{}% Wood chopped per second", sign(actual), actual),
            Modifiers::WoodAffinityChanceMultiplier => format!("{}{}% Wood affinity chance", sign(actual), actual),
            Modifiers::WoodAffinityMultiplier => format!("{}{}% Wood affinity gain", sign(actual), actual),
            Modifiers::WoodAffinityTimeMultiplier => format!("{}{}% Wood affinity time", sign(actual), actual),

            Modifiers::StoneBaseGain => format!("{}{} Stone mined per second", sign(actual), actual),
            Modifiers::StoneMultiplier => format!("{}{}% Stone mined per second", sign(actual), actual),
            Modifiers::StoneAffinityChanceMultiplier => format!("{}{} Stone affinity chance", sign(actual), actual),
            Modifiers::StoneAffinityMultiplier => format!("{}{}% Stone affinity gain", sign(actual), actual),
            Modifiers::StoneAffinityTimeMultiplier => format!("{}{}% Sone affinity time", sign(actual), actual),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Component)]
pub enum Base {
    MakeshiftTools,
}

fn makeshift_tool() -> AffixiveItemBase {
    AffixiveItemBase {
        name: "Makeshift Tools".to_string(),
        level: 0,
        tags: vec![],
        implicits: vec![],
    }
}

#[derive(Debug, Resource)]
pub struct ItemDatabase {
    bases: Vec<AffixiveItemBase>,
    modifiers: Vec<Modifier<Modifiers>>,
}

impl ItemDatabase {
    pub fn new() -> Self {
        let bases = vec![
            makeshift_tool()
        ];

        let modifiers = vec![];

        Self { bases, modifiers }
    }

    pub fn get_base(&self, base: Base) -> &AffixiveItemBase {
        let ix = match base {
            Base::MakeshiftTools => 0,
        };

        &self.bases[ix]
    }

    pub fn get_base_ix(base: Base) -> AffixiveItemBaseIndex {
        AffixiveItemBaseIndex(match base {
            Base::MakeshiftTools => 0,
        })
    }

    /// Make a new item with no modifiers or modifier slots of the specified base.
    pub fn create_basic(&self, base: Base) -> AffixiveItem<Modifiers> {
        AffixiveItem::new(&self.bases, &[], Self::get_base_ix(base), Quality::Quality(0))
    }

    pub fn item_name(&self, item: &AffixiveItem<Modifiers>) -> &str {
        item.name(&self.bases)
    }
}

impl Default for ItemDatabase {
    fn default() -> Self {
        Self::new()
    }
}