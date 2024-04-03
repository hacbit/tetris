mod common;
mod cube;
mod map;
mod ui;

use common::*;
use map::*;
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
        .init_state::<GameState>()
        .add_systems(Startup, setup_camera_system)
        .add_systems(OnEnter(GameState::GameOver), back_to_start_menu_system)
        // initialize settings
        .add_plugins(SettingsPlugin)
        // load background image and draw it
        // load start menu, handle button click event and switch to other states
        // draw game map while in Playing state
        .add_plugins((BackgroundPlugin, StartMenuPlugin, MapPlugin))
        .run();
}

fn setup_camera_system(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
