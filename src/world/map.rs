use crate::config::GameConfig;
use crate::sprite_order::SpriteOrder;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

fn startup(mut commands: Commands, asset_server: Res<AssetServer>, config: Res<GameConfig>) {
    let texture_handle: Handle<Image> = asset_server.load("tiles.png");
    let tile_size = config.basic.tile_size;

    let map_size = TilemapSize { x: 320, y: 320 };

    let mut parent_commands = commands.spawn_empty();
    let map_parent_entity = parent_commands.id();

    let mut tile_storage = TileStorage::empty(map_size);

    for x in 0..map_size.x {
        for y in 0..map_size.y {
            let tile_pos = TilePos { x, y };
            let tile_entity = parent_commands
                .with_child(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(map_parent_entity),
                    // texture_index:
                    ..Default::default()
                })
                .id();
            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    let map_tile_size = TilemapTileSize::new(tile_size, tile_size);
    let grid_size = map_tile_size.into();
    let map_type = TilemapType::default();

    commands.entity(map_parent_entity).insert(TilemapBundle {
        grid_size,
        map_type,
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(texture_handle),
        tile_size: map_tile_size,
        transform: get_tilemap_center_transform(
            &map_size,
            &grid_size,
            &map_type,
            SpriteOrder::MAP.z_index(),
        ),
        ..Default::default()
    });
}
