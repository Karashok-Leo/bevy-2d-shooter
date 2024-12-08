use crate::in_game::InGame;
use crate::input::CursorPosition;
use crate::player::Player;
use crate::state::GameState;
use crate::CAMERA_SCALING;
use bevy::prelude::*;
use bevy_pancam::*;

#[derive(Component)]
#[require(InGame)]
pub struct InGameCamera;

pub struct FollowCameraPlugin;

impl InGameCamera {
    pub fn new() -> impl Bundle {
        (
            InGameCamera,
            Camera2d,
            OrthographicProjection {
                scale: CAMERA_SCALING,
                ..OrthographicProjection::default_2d()
            },
            Msaa::Off,
            PanCam {
                grab_buttons: Vec::new(),
                move_keys: DirectionKeys::NONE,
                ..default()
            },
        )
    }
}

impl Plugin for FollowCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PanCamPlugin::default()).add_systems(
            Update,
            camera_following_player.run_if(in_state(GameState::InGame)),
        );
    }
}

fn camera_following_player(
    mut camera_transform: Single<&mut Transform, With<InGameCamera>>,
    player_query: Query<&Transform, (With<Player>, Without<InGameCamera>)>,
    cursor_position: Res<CursorPosition>,
) {
    let Ok(player_transform) = player_query.get_single() else {
        return;
    };
    let player_pos = player_transform.translation.truncate();
    let offset = (cursor_position.0 - player_pos).normalize_or_zero() * 66.0;
    let cam_pos = player_pos + offset;
    camera_transform.translation = camera_transform
        .translation
        .lerp(cam_pos.extend(0.0), 0.005);
}
