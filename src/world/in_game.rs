use crate::config::GameConfig;
use crate::resource::*;
use crate::sprite_order::SpriteOrder;
use crate::state::*;
use crate::world::collision::*;
use crate::world::damage::Health;
use crate::world::despawn::*;
use crate::world::gun::Gun;
use crate::world::owner::Owner;
use crate::world::player::Player;
use bevy::prelude::*;
use rand::Rng;

#[derive(Component, Default)]
pub struct InGameScoped;

#[derive(Default)]
pub struct InGamePlugin;

impl Plugin for InGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, despawn_recursive::<PostDespawn>)
            .add_systems(OnEnter(GameState::GameInit), setup_world)
            .add_systems(OnEnter(GameState::Running), enable_rigid_bodies)
            .add_systems(OnExit(GameState::Running), disable_rigid_bodies)
            .add_systems(OnExit(AppState::InGame), despawn_recursive::<InGameScoped>)
            .add_systems(OnExit(GameState::GameOver), despawn_recursive::<InGameScoped>)
            .add_systems(Update, check_game_over.run_if(in_state(GameState::Running)));
    }
}

fn setup_world(
    mut commands: Commands,
    texture_atlas: Res<GlobalTextureAtlas>,
    mut next_state: ResMut<NextState<GameState>>,
    config: Res<GameConfig>,
) {
    let mut player_commands = commands.spawn(Player::new(&texture_atlas, &config));
    player_commands.with_child((
        Gun::new(&texture_atlas, &config),
        Owner(player_commands.id()),
    ));
    spawn_world_decorations(&mut commands, &texture_atlas, &config);
    next_state.set(GameState::Running);
}

fn spawn_world_decorations(
    commands: &mut Commands,
    texture_atlas: &Res<GlobalTextureAtlas>,
    config: &Res<GameConfig>,
) {
    let mut rng = rand::thread_rng();
    for _ in 0..config.world.num_world_decorations {
        let x = rng.gen_range(-config.world.world_width..config.world.world_width);
        let y = rng.gen_range(-config.world.world_height..config.world.world_height);
        let sprite_index = rng.gen_range(24..=25);
        commands.spawn((
            InGameScoped,
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

fn check_game_over(
    mut next_state: ResMut<NextState<GameState>>,
    player: Single<&Health, With<Player>>,
) {
    if player.is_alive() {
        return;
    }
    next_state.set(GameState::GameOver);
}
