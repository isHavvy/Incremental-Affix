use std::fmt::Display;
use std::time::Duration;

use bevy::prelude::*;
use bevy::platform::collections::HashSet;

use crate::incremental::action::change::ResetPlayerAction;
use crate::incremental::affinity::Affinity;
use crate::incremental::ExplorationProgress;
use crate::ui::log::LogMessage;
use crate::incremental::stock::{StockKind, Stockyard};

pub use change::ChangeAction;

pub const NO_CURRENT_ACTION_DISPLAY: &str = "Doing Nothing";

mod change;

pub struct ActionPlugin;

impl Plugin for ActionPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app
        .init_resource::<ActionProgress>()
        .init_resource::<CurrentAction>()
        .init_resource::<KnownActions>()
        .insert_resource(ActionAffinity { affinity: Affinity::new(), timer: None })
        .init_resource::<AffinityTimer>()
        .add_observer(on_learn_action)
        .add_observer(change::on_change_action)
        .add_observer(change::on_reset_player_action)
        .add_systems(FixedUpdate, (progress_system, affinity_check_system))
        ;
    }
}

/// An action the player can perform.
/// 
/// The player can only perform one action at a time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub enum Action {
    Explore,
    GatherWood,
    GatherStone,
    Hunt,
    RenderCarcass,
    CookMeat,
    CreateFollowers,
}

impl Action {
    pub const LIST: &[Self] = &[
        Self::Explore,
        Self::GatherWood,
        Self::GatherStone,
        Self::Hunt,
        Self::RenderCarcass,
        Self::CookMeat,
        Self::CreateFollowers,
    ];

    pub const fn progresses(&self) -> bool {
        match self {
            Action::Explore => true,
            Action::CreateFollowers => true,
            _ => false,
        }
    }

    /// If the action passively increases a stock.
    pub const fn is_passive(self) -> bool {
        match self {
            | Action::GatherWood
            | Action::GatherStone
            | Action::Hunt
            => true,

            _ => false,
        }
    }

    pub fn progress_time(self) -> f32 {
        if self == Action::CreateFollowers { 30.0 } else { 5.0 }
    }
}

impl Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match *self {
            Self::Explore => "Explore",
            Self::GatherWood => "Gather Wood",
            Self::GatherStone => "Gather Stone",
            Self::Hunt => "Hunt",
            Self::RenderCarcass => "Render Carcasses",
            Self::CookMeat => "Cook Meat",
            Self::CreateFollowers => "Create Followers",
        })
    }
}

#[derive(Debug, Default, Resource)]
pub struct ActionProgress {
    pub percent: f32,
    time_seconds: f32,
}

impl ActionProgress {
    fn reset(&mut self) {
        self.percent = 0.0;
    }
}

#[derive(Debug, Default, Resource, Deref, DerefMut)]
pub struct CurrentAction(pub Option<Action>);

impl CurrentAction {
    fn set(&mut self, action: Action) {
        self.0 = Some(action)
    }

    fn reset(&mut self) {
        self.0 = None;
    }
}

impl Display for CurrentAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            None => f.write_str(NO_CURRENT_ACTION_DISPLAY),
            Some(action) => action.fmt(f)
        }
    }
}

#[derive(Debug, Resource, Deref, DerefMut)]
pub struct KnownActions(HashSet<Action>);

impl Default for KnownActions {
    fn default() -> Self {
        let mut set = HashSet::new();
        set.insert(Action::Explore);
        set.insert(Action::CreateFollowers);
        set.insert(Action::Hunt);
        set.insert(Action::RenderCarcass);
        set.insert(Action::CookMeat);

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
        Some(current_action ) if current_action.progresses() || current_action.is_passive() => current_action,
        _ => return,
    };

    progress.percent += time.delta().as_secs_f32() / progress.time_seconds;

    if progress.percent >= 1.0 {
        progress.percent -= 1.0;
        
        if !current_action.progresses() {
            return;
        }

        // This could also be changed to firing an event
        // if the code in here becomes too unweildy.
        // Or well, because the logic here is about a specific action
        // and not actions in general like this module should be.
        match current_action {
            Action::Explore => {
                exploration_progress.0 += 1;

                match exploration_progress.0 {
                    0 => {},
                    1 => {
                        stockyard[StockKind::BranchesAndPebbles] += 1.0;
                        log_event_writer.write(LogMessage("While exploring, you find some twigs and rocks on the ground.".to_string()));
                        log_event_writer.write(LogMessage("Furthermore, you notice there's a lot of trees and exposed stone.".to_string()));
                        log_event_writer.write(LogMessage("You get the idea to craft some makeshift tools to gather some wood and stone.".to_string()));

                        commands.trigger(LearnAction { action: Action::GatherWood });
                        commands.trigger(LearnAction { action: Action::GatherStone });
                        commands.trigger(ResetPlayerAction);
                    },
                    _ => {}
                }
            },

            Action::CreateFollowers => {
                stockyard[StockKind::Followers] += 1.0;
                stockyard[StockKind::Godpower] -= 10.0;
                commands.trigger(ResetPlayerAction);
            }
            
            _ => {
                panic!("Unhandled progressing action.")
            }
        }
    }
}

#[derive(Debug, Resource, Deref, DerefMut)]
struct AffinityTimer(Timer);

impl AffinityTimer {
    fn new() -> Self {
        let mut timer = Timer::from_seconds(5.0, TimerMode::Repeating);
        timer.pause();
        Self(timer)
    }
}

impl Default for AffinityTimer {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Resource)]
pub struct ActionAffinity {
    affinity: Affinity,
    timer: Option<Timer>,
}

impl ActionAffinity {
    fn reset(&mut self) {
        self.timer = None;
    }

    pub fn time_left(&self) -> Duration {
        self.timer.as_ref().map_or(Duration::ZERO, Timer::remaining)
    }
}

fn affinity_check_system(
    time: Res<Time>,

    current_action: ResMut<CurrentAction>,

    mut affinity_check_timer: ResMut<AffinityTimer>,
    mut action_affinity: ResMut<ActionAffinity>,
    mut stockyard: ResMut<Stockyard>,
) {
    let Some(current_action) = **current_action else { return; };

    if !current_action.is_passive() {
        return;
    }

    if affinity_check_timer.tick(time.delta()).just_finished() && action_affinity.affinity.check() {
        match current_action {
            Action::GatherWood => {
                stockyard[StockKind::Wood].set_player_action_has_affinity(true);
                action_affinity.timer = Some(Timer::new(action_affinity.affinity.time, TimerMode::Once));
            },
            Action::GatherStone => {
                stockyard[StockKind::Stone].set_player_action_has_affinity(true);
                action_affinity.timer = Some(Timer::new(action_affinity.affinity.time, TimerMode::Once));
            },

            _ => {
                panic!("Affinity occurred detected for an action without affinity.");
            }
        }
    }

    if let Some(timer) = &mut action_affinity.timer && timer.tick(time.delta()).just_finished() {
        match current_action {
            Action::GatherWood => {
                stockyard[StockKind::Wood].set_player_action_has_affinity(false);
                action_affinity.timer = None;
            },
            Action::GatherStone => {
                stockyard[StockKind::Stone].set_player_action_has_affinity(false);
                action_affinity.timer = None;
            },

            _ => {
                panic!("Affinity timer timed out for an action without affinity.");
            }
        }
    }
}