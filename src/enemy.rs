use crate::animation::*;
use crate::collision::Collider;
use crate::in_game::InGame;
use crate::physics::*;
use crate::player::Player;
use crate::resource::GlobalTextureAtlas;
use crate::state::GameState;
use crate::*;
use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use rand::prelude::SliceRandom;
use rand::Rng;
use std::time::Duration;

#[derive(Component)]
#[require(InGame)]
pub struct Enemy {
    pub health: f32,
}

pub struct EnemyPlugin;

impl Enemy {
    const SPRITE_INDEXES: [i32; 4] = [8, 12, 20, 28];
    pub fn new(texture_atlas: &Res<GlobalTextureAtlas>, player_pos: Vec2) -> impl Bundle {
        let mut rng = rand::thread_rng();
        let (x, y) = get_random_position_around(player_pos);
        let sprite_index = Self::SPRITE_INDEXES.choose(&mut rng).unwrap();
        let animation_indices = AnimationIndices::from_length(*sprite_index as usize, 4);
        (
            Enemy {
                health: ENEMY_HEALTH,
            },
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
        app.add_systems(
            Update,
            (
                spawn_enemies.run_if(on_timer(Duration::from_secs_f32(ENEMY_SPAWN_INTERVAL))),
                enemy_moving,
                enemy_facing,
                despawn_enemies,
            )
                .run_if(in_state(GameState::InGame)),
        );
    }
}

fn enemy_moving(
    player_transform: Single<&Transform, With<Player>>,
    mut enemy_query: Query<(&Transform, &mut Velocity), (With<Enemy>, Without<Player>)>,
) {
    for (transform, mut velocity) in enemy_query.iter_mut() {
        let direction = (player_transform.translation - transform.translation).normalize_or_zero();
        velocity.0 = direction * ENEMY_SPEED;
    }
}

fn enemy_facing(
    player_transform: Single<&Transform, With<Player>>,
    mut enemy_query: Query<(&Transform, &mut Sprite), (With<Enemy>, Without<Player>)>,
) {
    for (transform, mut sprite) in enemy_query.iter_mut() {
        sprite.flip_x = player_transform.translation.x < transform.translation.x;
    }
}

fn spawn_enemies(
    mut commands: Commands,
    texture_atlas: Res<GlobalTextureAtlas>,
    player_query: Query<&Transform, With<Player>>,
    enemy_query: Query<&Transform, (With<Enemy>, Without<Player>)>,
) {
    let Ok(player_transform) = player_query.get_single() else {
        return;
    };

    let num_enemies = enemy_query.iter().len();
    let enemy_spawn_count: usize = (MAX_NUM_ENEMIES - num_enemies).min(SPAWN_RATE_PER_SECOND);
    if enemy_spawn_count <= 0 {
        return;
    }

    let player_pos = player_transform.translation.truncate();
    for _ in 0..enemy_spawn_count {
        commands.spawn(Enemy::new(&texture_atlas, player_pos));
    }
}

fn despawn_enemies(mut commands: Commands, enemy_query: Query<(Entity, &Enemy)>) {
    for (entity, enemy) in enemy_query.iter() {
        if enemy.health <= 0.0 {
            commands.entity(entity).despawn_recursive();
        }
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
