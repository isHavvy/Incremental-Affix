use bevy::ui::InteractionDisabled;
use bevy::{prelude::*, ui_widgets::Activate};
use bevy::ui_widgets::{observe, Button};

use crate::incremental::action::{CanChop, CanMine, KnownActions, LearnAction};
use crate::incremental::{action::{ActionProgress, Action, CurrentAction}, ui::screen::Screen};

#[derive(Debug, Default, Resource)]
pub struct ActionProgressBar(Option<Entity>);

pub fn initialize_actions_screen(
    mut commands: Commands,

    container: Entity,

    action_progress: Res<ActionProgress>,
    known_actions: Res<KnownActions>,
    mut res_action_progress_bar: ResMut<ActionProgressBar>,
) {
    let screen = commands.spawn((
        Node {
            flex_direction: FlexDirection::Column,
            flex_grow: 1.,

            ..default()
        },
        Screen::Act,
        ChildOf(container)
    )).id();

    let action_progress_bar_outer = commands.spawn((
        Node {
            box_sizing: BoxSizing::BorderBox,
            height: px(25),
            width: px(400),

            margin: px(8).all(),
            border: px(2).all(),

            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::Center,

            ..default()
        },
        BackgroundColor(Color::srgb_u8(255, 255, 255)),
        BorderColor::all(Color::BLACK),
        ChildOf(screen),
    )).id();

    let action_progress_bar = commands.spawn((
        Node {
            height: Val::Percent(100.0),
            width: Val::Percent(action_progress.0),
            align_content: AlignContent::Center,
            ..default()
        },
        BackgroundColor(Color::srgb(1.0, 0.0, 0.0).into()),
        Text::new(""),
        ChildOf(action_progress_bar_outer),
    )).id();

    *res_action_progress_bar = ActionProgressBar(Some(action_progress_bar));

    for action in Action::LIST.iter().copied() {
        spawn_action_button(action, commands.reborrow(), &known_actions, screen);
    }
}

fn spawn_action_button(
    action: Action,

    mut commands: Commands,

    known_actions: &KnownActions,

    container: Entity,
) {
    commands.spawn((
        Node {
            display: if known_actions.contains(&action) { Display::Flex } else { Display::None },
            border: UiRect::all(Val::Px(2.)),
            height: Val::Px(25.0),
            width: Val::Px(200.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            margin: UiRect::all(Val::Px(8.0)),
            ..default()
        },
        BorderColor::all(Color::BLACK),

        action,

        children![(
            Button,
            InteractionDisabled,
            observe(on_press_button_action),
            action,
            Text::new(action.to_string()),
            TextFont {
                font_size: 20.0,
                ..default()
            },
            TextColor(Color::BLACK),
        )],

        ChildOf(container),
    ));
}

pub fn update_action_progress_bar(
    progress: Res<ActionProgress>,
    progress_bar: Res<ActionProgressBar>,
    mut progress_bar_style_query: Query<&mut Node>,
) {
    if let Some(progress_bar) = progress_bar.0 {
        let Ok(mut node) = progress_bar_style_query.get_mut(progress_bar) else { panic!("Progress bar entity must have a style.")};

        node.width = Val::Px(400.0 * progress.0);
    }
}

fn on_press_button_action(
    activate: On<Activate>,
    actions_query: Query<&Action>,
    mut current_action: ResMut<CurrentAction>,
    mut action_progress: ResMut<ActionProgress>,
    can_mine: Res<CanMine>,
    can_chop: Res<CanChop>,
) {
    let new_action = actions_query.get(activate.entity).expect("Action button must have an action entity.");

    match new_action {
        Action::Explore => {
            // Don't reset progress if they re-clicked on the same action.
            if current_action.0 == Some(Action::Explore) {
                return;
            }

            current_action.0 = Some(Action::Explore);
            action_progress.0 = 0.0
        },
        Action::GatherWood => {
            // Don't reset progress if they re-clicked on the same action.
            if current_action.0 == Some(Action::GatherWood) {
                return;
            }

            if !**can_chop {
                return;
            }

            current_action.0 = Some(Action::GatherWood);
            action_progress.0 = 0.0;
        },
        Action::GatherStone => {
            // Don't reset progress if they re-clicked on the same action.
            if current_action.0 == Some(Action::GatherStone) {
                return;
            }

            if !**can_mine {
                return;
            }

            current_action.0 = Some(Action::GatherStone);
            action_progress.0 = 0.0;
        }
        Action::CreateFollowers => todo!(),
    }
}

pub fn on_learn_action(
    event: On<LearnAction>,

    action_container_query: Query<(&Action, &mut Node)>,
) {
    action_container_query
    .into_iter()
    .find(|(action, _)| **action == event.action)
    .map(|(_, mut node)| node.display = Display::Flex);
}