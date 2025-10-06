use bevy::prelude::*;

use crate::incremental::{StockKind, Stockyard};

pub fn spawn_stocks_ui(commands: &mut Commands, sidebar: Entity, font: Handle<Font>) {
    commands.spawn((
        Node {
            margin: UiRect::bottom(Val::Px(20.0)),
            ..default()
        },
        Text::new("Resources"),
        TextFont {
            font,
            font_size: 30.0,
            ..default()
        },
        //TextColor(Color::srgb(0.9, 0.9, 0.9)),
        TextColor(Color::BLACK),
        ChildOf(sidebar),
    ));

    for stock_kind in StockKind::LIST.iter().cloned() {
        spawn_stocks_stock_kind_line(commands.reborrow(), sidebar, stock_kind);
    }
}

fn spawn_stocks_stock_kind_line(mut commands: Commands, parent: Entity, stock_kind: StockKind) {
    let container = commands.spawn((
        Node {
            flex_direction: FlexDirection::Row,
            ..default()
        },
        ChildOf(parent),
    )).id();

    commands.spawn((
        Text::new(stock_kind.to_string()),
        TextLayout::new_with_justify(Justify::Left),
        TextColor(Color::BLACK),
        ChildOf(container)
    ));

    commands.spawn((
        Node {
            ..default()
        },
        Text::new("0.0 / 100"),
        TextLayout::new_with_justify(Justify::Right),
        TextColor(Color::BLACK),
        stock_kind,
        ChildOf(container)
    ));
}

pub fn update_resources_sidebar(
    mut query: Query<(&mut Text, &StockKind)>,
    mut stockyard: ResMut<Stockyard>,
) {
    for (mut text, stock_kind) in query.iter_mut() {
        let stock = &mut stockyard[*stock_kind];

        if !stock.has_changed() {
            continue;
        }

        text.clear();
        stock.push_str(&mut *text);
        **text = format!("{}.{:0>2}", stock.current / 100, stock.current % 100).into();

        if let Some(maximum) = stock.maximum {
            text.push_str(&mut format!("/ {}", maximum / 100));
        }
    }
}