use crate::input::CursorPosition;
use crate::resource::*;
use crate::state::GameState;
use crate::world::bullet::Bullet;
use crate::*;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy::time::Stopwatch;

#[derive(Component)]
#[require(GunTimer)]
pub struct Gun;

#[derive(Component, Default)]
pub struct GunTimer(Stopwatch);

#[derive(Default)]
pub struct GunPlugin;

impl Gun {
    pub fn new(texture_atlas: &Res<GlobalTextureAtlas>) -> impl Bundle {
        (
            Gun,
            Transform::from_xyz(0.0, -4.0, SpriteOrder::Gun.z_index()),
            Sprite {
                anchor: Anchor::Custom(Vec2::new(-6.0 / 16.0, 0.0)),
                ..Sprite::from_atlas_image(
                    texture_atlas.image.clone().unwrap(),
                    TextureAtlas {
                        layout: texture_atlas.layout.clone().unwrap(),
                        index: 17,
                    },
                )
            },
        )
    }
}

impl Plugin for GunPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (shoot_bullets, update_gun_rotation).run_if(in_state(GameState::InGame)),
        );
    }
}

fn update_gun_rotation(
    cursor_position: Res<CursorPosition>,
    mut gun_query: Query<(&mut Transform, &GlobalTransform), With<Gun>>,
) {
    let Ok((mut gun_transform, gun_global_transform)) = gun_query.get_single_mut() else {
        return;
    };

    let gun_position = gun_global_transform.translation().truncate();
    let direction = (cursor_position.0 - gun_position).normalize_or_zero();
    let angle = direction.y.atan2(direction.x);
    gun_transform.rotation = Quat::from_rotation_z(angle);
}

fn shoot_bullets(
    mut commands: Commands,
    texture_atlas: Res<GlobalTextureAtlas>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut gun_query: Query<(&GlobalTransform, &mut GunTimer), With<Gun>>,
    time: Res<Time>,
) {
    let Ok((gun_transform, mut gun_timer)) = gun_query.get_single_mut() else {
        return;
    };

    gun_timer.0.tick(time.delta());

    if !mouse_input.pressed(MouseButton::Left) {
        return;
    }

    if gun_timer.0.elapsed_secs() < BULLET_SPAWN_INTERVAL {
        return;
    }
    gun_timer.0.reset();

    let gun_pos = gun_transform.translation().truncate();
    let gun_dir = gun_transform.right().truncate();

    for _ in 0..NUM_BULLETS_PER_SHOT {
        commands.spawn(Bullet::new(
            &texture_atlas,
            gun_dir,
            gun_pos + gun_dir * 12.0,
        ));
    }
}
