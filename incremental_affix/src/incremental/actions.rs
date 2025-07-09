use bevy::prelude::*;

use crate::incremental::{ExplorationProgress, StockKind, Stockyard};

#[derive(Debug, Default, Resource)]
pub struct ActionProgress(pub f32);

#[derive(Debug, Default, Resource)]
pub struct CurrentAction(pub Option<Actions>);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Component)]
pub enum Actions {
    Explore,
    GatherWood,
    CreateFollowers,
}

impl Actions {
    pub const LIST: &[Self] = &[
        Self::Explore,
        Self::GatherWood,
        Self::CreateFollowers,
    ];
}

impl std::fmt::Display for Actions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match *self {
            Self::Explore => "Explore",
            Self::GatherWood => "Gather Wood",
            Self::CreateFollowers => "Create Followers",
        })
    }
}

pub struct ActionPlugin;

impl Plugin for ActionPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app
        .init_resource::<ActionProgress>()
        .init_resource::<CurrentAction>()
        .add_systems(Update, (progress_system,))
        ;
    }
}

fn progress_system(
    time: Res<Time>,
    mut progress: ResMut<ActionProgress>,
    current_action: Res<CurrentAction>,
    mut stockyard: ResMut<Stockyard>,
    mut exploration_progress: ResMut<ExplorationProgress>,
) {
    let current_action = match current_action.0 {
        None => return,
        Some(current_action) => current_action
    };

    let duration = time.delta();

    progress.0 += duration.as_secs_f32() / 5.0;

    if progress.0 >= 1.0 {
        progress.0 -= 1.0;
        
        // This could also be changed to firing an event
        // if the code in here becomes too unweildy.
        match current_action {
            Actions::Explore => {
                exploration_progress.0 += 1;

                match exploration_progress.0 {
                    0 => {},
                    1 => {
                        stockyard[StockKind::Wood] += 500;
                        stockyard[StockKind::Stone] += 500;
                    },
                    _ => {}
                }
            },
            Actions::GatherWood => {
                stockyard[StockKind::Wood] += 100;
            }
            Actions::CreateFollowers => todo!(),
        }
    }
}