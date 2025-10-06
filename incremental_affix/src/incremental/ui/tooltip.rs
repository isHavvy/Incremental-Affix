use bevy::{prelude::*, window::PrimaryWindow};

const TOOLTIP_QUERY_EXPECT_ERROR_MSG: &str = "There should only be one tooltip entity. It should have a node.";

#[derive(Debug, Component, Clone, Copy, PartialEq, Eq)]
enum Tooltip {
    Hidden,
    Moving,
    Fixed,
}

#[derive(Debug)]
pub struct TooltipPlugin;

impl Plugin for TooltipPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, setup)
        .add_systems(PreUpdate, update_tooltip)
        .add_observer(on_fix_tooltip)
        .add_observer(on_show_tooltip)
        .add_observer(on_hide_tooltip)
        ;
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            ..default()
        },
        GlobalZIndex(1),
        Visibility::Hidden,

        Tooltip::Hidden,
    ));
}

fn update_tooltip(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut tooltip_query: Query<(&mut UiTransform, &Tooltip)>,
) {
    let window = window_query.single().expect(TOOLTIP_QUERY_EXPECT_ERROR_MSG);

    let Some(Vec2 { x, y }) = window.cursor_position() else { return; };

    let (mut transform, tooltip) = tooltip_query.single_mut().expect(TOOLTIP_QUERY_EXPECT_ERROR_MSG);
    if *tooltip != Tooltip::Moving { return; }

    transform.translation.x = Val::Px(x);
    transform.translation.y = Val::Px(y);
}

/// Fire this event to freeze the tooltip's position.
#[derive(Debug, Event)]
struct FixTooltip;

fn on_fix_tooltip(
    _event: On<FixTooltip>,
    mut tooltip_query: Query<&mut Tooltip>
) {
    let mut tooltip = tooltip_query.single_mut().expect(TOOLTIP_QUERY_EXPECT_ERROR_MSG);
    
    if *tooltip == Tooltip::Moving {
        *tooltip = Tooltip::Fixed;
    }
}

#[derive(Debug, Event)]
pub struct ShowTooltip {
    pub content: Entity,
}

fn on_show_tooltip(
    event: On<ShowTooltip>,
    mut commands: Commands,

    mut tooltip_query: Query<(&mut Visibility, &mut Tooltip, Entity)>,
) {
    let (mut visibility, mut tooltip, tooltip_entity) = tooltip_query.single_mut().expect(TOOLTIP_QUERY_EXPECT_ERROR_MSG);

    *visibility = Visibility::Visible;
    *tooltip = Tooltip::Moving;

    commands.entity(tooltip_entity)
    .despawn_children()
    .add_child(event.content);
}

#[derive(Debug, Event)]
pub struct HideTooltip;

fn on_hide_tooltip(
    _event: On<HideTooltip>,
    mut tooltip_query: Query<(&mut Visibility, &mut Tooltip)>,
) {
    let (mut visibility, mut tooltip) = tooltip_query.single_mut().expect(TOOLTIP_QUERY_EXPECT_ERROR_MSG);

    *visibility = Visibility::Hidden;
    *tooltip = Tooltip::Hidden;
}