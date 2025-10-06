mod engine;
//#[path = "game"]
//mod game_old;
mod incremental;

//use engine::drop_table;
use engine::item;
// use game::modifiers::GameModifierKind;
// use game::player::Player;

use bevy::{dev_tools::fps_overlay::{FpsOverlayConfig, FrameTimeGraphConfig}, prelude::*};
use incremental::*;

fn main() {
    App::new()
    .add_plugins((
        DefaultPlugins,

        IncrementalPlugin,

        ui::log::GameLogPlugin,
        incremental::action::ActionPlugin,
        ui::tooltip::TooltipPlugin,
        ui::UiPlugin,
        bevy::dev_tools::fps_overlay::FpsOverlayPlugin {
            config: FpsOverlayConfig {
                enabled: false,
                text_color: Color::BLACK,
                frame_time_graph_config: FrameTimeGraphConfig {
                    enabled: false,
                    ..default()
                },
                ..default()
            }
        },
        bevy::ui_widgets::ButtonPlugin,
    ))
    .run();
}

/*
fn _main2() {
    let implicits = game::item::initialize_implicits();
    let bases = game::item::initialize_bases();

    let prefixes = game::item::initialize_prefixes();
    let suffixes = game::item::initialize_suffixes();
    let drop_tables = drop_table::initialize_drop_tables(&bases, prefixes, suffixes);

    let bases_table = drop_tables.get::<drop_table::StorageBasesDropTable>().unwrap();
    let base_ix = bases_table.random();

    let mut random_item = item::AffixiveItem::new(&bases, &implicits, base_ix, item::Quality::Quality(1));
    
    if random_item.tags.contains(&item::AffixiveItemTag::Inventory) {
        let suffix_or_prefix_table = drop_tables.get::<drop_table::SuffixOrPrefixDropTable>().unwrap();
        match suffix_or_prefix_table.random() {
            drop_table::SuffixOrPrefix::Prefix => {
                let prefix_table = drop_tables.get::<drop_table::InventoryModifierPrefixes<GameModifierKind>>().unwrap();
                let mut prefix = prefix_table.random();
                prefix.randomize_actual();

                let _ = random_item.try_push_prefix(prefix);
            },

            drop_table::SuffixOrPrefix::Suffix => {
                let suffix_table = drop_tables.get::<drop_table::InventoryModifierSuffixes<GameModifierKind>>().unwrap();
                let mut suffix = suffix_table.random();
                suffix.randomize_actual();

                let _ = random_item.try_push_suffix(suffix);
            }
        }
    }

    println!("{}", random_item.display(&bases[..]));

    let mut player = Player::default();
    match player.try_equipping(random_item, &bases) {
        Ok(_) => println!("Inventory Volume: {}", player.get_inventory_volume()),
        Err(_) => { panic!("Player cannot equip?" )},
    }
}
    */