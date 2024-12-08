use crate::bullet::Bullet;
use crate::enemy::Enemy;
use crate::in_game::InGame;
use crate::player::Player;
use crate::state::*;
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use std::time::Duration;

#[derive(Component)]
#[require(InGame)]
struct DebugOverlay;

#[derive(Component)]
struct DebugText;

pub struct OverlayPlugin;

impl Plugin for OverlayPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FrameTimeDiagnosticsPlugin)
            .add_systems(OnEnter(GameState::GameInit), spawn_debug_texts)
            .add_systems(OnEnter(GameState::InGame), show_debug_overlay)
            .add_systems(OnExit(GameState::InGame), hide_debug_overlay)
            .add_systems(
                Update,
                update_debug_texts
                    .run_if(in_state(GameState::InGame).or(in_state(GameState::GameOver)))
                    .run_if(on_timer(Duration::from_secs_f32(0.2))),
            );
    }
}

fn spawn_debug_texts(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            DebugOverlay,
            Visibility::Hidden,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Start,
                justify_content: JustifyContent::Start,
                flex_direction: FlexDirection::Column,
                ..default()
            },
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        width: Val::Px(300.0),
                        height: Val::Px(200.0),
                        align_items: AlignItems::Center,
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        padding: UiRect::all(Val::Px(8.0)),
                        margin: UiRect::px(10.0, 10.0, 10.0, 0.0),
                        ..default()
                    },
                    BackgroundColor::from(Color::BLACK.with_alpha(0.9)),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new(""),
                        TextFont {
                            font: asset_server.load("monogram.ttf"),
                            font_size: 40.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                        DebugText,
                    ));
                });
        });
}

fn show_debug_overlay(mut query: Query<&mut Visibility, With<DebugOverlay>>) {
    for mut visibility in query.iter_mut() {
        *visibility = Visibility::Inherited;
    }
}

fn hide_debug_overlay(mut query: Query<&mut Visibility, With<DebugOverlay>>) {
    for mut visibility in query.iter_mut() {
        *visibility = Visibility::Hidden;
    }
}

fn update_debug_texts(
    mut query: Query<&mut Text, With<DebugText>>,
    diagnostics: Res<DiagnosticsStore>,
    enemy_query: Query<(), With<Enemy>>,
    bullet_query: Query<(), With<Bullet>>,
    player_query: Single<&Player>,
) {
    let Ok(mut text) = query.get_single_mut() else {
        return;
    };

    let num_enemies = enemy_query.iter().count();
    let num_bullets = bullet_query.iter().count();
    let player_health = player_query.health;
    if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(value) = fps.smoothed() {
            **text = format!("Fps: {value:.2}\nEnemies: {num_enemies}\nBullets: {num_bullets}\nPlayer Health: {player_health:.0}");
        }
    }
}
