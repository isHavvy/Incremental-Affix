use bevy::prelude::*;

pub mod action;
pub mod craft;
pub mod inventory;

/// Kinds of screens in the game
#[derive(Debug, Clone, Copy, PartialEq, Eq, Component)]
pub enum Screen {
    Act,
    Population,
    Inventory,
    Craft,
}

impl Screen {
    pub const LIST: &[Self] = &[Self::Act, Self::Population, Self::Inventory, Self::Craft];
}

impl ToString for Screen {
    fn to_string(&self) -> String {
        match self {
            Self::Act => "Act".into(),
            Self::Population => "Population".into(),
            Self::Inventory => "Inventory".into(),
            Self::Craft => "Craft".into(),
        }
    }
}

pub fn handle_screen_click(
    button_query: Query<(&Interaction, &Screen), (Changed<Interaction>, With<Button>)>,
    mut screen_query: Query<(&mut Node, &Screen), Without<Button>>,
) {
    for (interaction, next_visible_screen) in &button_query {
        match interaction {
            Interaction::Pressed => {},
            Interaction::Hovered => continue,
            Interaction::None => continue,
        }

        for (mut screen_node, screen) in &mut screen_query {
            screen_node.display = if screen == next_visible_screen { Display::Block } else { Display::None };
        }
    }
}