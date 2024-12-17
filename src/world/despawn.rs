use bevy::prelude::*;

#[derive(Component, Default)]
pub struct PostDespawn;

pub fn despawn_recursive<C: Component>(mut commands: Commands, query: Query<Entity, With<C>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
