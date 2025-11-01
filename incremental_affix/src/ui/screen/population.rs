use bevy::ecs::entity_disabling::Disabled;
use bevy::prelude::*;
use bevy::ui_widgets::Button;

use crate::ui::screen::Screen;

// #[TODO(Havvy)]: This enum should be incremental.
#[derive(Debug, Clone, Copy, Component)]
enum Job {
    ChopWood,
    Hunt,
    RenderCarcass,
    Cook,
}

impl Job {
    pub const LIST: &[Self] = &[Self::ChopWood, Self::Hunt, Self::RenderCarcass, Self::Cook];
}

impl ToString for Job {
    fn to_string(&self) -> String {
        match *self {
            Job::ChopWood => "Chop Wood".to_string(),
            Job::Hunt => "Hunt".to_string(),
            Job::RenderCarcass => "Render Carcasses".to_string(),
            Job::Cook => "Cook".to_string(),
        }
    }
}

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

    Job::LIST.iter().copied().for_each(|job| spawn_job_row(commands.reborrow(), job, screen))
}

fn spawn_job_row(
    mut commands: Commands,
    job: Job,
    parent: Entity,
) {
    let job_row = commands.spawn((
        Node {
            flex_direction: FlexDirection::Row,

            ..default()
        },

        job,
        
        ChildOf(parent),
    )).id();

    if matches!(job, Job::RenderCarcass) {
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
        Text::new(job.to_string()),
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
        // Activate Observer.

        children![(
            Text::new("+"),
            TextColor::BLACK,
        )],
        ChildOf(job_row),
    ));

}