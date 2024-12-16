use avian2d::prelude::PhysicsLayer;
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
