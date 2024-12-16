use crate::config::GameConfig;
use crate::resource::GlobalTextureAtlas;
use crate::sprite_order::SpriteOrder;
use crate::state::GameState;
use crate::world::collision::CollisionLayer;
use crate::world::damage::*;
use crate::world::enemy::Enemy;
use crate::world::in_game::InGame;
use avian2d::prelude::*;
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
            Transform::from_xyz(gun_pos.x, gun_pos.y, SpriteOrder::Bullet.z_index()),
            RigidBody::Dynamic,
            Collider::rectangle(2.0, 2.0),
            Sensor,
            CollisionLayers::new([CollisionLayer::Bullet], [CollisionLayer::Enemy]),
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
    mut bullet_query: Query<(&mut LinearVelocity, &BulletDirection), With<Bullet>>,
    config: Res<GameConfig>,
) {
    for (mut velocity, direction) in bullet_query.iter_mut() {
        let new_velocity = direction.0.normalize() * Vec2::splat(config.bullet.speed);
        velocity.x = new_velocity.x;
        velocity.y = new_velocity.y;
    }
}

fn despawn_bullets(
    mut commands: Commands,
    bullet_query: Query<(Entity, &SpawnInstant), With<Bullet>>,
    config: Res<GameConfig>,
) {
    for (bullet, instant) in bullet_query.iter() {
        if instant.0.elapsed().as_secs_f32() > config.bullet.lifetime {
            commands.entity(bullet).despawn_recursive();
        }
    }
}

fn on_hurt_enemy(
    bullet_query: Query<(), With<Bullet>>,
    enemy_query: Query<(), With<Enemy>>,
    mut collision_events: EventReader<CollisionStarted>,
    mut damage_events: EventWriter<DamageEvent>,
    config: Res<GameConfig>,
) {
    for event in collision_events.read() {
        if !bullet_query.contains(event.0) {
            continue;
        }
        if !enemy_query.contains(event.1) {
            continue;
        }
        damage_events.send(DamageEvent {
            target: event.1,
            context: DamageContext {
                damage: config.bullet.damage,
                damage_type: DamageType::Bullet.into(),
                attacker: None,
            },
            apply: true,
        });
    }
}
