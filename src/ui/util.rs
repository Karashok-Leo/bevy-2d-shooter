use bevy::prelude::*;
use bevy_button_released_plugin::ButtonsReleasedPlugin;

pub const NORMAL_BUTTON_BG_COLOR: Color = Color::srgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON_BG_COLOR: Color = Color::srgb(0.4, 0.4, 0.4);
pub const PRESSED_BUTTON_BG_COLOR: Color = Color::srgb(0.6, 0.6, 0.6);

#[derive(Default)]
pub struct UIUtilPlugin;

impl Plugin for UIUtilPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ButtonsReleasedPlugin)
            .add_systems(Update, handle_button_interaction);
    }
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
        BorderColor(Color::srgb(0.9, 0.9, 0.9)),
        BackgroundColor(NORMAL_BUTTON_BG_COLOR),
    )
}

pub fn text(font: Handle<Font>, str: impl Into<String>, font_size: f32) -> impl Bundle {
    (
        Text::new(str),
        TextFont {
            font,
            font_size,
            ..default()
        },
        TextColor(Color::srgb(0.9, 0.9, 0.9)),
    )
}

pub fn text_bg(font: Handle<Font>, str: impl Into<String>, font_size: f32) -> impl Bundle {
    (
        Text::new(str),
        TextFont {
            font,
            font_size,
            ..default()
        },
        TextColor(Color::srgb(0.2, 0.2, 0.2)),
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
