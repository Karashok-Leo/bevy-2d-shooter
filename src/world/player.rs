use crate::animation::*;
use crate::config::GameConfig;
use crate::input::*;
use crate::resource::*;
use crate::sprite_order::SpriteOrder;
use crate::state::GameState;
use crate::world::collision::CollisionLayer;
use crate::world::damage::*;
use crate::world::enemy::Enemy;
use crate::world::in_game::InGame;
use avian2d::prelude::*;
use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use std::time::Duration;

#[derive(Component)]
#[require(InGame)]
pub struct Player;

#[derive(Default)]
pub struct PlayerPlugin;

impl Player {
    pub fn new(texture_atlas: &Res<GlobalTextureAtlas>, config: &Res<GameConfig>) -> impl Bundle {
        let animation_indices = AnimationIndices::from_length(0, 4);
        (
            Player,
            Health::new(config.player.health),
            DamageCooldown::new(Duration::from_secs_f32(config.player.damage_cooldown)),
            DamageFlash,
            Transform::from_xyz(0.0, 0.0, SpriteOrder::Player.z_index()),
            RigidBody::Dynamic,
            LockedAxes::ROTATION_LOCKED,
            Collider::rectangle(config.player.collider_size, config.player.collider_size),
            CollisionLayers::new([CollisionLayer::Player], [CollisionLayer::Enemy]),
            Sprite::from_atlas_image(
                texture_atlas.image.clone().unwrap(),
                TextureAtlas {
                    layout: texture_atlas.layout.clone().unwrap(),
                    index: animation_indices.first,
                },
            ),
            animation_indices,
            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        )
    }
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, on_move.run_if(in_state(GameState::InGame)))
            .add_systems(
                Update,
                (
                    update_facing,
                    on_hurt
                        .run_if(on_timer(Duration::from_secs_f32(0.5)))
                        .in_set(DamagePhase::Post),
                    on_heal,
                    draw_player_hurt_box,
                )
                    .run_if(in_state(GameState::InGame)),
            );
    }
}

fn on_move(
    mut player_query: Query<(&mut AnimationIndices, &mut LinearVelocity), With<Player>>,
    move_vector: Res<MoveVector>,
    config: Res<GameConfig>,
) {
    let Ok((mut anim_indices, mut velocity)) = player_query.get_single_mut() else {
        return;
    };
    if move_vector.0.x == 0.0 && move_vector.0.y == 0.0 {
        anim_indices.with_first(0);
    } else {
        anim_indices.with_first(4);
    }
    velocity.0 = move_vector.0 * config.player.speed;
}

fn update_facing(
    mut player_query: Query<(&Transform, &mut Sprite), With<Player>>,
    cursor_position: Res<CursorPosition>,
) {
    let Ok((transform, mut sprite)) = player_query.get_single_mut() else {
        return;
    };
    sprite.flip_x = cursor_position.0.x < transform.translation.x;
}

fn on_hurt(
    enemy_query: Query<(), With<Enemy>>,
    player_query: Query<(), With<Player>>,
    mut collision_events: EventReader<CollisionStarted>,
    mut damage_events: EventWriter<DamageEvent>,
    config: Res<GameConfig>,
) {
    for event in collision_events.read() {
        if !enemy_query.contains(event.0) {
            continue;
        }
        if !player_query.contains(event.1) {
            continue;
        }
        damage_events.send(DamageEvent {
            target: event.1,
            context: DamageContext {
                damage: config.enemy.damage,
                damage_type: DamageType::Enemy.into(),
                attacker: Some(event.0),
            },
            apply: true,
        });
    }
}

fn on_heal(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Single<&mut Health, With<Player>>,
    config: Res<GameConfig>,
) {
    if !config.basic.debug {
        return;
    }
    if keyboard_input.just_released(KeyCode::KeyH) {
        player_query.heal(20.0);
    }
}

fn draw_player_hurt_box(
    mut gizmos: Gizmos,
    player_query: Single<&Transform, With<Player>>,
    config: Res<GameConfig>,
) {
    gizmos.circle_2d(
        Isometry2d::from_translation(player_query.translation.truncate()),
        config.player.collider_size,
        Color::srgb(1.0, 0.0, 0.0),
    );
}
