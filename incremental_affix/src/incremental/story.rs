//! Observers related to the story.

use bevy::{ecs::observer::IntoObserver, prelude::*};

use crate::incremental::{action::{Action, Explore, LearnAction, ResetPlayerAction}, item::{affixive_item::AffixiveItem, base::Base, craft::Crafted}, log::LogEntry, stock::{StockKind, stockyard::Stockyard}};

pub struct StoryPlugin;

impl Plugin for StoryPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, setup)
        ;
    }
}

#[derive(Debug, Component)]
struct StoryObservers {
    observer: Entity,
}

impl StoryObservers {
    fn replace<M>(&mut self, mut commands: Commands, observer: impl IntoObserver<M>) {
        self.clear(commands.reborrow());
        let entity = commands.add_observer(observer).id();
        self.observer = entity;
    }

    fn clear(&mut self, mut commands: Commands) {
        commands.entity(self.observer).despawn();
    }
}

fn setup(mut commands: Commands) {
    let on_first_explore = commands.add_observer(on_first_explore).id();
    commands.spawn(StoryObservers {
        observer: on_first_explore,
    });
}

fn on_first_explore(
    _event: On<Explore>,
    mut commands: Commands,
    mut observers: Single<&mut StoryObservers>,

    mut stockyard: ResMut<Stockyard>,
    mut log_event_writer: MessageWriter<LogEntry>,
) {
    observers.replace(commands.reborrow(), on_craft_makeshift_tools);

    stockyard[StockKind::BranchesAndPebbles] += 1.0;
    log_event_writer.write(LogEntry::from([
        "While exploring, you find some twigs and rocks on the ground.",
        "Furthermore, you notice there's a lot of trees and exposed stone.",
        "You get the idea to craft some makeshift tools to gather some wood and stone."
    ]));

    commands.trigger(LearnAction { action: Action::GatherWood });
    commands.trigger(LearnAction { action: Action::GatherStone });
    commands.trigger(ResetPlayerAction);
}

fn on_craft_makeshift_tools(
    event: On<Crafted>,
    commands: Commands,
    mut observers: Single<&mut StoryObservers>,

    item_query: Query<&AffixiveItem>,

    mut log_event_writer: MessageWriter<LogEntry>,
) {
    let item = item_query.get(event.crafted_item).expect("Entity for Crafted event must have an AffixiveItem component.");

    if item.base() != Base::MakeshiftTools {
        // A different item was crafted. Most likely a test item.
        return;
    }

    observers.clear(commands);
    
    log_event_writer.write(LogEntry::from([
        "You sit down and cobble together some makeshift logging and mining tools using the sticks and pebbles laying around.",
        "You weren't expecting the materials to make good tools, but they came out much more effective than you anticipated.",
        "It seems you are quite the toolmaker. But that doesn't help you remember who you are.",
        "At least now you can acquire wood and stone. Perhaps even use those to make better tools.",
    ]));
}