use bevy::prelude::Component;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Component)]
pub enum Screens {
    Act,
    Population,
    Inventory,
    Craft,
}

impl Screens {
    pub const LIST: &[Self] = &[Self::Act, Self::Population, Self::Inventory, Self::Craft];
}

impl ToString for Screens {
    fn to_string(&self) -> String {
        match self {
            Self::Act => "Act".into(),
            Self::Population => "Population".into(),
            Self::Inventory => "Inventory".into(),
            Self::Craft => "Craft".into(),
        }
    }
}