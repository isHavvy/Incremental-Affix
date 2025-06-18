use bevy::{color::palettes::css::BLACK, ecs::relationship::RelatedSpawnerCommands, prelude::*};

use crate::incremental::{self, actions::Actions, screens::Screens, StockKind, Stockyard};

#[derive(Debug, Default, Resource)]
pub struct ActionProgressBar(Option<Entity>);

/// I think ActionProgress exists somewhere else? Not sure.
/// This is a dummy struct to get the code to compile since
/// I left it in a non-compilable state.
#[derive(Debug, Default, Resource)]
pub struct ActionProgress;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
        .init_resource::<ActionProgress>()
        .init_resource::<ActionProgressBar>()
        .add_systems(Startup, setup)
        .add_systems(Update, (
            handle_action_click,
            update_resources_sidebar,
            update_action_progress_bar
        ))
        ;
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    action_progress: Res<ActionProgress>,
    action_progress_bar: ResMut<ActionProgressBar>,
) {
    let font = asset_server.load::<Font>("fonts/FiraSans-Bold.ttf");

    commands.spawn(Camera2d::default());

    let mut root_node = commands.spawn(Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        align_items: AlignItems::Center,
        //justify_content: JustifyContent::Center,
        ..default()
    });

    root_node.with_children(|root_node| {
        root_node
        .spawn((
            Node {
                width: Val::Percent(20.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                border: UiRect::right(Val::Px(2.0)),
                ..default()
            },
            BackgroundColor(Color::WHITE.into()),
            BorderColor(Color::BLACK),
        ))
        .with_children(|sidebar| { setup_resources_sidebar(sidebar, font.clone()); })
        ;

        root_node
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            border: UiRect::bottom(Val::Px(2.0)),
            ..default()
        })
        .with_children(|right_zone| {
            // Top bar with the screen switching stuff.
            right_zone.spawn((
                Node {
                    height: Val::Px(48.0),
                    width: Val::Percent(100.0),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.0, 0.8, 0.0).into()),
            ))
            .with_children(|top_bar| { setup_screens_bar(top_bar, font.clone()); });

            // Main Content Zone
            right_zone.spawn((
                Node {
                    flex_direction: FlexDirection::Column,
                    height: Val::Percent(100.0),
                    ..default()
                },
                BackgroundColor(Color::WHITE.into()),
            ))
            .with_children(|zone| initialize_actions(
                zone,
                font.clone(),
                &action_progress,
                action_progress_bar)
            )
            ;
        });
    });
}

fn setup_resources_sidebar(sidebar: &mut RelatedSpawnerCommands<'_, ChildOf>, font: Handle<Font>) {
    sidebar.spawn((
        Text::new("Resources"),
        TextFont {
            font,
            font_size: 30.0,
            ..default()
        },
        TextColor(Color::srgb(0.9, 0.9, 0.9)),
        // margin: UiRect::bottom(Val::Px(20.0)),
    ));

    for resource in crate::incremental::StockKind::LIST {
        sidebar.spawn((
            Node {
                ..default()
            },
            Text::new("0.0 / 100"),
            TextLayout::new_with_justify(JustifyText::Right),
            TextColor(Color::BLACK.into()),
            *resource,
        ));
    }
}

fn setup_screens_bar(bar: &mut RelatedSpawnerCommands<'_, ChildOf>, font: Handle<Font>) {
    for screen in Screens::LIST.iter().cloned() {
        bar.spawn((
            Button,
            Node {
                border: UiRect::all(Val::Px(2.)),
                height: Val::Px(40.0),
                width: Val::Auto,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect::right(Val::Px(5.0)),
                ..default()
            },
            BorderColor(Color::BLACK.into()),
            children![
                Text(screen.to_string()),
                TextColor(Color::BLACK.into()),
                TextFont {
                    font: font.clone(),
                    font_size: 20.0,
                    ..default()
                },
            ],
            screen
        ));
    }
}

fn initialize_actions(
    node: &mut RelatedSpawnerCommands<'_, ChildOf>, font: Handle<Font>,
    action_progress: &ActionProgress,
    mut res_action_progress_bar: ResMut<ActionProgressBar>,
) {
    let _action_progress_bar = node.spawn((
        Node {
            border: UiRect::all(Val::Px(2.)),
            height: Val::Px(25.0),
            width: Val::Px(400.0),
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::Center,
            margin: UiRect::all(Val::Px(8.0)),
            ..default()
        },
        BorderColor(Color::BLACK),
        children![]
    ))
    .with_children(|node| {
        let action_progress_bar = node.spawn((
            Node {
                height: Val::Percent(100.0),
                width: Val::Percent(/*action_progress.0*/ 100.0),
                ..default()
            },
            BackgroundColor(Color::srgb(1.0, 0.0, 0.0).into()),
        )).id();

        *res_action_progress_bar = ActionProgressBar(Some(action_progress_bar));
    });

    for action in Actions::LIST.iter().cloned() {
        node.spawn((
            Node {
                border: UiRect::all(Val::Px(2.)),
                height: Val::Px(25.0),
                width: Val::Px(200.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect::all(Val::Px(8.0)),
                ..default()
            },
            BorderColor(Color::BLACK.into()),
        ))
        .with_child((
            Button,
            action,
            Text::new(action.to_string()),
            TextFont {
                font: font.clone(),
                font_size: 20.0,
                ..default()
            },
            TextColor(Color::BLACK.into()),
        ));
    }
}

fn update_resources_sidebar(
    mut query: Query<(&mut Text, &incremental::StockKind)>,
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

fn update_action_progress_bar(
    progress: Res<ActionProgress>,
    progress_bar: Res<ActionProgressBar>,
    mut progress_bar_style_query: Query<(&mut Node,)>,
) {
    if let Some(progress_bar) = progress_bar.0 {
        let Ok(mut node) = progress_bar_style_query.get_mut(progress_bar) else { panic!("Progress bar entity must have a style.")};

        node.0.width = Val::Px(400.0 * (1.) /*progress.0 */);
    }
}

fn handle_action_click(
    query: Query<(&Interaction, &Actions), (Changed<Interaction>, With<Button>,)>,
    mut stockyard: ResMut<Stockyard>,
) {
    for (interaction, action) in &query {
        match interaction {
            Interaction::Pressed => {
                match action {
                    Actions::Explore => todo!(),
                    Actions::GatherWood => { stockyard[StockKind::Wood] += 100; },
                    Actions::CreateFollowers => todo!(),
                }
            },
            Interaction::Hovered => {},
            Interaction::None => {},
        }
    }
}