use crate::state::GameState;
use crate::KD_TREE_REFRESH_RATE;
use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use kd_tree::{KdPoint, KdTree};
use std::time::Duration;

#[derive(Component)]
pub struct Collider;

pub struct ColliderEntity {
    pub pos: Vec2,
    pub entity: Entity,
}

#[derive(Resource)]
pub struct ColliderKdTree(pub KdTree<ColliderEntity>);

#[derive(Default)]
pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ColliderKdTree>().add_systems(
            Update,
            update_kd_tree
                .run_if(on_timer(Duration::from_secs_f32(KD_TREE_REFRESH_RATE)))
                .run_if(in_state(GameState::InGame)),
        );
    }
}

fn update_kd_tree(
    mut tree: ResMut<ColliderKdTree>,
    query: Query<(&Transform, Entity), With<Collider>>,
) {
    let mut points = vec![];

    for (transform, entity) in query.iter() {
        points.push(ColliderEntity {
            pos: transform.translation.truncate(),
            entity,
        });
    }

    tree.0 = KdTree::build_by_ordered_float(points);
}

impl KdPoint for ColliderEntity {
    type Scalar = f32;
    type Dim = typenum::U2;

    fn at(&self, k: usize) -> f32 {
        if k == 0 {
            return self.pos.x;
        }

        self.pos.y
    }
}

impl Default for ColliderKdTree {
    fn default() -> Self {
        Self(KdTree::build_by_ordered_float(vec![]))
    }
}
