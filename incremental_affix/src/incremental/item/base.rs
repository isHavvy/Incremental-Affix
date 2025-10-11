use std::borrow::Cow;

use bevy::{platform::collections::HashMap, prelude::*};

use crate::incremental::item::affixive_item::{ImplicitIndex, ItemTag};

/// A cheap to produce/store tag to access affixive item bases.
/// 
/// To access an affixive base, get the `Res<ItemDatabase>` and
/// call `ItemDatabase.`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub enum Base {
    MakeshiftTools,
    SecondaryTools,
    StoneTools,
}

impl ToString for Base {
    fn to_string(&self) -> String {
        match self {
            Base::MakeshiftTools => "Makeshift Tools".to_string(),
            Base::SecondaryTools => "Secondary Tools".to_string(),
            Base::StoneTools => "Stone Tools".to_string(),
        }
    }
}

#[derive(Debug)]
pub(crate) struct AffixiveItemBase {
    pub name: Cow<'static, str>,
    pub tags: Vec<ItemTag>,
    pub implicits: Vec<ImplicitIndex>,
}

/// Creates the mapping from `Base`s to their `AffxiviteItemBase` data.
pub fn initialize() -> HashMap<Base, AffixiveItemBase> {
    let mut map = HashMap::new();

    map.insert(Base::MakeshiftTools, AffixiveItemBase {
        name: "Makeshift Tools".into(),
        tags: vec![ItemTag::Tool],
        implicits: vec![
            ImplicitIndex(0),
            ImplicitIndex(1),
        ],
    });

    map.insert(Base::SecondaryTools, AffixiveItemBase {
        name: "Secondary Tools".into(),
        tags: vec![ItemTag::Tool],
        implicits: vec![],
    });

    map.insert(Base::StoneTools, AffixiveItemBase {
        name: "Stone Tools".into(),
        tags: vec![ItemTag::Tool],
        implicits: vec![
            ImplicitIndex(0),
            ImplicitIndex(1),

        ]
    });

    return map;
}