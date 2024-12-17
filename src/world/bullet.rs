use crate::config::GameConfig;
use crate::resource::GlobalTextureAtlas;
use crate::sprite_order::SpriteOrder;
use crate::state::GameState;
use crate::world::collision::*;
use crate::world::damage::*;
use crate::world::despawn::PostDespawn;
use crate::world::enemy::Enemy;
use crate::world::in_game::InGameScoped;
use crate::world::owner::Owner;
use avian2d::prelude::*;
use bevy::prelude::*;
use rand::Rng;
use std::time::Duration;

#[derive(Component, Default)]
#[require(InGameScoped)]
pub struct Bullet;

#[derive(Component, Default)]
pub struct BulletDamage(pub f32);

#[derive(Component, Default)]
pub struct Lifespan(pub Timer);

#[derive(Component, Default)]
pub struct SpawnPoint(pub Vec2);

#[derive(Component, Default)]
#[require(SpawnPoint)]
pub struct MaxTravelDistance(pub f32);

#[derive(Component, Default)]
pub struct DespawnOnHit;

#[derive(Default)]
pub struct BulletPlugin;

impl Bullet {
    pub fn new(
        texture_atlas: &Res<GlobalTextureAtlas>,
        config: &Res<GameConfig>,
        gun_dir: Vec2,
        gun_pos: Vec2,
    ) -> impl Bundle {
        let mut rng = rand::thread_rng();
        let offset = Vec2::new(rng.gen_range(-0.5..0.5), rng.gen_range(-0.5..0.5));
        (
            Bullet,
            BulletDamage(config.bullet.damage),
            LinearVelocity((gun_dir + offset) * Vec2::splat(config.bullet.speed)),
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

impl Lifespan {
    pub fn new(duration: Duration) -> Self {
        Self(Timer::new(duration, TimerMode::Once))
    }
}

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                despawn_bullets_out_of_range,
                despawn_bullets_out_of_lifespan,
                on_hit_enemy.in_set(DamagePhase::Send),
            )
                .run_if(in_state(GameState::Running)),
        );
    }
}

fn despawn_bullets_out_of_lifespan(
    mut commands: Commands,
    bullet_query: Query<(Entity, &Lifespan), With<Bullet>>,
) {
    for (bullet, instant) in bullet_query.iter() {
        if instant.0.finished() {
            commands.entity(bullet).insert(PostDespawn);
        }
    }
}

fn despawn_bullets_out_of_range(
    mut commands: Commands,
    bullet_query: Query<(Entity, &GlobalTransform, &SpawnPoint, &MaxTravelDistance), With<Bullet>>,
) {
    for (bullet, transform, spawn_point, max_distance) in bullet_query.iter() {
        if transform.translation().truncate().distance(spawn_point.0) > max_distance.0 {
            commands.entity(bullet).insert(PostDespawn);
        }
    }
}

fn on_hit_enemy(
    mut commands: Commands,
    bullet_query: Query<(&BulletDamage, Option<&Owner>, Has<DespawnOnHit>), With<Bullet>>,
    enemy_query: Query<(), With<Enemy>>,
    mut collision_events: EventReader<Collision>,
    mut damage_events: EventWriter<DamageEvent>,
) {
    for event in collision_events.read() {
        let Some(bullet) = try_parse_collider(event.0.entity1, event.0.entity2, &bullet_query)
        else {
            continue;
        };
        let Some(enemy) = try_parse_collider(event.0.entity1, event.0.entity2, &enemy_query) else {
            continue;
        };
        let Ok((damage, owner, despawn_on_hit)) = bullet_query.get(bullet) else {
            continue;
        };
        damage_events.send(DamageEvent {
            target: enemy,
            context: DamageContext {
                damage: damage.0,
                damage_type: DamageType::Bullet.into(),
                attacker: owner.map(|owner| owner.0),
            },
            apply: true,
        });
        if despawn_on_hit {
            commands.entity(bullet).insert(PostDespawn);
        }
    }
}
