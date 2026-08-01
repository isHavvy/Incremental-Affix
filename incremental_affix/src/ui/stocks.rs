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
            font: FontSource::Handle(font),
            font_size: FontSize::Px(30.0),
            ..default()
        },
        TextColor(Color::BLACK),
        ChildOf(sidebar),
    ));

    for stock_kind in StockKind::LIST.iter().cloned() {
        commands.spawn_scene(bsn! {
            stock_kind_line(stock_kind)
            ChildOf(sidebar)
        });
    }
}

fn stock_kind_line(stock_kind: StockKind) -> impl Scene {
    bsn! {
        Node { flex_direction: FlexDirection::Row }
        Children [
            Text::new(stock_kind.to_string())
            TextLayout::justify(Justify::Left),
            TextColor(Color::BLACK),

            Node { flex_grow: 1.0 }
            Text::new("")
            TextLayout::justify(Justify::Right)
            template_value(stock_kind)
            Children [
                TextSpan::new("(0.00)")
                TextColor(Color::BLACK)
                TextFont { font_size: px(14.0) },

                TextSpan::new(" "),

                TextSpan::new("0.00/100")
                TextColor(Color::BLACK)
                TextFont { font_size: px(14.0) }
            ]
        ]
    }
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