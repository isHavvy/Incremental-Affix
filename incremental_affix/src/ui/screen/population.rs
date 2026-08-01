use bevy::ecs::entity_disabling::Disabled;
use bevy::prelude::*;
use bevy::ui_widgets::{Activate, Button};

use crate::incremental::job::{AssignFollower, JobKind, UnassignFollower};
use crate::ui::screen::Screen;

pub fn population_screen() -> impl Scene {
    bsn! {
        Node {
            display: Display::None,

            flex_direction: FlexDirection::Column,
            flex_grow: 1.0
        }
        Screen::Population

        Children [
            { JobKind::LIST.iter().copied().map(job_row).collect::<Vec<_>>() }
        ]
    }
}

fn job_row (job_kind: JobKind) -> impl Scene {
    let job_row = bsn! {
        Node { flex_direction: FlexDirection::Row }
        template_value(job_kind)

        Children[
            Node { margin: { px(4).right() } }
            Text::new("0")
            TextColor::BLACK,

            Node { margin: { px(4).right() } }
            Text::new(job_kind.to_string())
            TextColor::BLACK,

            Node {
                border: px(1),
                margin: { px(4).right() },
            }
            BorderColor::all(Color::BLACK)
            Button
            on(handle_minus_activate)
            Children [
                (Text::new("-") TextColor::BLACK)
            ],

            Node { border: px(1) }
            BorderColor::all(Color::BLACK)
            Button
            on(handle_plus_activate)
            Children[
                Text::new("+") TextColor::BLACK
            ]
        ]
    };

    if matches!(job_kind, JobKind::RenderCarcass) {
        Box::new(bsn! {
            job_row
            Disabled
        }) as Box<dyn Scene>
    } else {
        Box::new(job_row) as Box<dyn Scene>
    }
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

fn handle_minus_activate(
    event: On<Activate>,

    mut commands: Commands,

    parent_query: Query<&ChildOf>,
    job_kind_query: Query<&JobKind, With<Node>>
) {
    let job_row = parent_query.get(event.entity).unwrap().0;
    let job_kind = *job_kind_query.get(job_row).unwrap();

    commands.trigger(UnassignFollower {
        job_kind,
    });
}