mod common;
mod cube;
mod ui;

use common::*;
use ui::*;

use bevy::prelude::*;

fn main() {
    App::new()
        //.insert_resource(ClearColor(Color::rgba_u8(0x24, 0x3d, 0x51, 0xff)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 0.5,
        })
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Tetris".to_string(),
                        position: WindowPosition::new(IVec2::new(100, 100)),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .init_resource::<SettingsAsset>()
        .insert_resource(State::new(GameState::StartMenu))
        .add_systems(Startup, setup_camera)
        // initialize settings
        .add_plugins(SettingsPlugin)
        // load background image and draw it
        .add_plugins((BackgroundPlugin, StartMenuPlugin))
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
