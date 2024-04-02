use bevy::prelude::*;

pub const BACK: i32 = -1;
pub const MIDDLE: i32 = 0;
pub const FRONT: i32 = 1;
pub const BACKGROUND_TRANSPARENT: f32 = 0.3;

#[derive(States, Default, Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    #[default]
    StartMenu,
    Playing,
    Setting,
    GameOver,
}

pub fn despawn_entities_system<T: Component>(
    mut commands: Commands,
    query: Query<Entity, With<T>>,
) {
    for entity in query.iter() {
        if let Some(entity_commands) = commands.get_entity(entity) {
            entity_commands.despawn_recursive();
            info!("Despawned {:?}", std::any::type_name::<T>());
        } else {
            warn!(
                "Failed to despawn {:?}, entity not found",
                std::any::type_name::<T>()
            );
        }
    }
}
