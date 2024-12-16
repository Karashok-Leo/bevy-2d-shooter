use avian2d::prelude::PhysicsLayer;

#[derive(PhysicsLayer, Default)]
pub enum CollisionLayer {
    #[default]
    Player,
    Enemy,
    Bullet,
}
