use crate::resource::GlobalFont;
use crate::state::GameState;
use crate::ui::main_menu::back_to_main_menu;
use crate::ui::util::{button, text};
use crate::world::in_game::InGameScoped;
use bevy::prelude::*;
use bevy_button_released_plugin::OnButtonReleased;

#[derive(Default)]
pub struct PausePlugin;

impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                esc_pause.run_if(in_state(GameState::Running)),
                esc_continue.run_if(in_state(GameState::Paused)),
            ),
        )
        .add_systems(OnEnter(GameState::Paused), spawn_pause);
    }
}

fn esc_pause(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::Paused);
    }
}

fn esc_continue(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::Running);
    }
}

fn button_continue(
    _trigger: Trigger<OnButtonReleased>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    next_state.set(GameState::Running);
}

fn spawn_pause(mut commands: Commands, font: Res<GlobalFont>) {
    commands
        .spawn((
            InGameScoped,
            StateScoped(GameState::Paused),
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor::from(Color::BLACK.with_alpha(0.9)),
        ))
        .with_children(|parent| {
            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        text(font.handle.clone(), "Game Paused", 100.0),
                        Node {
                            margin: UiRect::all(Val::Px(50.0)),
                            ..default()
                        },
                    ));
                    let button_node = Node {
                        width: Val::Px(150.0),
                        height: Val::Px(80.0),
                        margin: UiRect::all(Val::Px(20.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    };
                    parent
                        .spawn((button(), button_node.clone()))
                        .observe(button_continue)
                        .with_child(text(font.handle.clone(), "Continue", 50.0));
                    parent
                        .spawn((button(), button_node.clone()))
                        .observe(back_to_main_menu)
                        .with_child(text(font.handle.clone(), "Back", 50.0));
                });
        });
}
