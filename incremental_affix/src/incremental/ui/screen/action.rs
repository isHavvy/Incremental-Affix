use bevy::{prelude::*, ui_widgets::Activate};
use bevy::ui_widgets::{observe, Button};

use crate::incremental::{action::{ActionProgress, Actions, CurrentAction}, ui::screen::Screen};

#[derive(Debug, Default, Resource)]
pub struct ActionProgressBar(Option<Entity>);

pub fn initialize_actions_screen(
    mut commands: Commands,
    parent: Entity,
    font: Handle<Font>,
    action_progress: &ActionProgress,
    mut res_action_progress_bar: ResMut<ActionProgressBar>,
) {
    let screen = commands.spawn((
        Node {
            flex_direction: FlexDirection::Column,
            flex_grow: 1.,

            ..default()
        },
        Screen::Act,
        ChildOf(parent)
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

    for action in Actions::LIST.iter().copied() {
        commands.spawn((
            Node {
                border: UiRect::all(Val::Px(2.)),
                height: Val::Px(25.0),
                width: Val::Px(200.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect::all(Val::Px(8.0)),
                ..default()
            },
            BorderColor::all(Color::BLACK),
            ChildOf(screen),
            children![(
                Button,
                observe(on_press_button_action),
                action,
                Text::new(action.to_string()),
                TextFont {
                    font: font.clone(),
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::BLACK),
            )],
        ));
    }
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
    actions_query: Query<&Actions>,
    mut current_action: ResMut<CurrentAction>,
    mut action_progress: ResMut<ActionProgress>,
) {
    let new_action = actions_query.get(activate.entity).expect("Action button must have an action entity.");

    match new_action {
        Actions::Explore => {
            // Don't reset progress if they re-clicked on the same action.
            if current_action.0 == Some(Actions::Explore) {
                return;
            }
            
            current_action.0 = Some(Actions::Explore);
            action_progress.0 = 0.0
        },
        Actions::GatherWood => {
            // Don't reset progress if they re-clicked on the same action.
            if current_action.0 == Some(Actions::GatherWood) {
                return;
            }

            current_action.0 = Some(Actions::GatherWood);
            action_progress.0 = 0.0;
        },
        Actions::GatherStone => {
            // Don't reset progress if they re-clicked on the same action.
            if current_action.0 == Some(Actions::GatherStone) {
                return;
            }

            current_action.0 = Some(Actions::GatherStone);
            action_progress.0 = 0.0;
        }
        Actions::CreateFollowers => todo!(),
    }
}