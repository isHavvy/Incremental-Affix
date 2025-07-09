use bevy::{color::palettes::css::BLACK, ecs::relationship::RelatedSpawnerCommands, prelude::*};

use crate::incremental::{self, actions::{ActionProgress, Actions, CurrentAction}, screens::Screen, StockKind, Stockyard};

#[derive(Debug, Default, Resource)]
pub struct ActionProgressBar(Option<Entity>);

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
        .init_resource::<ActionProgressBar>()
        .add_systems(Startup, setup)
        .add_systems(Update, (
            handle_action_click,
            handle_screen_click,
            update_resources_sidebar,
            update_action_progress_bar
        ))
        ;
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    action_progress: Res<incremental::actions::ActionProgress>,
    action_progress_bar: ResMut<ActionProgressBar>,
) {
    let font = asset_server.load::<Font>("fonts/FiraSans-Bold.ttf");

    commands.spawn(Camera2d::default());

    let root_node = commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            //justify_content: JustifyContent::Center,
            ..default()
        },
    ))
    .id();

    let sidebar = commands.spawn((
        Node {
            width: Val::Percent(20.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            border: UiRect::right(Val::Px(2.0)),
            ..default()
        },
        BackgroundColor(Color::WHITE.into()),
        BorderColor(Color::BLACK),
        ChildOf(root_node)
    ))
    .id();

    setup_resources_sidebar(&mut commands, sidebar, font.clone());

    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            border: UiRect::bottom(Val::Px(2.0)),
            ..default()
        },
        ChildOf(root_node)
    ))
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
            Screen::Act,
        ))
        .with_children(|zone| initialize_actions(
            zone,
            font.clone(),
            &action_progress,
            action_progress_bar)
        )
        ;

        right_zone.spawn((
            Node {
                flex_direction: FlexDirection::Column,
                height: Val::Percent(100.0),
                display: Display::None,
                ..default()
            },
            BackgroundColor(Color::WHITE),
            Screen::Craft,
        ));

        right_zone.spawn((
            Node {
                flex_direction: FlexDirection::Column,
                height: Val::Percent(100.0),
                display: Display::None,
                ..default()
            },
            BackgroundColor(Color::WHITE),
            Screen::Inventory,
        ));

        right_zone.spawn((
            Node {
                flex_direction: FlexDirection::Column,
                height: Val::Percent(100.0),
                display: Display::None,
                ..default()
            },
            BackgroundColor(Color::WHITE),
            Screen::Population,
        ));
    });
}

fn setup_resources_sidebar(commands: &mut Commands, sidebar: Entity, font: Handle<Font>) {
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

    commands.spawn_batch(crate::incremental::StockKind::LIST.into_iter().map(move |resource| (
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

fn setup_screens_bar(bar: &mut RelatedSpawnerCommands<'_, ChildOf>, font: Handle<Font>) {
    for screen in Screen::LIST.iter().cloned() {
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
            BorderColor(Color::BLACK),
            children![
                Text(screen.to_string()),
                TextColor(Color::BLACK),
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
    action_progress: &incremental::actions::ActionProgress,
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
    ))
    .with_children(|node| {
        let action_progress_bar = node.spawn((
            Node {
                height: Val::Percent(100.0),
                width: Val::Percent(action_progress.0),
                align_content: AlignContent::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(1.0, 0.0, 0.0).into()),
            Text::new(""),
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
            children![(
                Button,
                action,
                Text::new(action.to_string()),
                TextFont {
                    font: font.clone(),
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::BLACK.into()),
            )],
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
    progress: Res<incremental::actions::ActionProgress>,
    progress_bar: Res<ActionProgressBar>,
    mut progress_bar_style_query: Query<&mut Node>,
) {
    if let Some(progress_bar) = progress_bar.0 {
        let Ok(mut node) = progress_bar_style_query.get_mut(progress_bar) else { panic!("Progress bar entity must have a style.")};

        node.width = Val::Px(400.0 * progress.0);
    }
}

fn handle_action_click(
    button_query: Query<(&Interaction, &Actions), (Changed<Interaction>, With<Button>,)>,
    // mut stockyard: ResMut<Stockyard>,
    mut current_action: ResMut<CurrentAction>,
    mut action_progress: ResMut<ActionProgress>,
) {
    for (interaction, action) in &button_query {
        match interaction {
            Interaction::Pressed => {
                match action {
                    Actions::Explore => {
                        // Don't reset progress if they re-clicked on the same action.
                        if current_action.0 == Some(Actions::Explore) {
                            continue;
                        }
                        
                        current_action.0 = Some(Actions::Explore);
                        action_progress.0 = 0.0
                    },
                    Actions::GatherWood => {
                        // Don't reset progress if they re-clicked on the same action.
                        if current_action.0 == Some(Actions::GatherWood) {
                            continue;
                        }

                        current_action.0 = Some(Actions::GatherWood);
                        action_progress.0 = 0.0;
                    },
                    Actions::CreateFollowers => todo!(),
                }
            },
            Interaction::Hovered => {},
            Interaction::None => {},
        }
    }
}

fn handle_screen_click(
    button_query: Query<(&Interaction, &Screen), (Changed<Interaction>, With<Button>)>,
    mut screen_query: Query<(&mut Node, &Screen), Without<Button>>,
) {
    for (interaction, next_visible_screen) in &button_query {
        match interaction {
            Interaction::Pressed => {},
            Interaction::Hovered => continue,
            Interaction::None => continue,
        }

        for (mut screen_node, screen) in &mut screen_query {
            screen_node.display = if screen == next_visible_screen { Display::Block } else { Display::None };
        }
    }
}