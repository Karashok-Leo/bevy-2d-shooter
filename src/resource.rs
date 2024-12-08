use crate::state::GameState;
use crate::*;
use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct GlobalTextureAtlas {
    pub layout: Option<Handle<TextureAtlasLayout>>,
    pub image: Option<Handle<Image>>,
}

pub struct ResourcePlugin;

impl Plugin for ResourcePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GlobalTextureAtlas>()
            .add_systems(OnEnter(GameState::Loading), load_assets);
    }
}

fn load_assets(
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut texture_atlas: ResMut<GlobalTextureAtlas>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let layout = TextureAtlasLayout::from_grid(
        UVec2::new(TILE_W, TILE_H),
        SPRITE_SHEET_W,
        SPRITE_SHEET_H,
        None,
        None,
    );
    texture_atlas.layout = Some(texture_atlas_layouts.add(layout));
    texture_atlas.image = Some(asset_server.load(SPRITE_SHEET_PATH));
    next_state.set(GameState::MainMenu);
}
