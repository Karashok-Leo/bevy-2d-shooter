use bevy::prelude::*;
use bevy_button_released_plugin::ButtonsReleasedPlugin;
use bevy_tweening::{lens::*, *};
use rand::Rng;
use std::time::Duration;

#[derive(Default)]
pub struct GuiPlugin;

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((ButtonsReleasedPlugin, TweeningPlugin))
            .add_systems(Update, (handle_button_interaction, fade_event));
    }
}

pub const BORDER_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
pub const FONT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
pub const FONT_BG_COLOR: Color = Color::srgb(0.2, 0.2, 0.2);
pub const NORMAL_BUTTON_BG_COLOR: Color = Color::srgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON_BG_COLOR: Color = Color::srgb(0.4, 0.4, 0.4);
pub const PRESSED_BUTTON_BG_COLOR: Color = Color::srgb(0.6, 0.6, 0.6);

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

pub fn box_shadow() -> BoxShadow {
    BoxShadow {
        x_offset: Val::Percent(5.),
        y_offset: Val::Percent(5.),
        blur_radius: Val::Percent(5.),
        ..default()
    }
}

pub fn button() -> impl Bundle {
    (
        Button,
        box_shadow(),
        BorderColor(BORDER_COLOR),
        BackgroundColor(NORMAL_BUTTON_BG_COLOR),
    )
}

pub fn text(
    asset_server: &Res<AssetServer>,
    str: impl Into<String>,
    font_size: f32,
) -> impl Bundle {
    (
        Text::new(str),
        TextFont {
            font: asset_server.load("monogram.ttf"),
            font_size,
            ..default()
        },
        TextColor(FONT_COLOR),
    )
}

pub fn text_bg(
    asset_server: &Res<AssetServer>,
    str: impl Into<String>,
    font_size: f32,
) -> impl Bundle {
    (
        Text::new(str),
        TextFont {
            font: asset_server.load("monogram.ttf"),
            font_size,
            ..default()
        },
        TextColor(FONT_BG_COLOR),
    )
}

fn handle_button_interaction(
    mut interaction_query: Query<
        (&Interaction, &mut Node, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut node, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                node.left = Val::Px(5.0);
                node.bottom = Val::Px(-5.0);

                *color = PRESSED_BUTTON_BG_COLOR.into();
            }
            Interaction::Hovered => {
                node.left = Val::Auto;
                node.bottom = Val::Auto;

                *color = HOVERED_BUTTON_BG_COLOR.into();
            }
            Interaction::None => {
                node.left = Val::Auto;
                node.bottom = Val::Auto;

                *color = NORMAL_BUTTON_BG_COLOR.into();
            }
        }
    }
}

fn fade_event(mut commands: Commands, mut event_reader: EventReader<TweenCompleted>) {
    for event in event_reader.read() {
        if event.user_data != FADE_DATA {
            continue;
        }
        commands.entity(event.entity).despawn_recursive();
    }
}
