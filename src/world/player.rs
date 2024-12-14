use crate::animation::*;
use crate::input::*;
use crate::physics::*;
use crate::resource::*;
use crate::state::GameState;
use crate::world::collision::ColliderKdTree;
use crate::world::damage::*;
use crate::world::in_game::InGame;
use crate::*;
use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use std::time::Duration;

#[derive(Component)]
#[require(InGame)]
pub struct Player;

#[derive(Default)]
pub struct PlayerPlugin;

impl Player {
    pub fn new(texture_atlas: &Res<GlobalTextureAtlas>) -> impl Bundle {
        let animation_indices = AnimationIndices::from_length(0, 4);
        (
            Player,
            Health::new(PLAYER_HEALTH),
            DamageCooldown::new(Duration::from_secs_f32(PLAYER_DAMAGE_COOLDOWN)),
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
                player_moving,
                player_facing,
                player_hurting
                    .run_if(on_timer(Duration::from_secs_f32(0.5)))
                    .in_set(DamagePhase::Post),
            )
                .run_if(in_state(GameState::InGame)),
        );

        if DEBUG {
            app.add_systems(
                Update,
                (heal_player, draw_player_hurt_box).run_if(in_state(GameState::InGame)),
            );
        }
    }
}

fn player_moving(
    mut player_query: Query<(&mut AnimationIndices, &mut Velocity), With<Player>>,
    move_vector: Res<MoveVector>,
) {
    let Ok((mut anim_indices, mut velocity)) = player_query.get_single_mut() else {
        return;
    };
    if move_vector.0.x == 0.0 && move_vector.0.y == 0.0 {
        anim_indices.with_first(0);
    } else {
        anim_indices.with_first(4);
        velocity.0 = move_vector.0.extend(0.0) * PLAYER_SPEED;
    }
}

fn player_facing(
    mut player_query: Query<(&Transform, &mut Sprite), With<Player>>,
    cursor_position: Res<CursorPosition>,
) {
    let Ok((transform, mut sprite)) = player_query.get_single_mut() else {
        return;
    };
    sprite.flip_x = cursor_position.0.x < transform.translation.x;
}

fn player_hurting(
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    tree: Res<ColliderKdTree>,
    mut event_writer: EventWriter<DamageEvent>,
) {
    let Ok((entity, transform)) = player_query.get_single_mut() else {
        return;
    };
    let pos = transform.translation;

    for collider in tree.0.within_radius(&[pos.x, pos.y], PLAYER_HURT_RADIUS) {
        event_writer.send(DamageEvent {
            target: entity,
            context: DamageContext {
                damage: ENEMY_DAMAGE,
                damage_type: DamageType::Enemy.into(),
                attacker: Some(collider.entity),
            },
            apply: true,
        });
    }
}

fn heal_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Single<&mut Health, With<Player>>,
) {
    if keyboard_input.just_released(KeyCode::KeyH) {
        player_query.heal(20.0);
    }
}

fn draw_player_hurt_box(mut gizmos: Gizmos, player_query: Single<&Transform, With<Player>>) {
    gizmos.circle_2d(
        Isometry2d::from_translation(player_query.translation.truncate()),
        PLAYER_HURT_RADIUS,
        Color::srgb(1.0, 0.0, 0.0),
    );
}
