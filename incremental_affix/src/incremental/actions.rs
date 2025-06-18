use bevy::prelude::{Component, Resource};

#[derive(Debug, Default, Resource)]
pub struct ActionProgress(f32);

#[derive(Debug, Resource)]
pub struct CurrentAction(Actions);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Component)]
pub enum Actions {
    Explore,
    GatherWood,
    CreateFollowers,
}

impl Actions {
    pub const LIST: &[Self] = &[
        Self::Explore,
        Self::GatherWood,
        Self::CreateFollowers,
    ];
}

impl std::fmt::Display for Actions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match *self {
            Self::Explore => "Explore",
            Self::GatherWood => "Gather Wood",
            Self::CreateFollowers => "Create Followers",
        })
    }
}