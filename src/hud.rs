use crate::bullet::Bullet;
use crate::damage::*;
use crate::enemy::Enemy;
use crate::gui::*;
use crate::in_game::InGame;
use crate::player::Player;
use crate::resource::GlobalTextureAtlas;
use crate::state::*;
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use bevy::ui::widget::NodeImageMode;
use std::time::Duration;

#[derive(Component, Default)]
#[require(InGame)]
struct Hud;

#[derive(Component, Default)]
struct DebugText;

#[derive(Component, Default)]
struct Bar;

const BAR_WIDTH: f32 = 400.0;

#[derive(Component, Default)]
struct BarWidth(f32);

#[derive(Component, Default)]
struct BarBackground;

#[derive(Component, Default)]
struct BarForeground;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FrameTimeDiagnosticsPlugin)
            .add_systems(OnEnter(GameState::GameInit), spawn_hud)
            .add_systems(OnEnter(GameState::InGame), show_hud)
            .add_systems(OnExit(GameState::InGame), hide_hud)
            .add_systems(Update, on_enemy_damaged.in_set(DamagePhase::After))
            .add_systems(
                Update,
                (update_bar_width, update_bar_data).run_if(in_state(GameState::InGame)),
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
    texture_atlas: Res<GlobalTextureAtlas>,
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

    let slicer = TextureSlicer {
        border: BorderRect::square(2.0),
        center_scale_mode: SliceScaleMode::Stretch,
        sides_scale_mode: SliceScaleMode::Stretch,
        max_corner_scale: 10.0,
    };
    commands
        .spawn((
            Hud,
            Visibility::Hidden,
            Node {
                right: Val::Px(10.0),
                top: Val::Px(10.0),
                position_type: PositionType::Absolute,
                ..default()
            },
        ))
        .with_children(|parent| {
            let bar_node = Node {
                width: Val::Px(BAR_WIDTH),
                height: Val::Px(40.0),
                align_items: AlignItems::Start,
                justify_content: JustifyContent::Start,
                ..default()
            };
            let outer_bar_atlas = TextureAtlas {
                layout: texture_atlas.layout.clone().unwrap(),
                index: 62,
            };
            parent
                .spawn((
                    bar_node.clone(),
                    ImageNode::from_atlas_image(
                        texture_atlas.image.clone().unwrap(),
                        outer_bar_atlas,
                    )
                    .with_mode(NodeImageMode::Sliced(slicer.clone())),
                    Bar,
                ))
                .with_children(|parent| {
                    let inner_bar_atlas = TextureAtlas {
                        layout: texture_atlas.layout.clone().unwrap(),
                        index: 63,
                    };
                    parent
                        .spawn((
                            bar_node.clone(),
                            ImageNode::from_atlas_image(
                                texture_atlas.image.clone().unwrap(),
                                inner_bar_atlas.clone(),
                            )
                            .with_color(Color::srgb(0.8, 0.2, 0.2).with_alpha(0.6))
                            .with_mode(NodeImageMode::Sliced(slicer.clone())),
                            BarWidth(BAR_WIDTH),
                            BarBackground,
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                bar_node,
                                ImageNode::from_atlas_image(
                                    texture_atlas.image.clone().unwrap(),
                                    inner_bar_atlas,
                                )
                                .with_mode(NodeImageMode::Sliced(slicer.clone())),
                                BarWidth(BAR_WIDTH),
                                BarForeground,
                            ));
                        });
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

fn update_bar_width(mut bar_width: Query<(&mut Node, &BarWidth)>) {
    for (mut node, width) in bar_width.iter_mut() {
        node.width = Val::Px(width.0);
    }
}

const GRADUAL_CHANGE_SPEED: f32 = 0.005;

fn update_bar_data(
    mut foreground_query: Single<&mut BarWidth, With<BarForeground>>,
    mut background_query: Single<&mut BarWidth, (With<BarBackground>, Without<BarForeground>)>,
    health: Single<&Health, With<Player>>,
) {
    let health_ratio = health.current() / health.max();
    let target_width = BAR_WIDTH * health_ratio;

    // heal - foreground changes gradually
    if target_width > foreground_query.0 {
        foreground_query.0 = foreground_query.0.lerp(target_width, GRADUAL_CHANGE_SPEED);
    }
    // damage - foreground changes suddenly
    else {
        foreground_query.0 = target_width;
    }

    // damage - background changes gradually
    if target_width < background_query.0 {
        background_query.0 = background_query.0.lerp(target_width, GRADUAL_CHANGE_SPEED);
    }
    // heal - background changes suddenly
    else {
        background_query.0 = target_width;
    }
}
