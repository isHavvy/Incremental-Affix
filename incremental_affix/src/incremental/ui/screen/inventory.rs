use bevy::prelude::*;
use bevy::ui_widgets::{observe, Activate, Button};

use crate::incremental::{item::{affixive_item::{AffixiveItem, ItemTag}, item_slot::ItemSlotTag, ItemDatabase}, ui::screen::Screen};

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

fn spawn_slot(mut commands: Commands, parent: Entity, slot_kind: ItemSlotTag) -> Entity {
    let container = commands.spawn((
        Node {
            box_sizing: BoxSizing::BorderBox,
            width: px(150),
            margin: px(4).all(),
            border: px(2).all(),

            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            
            ..Default::default()
        },
        BorderColor::all(Color::BLACK),
        BackgroundColor(Color::srgb(0., 0.7, 0.)),

        Text::new("Test"),

        slot_kind,

        ChildOf(parent),
    )).id();

    let _slot_name = commands.spawn((
        Text::new("Tool"),
        ChildOf(container),
    ));

    return container;
}

pub fn spawn_inventory_item(
    commands: &mut Commands,
    inventory_screen: &InventoryScreen,
    item_entity: Entity,
    item_name: String,
) {
    let line = commands.spawn((
        Node {
        ..default()
        },
        CorrespondingItem(item_entity),
        ChildOf(inventory_screen.get())
    )).id();

    commands.spawn((
        Node {
            border: px(1).all(),
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

    item_database: Res<ItemDatabase>,
    active_slot: Res<ActiveSlot>,

    //button_query: Query<(&Interaction, &ChildOf), (Changed<Interaction>, With<Button>, With<EquipAction>)>,
    parent_query: Query<&ChildOf>,
    corresponding_item_query: Query<&CorrespondingItem>,
    item_query: Query<&AffixiveItem>,
    item_slot_tag_query: Query<&ItemSlotTag>
) {
    let item_slot = parent_query.get(activate.entity).unwrap().parent();
    let corresponding_item = corresponding_item_query.get(item_slot).unwrap().0;
    let item = item_query
    .get(corresponding_item)
    .expect("Corresponding item entity must have an item.");

    println!("{}", item_database.display_item(item));

    let item_slot_tag = *item_slot_tag_query.get(active_slot.0)
    .expect("Active item slot must have a tag.");

    let item_tag = ItemTag::from(item_slot_tag);

    if !item_database.item_has_tag(item, item_tag) {
        return;
    }
}