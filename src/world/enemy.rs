use crate::animation::*;
use crate::config::GameConfig;
use crate::physics::*;
use crate::resource::GlobalTextureAtlas;
use crate::sprite_order::SpriteOrder;
use crate::state::GameState;
use crate::world::collision::Collider;
use crate::world::damage::*;
use crate::world::in_game::InGame;
use crate::world::player::Player;
use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use rand::prelude::SliceRandom;
use rand::Rng;
use std::time::Duration;

#[derive(Component)]
#[require(InGame)]
pub struct Enemy;

#[derive(Default)]
pub struct EnemyPlugin;

impl Enemy {
    const SPRITE_INDEXES: [i32; 4] = [8, 12, 20, 28];
    pub fn new(
        texture_atlas: &Res<GlobalTextureAtlas>,
        config: &Res<GameConfig>,
        player_pos: Vec2,
    ) -> impl Bundle {
        let mut rng = rand::thread_rng();
        let (x, y) = get_random_position_around(player_pos);
        let sprite_index = Self::SPRITE_INDEXES.choose(&mut rng).unwrap();
        let animation_indices = AnimationIndices::from_length(*sprite_index as usize, 4);
        (
            Enemy,
            Health::new(config.enemy.enemy_health),
            DamageCooldown::new(Duration::from_secs_f32(config.enemy.enemy_damage_cooldown)),
            DamageFlash,
            physical_transform(Transform::from_xyz(x, y, SpriteOrder::Enemy.z_index())),
            Sprite::from_atlas_image(
                texture_atlas.image.clone().unwrap(),
                TextureAtlas {
                    layout: texture_atlas.layout.clone().unwrap(),
                    index: animation_indices.first,
                },
            ),
            animation_indices,
            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
            Collider,
        )
    }
}

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        let spawn_interval = app
            .world()
            .get_resource::<GameConfig>()
            .unwrap()
            .enemy
            .enemy_spawn_interval;
        app.add_systems(OnEnter(GameState::InGame), spawn_dummy)
            .add_systems(
                Update,
                (
                    spawn_enemies.run_if(on_timer(Duration::from_secs_f32(spawn_interval))),
                    on_move,
                    update_facing,
                    despawn_enemies,
                    draw_enemy_hurt_box,
                )
                    .run_if(in_state(GameState::InGame)),
            );
    }
}

fn on_move(
    player_transform: Single<&Transform, With<Player>>,
    mut enemy_query: Query<(&Transform, &mut Velocity), (With<Enemy>, Without<Player>)>,
    config: Res<GameConfig>,
) {
    for (transform, mut velocity) in enemy_query.iter_mut() {
        let direction = (player_transform.translation - transform.translation).normalize_or_zero();
        velocity.0 = direction * config.enemy.enemy_speed;
    }
}

fn update_facing(
    player_transform: Single<&Transform, With<Player>>,
    mut enemy_query: Query<(&Transform, &mut Sprite), (With<Enemy>, Without<Player>)>,
) {
    for (transform, mut sprite) in enemy_query.iter_mut() {
        sprite.flip_x = player_transform.translation.x < transform.translation.x;
    }
}

fn spawn_dummy(
    mut commands: Commands,
    texture_atlas: Res<GlobalTextureAtlas>,
    player_query: Query<&Transform, With<Player>>,
    config: Res<GameConfig>,
) {
    if !config.basic.debug {
        return;
    }
    let Ok(player_transform) = player_query.get_single() else {
        return;
    };

    let animation_indices = AnimationIndices::from_length(8, 4);

    let player_pos = player_transform.translation.truncate();
    commands.spawn((
        Enemy,
        Health::new(config.enemy.enemy_health * 100.0),
        DamageCooldown::new(Duration::from_secs_f32(config.enemy.enemy_damage_cooldown)),
        DamageFlash,
        Transform::from_xyz(
            player_pos.x + 100.0,
            player_pos.y,
            SpriteOrder::Enemy.z_index(),
        ),
        Sprite::from_atlas_image(
            texture_atlas.image.clone().unwrap(),
            TextureAtlas {
                layout: texture_atlas.layout.clone().unwrap(),
                index: animation_indices.first,
            },
        ),
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        Collider,
    ));
}

fn spawn_enemies(
    mut commands: Commands,
    texture_atlas: Res<GlobalTextureAtlas>,
    player_query: Query<&Transform, With<Player>>,
    enemy_query: Query<&Transform, (With<Enemy>, Without<Player>)>,
    config: Res<GameConfig>,
) {
    if config.basic.debug {
        return;
    }
    let Ok(player_transform) = player_query.get_single() else {
        return;
    };

    let num_enemies = enemy_query.iter().len();
    let enemy_spawn_count: usize =
        (config.enemy.max_num_enemies - num_enemies).min(config.enemy.spawn_rate_per_second);
    if enemy_spawn_count <= 0 {
        return;
    }

    let player_pos = player_transform.translation.truncate();
    for _ in 0..enemy_spawn_count {
        commands.spawn(Enemy::new(&texture_atlas, &config, player_pos));
    }
}

fn despawn_enemies(mut commands: Commands, enemy_query: Query<(Entity, &Health), With<Enemy>>) {
    for (entity, health) in enemy_query.iter() {
        if health.is_alive() {
            continue;
        }
        commands.entity(entity).despawn_recursive();
    }
}

fn get_random_position_around(pos: Vec2) -> (f32, f32) {
    let mut rng = rand::thread_rng();
    let radius = rng.gen_range(400.0..1500.0);
    let angle = rng.gen_range(0.0..std::f32::consts::PI * 2.0);
    let x = pos.x + angle.cos() * radius;
    let y = pos.y + angle.sin() * radius;
    (x, y)
}

fn draw_enemy_hurt_box(
    mut gizmos: Gizmos,
    enemy_query: Query<&Transform, With<Enemy>>,
    config: Res<GameConfig>,
) {
    for transform in enemy_query.iter() {
        gizmos.circle_2d(
            Isometry2d::from_translation(transform.translation.truncate()),
            config.enemy.enemy_hurt_radius,
            Color::srgb(1.0, 0.0, 0.0),
        );
    }
}
