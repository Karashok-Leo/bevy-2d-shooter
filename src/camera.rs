use crate::input::CursorPosition;
use crate::state::GameState;
use crate::world::in_game::InGame;
use crate::world::player::Player;
use crate::CAMERA_SCALING;
use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::prelude::*;

#[derive(Component)]
#[require(InGame)]
pub struct SmoothCamera;

#[derive(Component)]
#[require(Camera)]
pub struct CameraZoom;

#[derive(Resource, Default)]
pub struct ZoomScale(pub f32);

pub struct SmoothCameraPlugin;

impl SmoothCamera {
    pub fn new() -> impl Bundle {
        (
            SmoothCamera,
            Camera2d,
            OrthographicProjection {
                scale: CAMERA_SCALING,
                ..OrthographicProjection::default_2d()
            },
            Msaa::Off,
            CameraZoom,
        )
    }
}

impl Plugin for SmoothCameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ZoomScale(CAMERA_SCALING)).add_systems(
            Update,
            (
                camera_following_player,
                scroll_offset_from_events,
                do_camera_zoom,
            )
                .run_if(in_state(GameState::InGame)),
        );
    }
}

const FOLLOW_SPEED: f32 = 0.01;

fn camera_following_player(
    mut camera_transform: Single<&mut Transform, With<SmoothCamera>>,
    player_query: Query<&Transform, (With<Player>, Without<SmoothCamera>)>,
    cursor_position: Res<CursorPosition>,
) {
    let Ok(player_transform) = player_query.get_single() else {
        return;
    };
    let player_pos = player_transform.translation.truncate();
    let offset = (cursor_position.0 - player_pos).normalize_or_zero() * 50.0;
    let cam_pos = player_pos + offset;
    camera_transform.translation = camera_transform
        .translation
        .lerp(cam_pos.extend(0.0), FOLLOW_SPEED);
}

const ZOOM_SPEED: f32 = 0.01;

fn do_camera_zoom(
    mut query: Query<&mut OrthographicProjection, With<CameraZoom>>,
    zoom_scale: Res<ZoomScale>,
) {
    for mut proj in &mut query {
        proj.scale = proj.scale.lerp(zoom_scale.0, ZOOM_SPEED);
    }
}

const PIXELS_PER_LINE: f32 = 100.;
const ZOOM_SENSITIVITY: f32 = 0.001;

/// Consumes `MouseWheel` event reader and calculates a single scalar,
/// representing positive or negative scroll offset.
fn scroll_offset_from_events(
    mut scroll_events: EventReader<MouseWheel>,
    mut zoom_scale: ResMut<ZoomScale>,
) {
    let scroll_offset = scroll_events
        .read()
        .map(|ev| match ev.unit {
            MouseScrollUnit::Pixel => ev.y,
            MouseScrollUnit::Line => ev.y * PIXELS_PER_LINE,
        })
        .sum::<f32>();
    zoom_scale.0 *= 1. - scroll_offset * ZOOM_SENSITIVITY;
}
