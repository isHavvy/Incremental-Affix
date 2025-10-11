use bevy::color::palettes::css::GRAY;
use bevy::ui::InteractionDisabled;
use bevy::ui_widgets::Activate;
use bevy::prelude::*;
use bevy::ui_widgets::{observe, Button};

use crate::incremental::action::{Action, ActionProgress, CanChop, CanMine, CurrentAction, KnownActions, LearnAction};
use crate::ui::screen::Screen;

const BUTTON_ENABLED_COLOR: Color = Color::BLACK;
const BUTTON_DISABLED_COLOR: Color = Color::Srgba(GRAY);

pub struct ActionScreenPlugin;

impl Plugin for ActionScreenPlugin {
    fn build(&self, app: &mut App) {
        app

        .init_resource::<ActionProgressBar>()

        .add_systems(Update, (
            update_action_progress_bar,
            on_changed_can_mine_system,
            on_changed_can_chop_system
        ))

        .add_observer(on_learn_action)

        ;
    }
}

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
    let action_is_known = known_actions.contains(&action);

    let mut button_container = commands.spawn((
        Node {
            display: if action_is_known { Display::Flex } else { Display::None },
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

        Button,
        observe(on_press_button_action),

        ChildOf(container),
    ));

    if !action_is_known {
        button_container.insert(InteractionDisabled);
    }

    let button_container = button_container.id();

    commands.spawn((
        Text::new(action.to_string()),
        TextFont {
            font_size: 20.0,
            ..default()
        },
        TextColor(if action_is_known { BUTTON_ENABLED_COLOR } else { BUTTON_DISABLED_COLOR }),

        ChildOf(button_container),
    ));
}

fn update_action_progress_bar(
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

fn on_learn_action(
    event: On<LearnAction>,

    action_container_query: Query<(&Action, &mut Node)>,
) {
    eprintln!("Action learned.");
    action_container_query
    .into_iter()
    .find(|(action, _)| **action == event.action)
    .map(|(_, mut node)| { node.display = Display::Flex; });
}

fn on_changed_can_mine_system(
    mut commands: Commands,

    can_mine: Res<CanMine>,

    action_container_query: Query<(Entity, &Action, &Children), With<Node>>,
    mut text_color_query: Query<&mut TextColor>,
) {
    if !can_mine.is_changed() {
        return;
    }

    action_container_query.iter()
    .find(|(_entity, action, _children,)| **action == Action::GatherStone)
    .map(|(entity, _action, children)| {
        if can_mine.0 {
            commands.entity(entity)
            .remove::<InteractionDisabled>();

            text_color_query.get_mut(children[0]).unwrap().0 = BUTTON_ENABLED_COLOR;
        } else {
            commands.entity(entity)
            .insert(InteractionDisabled);

            text_color_query.get_mut(children[0]).unwrap().0 = BUTTON_DISABLED_COLOR;
        }
    });
}

fn on_changed_can_chop_system(
    mut commands: Commands,

    can_chop: Res<CanChop>,

    action_container_query: Query<(Entity, &Action, &Children), With<Node>>,
    mut text_color_query: Query<&mut TextColor>,
) {
    if !can_chop.is_changed() {
        return;
    }

    action_container_query.iter()
    .find(|(_entity, action, _children,)| **action == Action::GatherWood)
    .map(|(entity, _action, children)| {
        if can_chop.0 {
            commands.entity(entity)
            .remove::<InteractionDisabled>();

            text_color_query.get_mut(children[0]).unwrap().0 = BUTTON_ENABLED_COLOR;
        } else {
            commands.entity(entity)
            .insert(InteractionDisabled);

            text_color_query.get_mut(children[0]).unwrap().0 = BUTTON_DISABLED_COLOR;
        }
    });
}