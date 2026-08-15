use bevy::prelude::*;
use bevy::ui_widgets::{Activate, Button};

use crate::incremental::item::affixive_item::PushAffixError;
use crate::incremental::item::equipment::Equipped;
use crate::incremental::item::item_database::ItemDatabase;
use crate::incremental::item::{item_slot::{ItemSlot, ItemSlotTag}, craft::Crafted};
use crate::incremental::item::{affixive_item::{AffixiveItem, ItemTag}};
use crate::incremental::log::LogEntry;
use crate::ui::tooltip::{HideTooltip, ShowTooltip};
use crate::ui::item::spawn_item_details;
use crate::ui::screen::{Screen, screen_title};

#[derive(Debug, Clone, Component, FromTemplate)]
pub struct InventoryList(Entity);

impl InventoryList {
    pub fn get(&self) -> Entity {
        self.0
    }
}

#[derive(Debug, Clone, Component, FromTemplate)]
pub struct CorrespondingItem(Entity);

#[derive(Debug, Clone, Copy, Default, Component)]
pub struct ActiveSlot;

pub fn inventory_screen() -> impl Scene {
    bsn! {
        Node {
            display: Display::None,

            flex_direction: FlexDirection::Column,
        }

        Screen::Inventory
        InventoryList(#InventoryList)

        Children [
            screen_title("Inventory"),

            // --

            (
                #Slots
                Node {
                    flex_direction: FlexDirection::Row,
                    height: px(150)
                }
                BackgroundColor(Color::srgb_u8(137, 81, 41))
                Children [
                    #ToolSlot
                    ActiveSlot
                    slot(ItemSlotTag::Tool),

                    #HuntSlot
                    slot(ItemSlotTag::Hunt),
                ]
            ),

            // ---

            #InventoryList
            Node {
                flex_direction: FlexDirection::Column,
            }
            BackgroundColor(Color::srgb_u8(67, 111, 71))
        ]
    }
}

fn slot(slot_tag: ItemSlotTag) -> impl Scene {
    bsn!{
        Node {
            flex_direction: FlexDirection::Column,

            box_sizing: BoxSizing::BorderBox,
            width: px(150),
            border: { px(2).all() },
            margin: { px(4).all() },

            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
        }
        BorderColor::all(Color::BLACK)
        BackgroundColor(Color::srgb(0., 0.7, 0.))

        Button
        on(on_slot_hover)
        on(on_out_hide_tooltip)
        on(on_slot_activate)

        ItemSlot {
            tag: slot_tag,
            item: None,
        }

        Children [
            #SlotName
            Text::new(slot_tag.to_string())
        ]
    }
}

pub fn on_item_craft(
    event: On<Crafted>,
    mut commands: Commands,

    inventory_list: Single<&InventoryList>,

    item_query: Query<&AffixiveItem>,

    mut log_event_writer: MessageWriter<LogEntry>,
) {
    let item = item_query.get(event.crafted_item).unwrap();
    commands.spawn_scene(bsn! {
        inventory_item(event.crafted_item, item.name().to_string())
        ChildOf({ inventory_list.get() })
    });
    log_event_writer.write(LogEntry(format!("Crafted '{}'", item.name())));
}

pub fn inventory_item(item_entity: Entity, item_name: String) -> impl Scene {
    bsn! {
        #Line
        Node
        CorrespondingItem(item_entity)

        on(on_inventory_hover)
        on(on_out_hide_tooltip)

        Children [
            line_button("E")
            on(on_activate_button_equip),

            line_button("R")
            on(on_activate_button_roll),

            Text(item_name)
            TextColor::BLACK
        ]
    }
}

pub fn line_button(text: &'static str) -> impl Scene {
    bsn!{
        Node {
            border: px(1),
            margin: px(4),
        }
        BorderColor::all(Color::BLACK)
        Button
        Children [
            Text::new(text)
            TextColor::BLACK
        ]
    }
}

fn on_activate_button_equip(
    event: On<Activate>,
    mut commands: Commands,

    active_slot: Single<(&ActiveSlot, Entity)>,
    inventory_screen: Single<&InventoryList>,
    item_db: Res<ItemDatabase>,

    parent_query: Query<&ChildOf>,
    corresponding_item_query: Query<&CorrespondingItem>,
    item_query: Query<&AffixiveItem>,
    mut item_slot_query: Query<&mut ItemSlot>
) {
    let item_node = parent_query.get(event.entity).unwrap().parent();
    let corresponding_item = corresponding_item_query.get(item_node).unwrap().0;

    let item = item_query.get(corresponding_item)
    .expect("Corresponding item entity must have an item component.");

    let mut item_slot = item_slot_query.get_mut(active_slot.1)
    .expect("Active slot resource must have an item slot component.");

    let item_tag = ItemTag::from(item_slot.tag);

    if !item_db.item_has_tag(item, item_tag) {
        return;
    }

    let previous_item = item_slot.item.replace(corresponding_item);

    if let Some(previous_item_entity) = previous_item {
        let previous_item = item_query.get(previous_item_entity)
        .expect("Item entity in an item slot must have an item entity.");

        let name = previous_item.name().to_string();

        commands.spawn_scene(bsn!{
            inventory_item(previous_item_entity, name.to_string())
            ChildOf({ inventory_screen.get() })
        });
    }
    commands.entity(item_node).despawn();

    commands.trigger(Equipped { item: corresponding_item });
}

fn on_activate_button_roll(
    event: On<Activate>,

    db: Res<ItemDatabase>,

    mut log_writer: MessageWriter<LogEntry>,

    parent_query: Query<&ChildOf>,
    corresponding_item_query: Query<&CorrespondingItem>,
    mut item_query: Query<&mut AffixiveItem>,
) {
    let item_node = parent_query.get(event.entity).unwrap().parent();
    let corresponding_item = corresponding_item_query.get(item_node).unwrap().0;

    let mut item = item_query.get_mut(corresponding_item)
    .expect("Corresponding item entity must have an item component.");

    item.increase_quality_to(1);
    match db.try_push_random_prefix(&mut item) {
        Ok(_) => {},
        Err(PushAffixError::AffixiveItemIsFixed) => {
            log_writer.write(LogEntry::new("You cannot modify the affixes of this."));
        },
        Err(PushAffixError::AffixiveItemQualityTooLow) => {
            log_writer.write(LogEntry::new("Cannot add prefix. Item quality too low."));
        },
    }

    match db.try_push_random_suffix(&mut item) {
        Ok(_) => {},
        Err(PushAffixError::AffixiveItemIsFixed) => {
            log_writer.write(LogEntry::new("You cannot modify the affixes of this."));
        },
        Err(PushAffixError::AffixiveItemQualityTooLow) => {
            log_writer.write(LogEntry::new("Cannot add prefix. Item quality too low."));
        },
    }
}

fn on_inventory_hover(
    event: On<Pointer<Over>>,
    mut commands: Commands,

    corresponding_item_query: Query<&CorrespondingItem>,
    item_query: Query<&AffixiveItem>,
) {
    let item_entity = corresponding_item_query.get(event.entity)
    .expect("Corresponding item must be on this entity.").0;

    let item = item_query.get(item_entity)
    .expect("Item entity must have item component.");

    let content = spawn_item_details(commands.reborrow(), item);
    commands.trigger(ShowTooltip { content });
}

fn on_slot_hover(
    event: On<Pointer<Over>>,
    mut commands: Commands,

    item_slot: Query<&ItemSlot>,
    item_query: Query<&AffixiveItem>,
) {
    let item_slot = item_slot.get(event.entity).expect("Item slot node must have an item slot component.");
    let Some(item_entity) = item_slot.item else { return /* if no item, no tooltip to show */; };
    let item = item_query.get(item_entity).expect("Item entity must have item component.");
    let content = spawn_item_details(commands.reborrow(), item);
    commands.trigger(ShowTooltip { content });
}

fn on_out_hide_tooltip(
    _event: On<Pointer<Out>>,
    mut commands: Commands,
) {
    commands.trigger(HideTooltip);
}

fn on_slot_activate(
    event: On<Activate>,
    mut commands: Commands,

    active_slot: Single<(&ActiveSlot, Entity)>,
) {
    commands.get_entity(active_slot.1).unwrap().remove::<ActiveSlot>();
    commands.get_entity(event.entity).unwrap().insert(ActiveSlot);
}