use crate::physics::*;
use crate::resource::GlobalTextureAtlas;
use crate::state::GameState;
use crate::world::collision::ColliderKdTree;
use crate::world::damage::*;
use crate::world::in_game::InGame;
use crate::*;
use bevy::prelude::*;
use rand::Rng;
use std::time::Instant;

#[derive(Component)]
#[require(InGame, SpawnInstant(|| SpawnInstant(Instant::now())))]
pub struct Bullet;

#[derive(Component)]
pub struct SpawnInstant(Instant);

#[derive(Component, Default)]
pub struct BulletDirection(Vec2);

#[derive(Default)]
pub struct BulletPlugin;

impl Bullet {
    pub fn new(
        texture_atlas: &Res<GlobalTextureAtlas>,
        gun_dir: Vec2,
        gun_pos: Vec2,
    ) -> impl Bundle {
        let mut rng = rand::thread_rng();
        let offset = Vec2::new(rng.gen_range(-0.5..0.5), rng.gen_range(-0.5..0.5));
        (
            Bullet,
            BulletDirection(gun_dir + offset),
            SpawnInstant(Instant::now()),
            physical_transform(Transform::from_xyz(
                gun_pos.x,
                gun_pos.y,
                SpriteOrder::Bullet.z_index(),
            )),
            Sprite::from_atlas_image(
                texture_atlas.image.clone().unwrap(),
                TextureAtlas {
                    layout: texture_atlas.layout.clone().unwrap(),
                    index: 16,
                },
            ),
        )
    }
}

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                bullet_moving,
                despawn_bullets,
                bullet_damage.in_set(DamagePhase::Post),
            )
                .run_if(in_state(GameState::InGame)),
        );
    }
}

fn bullet_moving(mut bullet_query: Query<(&mut Velocity, &BulletDirection), With<Bullet>>) {
    for (mut velocity, direction) in bullet_query.iter_mut() {
        velocity.0 = (direction.0.normalize() * Vec2::splat(BULLET_SPEED)).extend(0.0);
    }
}

fn despawn_bullets(
    mut commands: Commands,
    bullet_query: Query<(Entity, &SpawnInstant), With<Bullet>>,
) {
    for (bullet, instant) in bullet_query.iter() {
        if instant.0.elapsed().as_secs_f32() > BULLET_TIME_SECS {
            commands.entity(bullet).despawn_recursive();
        }
    }
}

fn bullet_damage(
    bullet_query: Query<&Transform, With<Bullet>>,
    tree: Res<ColliderKdTree>,
    mut event_writer: EventWriter<DamageEvent>,
) {
    if bullet_query.is_empty() {
        return;
    }

    for bullet_transform in bullet_query.iter() {
        let pos = bullet_transform.translation;
        for collider in tree.0.within_radius(&[pos.x, pos.y], ENEMY_HURT_RADIUS) {
            event_writer.send(DamageEvent {
                target: collider.entity,
                context: DamageContext {
                    damage: BULLET_DAMAGE,
                    damage_type: DamageType::Bullet.into(),
                    attacker: None,
                },
                apply: true,
            });
        }
    }
}
