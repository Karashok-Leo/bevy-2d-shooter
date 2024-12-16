use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Despawn;

pub fn auto_despawn(mut commands: Commands, query: Query<Entity, With<Despawn>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
