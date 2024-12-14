use crate::bullet::Bullet;
use crate::damage::*;
use crate::enemy::Enemy;
use crate::in_game::InGame;
use crate::player::Player;
use crate::state::*;
use crate::ui::gui::*;
use crate::ui::player_health_bar::*;
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use std::time::Duration;

#[derive(Component, Default)]
#[require(InGame)]
pub(crate) struct Hud;

#[derive(Component, Default)]
struct DebugText;

#[derive(Default)]
pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FrameTimeDiagnosticsPlugin)
            .add_systems(
                OnEnter(GameState::GameInit),
                (spawn_hud, spawn_player_health_bar),
            )
            .add_systems(OnEnter(GameState::InGame), show_hud)
            .add_systems(OnExit(GameState::InGame), hide_hud)
            .add_systems(Update, on_enemy_damaged.in_set(DamagePhase::After))
            .add_systems(
                Update,
                update_player_health_bar.run_if(in_state(GameState::InGame)),
            )
            .add_systems(
                Update,
                update_debug_texts
                    .run_if(in_state(GameState::InGame).or(in_state(GameState::GameOver)))
                    .run_if(on_timer(Duration::from_secs_f32(0.2))),
            );
    }
}

fn spawn_hud(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands
        .spawn((
            Hud,
            Visibility::Hidden,
            Node {
                left: Val::Px(10.0),
                top: Val::Px(10.0),
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
                        ..default()
                    },
                    BackgroundColor::from(Color::BLACK.with_alpha(0.9)),
                ))
                .with_children(|parent| {
                    parent.spawn((text(&asset_server, "", 40.0), DebugText));
                });
        });
}

fn show_hud(mut query: Query<&mut Visibility, With<Hud>>) {
    for mut visibility in query.iter_mut() {
        *visibility = Visibility::Inherited;
    }
}

fn hide_hud(mut query: Query<&mut Visibility, With<Hud>>) {
    for mut visibility in query.iter_mut() {
        *visibility = Visibility::Hidden;
    }
}

fn on_enemy_damaged(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut event_reader: EventReader<DamageEvent>,
    mut enemy_query: Query<&GlobalTransform>,
    // mut enemy_query: Query<&GlobalTransform, With<Enemy>>,
) {
    for event in event_reader.read() {
        if !event.apply {
            continue;
        }
        if let Ok(transform) = enemy_query.get_mut(event.target) {
            // pop text
            let mut pop_transform = transform.compute_transform();
            pop_transform.translation.z += 100.0;
            commands.spawn((
                Hud,
                popup_text(
                    &asset_server,
                    event.context.damage.to_string(),
                    pop_transform,
                    TextColor(Color::srgb(0.8, 0.1, 0.1)),
                ),
            ));
        }
    }
}

fn update_debug_texts(
    mut query: Query<&mut Text, With<DebugText>>,
    diagnostics: Res<DiagnosticsStore>,
    enemy_query: Query<(), With<Enemy>>,
    bullet_query: Query<(), With<Bullet>>,
    player_query: Single<&Health, With<Player>>,
) {
    let Ok(mut text) = query.get_single_mut() else {
        return;
    };

    let num_enemies = enemy_query.iter().count();
    let num_bullets = bullet_query.iter().count();
    let player_health = player_query.current();
    if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(value) = fps.smoothed() {
            **text = format!("Fps: {value:.2}\nEnemies: {num_enemies}\nBullets: {num_bullets}\nPlayer Health: {player_health:.0}");
        }
    }
}