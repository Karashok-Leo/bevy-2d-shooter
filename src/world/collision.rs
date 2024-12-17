use avian2d::prelude::{PhysicsLayer, RigidBody, RigidBodyDisabled};
use bevy::ecs::query::{QueryData, QueryFilter};
use bevy::prelude::*;

#[derive(PhysicsLayer, Default)]
pub enum CollisionLayer {
    #[default]
    Player,
    Enemy,
    Bullet,
}

pub fn try_parse_collider<D: QueryData, F: QueryFilter>(
    entity1: Entity,
    entity2: Entity,
    query: &Query<D, F>,
) -> Option<Entity> {
    if query.contains(entity1) {
        Some(entity1)
    } else if query.contains(entity2) {
        Some(entity2)
    } else {
        None
    }
}

pub fn enable_rigid_bodies(mut commands: Commands, query: Query<Entity, With<RigidBodyDisabled>>) {
    for entity in query.iter() {
        commands.entity(entity).remove::<RigidBodyDisabled>();
    }
}

pub fn disable_rigid_bodies(mut commands: Commands, query: Query<Entity, With<RigidBody>>) {
    for entity in query.iter() {
        commands.entity(entity).insert(RigidBodyDisabled);
    }
}
