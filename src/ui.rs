use bevy::prelude::*;

use crate::common::*;

#[derive(Component)]
pub struct Background;

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_background)
            .add_systems(Update, update_background);
    }
}

fn setup_background(mut commands: Commands, settings_assets: Res<SettingsAsset>) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(1.0, 1.0, 1.0, BACKGROUND_TRANSPARENT),
                ..default()
            },
            texture: settings_assets.background.clone(),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, BACK as f32)),
            ..default()
        },
        Background,
    ));
}

fn update_background(
    mut background_query: Query<&mut Sprite, With<Background>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyB) {
        if let Ok(mut sprite) = background_query.get_single_mut() {
            if sprite.color.a() == BACKGROUND_TRANSPARENT {
                sprite.color = Color::rgba(1.0, 1.0, 1.0, 1.0);
            } else {
                sprite.color = Color::rgba(1.0, 1.0, 1.0, BACKGROUND_TRANSPARENT);
            }
        }
    }
}

#[derive(Component)]
pub struct SettingsMenuComponent;

#[derive(Resource, Debug, Default)]
pub struct SettingsAsset {
    // others

    // settings font
    pub font: Handle<Font>,
    // background image
    // if empty, use default background color
    // if use relative path, it is relative to the assets/ directory
    pub background: Handle<Image>,
}

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, setup_settings_system)
            .add_systems(Update, pause_or_continue_system)
            .add_systems(OnEnter(GameState::Setting), display_settings_menu_system)
            .add_systems(
                OnExit(GameState::Setting),
                despawn_entities_system::<SettingsMenuComponent>,
            );
    }
}

fn setup_settings_system(
    mut settings_assets: ResMut<SettingsAsset>,
    asset_server: Res<AssetServer>,
) {
    let font = asset_server.load("fonts/CommitMonoNerdFont-Bold.otf");
    let background = asset_server.load("textures/background.png");
    *settings_assets = SettingsAsset { font, background };
}

fn pause_or_continue_system(
    mut state: ResMut<State<GameState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        match state.get() {
            GameState::Playing => {
                *state = State::new(GameState::Setting);
                info!("Pause");
            }
            GameState::Setting => {
                *state = State::new(GameState::Playing);
                info!("Continue");
            }
            _ => {}
        }
    }
}

fn display_settings_menu_system(mut commands: Commands, settings_assets: Res<SettingsAsset>) {
    commands
        .spawn((
            SettingsMenuComponent,
            NodeBundle {
                style: Style {
                    width: Val::Percent(60.0),
                    height: Val::Percent(60.0),
                    ..default()
                },
                z_index: ZIndex::Global(FRONT),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text {
                    sections: vec![TextSection {
                        value: "Settings".into(),
                        style: TextStyle {
                            font: settings_assets.font.clone(),
                            font_size: 60.0,
                            color: Color::WHITE,
                        },
                    }],
                    ..default()
                },
                ..default()
            });
        });
}

#[derive(Component)]
pub struct StartMenuComponent;

pub struct StartMenuPlugin;

impl Plugin for StartMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, display_start_menu_system)
            .add_systems(OnEnter(GameState::StartMenu), display_start_menu_system)
            .add_systems(
                OnExit(GameState::StartMenu),
                despawn_entities_system::<StartMenuComponent>,
            )
            .add_systems(
                Update,
                game_start_system.run_if(in_state(GameState::StartMenu)),
            );
    }
}

fn display_start_menu_system(
    mut commands: Commands,
    start_menu_query: Query<Entity, With<StartMenuComponent>>,
    settings_assets: Res<SettingsAsset>,
) {
    if start_menu_query.iter().len() == 0 {
        commands
            .spawn((
                StartMenuComponent,
                NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    z_index: ZIndex::Global(MIDDLE),
                    ..default()
                },
            ))
            .with_children(|parent| {
                parent.spawn(TextBundle {
                    style: Style {
                        width: Val::Px(200.0),
                        height: Val::Px(70.0),
                        border: UiRect::all(Val::Px(2.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    text: Text {
                        sections: vec![TextSection {
                            value: "Tetris".into(),
                            style: TextStyle {
                                font: settings_assets.font.clone(),
                                font_size: 60.0,
                                color: Color::WHITE,
                            },
                        }],
                        ..default()
                    },
                    transform: Transform::from_translation(Vec3::new(0.0, 200.0, MIDDLE as f32)),
                    ..default()
                });
                parent
                    .spawn(ButtonBundle {
                        style: Style {
                            width: Val::Px(150.0),
                            height: Val::Px(65.0),
                            border: UiRect::all(Val::Px(3.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        transform: Transform::from_translation(Vec3::new(
                            0.0,
                            -100.0,
                            MIDDLE as f32,
                        )),
                        border_color: BorderColor(Color::WHITE),
                        background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                        ..default()
                    })
                    .with_children(|parent| {
                        parent.spawn(TextBundle::from_section(
                            "Play",
                            TextStyle {
                                font: settings_assets.font.clone(),
                                font_size: 40.0,
                                color: Color::rgb(0.9, 0.5, 0.8),
                            },
                        ));
                    });
            });
    } else {
        warn!("StartMenuComponent already exists");
    }
}

fn game_start_system(
    interaction_query: Query<&Interaction, With<Button>>,
    mut state: ResMut<State<GameState>>,
) {
    if let Ok(interaction) = interaction_query.get_single() {
        if *interaction == Interaction::Pressed {
            info!("Game Start");
            *state = State::new(GameState::Playing);
        }
    }
}
