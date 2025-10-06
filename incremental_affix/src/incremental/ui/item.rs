use bevy::prelude::*;

use crate::incremental::item::{affixive_item::AffixiveItem, ItemDatabase};

pub fn spawn_item_details(
    mut commands: Commands,
    item: &AffixiveItem,
    db: &ItemDatabase
) -> Entity {
    let item_box = commands.spawn((
        Node {
            flex_direction: FlexDirection::Column,

            width: px(160),

            border: px(1).all(),

            ..default()
        },
        BackgroundColor(Color::srgb(0.4, 0.4, 0.4)),
        BorderColor::all(Color::BLACK),
    )).id();

    commands.spawn((
        Node {
            border: px(1).bottom(),
            justify_content: JustifyContent::Center,

            ..default()
        },
        BorderColor::all(Color::BLACK),

        children![(
            Text::new(db.item_name(&item)),
            TextFont { font_size: 16.0, ..default() }
        )],

        ChildOf(item_box),
    ));

    for implicit in item.implicits() {
        commands.spawn((
            Node {
                ..default()
            },

            children![(
                Text::new(format!("* {}", implicit.display())),
                TextFont { font_size: 14.0, ..default() }
            )],

            ChildOf(item_box),
        ));
    }

    for tag in item.tags.iter().copied() {
        commands.spawn((
            Node {
                ..default()
            },

            children![(
                Text::new(format!("[{}]", tag)),
                TextFont { font_size: 14.0, ..default() }
            )],

            ChildOf(item_box),
        ));
    }

    item_box
}