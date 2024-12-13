#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::*;
use bevy::window::PresentMode;
use bevy_2d_shooter::animation::AnimatorPlugin;
use bevy_2d_shooter::bullet::BulletPlugin;
use bevy_2d_shooter::camera::FollowCameraPlugin;
use bevy_2d_shooter::collision::CollisionPlugin;
use bevy_2d_shooter::damage::DamagePlugin;
use bevy_2d_shooter::enemy::EnemyPlugin;
use bevy_2d_shooter::game_over::GameOverPlugin;
use bevy_2d_shooter::gui::GuiPlugin;
use bevy_2d_shooter::gun::GunPlugin;
use bevy_2d_shooter::in_game::InGamePlugin;
use bevy_2d_shooter::input::InputHandlerPlugin;
use bevy_2d_shooter::main_menu::MainMenuPlugin;
use bevy_2d_shooter::hud::HudPlugin;
use bevy_2d_shooter::physics::PhysicsPlugin;
use bevy_2d_shooter::player::PlayerPlugin;
use bevy_2d_shooter::resource::ResourcePlugin;
use bevy_2d_shooter::state::GameState;
use bevy_2d_shooter::*;
use bevy_dev_tools::ui_debug_overlay::DebugUiPlugin;

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
            CollisionPlugin,
            AnimatorPlugin,
            FollowCameraPlugin,
            (PlayerPlugin, GunPlugin, BulletPlugin, EnemyPlugin),
            DamagePlugin,
            (
                MainMenuPlugin,
                InGamePlugin,
                GameOverPlugin,
                GuiPlugin,
                HudPlugin,
            ),
            DebugUiPlugin,
        ))
        .init_state::<GameState>()
        .insert_resource(ClearColor(Color::srgb_u8(
            BG_COLOR.0, BG_COLOR.1, BG_COLOR.2,
        )))
        .run();
}
