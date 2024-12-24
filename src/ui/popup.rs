use crate::ui::hud::Hud;
use crate::ui::interpolate::text_color;
use bevy::prelude::*;
use bevy_tween::bevy_time_runner::TimeRunnerEnded;
use bevy_tween::{combinator::*, prelude::*, tween::AnimationTarget};
use rand::Rng;
use std::time::Duration;

#[derive(Component, Default)]
pub struct Popup;

#[derive(Default)]
pub struct PopupPlugin;

impl Plugin for PopupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, on_fade);
    }
}

pub fn popup_text(
    commands: &mut Commands,
    font: Handle<Font>,
    text: String,
    transform: Transform,
    color: TextColor,
) {
    let mut rng = rand::thread_rng();
    let target = AnimationTarget.into_target();
    let mut transform_state = target.transform_state(transform);
    commands
        .spawn((
            Hud,
            Popup,
            AnimationTarget,
            transform.with_scale(Vec3::splat(0.1)),
            Text2d(text),
            color,
            TextFont {
                font,
                font_size: 20.0,
                ..default()
            },
        ))
        .animation()
        .insert(sequence((
            parallel((
                tween(
                    Duration::from_secs_f32(0.5),
                    EaseKind::QuarticOut,
                    transform_state.translation_by(Vec3::new(
                        rng.gen_range(-5.0..5.0),
                        rng.gen_range(-5.0..5.0) + 15.0,
                        0.0,
                    )),
                ),
                tween(
                    Duration::from_secs_f32(0.5),
                    EaseKind::QuarticOut,
                    transform_state.scale_to(Vec3::ONE),
                ),
            )),
            forward(Duration::from_secs_f32(0.5)),
            tween(
                Duration::from_secs_f32(0.3),
                EaseKind::QuarticOut,
                target.with(text_color(color.0, color.0.with_alpha(0.0))),
            ),
        )));
}

fn on_fade(
    mut commands: Commands,
    query: Query<(), With<Popup>>,
    mut event_reader: EventReader<TimeRunnerEnded>,
) {
    for event in event_reader.read() {
        if !query.contains(event.time_runner) {
            continue;
        }
        commands.entity(event.time_runner).despawn_recursive();
    }
}
