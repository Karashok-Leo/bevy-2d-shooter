use crate::state::AppState;
use bevy::prelude::*;

#[derive(Resource)]
pub struct GlobalSpriteSheet(pub SpriteSheet);

#[derive(Resource)]
pub struct TileSet(pub SpriteSheet);

pub struct SpriteSheet {
    pub layout: Handle<TextureAtlasLayout>,
    pub image: Handle<Image>,
}

#[derive(Resource)]
pub struct GlobalFont {
    pub handle: Handle<Font>,
}

pub struct ResourcePlugin;

impl SpriteSheet {
    pub fn to_sprite(&self, index: usize) -> Sprite {
        Sprite::from_atlas_image(
            self.image.clone(),
            TextureAtlas {
                layout: self.layout.clone(),
                index,
            },
        )
    }

    pub fn to_image_node(&self, index: usize) -> ImageNode {
        ImageNode::from_atlas_image(
            self.image.clone(),
            TextureAtlas {
                layout: self.layout.clone(),
                index,
            },
        )
    }
}

impl Plugin for ResourcePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Loading), load_assets);
    }
}

fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut layouts_assets: ResMut<Assets<TextureAtlasLayout>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(16), 8, 8, None, None);
    let layout_handle = layouts_assets.add(layout);
    commands.insert_resource(GlobalSpriteSheet(SpriteSheet {
        layout: layout_handle.clone(),
        image: asset_server.load("sprites.png"),
    }));
    commands.insert_resource(TileSet(SpriteSheet {
        layout: layout_handle,
        image: asset_server.load("tiles.png"),
    }));
    commands.insert_resource(GlobalFont {
        handle: asset_server.load("monogram.ttf"),
    });
    next_state.set(AppState::MainMenu);
}
