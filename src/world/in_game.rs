use crate::camera::SmoothCamera;
use crate::resource::*;
use crate::state::GameState;
use crate::world::damage::Health;
use crate::world::gun::Gun;
use crate::world::player::Player;
use crate::*;
use bevy::prelude::*;
use rand::Rng;

#[derive(Component, Default)]
pub struct InGame;

#[derive(Default)]
pub struct InGamePlugin;

impl Plugin for InGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::GameInit), setup_world)
            .add_systems(OnExit(GameState::GameOver), despawn_in_game)
            .add_systems(Update, check_game_over.run_if(in_state(GameState::InGame)));
    }
}

fn setup_world(
    mut commands: Commands,
    texture_atlas: Res<GlobalTextureAtlas>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    commands.spawn(SmoothCamera::new());
    commands
        .spawn(Player::new(&texture_atlas))
        .with_child(Gun::new(&texture_atlas));
    spawn_world_decorations(&mut commands, &texture_atlas);
    next_state.set(GameState::InGame);
}

fn spawn_world_decorations(commands: &mut Commands, texture_atlas: &Res<GlobalTextureAtlas>) {
    let mut rng = rand::thread_rng();
    for _ in 0..NUM_WORLD_DECORATIONS {
        let x = rng.gen_range(-WORLD_W..WORLD_W);
        let y = rng.gen_range(-WORLD_H..WORLD_H);
        let sprite_index = rng.gen_range(24..=25);
        commands.spawn((
            InGame,
            Sprite::from_atlas_image(
                texture_atlas.image.clone().unwrap(),
                TextureAtlas {
                    layout: texture_atlas.layout.clone().unwrap(),
                    index: sprite_index,
                },
            ),
            Transform::from_xyz(x, y, SpriteOrder::GRASS.z_index()),
        ));
    }
}

fn despawn_in_game(mut commands: Commands, in_game_query: Query<Entity, With<InGame>>) {
    for in_game in in_game_query.iter() {
        commands.entity(in_game).despawn_recursive();
    }
}

fn check_game_over(
    mut next_state: ResMut<NextState<GameState>>,
    player: Single<&Health, With<Player>>,
) {
    if player.is_alive() {
        return;
    }
    next_state.set(GameState::GameOver);
}
