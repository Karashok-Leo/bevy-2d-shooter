use crate::config::GameConfig;
use crate::physics::*;
use crate::resource::GlobalTextureAtlas;
use crate::sprite_order::SpriteOrder;
use crate::state::GameState;
use crate::world::collision::ColliderKdTree;
use crate::world::damage::*;
use crate::world::in_game::InGame;
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
                on_move,
                despawn_bullets,
                on_hurt_enemy.in_set(DamagePhase::Post),
            )
                .run_if(in_state(GameState::InGame)),
        );
    }
}

fn on_move(
    mut bullet_query: Query<(&mut Velocity, &BulletDirection), With<Bullet>>,
    config: Res<GameConfig>,
) {
    for (mut velocity, direction) in bullet_query.iter_mut() {
        velocity.0 = (direction.0.normalize() * Vec2::splat(config.gun.bullet_speed)).extend(0.0);
    }
}

fn despawn_bullets(
    mut commands: Commands,
    bullet_query: Query<(Entity, &SpawnInstant), With<Bullet>>,
    config: Res<GameConfig>,
) {
    for (bullet, instant) in bullet_query.iter() {
        if instant.0.elapsed().as_secs_f32() > config.gun.bullet_lifetime {
            commands.entity(bullet).despawn_recursive();
        }
    }
}

fn on_hurt_enemy(
    bullet_query: Query<&Transform, With<Bullet>>,
    tree: Res<ColliderKdTree>,
    mut event_writer: EventWriter<DamageEvent>,
    config: Res<GameConfig>,
) {
    if bullet_query.is_empty() {
        return;
    }

    for bullet_transform in bullet_query.iter() {
        let pos = bullet_transform.translation;
        for collider in tree
            .0
            .within_radius(&[pos.x, pos.y], config.enemy.enemy_hurt_radius)
        {
            event_writer.send(DamageEvent {
                target: collider.entity,
                context: DamageContext {
                    damage: config.gun.bullet_damage,
                    damage_type: DamageType::Bullet.into(),
                    attacker: None,
                },
                apply: true,
            });
        }
    }
}
