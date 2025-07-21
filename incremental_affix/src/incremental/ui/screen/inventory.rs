use bevy::prelude::*;

use crate::incremental::ui::screen::Screen;

#[derive(Debug, Resource)]
pub struct InventoryScreen(Entity);

impl InventoryScreen {
    pub fn get(&self) -> Entity {
        self.0.clone()
    }
}

pub fn spawn_inventory_screen(mut commands: Commands, parent: Entity) {
    let inventory_screen = commands.spawn((
        Node {
            flex_direction: FlexDirection::Column,
            height: Val::Percent(100.0),
            width: Val::Percent(100.0),
            ..default()
        },
        BackgroundColor(Color::srgb_u8(238, 223, 187)),
        Screen::Inventory,
        ChildOf(parent),
    )).id();

    commands.insert_resource(InventoryScreen(inventory_screen));
}