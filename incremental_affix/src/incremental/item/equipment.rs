use bevy::prelude::*;

#[derive(Debug, Default, Resource)]
pub struct Slots {
    #[expect(unused)]
    tools: Option<Entity>,
}