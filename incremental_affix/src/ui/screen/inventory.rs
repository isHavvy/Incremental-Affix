use bevy::prelude::*;
use bevy::ui_widgets::{observe, Activate, Button};

use crate::incremental::item::equipment::Equipped;
use crate::incremental::item::item_database::ItemDatabase;
use crate::incremental::item::{item_slot::{ItemSlot, ItemSlotTag}, CraftEvent};
use crate::incremental::item::{affixive_item::{AffixiveItem, ItemTag}};
use crate::ui::tooltip::{HideTooltip, ShowTooltip};
use crate::ui::item::spawn_item_details;
use crate::ui::screen::Screen;

#[derive(Debug, Resource)]
pub struct InventoryScreen(Entity);

impl InventoryScreen {
    pub fn get(&self) -> Entity {
        self.0.clone()
    }
}

#[derive(Debug, Component)]
pub struct CorrespondingItem(Entity);

#[derive(Debug, Resource)]
pub struct ActiveSlot(Entity);

pub fn spawn_inventory_screen(mut commands: Commands, parent: Entity) {
    let inventory_screen = commands.spawn((
        Node {
            display: Display::None,

            flex_direction: FlexDirection::Column,

            ..default()
        },

        Screen::Inventory,

        ChildOf(parent),
    )).id();

    let slots = commands.spawn((
        Node {
            flex_direction: FlexDirection::Row,
            height: px(150),

            ..default()
        },
        BackgroundColor(Color::srgb_u8(137, 81, 41)),
        ChildOf(inventory_screen)
    )).id();

    let tool_slot = spawn_slot(commands.reborrow(), slots, ItemSlotTag::Tool);

    commands.insert_resource(ActiveSlot(tool_slot));
    commands.insert_resource(InventoryScreen(inventory_screen));
}

fn spawn_slot(mut commands: Commands, parent: Entity, slot_tag: ItemSlotTag) -> Entity {
    let container = commands.spawn((
        Node {
            flex_direction: FlexDirection::Column,

            box_sizing: BoxSizing::BorderBox,
            width: px(150),
            margin: px(4).all(),
            border: px(2).all(),

            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            
            ..Default::default()
        },
        BorderColor::all(Color::BLACK),
        BackgroundColor(Color::srgb(0., 0.7, 0.)),

        observe(on_slot_hover),
        observe(on_out_hide_tooltip),

        ItemSlot {
            tag: slot_tag,
            item: None,
        },

        ChildOf(parent),
    )).id();

    let _slot_name = commands.spawn((
        Text::new("Tool"),
        ChildOf(container),
    ));

    return container;
}

pub fn on_item_craft(
    event: On<CraftEvent>,
    mut commands: Commands,

    inventory_screen: Res<InventoryScreen>,

    item_query: Query<&AffixiveItem>,
) {
    let item = item_query.get(event.crafted_item).unwrap();
    spawn_inventory_item(commands.reborrow(), &*inventory_screen, event.crafted_item, item.name().to_string());
}

pub fn spawn_inventory_item(
    mut commands: Commands,
    inventory_screen: &InventoryScreen,
    item_entity: Entity,
    item_name: String,
) {
    let line = commands.spawn((
        Node {
            ..default()
        },

        CorrespondingItem(item_entity),

        observe(on_inventory_hover),
        observe(on_out_hide_tooltip),

        ChildOf(inventory_screen.get())
    )).id();

    commands.spawn((
        Node {
            border: px(1).all(),
            margin: px(4).right(),

            ..default()
        },
        BorderColor::all(Color::BLACK),

        Button,
        observe(on_activate_button_equip),

        children![(
            Text::new("E"),
            TextColor(Color::BLACK),
        )],

        ChildOf(line),
    ));

    commands.spawn((
        Text(item_name),
        TextColor(Color::BLACK),
        ChildOf(line)
    ));
}

pub fn on_activate_button_equip(
    activate: On<Activate>,
    mut commands: Commands,

    active_slot: Res<ActiveSlot>,
    inventory_screen: Res<InventoryScreen>,
    item_db: Res<ItemDatabase>,

    parent_query: Query<&ChildOf>,
    corresponding_item_query: Query<&CorrespondingItem>,
    item_query: Query<&AffixiveItem>,
    mut item_slot_query: Query<&mut ItemSlot>
) {
    let item_node = parent_query.get(activate.entity).unwrap().parent();
    let corresponding_item = corresponding_item_query.get(item_node).unwrap().0;

    let item = item_query.get(corresponding_item)
    .expect("Corresponding item entity must have an item component.");

    let mut item_slot = item_slot_query.get_mut(active_slot.0)
    .expect("Active slot resource must have an item slot component.");

    let item_tag = ItemTag::from(item_slot.tag);

    if !item_db.item_has_tag(item, item_tag) {
        return;
    }

    let previous_item = std::mem::replace(&mut item_slot.item, Some(corresponding_item));

    if let Some(previous_item_entity) = previous_item {
        let previous_item = item_query.get(previous_item_entity)
        .expect("Item entity in an item slot must have an item entity.");

        let name = previous_item.name().to_string();

        spawn_inventory_item(commands.reborrow(), &*inventory_screen, previous_item_entity, name.to_string());
    }
    commands.entity(item_node).despawn();

    commands.trigger(Equipped { item: corresponding_item });
}

fn on_inventory_hover(
    event: On<Pointer<Over>>,
    mut commands: Commands,

    corresponding_item_query: Query<&CorrespondingItem>,
    item_query: Query<&AffixiveItem>,
) {
    let item_entity = corresponding_item_query.get(event.entity).expect("Corresponding item must be on this entity.").0;
    let item = item_query.get(item_entity).expect("Item entity must have item component.");
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