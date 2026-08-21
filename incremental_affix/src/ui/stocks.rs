use std::{collections::HashMap, fmt::Write};

use bevy::prelude::*;
use bevy::text::FontSourceTemplate;

use crate::incremental::{DotPerSecond, PerSecond, stock::{StockKind, producer_consumer::StockyardProducerConsumer, stockyard::Stockyard}};

pub fn stockyard_ui() -> impl Scene {
    let stock_lines: Vec<_> = StockKind::LIST.iter().cloned().map(stock_kind_line).collect();

    bsn!{
        Node {
            margin: UiRect::bottom(px(20)),
            flex_direction: FlexDirection::Column,
        }

        Children[
            Text::new("Resources")
            TextFont {
                font: FontSourceTemplate::Handle("fonts/FiraSans-Bold.ttf"),
                font_size: px(30)
            }
            TextColor(Color::BLACK)
            Underline,

            // ---

            { stock_lines }
        ]
    }
}

fn stock_kind_line(stock_kind: StockKind) -> impl Scene {
    bsn! {
        Node {
            flex_direction: FlexDirection::Row,
            width: px(300),
            overflow: Overflow::clip(),
        }
        template_value(stock_kind)
        Children [
            (
            // Name
            stock_kind_cell(80)
            Text::new(stock_kind.to_string())
            ),

            (
            // Current Stock
            stock_kind_cell(50)
            Text::new("0.00")
            ),

            (
            // Max Stock
            stock_kind_cell(60)
            Text::new(format!("/{}", "100"))
            TextColor(Color::srgb(0.4, 0.4, 0.4))
            ),

            (
            // Diff
            stock_kind_cell(70)
            // This invisible 1 pixel border somehow prevents
            // this cell from overflowing to a second line when
            // it has text. Why? No clue.
            Node {
               border: { px(1).right() }
            }
            BorderColor::all(Color::srgba(0.0, 0.0, 0.0, 0.0))
            Text::new("")
            )
        ]
    }
}

fn stock_kind_cell(width: usize) -> impl Scene {
    bsn! {
        Node { width: px(width), }
        TextColor::BLACK
        TextFont { font_size: px(12) }
    }
}

pub fn update_stockyard_sidebar(
    mut query: Query<(&Children, &StockKind)>,
    mut text_query: Query<&mut Text>,

    mut stockyard: ResMut<Stockyard>,

    sps_query: Query<&StockyardProducerConsumer>,

    mut changes_per_second: Local<HashMap<StockKind, PerSecond>>,
) {
    for (children, stock_kind) in query.iter_mut() {
        let stock = &mut stockyard[*stock_kind];

        if stock.has_changed() {
            let mut current_text = text_query.get_mut(children[1]).unwrap();
            current_text.clear();
            stock.push_str_current(&mut current_text.0);
        }

        // #[TODO(Havvy)]: Update this when the maximum changes. Currently it doesn't, so :shrug:.
        let _maximum_text = text_query.get_mut(children[2]).unwrap();

        let mut change_text = text_query.get_mut(children[3]).unwrap();

        let change_per_second: PerSecond = sps_query.iter()
        .map(|sps| sps.per_second_for_stock(*stock_kind))
        .sum();

        let last_change_per_second = changes_per_second.entry(*stock_kind).or_insert_with(|| 0.per_second());

        if *last_change_per_second != change_per_second {
            change_text.clear();
            let _ = write!(&mut change_text.0, "{}", change_per_second);
            *last_change_per_second = change_per_second;
        }
    }
}