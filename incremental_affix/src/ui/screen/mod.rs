use bevy::ecs::VariantDefaults;
use bevy::prelude::*;
use bevy::text::FontSourceTemplate;
use bevy::ui_widgets::{Activate, Button};

use crate::incremental::action::KnownActions;

pub mod action;
pub mod craft;
pub mod inventory;
pub mod population;

/// Kinds of screens in the game ui
#[derive(Debug, Clone, Copy, PartialEq, Eq, Component, Default, VariantDefaults)]
pub enum Screen {
    #[default] // Needed to use in BSN.
    Act,
    Population,
    Inventory,
    Craft,
}

impl Screen {
    pub const LIST: &[Self] = &[Self::Act, Self::Population, Self::Inventory, Self::Craft];
}

impl std::fmt::Display for Screen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Act => "Act",
            Self::Population => "Population",
            Self::Inventory => "Inventory",
            Self::Craft => "Craft",
        })
    }
}

pub fn screens_ui(
    known_actions: Res<KnownActions>,
) -> (impl Scene, impl Scene) {
    (
        bsn! {
            #ScreenSelectBar
            Node {
                height: px(48),
                width: percent(100),

                align_self: AlignSelf::Center,
            }
            BackgroundColor(Color::srgb(0.0, 0.8, 0.0))
            Children [ { screens_select_bar_buttons() } ]
        },

        bsn! {
            #ScreenContainer
            Node {
                flex_grow: 1.0,
                overflow: Overflow::scroll_y(),

                padding: UiRect { left: px(10), right: px(0), top: px(10), bottom: px(0) },
            }

            Children [
                action::actions_screen(known_actions),
                craft::crafting_screen(),
                inventory::inventory_screen(),
                population::population_screen(),
            ]
        }
    )
}

fn screens_select_bar_buttons() -> impl SceneList {
    Screen::LIST.iter().cloned()
    .map(|screen| bsn! {
        Node {
            height: px(40),

            border: { px(2).all() },
            margin: { px(5).right() },

            align_content: AlignContent::Center,
        }
        BorderColor::all(Color::BLACK)

        Button
        template_value(screen)
        on(on_activate_button_screen_change)

        Children [
            Text({ screen.to_string() })
            TextColor(Color::WHITE)
            TextFont {
                font: FontSourceTemplate::Handle("fonts/FiraSans-Bold.ttf"),
                font_size: FontSize::Px(20.0),
            },
        ]
    })
    .collect::<Vec<_>>()
}

pub fn on_activate_button_screen_change(
    activate: On<Activate>,
    screen_query: Query<&Screen>,
    mut screen_node_query: Query<(&mut Node, &Screen), Without<Button>>,
) {
    let next_visible_screen = screen_query.get(activate.entity).expect("Screen button must have a screen entity.");

    for (mut screen_node, screen) in &mut screen_node_query {
        screen_node.display = if screen == next_visible_screen { Display::Block } else { Display::None };
    }
}

fn screen_title(title: &'static str) -> impl Scene {
    bsn! {
        Text::new(title)
        TextColor::BLACK
        TextFont { font_size: px(32.0) }
    }
}