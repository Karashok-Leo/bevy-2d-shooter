use crate::animation::*;
use crate::collision::ColliderKdTree;
use crate::in_game::InGame;
use crate::input::*;
use crate::physics::*;
use crate::resource::*;
use crate::state::GameState;
use crate::*;
use bevy::prelude::*;

#[derive(Component)]
#[require(InGame)]
pub struct Player {
    pub health: f32,
}

pub struct PlayerPlugin;

impl Player {
    pub fn new(texture_atlas: &Res<GlobalTextureAtlas>) -> impl Bundle {
        let animation_indices = AnimationIndices::from_length(0, 4);
        (
            Player {
                health: PLAYER_HEALTH,
            },
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
            (player_moving, player_facing, player_hurting).run_if(in_state(GameState::InGame)),
        );
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

fn player_hurting(mut player_query: Query<(&Transform, &mut Player)>, tree: Res<ColliderKdTree>) {
    let Ok((transform, mut player)) = player_query.get_single_mut() else {
        return;
    };
    let pos = transform.translation;

    for _collider in tree.0.within_radius(&[pos.x, pos.y], 5.0) {
        player.health -= ENEMY_DAMAGE;
        if player.health < 0.0 {
            player.health = 0.0;
        }
    }
}
