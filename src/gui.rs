use bevy::prelude::*;
use bevy_button_released_plugin::ButtonsReleasedPlugin;

pub struct GuiPlugin;

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ButtonsReleasedPlugin)
            .add_systems(Update, handle_button_interaction);
    }
}

const BORDER_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
const FONT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
const NORMAL_BUTTON_BG: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON_BG: Color = Color::srgb(0.4, 0.4, 0.4);
const PRESSED_BUTTON_BG: Color = Color::srgb(0.6, 0.6, 0.6);

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
        BackgroundColor(NORMAL_BUTTON_BG),
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

                *color = PRESSED_BUTTON_BG.into();
            }
            Interaction::Hovered => {
                node.left = Val::Auto;
                node.bottom = Val::Auto;

                *color = HOVERED_BUTTON_BG.into();
            }
            Interaction::None => {
                node.left = Val::Auto;
                node.bottom = Val::Auto;

                *color = NORMAL_BUTTON_BG.into();
            }
        }
    }
}
