use bevy::prelude::*;

use crate::incremental::{StockKind, Stockyard};

pub fn setup_resources_sidebar(commands: &mut Commands, sidebar: Entity, font: Handle<Font>) {
    commands.spawn((
        Text::new("Resources"),
        TextFont {
            font,
            font_size: 30.0,
            ..default()
        },
        TextColor(Color::srgb(0.9, 0.9, 0.9)),
        // margin: UiRect::bottom(Val::Px(20.0)),
        ChildOf(sidebar),
    ));

    commands.spawn_batch(StockKind::LIST.into_iter().map(move |resource| (
        Node {
            ..default()
        },
        Text::new("0.0 / 100"),
        TextLayout::new_with_justify(JustifyText::Right),
        TextColor(Color::BLACK.into()),
        *resource,
        ChildOf(sidebar)
    )));
}

pub fn update_resources_sidebar(
    mut query: Query<(&mut Text, &StockKind)>,
    stockyard: Res<Stockyard>,
) {
    for (mut text, stock_kind) in query.iter_mut() {
        let stock = &stockyard[*stock_kind];
        **text = format!("{}.{:0>2}", stock.current / 100, stock.current % 100).into();

        if let Some(maximum) = stock.maximum {
            text.push_str(&mut format!("/ {}", maximum / 100));
        }
    }
}