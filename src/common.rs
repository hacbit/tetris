use bevy::{
    prelude::*,
    ui::{FocusPolicy, UiPassNode},
};

#[derive(Resource, Debug, Default)]
pub struct BackgroundAsset {
    pub background: Handle<Image>,
}

#[derive(Component)]
pub struct Background;

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BackgroundAsset>()
            .add_systems(Startup, setup_background)
            .add_systems(Update, draw_background);
    }
}

fn setup_background(
    mut background_asset: ResMut<BackgroundAsset>,
    settings_asset: Res<SettingsAsset>,
    asset_server: Res<AssetServer>,
) {
    *background_asset = BackgroundAsset {
        background: asset_server.load(settings_asset.background_path.clone()),
    };
}

fn draw_background(mut commands: Commands, background_asset: Res<BackgroundAsset>) {
    commands
        .spawn((NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            ..default()
        },))
        .with_children(|parent| {
            parent.spawn((
                ImageBundle {
                    image: background_asset.background.clone().into(),
                    ..default()
                },
                Background,
            ));
        });
}

#[derive(Resource, Debug, Default)]
pub struct SettingsAsset {
    // others
    // specify background image path
    // if empty, use default background color
    // if use relative path, it is relative to the assets/ directory
    pub background_path: String,
    // if true and background_path is valid, use specified background image
    pub enable_background_image: bool,
}

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SettingsAsset {
            background_path: "textures/background.png".to_string(),
            enable_background_image: true,
        });
    }
}

#[derive(States, Default, Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    #[default]
    StartMenu,
    Playing,
    Setting,
    GameOver,
}
