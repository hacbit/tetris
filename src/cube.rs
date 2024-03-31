use bevy::prelude::*;

pub struct CubePlugin;

impl Plugin for CubePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_cubes);
    }
}

fn spawn_cubes(_commands: Commands) {}
