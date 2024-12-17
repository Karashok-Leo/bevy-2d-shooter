use crate::resource::GlobalFont;
use crate::ui::hud::Hud;
use crate::ui::util::text;
use crate::world::bullet::Bullet;
use crate::world::damage::Health;
use crate::world::enemy::Enemy;
use crate::world::player::Player;
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;

#[derive(Component, Default)]
pub struct DebugText;

pub fn spawn_debug_panel(mut commands: Commands, font: Res<GlobalFont>) {
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
                    parent.spawn((text(font.handle.clone(), "", 40.0), DebugText));
                });
        });
}

pub fn update_debug_texts(
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
