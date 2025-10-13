//! Message log for the game.

use bevy::prelude::*;

pub struct GameLogPlugin;

impl GameLogPlugin {
    /// Builds the log UI and registers the the game log ui entity for the system.
    /// 
    /// Only call once in the application.
    pub fn setup_log_ui(mut commands: Commands, parent: Entity) {
        let log_ui = commands.spawn((
            ChildOf(parent),
            Node {
                ..default()
            },
            BackgroundColor(Color::srgb_u8(15, 15, 15)),
        )).id();

        commands.insert_resource(LogUi(log_ui));
    }
}

#[derive(Debug, Resource)]
struct LogUi(Entity);

impl LogUi {
    fn entity(&self) -> Entity {
        self.0.clone()
    }
}

// #[TODO(Havvy)]: Move this LogMessage to crate::incremental::log::LogMessage.
//                 Currently this is the only reference to crate::ui
//                 from crate::incremental, and that dependency needs
//                 to be broken.
#[derive(Debug, Message)]
pub struct LogMessage(pub String);

impl Plugin for GameLogPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_message::<LogMessage>()
        .add_systems(Update, (handle_log_event,));
    }
}

fn handle_log_event(
    mut commands: Commands,
    mut log_events: MessageReader<LogMessage>,
    log_ui: Res<LogUi>,
) {
    for event in log_events.read() {
        commands.spawn((
            ChildOf(log_ui.entity()),
            Text(event.0.clone())
        ));
    }
}