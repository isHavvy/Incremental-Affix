use bevy::prelude::*;

use crate::incremental::item::affixive_item::AffixiveItem;

pub fn spawn_item_details(
    mut commands: Commands,

    item: &AffixiveItem,
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
            Text::new(item.name().to_string()),
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

    let mut prefixes = item.prefixes().peekable();

    if prefixes.peek().is_some() {
        let prefixes_node = commands.spawn((
            Node {
                border: px(1).all(),
                ..default()
            },

            ChildOf(item_box),
        )).id();

        for prefix in prefixes {
            commands.spawn((
                Node {
                    ..default()
                },

                children![(
                    Text::new(format!("P {}", prefix.display())),
                    TextFont { font_size: 14.0, ..default() }
                )],

                ChildOf(prefixes_node),
            ));
        }
    }

    let mut suffixes = item.suffixes().peekable();

    if suffixes.peek().is_some() {
        let suffixes_node = commands.spawn((
            Node {
                border: px(1).all(),
                ..default()
            },

            ChildOf(item_box),
        )).id();

        for suffix in suffixes {
            commands.spawn((
                Node {
                    ..default()
                },

                children![(
                    Text::new(format!("S {}", suffix.display())),
                    TextFont { font_size: 14.0, ..default() }
                )],

                ChildOf(suffixes_node),
            ));
        }
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