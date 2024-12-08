use crate::gui::*;
use crate::state::GameState;
use bevy::prelude::*;
use bevy_button_released_plugin::*;

#[derive(Component, Default)]
pub struct MainMenu;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::MainMenu), setup_main_menu)
            .add_systems(OnExit(GameState::MainMenu), despawn_main_menu);
    }
}

fn setup_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((MainMenu, Camera2d));
    commands
        .spawn((
            MainMenu,
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
                        text(&asset_server, "2D Shooter", 100.0),
                        // box_shadow(),
                        BackgroundColor::from(Color::BLACK.with_alpha(0.6)),
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
                        .with_child(text(&asset_server, "Play", 50.0));
                    parent
                        .spawn((button(), button_node.clone()))
                        .observe(on_quit)
                        .with_child(text(&asset_server, "Quit", 50.0));
                });
        });
}

fn despawn_main_menu(mut commands: Commands, main_menu_query: Query<Entity, With<MainMenu>>) {
    for main_menu in main_menu_query.iter() {
        commands.entity(main_menu).despawn_recursive();
    }
}

fn on_start(_trigger: Trigger<OnButtonReleased>, mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::GameInit);
}

fn on_quit(_trigger: Trigger<OnButtonReleased>, mut exit: EventWriter<AppExit>) {
    exit.send(AppExit::Success);
}
