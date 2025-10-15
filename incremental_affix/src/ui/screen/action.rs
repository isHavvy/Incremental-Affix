use std::fmt::Write as _;

use bevy::color::palettes::css::{self, GRAY};
use bevy::ui::InteractionDisabled;
use bevy::ui_widgets::Activate;
use bevy::prelude::*;
use bevy::ui_widgets::{observe, Button};

use crate::incremental::action::{Action, ActionAffinity, ActionProgress, ChangeAction, CurrentAction, KnownActions, LearnAction, NO_CURRENT_ACTION_DISPLAY};
use crate::incremental::stats::PlayerActionsStats;
use crate::ui::screen::Screen;

const BUTTON_ENABLED_COLOR: Color = Color::BLACK;
const BUTTON_DISABLED_COLOR: Color = Color::Srgba(GRAY);
const ACTION_BAR_WIDTH: Val = Val::Px(400.0);

pub struct ActionScreenPlugin;

impl Plugin for ActionScreenPlugin {
    fn build(&self, app: &mut App) {
        app

        .add_systems(Update, (
            update_action_bar_progress_bar,
            update_action_bar_affinity_bar,
            on_changed_player_stats_system,
            on_current_action_change_system,
        ))

        .add_observer(on_learn_action)
        ;
    }
}

#[derive(Debug, Resource)]
struct ActionProgressBar {
    /// The bar that fills up as action progress occurs.
    progress_bar: Entity,

    /// The bar that fills down as affinity time is used up.
    affinity_bar: Entity,

    /// The text node inside the action bar.
    text: Entity,
}

pub fn initialize_actions_screen(
    mut commands: Commands,
    container: Entity,
    known_actions: Res<KnownActions>,
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

    spawn_action_bar(commands.reborrow(), screen);

    for action in Action::LIST.iter().copied() {
        spawn_action_button(action, commands.reborrow(), &known_actions, screen);
    }
}

fn spawn_action_bar(
    mut commands: Commands,
    container: Entity,
) {
    let outer = commands.spawn((
        Node {
            box_sizing: BoxSizing::ContentBox,
            height: px(21),
            width: ACTION_BAR_WIDTH,

            border: px(2).all(),

            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::Center,

            ..default()
        },
        BackgroundColor(Color::WHITE),
        BorderColor::all(Color::BLACK),
        ChildOf(container),
    )).id();

    let progress_bar = commands.spawn((
        Node {
            width: percent(0),
            height: percent(100),

            align_content: AlignContent::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        BackgroundColor(Color::srgb(1.0, 0.0, 0.0)),
        ZIndex(0),

        ChildOf(outer),
    )).id();

    let affinity_bar = commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            width: ACTION_BAR_WIDTH,
            top: percent(67),
            height: percent(33),
            ..default()
        },
        BackgroundColor(css::LIMEGREEN.into()),
        ZIndex(1),

        ChildOf(outer),
    )).id();

    let text = commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            width: ACTION_BAR_WIDTH,
            ..default()
        },
        ZIndex(2),

        Text::new(NO_CURRENT_ACTION_DISPLAY),
        TextColor(Color::BLACK),
        TextLayout {
            justify: Justify::Center,
            ..default()
        },

        ChildOf(outer),
    )).id();

    commands.insert_resource(ActionProgressBar {
        text,
        progress_bar,
        affinity_bar,
    });
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

fn update_action_bar_progress_bar(
    progress: Res<ActionProgress>,
    progress_bar: Res<ActionProgressBar>,
    mut node_query: Query<&mut Node>,
) {
    let progress_bar = progress_bar.progress_bar;
    let mut node = node_query.get_mut(progress_bar).expect("Progress bar entity must have a Node component.");
    node.width = ACTION_BAR_WIDTH * progress.0;
}

fn on_current_action_change_system(
    current_action: Res<CurrentAction>,
    progress_bar: Res<ActionProgressBar>,

    mut text_query: Query<&mut Text>,
    mut node_query: Query<&mut Node>,
) {
    if !current_action.is_changed() {
        return;
    }

    let mut node = node_query.get_mut(progress_bar.affinity_bar).expect("Affinity bar entity must have a Node component.");
    node.width = percent(0);

    let mut text = text_query.get_mut(progress_bar.text).expect("Progress bar text entity must have a Text component.");
    text.clear();
    let _ = write!(text.0, "{}", *current_action);
}

fn on_press_button_action(
    activate: On<Activate>,
    mut commands: Commands,
    actions_query: Query<&Action>,
) {
    let new_action = actions_query.get(activate.entity).expect("Action button must have an Action component.");
    commands.trigger(ChangeAction::new(*new_action));    
}

fn on_learn_action(
    event: On<LearnAction>,

    action_container_query: Query<(&Action, &mut Node)>,
) {
    action_container_query
    .into_iter()
    .find(|(action, _)| **action == event.action)
    .map(|(_, mut node)| { node.display = Display::Flex; });
}

// #[TODO(Havvy)]: Instead of checking every time the player stats change,
//                 have the player stats fire events when the base goes to or from zero.
fn on_changed_player_stats_system(
    mut commands: Commands,

    player_actions_bonuses: Res<PlayerActionsStats>,

    action_container_query: Query<(Entity, &Action, &Children), With<Node>>,
    mut text_color_query: Query<&mut TextColor>,
) {
    if !player_actions_bonuses.is_changed() {
        return;
    }

    action_container_query.iter()
    .filter_map(|(entity, action, children)| {
        player_actions_bonuses
        .get_bonuses(*action)
        .map(|bonuses| bonuses.has_base_gain())
        .map(|enabled| (entity, children, enabled))
    })
    .for_each(|(entity, children, enabled)| {
        let text_color = &mut text_color_query.get_mut(children[0])
        .expect("Action button should have one child with a TextColor component.")
        .0;

        if enabled {
            commands.entity(entity)
            .remove::<InteractionDisabled>();

            *text_color = BUTTON_ENABLED_COLOR;
        } else {
            commands.entity(entity)
            .insert(InteractionDisabled);

            *text_color = BUTTON_DISABLED_COLOR;
        }
    });
}

fn update_action_bar_affinity_bar(
    action_bar: Res<ActionProgressBar>,
    action_affinity: Res<ActionAffinity>,

    mut node_query: Query<&mut Node>,
) {
    let percent = action_affinity.time_left().as_secs_f32() / 5.0;

    let mut affinity_bar_node = node_query.get_mut(action_bar.affinity_bar).expect("Affinity bar entity must have a Node component.");
    affinity_bar_node.width = ACTION_BAR_WIDTH * percent;
}