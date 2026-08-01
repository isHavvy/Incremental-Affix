use std::fmt::Write as _;

use bevy::color::palettes::css::{self, GRAY};
use bevy::ui::InteractionDisabled;
use bevy::ui_widgets::Activate;
use bevy::prelude::*;
use bevy::ui_widgets::Button;

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

#[derive(Debug, Component, FromTemplate)]
struct ActionProgressBar {
    /// The bar that fills up as action progress occurs.
    progress_bar: Entity,

    /// The bar that fills down as affinity time is used up.
    affinity_bar: Entity,

    /// The text node inside the action bar.
    text: Entity,
}

pub fn actions_screen(known_actions: Res<KnownActions>) -> impl Scene {
    let action_buttons = Action::LIST.iter()
    .copied()
    .map(|action| (action, known_actions.contains(&action)))
    .map(action_button)
    .collect::<Vec<_>>();

    bsn!{
        Node {
            flex_direction: FlexDirection::Column,
            flex_grow: 1.0
        }
        Screen::Act

        Children [
            action_bar(),
            { action_buttons }
        ]
    }
}

fn action_bar() -> impl Scene {
    bsn!{
        Node {
            box_sizing: BoxSizing::ContentBox,
            height: px(21),
            width: ACTION_BAR_WIDTH,

            border: px(2),

            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::Center
        }
        BackgroundColor(Color::WHITE)
        BorderColor::all(Color::BLACK)

        ActionProgressBar {
            progress_bar: #ProgressBar,
            affinity_bar: #AffinityBar,
            text: #Text
        }

        Children [
            #ProgressBar
            Node {
                width: percent(0),
                height: percent(100),

                align_content: AlignContent::Center,
                justify_content: JustifyContent::Center,
            }
            BackgroundColor(Color::srgb(1.0, 0.0, 0.0))
            ZIndex(0),

            #AffinityBar
            Node {
                position_type: PositionType::Absolute,
                width: ACTION_BAR_WIDTH,
                top: percent(67),
                height: percent(33),
            }
            BackgroundColor(css::LIMEGREEN)
            ZIndex(1),

            #Text
            Node {
                position_type: PositionType::Absolute,
                width: ACTION_BAR_WIDTH
            }
            ZIndex(2)
            Text::new(NO_CURRENT_ACTION_DISPLAY)
            TextColor(Color::BLACK)
            TextLayout::justify(Justify::Center)
        ]
    }
}

fn action_button((action, action_is_known): (Action, bool)) -> impl Scene {
    let scene = bsn! {
        Node {
            display: { if action_is_known { Display::Flex } else { Display::None } },
            border: UiRect::all(Val::Px(2.)),
            height: Val::Px(25.0),
            width: Val::Px(200.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            margin: UiRect::all(Val::Px(8.0)),
        }
        BorderColor::all(Color::BLACK)

        template_value(action)

        Button
        on(on_press_button_action)

        Children [
            Text::new(action.to_string())
            TextFont { font_size: px(20.0) }
            TextColor({ if action_is_known { BUTTON_ENABLED_COLOR } else { BUTTON_DISABLED_COLOR } })
        ]
    };

    if !action_is_known {
        Box::new(bsn! {
            scene
            InteractionDisabled
        }) as Box<dyn Scene>
    } else {
        Box::new(scene) as Box<dyn Scene>
    }
}

fn update_action_bar_progress_bar(
    progress: Res<ActionProgress>,
    progress_bar: Single<&ActionProgressBar>,
    mut node_query: Query<&mut Node>,
) {
    let progress_bar = progress_bar.progress_bar;
    let mut node = node_query.get_mut(progress_bar).expect("Progress bar entity must have a Node component.");
    node.width = ACTION_BAR_WIDTH * progress.percent;
}

fn on_current_action_change_system(
    current_action: Res<CurrentAction>,
    progress_bar: Single<&ActionProgressBar>,

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
    action_bar: Single<&ActionProgressBar>,
    action_affinity: Res<ActionAffinity>,

    mut node_query: Query<&mut Node>,
) {
    let percent = action_affinity.time_left().as_secs_f32() / 5.0;

    let mut affinity_bar_node = node_query.get_mut(action_bar.affinity_bar).expect("Affinity bar entity must have a Node component.");
    affinity_bar_node.width = ACTION_BAR_WIDTH * percent;
}