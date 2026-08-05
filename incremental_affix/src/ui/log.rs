//! Message log UI

use bevy::prelude::*;
use bevy::ui_widgets::ScrollArea;

use crate::incremental::log::LogEntry;

pub struct LogUiPlugin;

impl Plugin for LogUiPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Update, (handle_log_event,));
    }
}

// #[TODO(Havvy)]: Implement scrollbars.
/// Creates a scene for the log's UI.
/// 
/// This function should only ever be called once in a World.
pub fn log_ui() -> impl Scene {
    bsn! {
        Node {
            flex_direction: FlexDirection::Column,
            max_height: px(256),
            overflow: Overflow::scroll_y(),
            scrollbar_width: 16.0,
        }
        BackgroundColor(Color::srgb_u8(15, 15, 15))
        ScrollArea

        LogUi

        Children[]
    }
}

#[derive(Debug, Clone, Default, Component)]
#[require(Node, Children)]
struct LogUi;

fn handle_log_event(
    mut commands: Commands,
    mut log_events: MessageReader<LogEntry>,
    log_ui_entity: Single<Entity, With<LogUi>>,
) {
    let log_ui_entity = *log_ui_entity;
    let log_events = log_events.read().map(|lm| &lm.0).cloned();

    for message in log_events {
        commands.spawn_scene(bsn! {
            ChildOf(log_ui_entity)
            Text(message)
        });
    }
}