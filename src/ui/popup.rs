use bevy::prelude::*;
use bevy_tweening::{lens::*, *};
use rand::Rng;
use std::time::Duration;

#[derive(Default)]
pub struct PopupPlugin;

impl Plugin for PopupPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TweeningPlugin).add_systems(Update, on_fade);
    }
}

pub fn popup_text(
    asset_server: &Res<AssetServer>,
    text: String,
    transform: Transform,
    text_color: TextColor,
) -> impl Bundle {
    (
        transform,
        pop_tween(
            Duration::from_secs_f32(0.0),
            Duration::from_secs_f32(0.5),
            &transform,
        ),
        fade_tween(
            Duration::from_secs_f32(0.5),
            Duration::from_secs_f32(0.3),
            &text_color,
        ),
        Text2d(text),
        text_color,
        TextFont {
            font: asset_server.load("monogram.ttf"),
            font_size: 20.0,
            ..default()
        },
    )
}

pub fn pop_tween(
    delay: Duration,
    duration: Duration,
    transform: &Transform,
) -> Animator<Transform> {
    let mut rng = rand::thread_rng();
    let translation = Tween::new(
        EaseFunction::QuarticOut,
        duration,
        TransformPositionLens {
            start: transform.translation,
            end: transform.translation
                + Vec3::new(
                rng.gen_range(-5.0..5.0),
                rng.gen_range(-5.0..5.0) + 15.0,
                0.0,
            ),
        },
    );
    let scale = Tween::new(
        EaseFunction::QuarticOut,
        duration,
        TransformScaleLens {
            start: Vec3::splat(0.01),
            end: Vec3::ONE,
        },
    );
    if delay.is_zero() {
        let tracks = Tracks::new([translation, scale]);
        Animator::new(tracks)
    } else {
        let tracks = Delay::new(delay).then(Tracks::new([translation, scale]));
        Animator::new(tracks)
    }
}

const FADE_DATA: u64 = 66;

pub fn fade_tween(
    delay: Duration,
    duration: Duration,
    text_color: &TextColor,
) -> Animator<TextColor> {
    let fade = Tween::new(
        EaseFunction::QuinticIn,
        duration,
        TextColorLens {
            start: text_color.0,
            end: text_color.0.with_alpha(0.0),
        },
    )
        .with_completed_event(FADE_DATA);
    Animator::new(Delay::new(delay).then(fade))
}

fn on_fade(mut commands: Commands, mut event_reader: EventReader<TweenCompleted>) {
    for event in event_reader.read() {
        if event.user_data != FADE_DATA {
            continue;
        }
        commands.entity(event.entity).despawn_recursive();
    }
}
