use crate::state::GameState;
use bevy::prelude::*;

pub struct InputHandlerPlugin;

/// A vector representing the player's input, accumulated over all frames that ran
/// since the last time the physics simulation was advanced.
#[derive(Resource, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
pub struct MoveVector(pub Vec2);

#[derive(Resource, Default)]
pub struct CursorPosition(pub Vec2);

impl Plugin for InputHandlerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MoveVector>()
            .init_resource::<CursorPosition>()
            .add_systems(
                Update,
                (update_move_vector, update_cursor_position).run_if(in_state(GameState::InGame)),
            );
    }
}

fn update_move_vector(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut move_vector: ResMut<MoveVector>,
) {
    let w_key = keyboard_input.any_pressed([KeyCode::KeyW, KeyCode::ArrowUp]);
    let a_key = keyboard_input.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]);
    let s_key = keyboard_input.any_pressed([KeyCode::KeyS, KeyCode::ArrowDown]);
    let d_key = keyboard_input.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]);

    let mut delta = Vec2::ZERO;
    if w_key {
        delta.y += 1.0;
    }
    if a_key {
        delta.x -= 1.0;
    }
    if s_key {
        delta.y -= 1.0;
    }
    if d_key {
        delta.x += 1.0;
    }

    move_vector.0 = delta.normalize_or_zero();
}

fn update_cursor_position(
    camera_query: Single<(&Camera, &GlobalTransform)>,
    window: Single<&Window>,
    mut cursor_position: ResMut<CursorPosition>,
) {
    let (camera, camera_transform) = *camera_query;

    let Some(cursor_window_pos) = window.cursor_position() else {
        return;
    };

    // Calculate a world position based on the cursor's position.
    let Ok(point) = camera.viewport_to_world_2d(camera_transform, cursor_window_pos) else {
        return;
    };

    cursor_position.0 = point;
}
