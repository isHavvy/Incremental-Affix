use bevy::prelude::*;

use crate::incremental::item::affixive_item::{AffixiveItemBase, ImplicitIndex, ItemTag};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Component)]
pub enum Base {
    MakeshiftTools,
    SecondaryTools,
}

impl ToString for Base {
    fn to_string(&self) -> String {
        match self {
            Base::MakeshiftTools => "Makeshift Tools".to_string(),
            Base::SecondaryTools => "Secondary Tools".to_string(),
        }
    }
}

pub fn makeshift_tools() -> AffixiveItemBase {
    AffixiveItemBase {
        name: "Makeshift Tools".to_string(),
        level: 0,
        tags: vec![ItemTag::Tool],
        implicits: vec![
            ImplicitIndex(0),
            ImplicitIndex(1),
        ],
    }
}

pub fn secondary_tools() -> AffixiveItemBase {
    AffixiveItemBase {
        name: "Secondary Tools".to_string(),
        level: 0,
        tags: vec![ItemTag::Tool],
        implicits: vec![],
    }
}