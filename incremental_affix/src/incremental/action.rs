use bevy::prelude::*;
use bevy::platform::collections::HashSet;

use crate::incremental::ExplorationProgress;
use crate::ui::log::LogMessage;
use crate::incremental::stock::{StockKind, Stockyard};

pub struct ActionPlugin;

impl Plugin for ActionPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app
        .init_resource::<ActionProgress>()
        .init_resource::<CurrentAction>()
        .init_resource::<KnownActions>()
        .insert_resource(CanMine(false))
        .insert_resource(CanChop(false))
        .add_observer(on_learn_action)
        .add_systems(FixedUpdate, (progress_system,))
        ;
    }
}

#[derive(Debug, Default, Resource, Deref)]
pub struct ActionProgress(pub f32);

#[derive(Debug, Default, Resource, Deref)]
pub struct CurrentAction(pub Option<Action>);

#[derive(Debug, Resource, Deref, DerefMut)]
pub struct KnownActions(HashSet<Action>);

#[derive(Debug, Resource, Deref, DerefMut)]
pub struct CanMine(pub bool);

#[derive(Debug, Resource, Deref, DerefMut)]
pub struct CanChop(pub bool);

impl Default for KnownActions {
    fn default() -> Self {
        let mut set = HashSet::new();
        set.insert(Action::Explore);
        set.insert(Action::CreateFollowers);

        Self(set)
    }
}

#[derive(Debug, Event)]
pub struct LearnAction {
    pub action: Action
}

fn on_learn_action(
    event: On<LearnAction>,
    mut known_actions: ResMut<KnownActions>,
) {
    known_actions.insert(event.action);
}

/// An action the player can perform.
/// 
/// The player can only perform one action at a time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub enum Action {
    Explore,
    GatherWood,
    GatherStone,
    CreateFollowers,
}

impl Action {
    pub const LIST: &[Self] = &[
        Self::Explore,
        Self::GatherWood,
        Self::GatherStone,
        Self::CreateFollowers,
    ];
}

impl std::fmt::Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match *self {
            Self::Explore => "Explore",
            Self::GatherWood => "Gather Wood",
            Self::GatherStone => "Gather Stone",
            Self::CreateFollowers => "Create Followers",
        })
    }
}

fn progress_system(
    mut commands: Commands,
    time: Res<Time>,
    mut progress: ResMut<ActionProgress>,
    current_action: Res<CurrentAction>,
    mut stockyard: ResMut<Stockyard>,
    mut exploration_progress: ResMut<ExplorationProgress>,
    mut log_event_writer: MessageWriter<LogMessage>,
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
            Action::Explore => {
                exploration_progress.0 += 1;

                match exploration_progress.0 {
                    0 => {},
                    1 => {
                        stockyard[StockKind::BranchesAndPebbles] += 1;
                        log_event_writer.write(LogMessage("While exploring, you find some twigs and rocks on the ground.".to_string()));
                        log_event_writer.write(LogMessage("Furthermore, you notice there's a lot of trees and exposed stone.".to_string()));
                        log_event_writer.write(LogMessage("You get the idea to craft some makeshift tools to gather some wood and stone.".to_string()));

                        commands.trigger(LearnAction { action: Action::GatherWood });
                        commands.trigger(LearnAction { action: Action::GatherStone });
                    },
                    _ => {}
                }
            },
            Action::GatherWood => {
                stockyard[StockKind::Wood] += 100;
            }
            Action::GatherStone => {
                stockyard[StockKind::Stone] += 100;
            }
            Action::CreateFollowers => todo!(),
        }
    }
}