use bevy::ecs::entity_disabling::Disabled;
use bevy::ecs::system::entity_command::observe;
use bevy::prelude::*;
use bevy::ui_widgets::{Activate, Button};

use crate::incremental::job::{AssignFollower, JobKind};
use crate::ui::screen::Screen;

pub fn spawn_population_screen(mut commands: Commands, screen_container: Entity) {
    let screen = commands.spawn((
        Node {
            display: Display::None,

            flex_direction: FlexDirection::Column,
            flex_grow: 1.,

            //height: percent(100.0),
            ..default()
        },
        Screen::Population,
        ChildOf(screen_container),
    )).id();

    JobKind::LIST.iter().copied().for_each(|job| spawn_job_row(commands.reborrow(), job, screen))
}

fn spawn_job_row(
    mut commands: Commands,
    job_kind: JobKind,
    parent: Entity,
) {
    let job_row = commands.spawn((
        Node {
            flex_direction: FlexDirection::Row,

            ..default()
        },

        job_kind,
        
        ChildOf(parent),
    )).id();

    if matches!(job_kind, JobKind::RenderCarcass) {
        commands.entity(job_row).insert(Disabled);
    }

    commands.spawn((
        Node {
            margin: px(4).right(),
            ..default()
        },
        Text::new("0"),
        TextColor::BLACK,

        ChildOf(job_row),
    ));

    commands.spawn((
        Node {
            margin: px(4).right(),
            ..default()
        },
        Text::new(job_kind.to_string()),
        TextColor::BLACK,

        ChildOf(job_row),
    ));

    commands.spawn((
        Node {
            border: px(1).all(),
            margin: px(4).right(),

            ..default()
        },
        BorderColor::all(Color::BLACK),

        Button,
        // Activate Observer.

        children![(
            Text::new("-"),
            TextColor::BLACK,
        )],
        ChildOf(job_row),
    ));

    commands.spawn((
        Node {
            border: px(1).all(),

            ..default()
        },
        BorderColor::all(Color::BLACK),

        Button,
        // observe(handle_plus_activate),

        children![(
            Text::new("+"),
            TextColor::BLACK,
        )],
        ChildOf(job_row),
    ));

}

fn handle_plus_activate(
    event: On<Activate>,

    mut commands: Commands,

    parent_query: Query<&ChildOf>,
    job_kind_query: Query<&JobKind, With<Node>>
) {
    let job_row = parent_query.get(event.entity).unwrap().0;
    let job_kind = *job_kind_query.get(job_row).unwrap();

    commands.trigger(AssignFollower {
        job_kind,
    });
}