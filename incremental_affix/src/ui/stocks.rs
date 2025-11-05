use bevy::prelude::*;

use crate::incremental::stock::{StockKind, stockyard::Stockyard};

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
            flex_grow: 1.0,
            ..default()
        },

        Text::new(""),
        TextLayout::new_with_justify(Justify::Right),

        stock_kind,

        children![
            (TextSpan::new("(0.00)"), TextColor(Color::BLACK), TextFont { font_size: 14.0, ..default() },),
            TextSpan::new(" "),
            (TextSpan::new("0.00/100"), TextColor(Color::BLACK), TextFont { font_size: 14.0, ..default() },),
        ],

        ChildOf(container)
    ));
}

pub fn update_resources_sidebar(
    mut query: Query<(&Children, &StockKind), With<Text>>,
    mut span_query: Query<&mut TextSpan>,
    mut stockyard: ResMut<Stockyard>,
) {
    for (children, stock_kind) in query.iter_mut() {
        let stock = &mut stockyard[*stock_kind];

        if !stock.has_changed() {
            continue;
        }

        let change_text_span = &mut **span_query.get_mut(children[2]).unwrap();
        change_text_span.clear();
        stock.push_str_current_and_maximum(change_text_span);

        let value_text_span = &mut **span_query.get_mut(children[0]).unwrap();
        value_text_span.clear();
        stock.push_str_change_per_second(value_text_span);
    }
}