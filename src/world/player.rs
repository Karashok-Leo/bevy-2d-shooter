use crate::animation::*;
use crate::config::GameConfig;
use crate::input::*;
use crate::physics::*;
use crate::resource::*;
use crate::sprite_order::SpriteOrder;
use crate::state::GameState;
use crate::world::collision::ColliderKdTree;
use crate::world::damage::*;
use crate::world::in_game::InGame;
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
            Health::new(config.player.player_health),
            DamageCooldown::new(Duration::from_secs_f32(
                config.player.player_damage_cooldown,
            )),
            DamageFlash,
            physical_transform(Transform::from_xyz(0.0, 0.0, SpriteOrder::Player.z_index())),
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
        app.add_systems(
            Update,
            (
                on_move,
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
    mut player_query: Query<(&mut AnimationIndices, &mut Velocity), With<Player>>,
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
        velocity.0 = move_vector.0.extend(0.0) * config.player.player_speed;
    }
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
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    tree: Res<ColliderKdTree>,
    mut event_writer: EventWriter<DamageEvent>,
    config: Res<GameConfig>,
) {
    let Ok((entity, transform)) = player_query.get_single_mut() else {
        return;
    };
    let pos = transform.translation;

    for collider in tree
        .0
        .within_radius(&[pos.x, pos.y], config.player.player_hurt_radius)
    {
        event_writer.send(DamageEvent {
            target: entity,
            context: DamageContext {
                damage: config.enemy.enemy_damage,
                damage_type: DamageType::Enemy.into(),
                attacker: Some(collider.entity),
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
        config.player.player_hurt_radius,
        Color::srgb(1.0, 0.0, 0.0),
    );
}
