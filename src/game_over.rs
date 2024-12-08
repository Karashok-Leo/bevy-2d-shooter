use crate::gui::*;
use crate::state::GameState;
use bevy::prelude::*;
use bevy_button_released_plugin::OnButtonReleased;

#[derive(Component, Default)]
pub struct GameOver;

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::GameOver), game_over)
            .add_systems(OnExit(GameState::GameOver), back_to_main_menu);
    }
}

fn game_over(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            GameOver,
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
                        text(&asset_server, "Game Over", 100.0),
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
                        .observe(on_back)
                        .with_child(text(&asset_server, "Back", 50.0));
                    parent
                        .spawn((button(), button_node.clone()))
                        .observe(on_restart)
                        .with_child(text(&asset_server, "Restart", 50.0));
                });
        });
}

fn back_to_main_menu(mut commands: Commands, game_over_query: Query<Entity, With<GameOver>>) {
    for game_over in game_over_query.iter() {
        commands.entity(game_over).despawn_recursive();
    }
}

fn on_back(_trigger: Trigger<OnButtonReleased>, mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::MainMenu);
}

fn on_restart(_trigger: Trigger<OnButtonReleased>, mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::GameInit);
}
