#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use avian2d::interpolation::PhysicsInterpolationPlugin;
use avian2d::prelude::Gravity;
use avian2d::PhysicsPlugins;
use bevy::prelude::*;
use bevy::window::PresentMode;
use bevy_2d_shooter::animation::AnimatorPlugin;
use bevy_2d_shooter::camera::SmoothCameraPlugin;
use bevy_2d_shooter::config::*;
use bevy_2d_shooter::input::InputHandlerPlugin;
use bevy_2d_shooter::resource::ResourcePlugin;
use bevy_2d_shooter::state::*;
use bevy_2d_shooter::ui::UIPlugins;
use bevy_2d_shooter::world::WorldPlugins;

fn main() {
    let config = get_config();
    let tile_size = config.basic.tile_size;
    let ww = config.basic.window_width;
    let wh = config.basic.window_height;
    let bg_color = Color::srgb_u8(
        config.world.background_color.0,
        config.world.background_color.1,
        config.world.background_color.2,
    );
    App::new()
        .insert_resource(ClearColor(bg_color))
        .insert_resource(Gravity::ZERO)
        .insert_resource(config)
        .add_plugins((
            configured_default_plugins(ww, wh),
            configured_physics_plugins(tile_size),
            ConfigPlugin,
            ResourcePlugin,
            InputHandlerPlugin,
            AnimatorPlugin,
            SmoothCameraPlugin,
            WorldPlugins,
            UIPlugins,
        ))
        .init_state::<AppState>()
        .add_sub_state::<GameState>()
        .enable_state_scoped_entities::<AppState>()
        .enable_state_scoped_entities::<GameState>()
        .add_systems(Startup, configure_gizmos)
        .run();
}

fn configured_default_plugins(window_width: f32, window_height: f32) -> impl PluginGroup {
    DefaultPlugins
        .set(ImagePlugin::default_nearest())
        .set(WindowPlugin {
            primary_window: Some(Window {
                // mode: bevy::window::WindowMode::Fullscreen(MonitorSelection::Current),
                resizable: true,
                focused: true,
                resolution: (window_width, window_height).into(),
                // set vsync to false
                present_mode: PresentMode::Immediate,
                ..default()
            }),
            ..default()
        })
}

fn configured_physics_plugins(tile_size: f32) -> impl PluginGroup {
    PhysicsPlugins::default()
        .with_length_unit(tile_size)
        .set(PhysicsInterpolationPlugin::interpolate_translation_all())
}

fn configure_gizmos(config: Res<GameConfig>, mut config_store: ResMut<GizmoConfigStore>) {
    let (gizmos_config, _) = config_store.config_mut::<DefaultGizmoConfigGroup>();
    gizmos_config.enabled = config.basic.debug;
}
