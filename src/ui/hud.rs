use crate::state::*;
use crate::ui::damage_popup::on_enemy_damaged;
use crate::ui::debug_panel::*;
use crate::ui::player_health_bar::*;
use crate::world::damage::*;
use crate::world::in_game::InGame;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use std::time::Duration;

#[derive(Component, Default)]
#[require(InGame)]
pub(crate) struct Hud;

#[derive(Default)]
pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FrameTimeDiagnosticsPlugin)
            .add_systems(
                OnEnter(GameState::GameInit),
                (spawn_debug_panel, spawn_player_health_bar),
            )
            .add_systems(OnEnter(GameState::InGame), show_hud)
            .add_systems(OnExit(GameState::InGame), hide_hud)
            .add_systems(Update, on_enemy_damaged.in_set(DamagePhase::After))
            .add_systems(
                Update,
                (
                    update_player_health_bar,
                    update_debug_texts.run_if(on_timer(Duration::from_secs_f32(0.2))),
                )
                    .run_if(in_state(GameState::InGame).or(in_state(GameState::GameOver))),
            );
    }
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
