use crate::resource::GlobalFont;
use crate::state::{AppState, GameState};
use crate::ui::util::*;
use bevy::prelude::*;
use bevy_button_released_plugin::*;

#[derive(Component, Default)]
pub struct MainMenu;

#[derive(Default)]
pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), spawn_main_menu);
    }
}

fn spawn_main_menu(mut commands: Commands, font: Res<GlobalFont>) {
    commands
        .spawn((
            MainMenu,
            StateScoped(AppState::MainMenu),
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
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
                        text(font.handle.clone(), "2D Shooter", 100.0),
                        Node {
                            margin: UiRect::all(Val::Px(50.0)),
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
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
                        .observe(on_start)
                        .with_child(text(font.handle.clone(), "Play", 50.0));
                    parent
                        .spawn((button(), button_node.clone()))
                        .observe(on_quit)
                        .with_child(text(font.handle.clone(), "Quit", 50.0));
                });
        });
}

fn on_start(
    _trigger: Trigger<OnButtonReleased>,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    next_app_state.set(AppState::InGame);
    next_game_state.set(GameState::GameInit);
}

fn on_quit(_trigger: Trigger<OnButtonReleased>, mut exit: EventWriter<AppExit>) {
    exit.send(AppExit::Success);
}

pub fn back_to_main_menu(
    _trigger: Trigger<OnButtonReleased>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    next_state.set(AppState::MainMenu);
}
