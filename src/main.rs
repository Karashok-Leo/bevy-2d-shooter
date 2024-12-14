#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::*;
use bevy::window::PresentMode;
use bevy_2d_shooter::animation::AnimatorPlugin;
use bevy_2d_shooter::camera::SmoothCameraPlugin;
use bevy_2d_shooter::input::InputHandlerPlugin;
use bevy_2d_shooter::physics::PhysicsPlugin;
use bevy_2d_shooter::resource::ResourcePlugin;
use bevy_2d_shooter::state::GameState;
use bevy_2d_shooter::ui::UIPlugins;
use bevy_2d_shooter::world::WorldPlugins;
use bevy_2d_shooter::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        // mode: bevy::window::WindowMode::Fullscreen(MonitorSelection::Current),
                        resizable: true,
                        focused: true,
                        resolution: (WW, WH).into(),
                        // set vsync to false
                        present_mode: PresentMode::Immediate,
                        ..default()
                    }),
                    ..default()
                }),
            ResourcePlugin,
            InputHandlerPlugin,
            PhysicsPlugin,
            AnimatorPlugin,
            SmoothCameraPlugin,
            WorldPlugins,
            UIPlugins,
        ))
        .init_state::<GameState>()
        .insert_resource(ClearColor(Color::srgb_u8(
            BG_COLOR.0, BG_COLOR.1, BG_COLOR.2,
        )))
        .run();
}
