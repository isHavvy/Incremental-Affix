//! Observers related to the story.

use bevy::{ecs::observer::IntoObserver, prelude::*};

use crate::incremental::{action::{Action, Explore, LearnAction, ResetPlayerAction}, item::{affixive_item::AffixiveItem, base::Base, craft::{Crafted, Recipe}}, log::LogEntry, stock::{StockKind, on_total::OnStockTotalProduced, stockyard::Stockyard}};

pub struct StoryPlugin;

impl Plugin for StoryPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, setup.in_set(super::IncrementalStartupSystemSet))
        ;
    }
}

#[derive(Debug, Component)]
struct StoryObservers {
    observer: Entity,
    count: u32,
}

impl StoryObservers {
    fn replace<M>(&mut self, mut commands: Commands, observer: impl IntoObserver<M>) {
        self.clear(commands.reborrow());
        let entity = commands.add_observer(observer).id();
        self.observer = entity;
    }

    fn clear(&mut self, mut commands: Commands) {
        commands.entity(self.observer).despawn();
        self.count = 0;
    }
}

fn setup(mut commands: Commands) {
    let on_first_explore = commands.add_observer(on_first_explore).id();
    commands.spawn(StoryObservers {
        observer: on_first_explore,
        count: 0,
    });

    let system_id = commands.register_system(on_five_stone_mined);
    commands.spawn(OnStockTotalProduced {
        stock_kind: StockKind::Stone,
        total_produced: 5.0,
        on_total_produced: system_id,
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
        "Walking around, taking a look around, you are in a dense forest with lots of rocky hills.",
        "The first thing that comes to mind is how beautiful it is here.",
        "The second is that if you had some tools, you could easily harvest a tree or some of the stone.",
        "Looking down at the twigs and stones beneath your feet, a design comes to mind to turn them into tools to do so.",
        "You pick up the twigs and stones."
    ]));

    commands.trigger(LearnAction { action: Action::GatherWood });
    commands.trigger(LearnAction { action: Action::GatherStone });
    commands.trigger(ResetPlayerAction);
}

fn on_craft_makeshift_tools(
    event: On<Crafted>,
    mut commands: Commands,
    mut observers: Single<&mut StoryObservers>,

    item_query: Query<&AffixiveItem>,

    mut log_event_writer: MessageWriter<LogEntry>,
) {
    let item = item_query.get(event.crafted_item).expect("Entity for Crafted event must have an AffixiveItem component.");

    if item.base() != Base::MakeshiftTools {
        // A different item was crafted. Most likely a test item.
        return;
    }

    observers.replace(commands.reborrow(), on_craft_stone_tools);
    
    log_event_writer.write(LogEntry::from([
        "You sit down and cobble together some makeshift logging and mining tools using the sticks and pebbles laying around.",
        "You weren't expecting the materials to make good tools, but they came out much more effective than you anticipated.",
        "It seems you are quite the toolmaker. But that doesn't help you remember who you are.",
        "At least now you can acquire wood and stone. Perhaps even use those to make better tools.",
    ]));

    commands.spawn_scene(bsn!{
        Recipe {
            base: Base::StoneTools,
            resources: smallvec::smallvec![(StockKind::Wood, 5.0), (StockKind::Stone, 5.0)],
        }
    });
}

fn on_craft_stone_tools(
    event: On<Crafted>,
    mut commands: Commands,
    mut observers: Single<&mut StoryObservers>,

    item_query: Query<&AffixiveItem>,

    mut log_event_writer: MessageWriter<LogEntry>,
) {
    let item = item_query.get(event.crafted_item).expect("Entity for Crafted event must have an AffixiveItem component.");

    if item.base() != Base::StoneTools {
        return;
    }

    observers.replace(commands.reborrow(), on_second_explore);

    log_event_writer.write(LogEntry::from([
        "After sitting down and crafting the stone tools, you again note the ease at which you made these.",
        "Perhaps you are a mighty crafter?",
        "Perhaps some more walking around will help you regain your memory."
    ]));
}

fn on_second_explore(
    _event: On<Explore>,
    mut commands: Commands,
    mut observers: Single<&mut StoryObservers>,

    mut log_event_writer: MessageWriter<LogEntry>,
) {
    observers.count += 1;

    match observers.count {
        5 => {
            log_event_writer.write(LogEntry::from([
                "Walking around helps clear your head, and you can feel something coming back.",
                "You can't quite tell what it is yet.",
                "While hoping for some memory, you look up and see an animal run by.",
                "With how long you've been awake, you feel like you should have been hungry or thirsty by now.",
                "Despite not needing to eat, you do consider you can make a hunting bow.",
            ]));

            commands.spawn_scene(bsn!{
                Recipe {
                    base: Base::WoodenHunt,
                    resources: smallvec::smallvec![(StockKind::Wood, 5.0)],
                }
            });
        },

        10 => {
            observers.clear(commands.reborrow());

            log_event_writer.write(LogEntry::from([
                "You remember!",
                "\n",
                "You are a minor deity.",
                "<<Stuff about creating followers here.>>"
            ]));

            commands.trigger(LearnAction { action: Action::CreateFollowers });
            commands.trigger(ResetPlayerAction);
        },

        _ => { /* do nothing */}
    }
}

fn on_five_stone_mined(
    mut stockyard: ResMut<Stockyard>,
    mut log_event_writer: MessageWriter<LogEntry>,
) {
    stockyard[StockKind::Diamond] += 1.0;
    log_event_writer.write(LogEntry("While mining stone, you notice a diamond.".to_string()));
}