use std::fmt::Display;
use std::time::Duration;

use bevy::prelude::*;
use bevy::platform::collections::HashSet;

use crate::incremental::critical::Critical;
use crate::incremental::ExplorationProgress;
use crate::ui::log::LogMessage;
use crate::incremental::stock::{StockKind, Stockyard};

pub const NO_CURRENT_ACTION_DISPLAY: &str = "Doing Nothing";

pub struct ActionPlugin;

impl Plugin for ActionPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app
        .init_resource::<ActionProgress>()
        .init_resource::<CurrentAction>()
        .init_resource::<KnownActions>()
        .insert_resource(MineSpeed(0.))
        .insert_resource(ChopSpeed(0.))
        .insert_resource(ActionCritical { critical: Critical::new(), timer: None })
        .init_resource::<CriticalTimer>()
        .add_observer(on_learn_action)
        .add_observer(on_change_action)
        .add_systems(FixedUpdate, (progress_system, critical_check_system))
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
    CreateFollowers,
}

impl Action {
    pub const LIST: &[Self] = &[
        Self::Explore,
        Self::GatherWood,
        Self::GatherStone,
        Self::CreateFollowers,
    ];

    pub const fn progresses(&self) -> bool {
        match self {
            Action::Explore => true,
            Action::GatherWood => false,
            Action::GatherStone => false,
            Action::CreateFollowers => todo!(),
        }
    }

    /// If the action passively increases a stock.
    pub const fn is_passive(self) -> bool {
        match self {
            | Action::GatherWood
            | Action::GatherStone
            => true,

            _ => false,
        }
    }
}

impl Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match *self {
            Self::Explore => "Explore",
            Self::GatherWood => "Gather Wood",
            Self::GatherStone => "Gather Stone",
            Self::CreateFollowers => "Create Followers",
        })
    }
}

#[derive(Debug, Default, Resource, Deref, DerefMut)]
pub struct ActionProgress(pub f32);

impl ActionProgress {
    fn reset(&mut self) {
        self.0 = 0.0;
    }
}

#[derive(Debug, Default, Resource, Deref, DerefMut)]
pub struct CurrentAction(pub Option<Action>);

impl CurrentAction {
    fn set(&mut self, action: Action) {
        self.0 = Some(action)
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

#[derive(Debug, Resource, Deref, DerefMut)]
pub struct MineSpeed(f64);

impl MineSpeed {
    pub fn can_mine(&self) -> bool {
        self.0 != 0.0
    }

    pub fn set(&mut self, value: f64) {
        self. 0 = value;
    }
}

#[derive(Debug, Resource, Deref, DerefMut)]
pub struct ChopSpeed(f64);

impl ChopSpeed {
    pub fn can_chop(&self) -> bool {
        self.0 != 0.0
    }

    pub fn set(&mut self, value: f64) {
        self. 0 = value;
    }
}

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

    **progress += time.delta().as_secs_f32() / 5.0;

    if **progress >= 1.0 {
        **progress -= 1.0;
        
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
            
            _ => {
                panic!("Unhandled progressing action.")
            }
        }
    }
}

#[derive(Debug, Resource, Deref, DerefMut)]
struct CriticalTimer(Timer);

impl CriticalTimer {
    fn new() -> Self {
        let mut timer = Timer::from_seconds(5.0, TimerMode::Repeating);
        timer.pause();
        Self(timer)
    }
}

impl Default for CriticalTimer {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Resource)]
pub struct ActionCritical {
    critical: Critical,
    timer: Option<Timer>,
}

impl ActionCritical {
    fn reset(&mut self) {
        self.timer = None;
    }

    pub fn time_left(&self) -> Duration {
        self.timer.as_ref().map_or(Duration::ZERO, Timer::remaining)
    }
}

fn critical_check_system(
    time: Res<Time>,

    current_action: ResMut<CurrentAction>,

    chop_speed: Res<ChopSpeed>,
    mine_speed: Res<MineSpeed>,

    mut critical_check_timer: ResMut<CriticalTimer>,
    mut action_critical: ResMut<ActionCritical>,
    mut stockyard: ResMut<Stockyard>,
) {
    let Some(current_action) = **current_action else { return; };

    if !current_action.is_passive() {
        return;
    }

    if critical_check_timer.tick(time.delta()).just_finished() && action_critical.critical.check() {
        match current_action {
            Action::GatherWood => {
                stockyard[StockKind::Wood].change *= action_critical.critical.multiplier;
                action_critical.timer = Some(Timer::new(action_critical.critical.time, TimerMode::Once));
            },
            Action::GatherStone => {
                stockyard[StockKind::Stone].change *= action_critical.critical.multiplier;
                action_critical.timer = Some(Timer::new(action_critical.critical.time, TimerMode::Once));
            },

            _ => {
                panic!("Critical hit detected for an action without a critical.");
            }
        }
    }

    if let Some(timer) = &mut action_critical.timer && timer.tick(time.delta()).just_finished() {
        match current_action {
            Action::GatherWood => {
                stockyard[StockKind::Wood].change = chop_speed.0;
                action_critical.timer = None;
            },
            Action::GatherStone => {
                stockyard[StockKind::Stone].change = mine_speed.0;
                action_critical.timer = None;
            },

            _ => {
                panic!("Critical hit timer timed out for an action without a critical.");
            }
        }
    }
}

#[derive(Debug, Event)]
pub struct ChangeAction {
    pub action: Action
}

impl ChangeAction {
    pub fn new(action: Action) -> Self {
        ChangeAction {
            action
        }
    }
}

fn on_change_action(
    event: On<ChangeAction>,

    mut stockyard: ResMut<Stockyard>,
    chop_speed: Res<ChopSpeed>,
    mine_speed: Res<MineSpeed>,

    mut current_action: ResMut<CurrentAction>,
    mut action_progress: ResMut<ActionProgress>,
    mut action_critical: ResMut<ActionCritical>,
    mut critical_timer: ResMut<CriticalTimer>,
) {
    // Changing to current action. Disregard.
    if Some(event.action) == current_action.0 {
        return;
    }

    action_progress.reset();
    action_critical.reset();
    critical_timer.reset();
    

    match current_action.0 {
        None | Some(Action::Explore) | Some(Action::CreateFollowers) => {},

        Some(Action::GatherWood) => {
            stockyard[StockKind::Wood].change = 0.0;
        },

        Some(Action::GatherStone) => {
            stockyard[StockKind::Stone].change = 0.0;
        },
    }


    current_action.set(event.action);

    match event.action {
        Action::Explore => {},
        Action::GatherWood => {
            stockyard[StockKind::Wood].change = (chop_speed.0 * 5.) as _;
            critical_timer.unpause();
        },
        Action::GatherStone => {
            stockyard[StockKind::Stone].change = (mine_speed.0 * 5.) as _;
            critical_timer.unpause();
        },
        Action::CreateFollowers => {},
    }
}