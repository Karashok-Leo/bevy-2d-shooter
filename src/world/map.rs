use crate::config::{GameConfig, MapConfig};
use crate::resource::TileSet;
use crate::sprite_order::SpriteOrder;
use crate::state::GameState;
use crate::world::in_game::InGameScoped;
use avian2d::math::PI;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use noise::{NoiseFn, Perlin};
use rand::Rng;

#[derive(Default)]
pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TilemapPlugin)
            .add_systems(OnEnter(GameState::GameInit), spawn_map);
    }
}

pub fn spawn_map(mut commands: Commands, sheet: Res<TileSet>, config: Res<GameConfig>) {
    let tile_size = config.basic.tile_size;

    let mut parent_commands = commands.spawn(InGameScoped);
    let map_parent_entity = parent_commands.id();

    let map_size = TilemapSize {
        x: config.map.map_w,
        y: config.map.map_h,
    };
    let mut tile_storage = TileStorage::empty(map_size);

    let map_data = get_map(map_size.x, map_size.y, &config.map);

    parent_commands.with_children(|parent| {
        for x in 0..map_size.x {
            for y in 0..map_size.y {
                let tile_pos = TilePos { x, y };
                let tile_entity = parent
                    .spawn(TileBundle {
                        position: tile_pos,
                        tilemap_id: TilemapId(map_parent_entity),
                        texture_index: TileTextureIndex(map_data[x as usize][y as usize]),
                        ..Default::default()
                    })
                    .id();
                tile_storage.set(&tile_pos, tile_entity);
            }
        }
    });

    let map_tile_size = TilemapTileSize::new(tile_size, tile_size);
    let grid_size = map_tile_size.into();
    let map_type = TilemapType::Square;

    commands.entity(map_parent_entity).insert(TilemapBundle {
        grid_size,
        map_type,
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(sheet.0.image.clone()),
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

fn get_map(width: u32, height: u32, map_config: &MapConfig) -> Vec<Vec<u32>> {
    let mut rng = rand::thread_rng();
    let seed = rng.gen();
    let perlin = Perlin::new(seed);

    let mut min = f32::MAX;
    let mut max = f32::MIN;

    let mut map = Vec::with_capacity(width as usize);
    for x in 0..width {
        let mut row = Vec::with_capacity(height as usize);
        for y in 0..height {
            let value: u32 = {
                let heightmap = noise(&perlin, map_config.scale, x, y, width / 2, height / 2);

                min = min.min(heightmap);
                max = max.max(heightmap);

                if heightmap > map_config.grass_height {
                    0 // grass
                } else if heightmap > map_config.sand_height {
                    1 // sand
                } else {
                    2 // water
                }
            };

            row.push(value);
        }
        map.push(row);
    }

    println!("min: {}, max: {}", min, max);
    map
}

fn noise(perlin: &Perlin, scale: f32, x: u32, y: u32, a: u32, b: u32) -> f32 {
    let (x, y, a, b) = (x as f32, y as f32, a as f32, b as f32);
    let (point_x, point_y) = (x - a, y - b);
    let d = (point_x * point_x) / (a * a) + (point_y * point_y) / (b * b);
    if d > 1. {
        return -1.0;
    }
    let h = (PI / 2.0 * d.sqrt()).cos();

    perlin.get([x as f64 / scale as f64, y as f64 / scale as f64]) as f32 + h
}
