use bevy::prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgba_u8(27, 21, 45, 235)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 0.5,
        })
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Tetris".to_string(),
                position: WindowPosition::new(IVec2::new(100, 100)),
                ..default()
            }),
            ..default()
        }))
        .run();
}