use crate::input::CursorPosition;
use crate::state::GameState;
use crate::world::player::Player;
use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::prelude::*;

#[derive(Component)]
// #[require(InGame)]
pub struct SmoothCamera;

#[derive(Component)]
#[require(Camera)]
pub struct CameraZoom;

#[derive(Resource, Default)]
pub struct ZoomScale(pub f32);

pub struct SmoothCameraPlugin;

pub const INITIAL_CAMERA_SCALE: f32 = 0.32;

impl SmoothCamera {
    pub fn new() -> impl Bundle {
        (
            Camera2d,
            OrthographicProjection {
                scale: INITIAL_CAMERA_SCALE,
                ..OrthographicProjection::default_2d()
            },
            Msaa::Off,
            SmoothCamera,
            CameraZoom,
        )
    }
}

impl Plugin for SmoothCameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ZoomScale(INITIAL_CAMERA_SCALE))
            .add_systems(Startup, spawn_camera)
            .add_systems(OnEnter(GameState::GameInit), reset_camera_scale)
            .add_systems(
                Update,
                (
                    camera_following_player,
                    scroll_offset_from_events,
                    do_camera_zoom,
                )
                    .run_if(in_state(GameState::Running)),
            );
    }
}

const FOLLOW_SPEED: f32 = 0.01;

fn spawn_camera(mut commands: Commands) {
    commands.spawn(SmoothCamera::new());
}

fn reset_camera_scale(mut zoom_scale: ResMut<ZoomScale>) {
    zoom_scale.0 = INITIAL_CAMERA_SCALE;
}

fn camera_following_player(
    mut camera_transform: Single<&mut Transform, With<SmoothCamera>>,
    player_query: Query<&GlobalTransform, (With<Player>, Without<SmoothCamera>)>,
    cursor_position: Res<CursorPosition>,
) {
    let Ok(player_transform) = player_query.get_single() else {
        return;
    };
    let player_pos = player_transform.translation().truncate();
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
