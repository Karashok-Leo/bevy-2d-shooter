use crate::resource::GlobalFont;
use crate::state::*;
use crate::ui::main_menu::back_to_main_menu;
use crate::ui::util::*;
use crate::world::in_game::InGameScoped;
use bevy::prelude::*;
use bevy_button_released_plugin::OnButtonReleased;

#[derive(Component, Default)]
#[require(InGameScoped)]
pub struct GameOver;

#[derive(Default)]
pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::GameOver), spawn_game_over);
    }
}

fn spawn_game_over(mut commands: Commands, font: Res<GlobalFont>) {
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
                        text(font.handle.clone(), "Game Over", 100.0),
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
                        .observe(back_to_main_menu)
                        .with_child(text(font.handle.clone(), "Back", 50.0));
                    parent
                        .spawn((button(), button_node.clone()))
                        .observe(on_restart)
                        .with_child(text(font.handle.clone(), "Restart", 50.0));
                });
        });
}

fn on_restart(_trigger: Trigger<OnButtonReleased>, mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::GameInit);
}
